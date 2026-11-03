//! Build script for `stp-sys`.
//!
//! With the (default) `vendor` feature enabled this script obtains the STP
//! sources, configures and builds a **static** build of STP's 3.x library
//! through the `cmake` crate, merges every archive the build needs into a
//! single `libstp_sys.a`, renames the symbols of the bundled SAT backends
//! (CaDiCaL, CryptoMiniSat, MiniSat and Kissat) so they cannot collide with
//! another copy of the same library in the link, and emits the link
//! instructions Cargo needs. Without `vendor`, it instead links against an STP
//! 3.x installation that is already present on the system.
//!
//! Environment variables understood by the vendored path:
//!
//! * `STP_SYS_SOURCE_DIR` -- use this STP checkout instead of downloading one.
//! * `STP_SYS_BUILD_DIR`  -- use this CMake build directory. When it is named
//!   `build`, its parent is used as the CMake prefix and the directory itself
//!   becomes the build tree; otherwise the build tree is `<dir>/build`.
//! * `STP_SYS_STP_REV`    -- git revision of `stp/stp` to download (defaults to
//!   the revision this crate was tested against).
//! * `STP_SYS_DEPS_DIR`   -- shared install prefix for STP's dependencies.
//!
//! The system path understands:
//!
//! * `STP_SYS_PREFIX`     -- installation prefix holding `lib/` (and,
//!   optionally, `include/`).
//! * `STP_SYS_LIB_DIR`    -- directory holding the STP libraries.
//! * `STP_SYS_STATIC`     -- link `libstp`-style static archives from
//!   `STP_SYS_LIB_DIR` instead of shared libraries.

use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// Upstream repository used for vendored builds.
const STP_REPO: &str = "https://github.com/stp/stp.git";
/// Tarball endpoint for the pinned revision (used when `git` is unavailable).
const STP_TARBALL: &str = "https://codeload.github.com/stp/stp/tar.gz";
/// The revision of `stp/stp` the vendored build is pinned to. Bump this
/// together with the bindings whenever STP's C API changes.
const STP_REV: &str = "9c65fadde0b8835b418c604eda96a30042e2e6bf";
/// Prefixes applied to the symbols of the vendored SAT backends before their
/// archives are merged into `libstp_sys.a`.
///
/// Each of these libraries can also appear in the final link through another
/// crate: cvc5-sys and bitwuzla-sys bundle upstream CaDiCaL, and z3-sys
/// bundles its own SAT engine. They use the same symbol names but are built
/// from different versions -- STP's CaDiCaL is a fork with a different ABI,
/// and its CryptoMiniSat and MiniSat are separate implementations. Renaming
/// the copies bundled here keeps every solver on the library it was built
/// against.
const CADICAL_SYMBOL_PREFIX: &str = "stp_cadical_";
const CRYPTOMINISAT_SYMBOL_PREFIX: &str = "stp_cryptominisat_";
const MINISAT_SYMBOL_PREFIX: &str = "stp_minisat_";
const KISSAT_SYMBOL_PREFIX: &str = "stp_kissat_";

fn main() {
	println!("cargo:rerun-if-changed=build.rs");
	for var in [
		"STP_SYS_SOURCE_DIR",
		"STP_SYS_BUILD_DIR",
		"STP_SYS_STP_REV",
		"STP_SYS_DEPS_DIR",
		"STP_SYS_PREFIX",
		"STP_SYS_LIB_DIR",
		"STP_SYS_STATIC",
	] {
		println!("cargo:rerun-if-env-changed={var}");
	}

	if env::var_os("CARGO_FEATURE_VENDOR").is_some() {
		build_vendored();
	} else {
		link_system();
	}
}

// ---------------------------------------------------------------------------
// Vendored build
// ---------------------------------------------------------------------------

fn build_vendored() {
	let out_dir = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR is set by Cargo"));
	let target_dir = target_dir(&out_dir);
	let work_dir = target_dir.join("stp-sys");
	fs::create_dir_all(&work_dir).expect("create stp-sys work directory");

	let source_dir = match env::var_os("STP_SYS_SOURCE_DIR") {
		Some(dir) => PathBuf::from(dir),
		None => {
			let dir = work_dir.join("src");
			// Prefer the source tree recorded by an existing CMake cache: it
			// lets a populated build directory be reused even if the checkout
			// lives somewhere other than our default `src`.
			if let Some(cached) = cached_source(&work_dir.join("build")) {
				cached
			} else {
				ensure_source(&dir);
				dir
			}
		}
	};

	let (cmake_prefix, build_tree) = cmake_dirs(&work_dir);

	let shared_deps = env::var_os("STP_SYS_DEPS_DIR");
	let shared_cms = shared_deps.as_ref().is_some_and(|deps| {
		let deps = Path::new(deps);
		["lib", "lib64"]
			.iter()
			.any(|lib| deps.join(lib).join("cmake").join("cryptominisat5").is_dir())
	});

	let mut config = cmake::Config::new(&source_dir);
	config
		.out_dir(&cmake_prefix)
		// Static `libstp.a`. STP documents `STATICCOMPILE=ON` as the way to
		// ask for a static build; it forces `BUILD_SHARED_LIBS=OFF` itself.
		.define("CMAKE_BUILD_TYPE", "Release")
		.define("STATICCOMPILE", "ON")
		// We only need the 3.x library; do not build the command-line tools,
		// the 2.x compatibility library, the tests or the manpage.
		.define("BUILD_EXECUTABLES", "OFF")
		.define("ENABLE_TESTING", "OFF")
		.define("ENABLE_PYTHON_API", "OFF")
		.define("BUILD_MANPAGE", "OFF")
		// Fetch ABC, CaDiCaL, CryptoMiniSat, MiniSat, Kissat, SymFPU, LibBF
		// and IMath as needed. CryptoMiniSat needs GMP (through pkg-config) and
		// MiniSat needs zlib; STP's pinned CryptoMiniSat is built with
		// NOCADICAL, so it does not bring a second CaDiCaL of its own.
		.define("ENABLE_AUTO_DOWNLOAD", "ON")
		.define("ENABLE_ASSERTIONS", "OFF")
		// Use the C library allocator: it removes STP's malloc override so the
		// resulting archive composes cleanly with the Rust runtime.
		.define("STP_ALLOCATOR", "system")
		// Build every SAT backend STP supports. The tests in
		// `tests/sat_backends.rs` verify that each one is present at runtime.
		.define("USE_CADICAL", "ON")
		.define("USE_CRYPTOMINISAT", "ON")
		.define("USE_MINISAT", "ON")
		.define("USE_KISSAT", "ON")
		// The `stp` target is the 3.x C API library.
		.build_target("stp");

	// Always use STP's pinned CryptoMiniSat rather than an installed CMake
	// package, unless a shared dependency prefix already contains one. That
	// keeps the build static and reproducible, and the pin is built with
	// NOCADICAL so it brings no second CaDiCaL. (A package export names the bare
	// target `cryptominisat5`, which is not a static archive the merged build
	// can link.)
	if !shared_cms {
		config.define("CMAKE_DISABLE_FIND_PACKAGE_cryptominisat5", "ON");
	} else {
		config.define("CMAKE_DISABLE_FIND_PACKAGE_cryptominisat5", "OFF");
	}

	if let Some(deps) = &shared_deps {
		config.define("STP_DEP_DIR", deps.as_os_str());
		config.define("FETCHCONTENT_BASE_DIR", deps.as_os_str());
	}

	// `cmake::Config::build` runs the configure and build steps; the returned
	// path is the CMake prefix. The build tree lives at `<prefix>/build`.
	config.build();

	let stp_archive = build_tree.join("lib").join("libstp.a");
	assert!(
		stp_archive.is_file(),
		"the STP build did not produce {}",
		stp_archive.display()
	);

	let (dependencies, extra_system) = dependency_archives(&build_tree);
	let combined = merge_archives(&work_dir, &stp_archive, &dependencies);
	emit_link_flags(&combined, &extra_system);
}

/// Returns the Cargo target directory that owns `out_dir`.
///
/// `OUT_DIR` ends in `target/<profile>/build/<pkg>-<hash>/out`, so walking up
/// far enough reaches `target`. `CARGO_TARGET_DIR` wins when it is set.
fn target_dir(out_dir: &Path) -> PathBuf {
	if let Some(dir) = env::var_os("CARGO_TARGET_DIR") {
		return PathBuf::from(dir);
	}
	out_dir
		.ancestors()
		.find(|p| p.file_name() == Some("target".as_ref()))
		.map(Path::to_path_buf)
		.unwrap_or_else(|| out_dir.to_path_buf())
}

/// Returns the CMake prefix and build tree to use.
fn cmake_dirs(work_dir: &Path) -> (PathBuf, PathBuf) {
	if let Some(dir) = env::var_os("STP_SYS_BUILD_DIR") {
		let dir = PathBuf::from(dir);
		if dir.file_name() == Some(OsStr::new("build")) {
			if let Some(parent) = dir.parent().filter(|p| !p.as_os_str().is_empty()) {
				let prefix = parent.to_path_buf();
				return (prefix, dir);
			}
		}
		let tree = dir.join("build");
		return (dir, tree);
	}

	(work_dir.to_path_buf(), work_dir.join("build"))
}

/// Returns the source directory recorded by an existing CMake cache, if it
/// still exists.
fn cached_source(build_dir: &Path) -> Option<PathBuf> {
	let cache = fs::read_to_string(build_dir.join("CMakeCache.txt")).ok()?;
	for line in cache.lines() {
		if let Some(source) = line.strip_prefix("CMAKE_HOME_DIRECTORY:INTERNAL=") {
			let source = PathBuf::from(source);
			if source.join("CMakeLists.txt").is_file() {
				return Some(source);
			}
		}
	}
	None
}

/// Ensures an STP checkout exists at `dir`, downloading one if necessary.
fn ensure_source(dir: &Path) {
	if dir.join("CMakeLists.txt").is_file() {
		return;
	}
	let _ = fs::remove_dir_all(dir);
	if let Some(parent) = dir.parent() {
		fs::create_dir_all(parent).expect("create source parent directory");
	}
	let rev = env::var("STP_SYS_STP_REV").unwrap_or_else(|_| STP_REV.to_owned());

	if git_clone(dir, &rev) {
		return;
	}
	fetch_tarball(dir, &rev);
}

fn git_clone(dir: &Path, rev: &str) -> bool {
	let ok = Command::new("git")
		.args(["clone", "--quiet", "--filter=blob:none", STP_REPO])
		.arg(dir)
		.status()
		.map(|s| s.success())
		.unwrap_or(false);
	if !ok {
		return false;
	}
	Command::new("git")
		.args(["-C"])
		.arg(dir)
		.args(["checkout", "--quiet", rev])
		.status()
		.map(|s| s.success())
		.unwrap_or(false)
}

fn fetch_tarball(dir: &Path, rev: &str) {
	let tarball = dir.with_extension("tar.gz");
	let url = format!("{STP_TARBALL}/{rev}");
	run(
		Command::new("curl")
			.args(["-L", "--fail", "--retry", "3", "-o"])
			.arg(&tarball)
			.arg(&url),
		"download the STP source tarball",
	);
	fs::create_dir_all(dir).expect("create source directory");
	run(
		Command::new("tar")
			.args(["xzf"])
			.arg(&tarball)
			.arg("-C")
			.arg(dir)
			.arg("--strip-components=1"),
		"extract the STP source tarball",
	);
}

/// Reads the dependency archives from CMake's build-tree export, falling back
/// to the dependency install prefix when the export is unavailable.
fn dependency_archives(build_tree: &Path) -> (Vec<PathBuf>, Vec<String>) {
	let lib_dirs = dependency_lib_dirs(build_tree);
	let targets = build_tree.join("STPTargets.cmake");
	if let Ok(contents) = fs::read_to_string(&targets) {
		if let Some(line) = stp_link_libraries(&contents) {
			let (mut archives, mut system) = parse_link_libraries(line, build_tree, &lib_dirs);
			if !archives.is_empty() {
				archives.retain(|archive| archive.is_file());
				dedupe(&mut archives);
				// An installed CryptoMiniSat names its bare target; its only
				// non-archive dependency is GMP.
				if archives.iter().any(|archive| is_cryptominisat_archive(archive)) && !system.iter().any(|lib| lib == "gmp") {
					system.push("gmp".to_owned());
				}
				dedupe_strings(&mut system);
				return (archives, system);
			}
		}
	}

	let mut archives = Vec::new();
	if let Some(lib_dir) = lib_dirs.iter().find(|dir| dir.is_dir()) {
		if let Ok(entries) = fs::read_dir(lib_dir) {
			for entry in entries.flatten() {
				let path = entry.path();
				if path.extension() == Some(OsStr::new("a")) {
					archives.push(path);
				}
			}
		}
	}
	archives.sort();
	(archives, Vec::new())
}

/// Directories that may hold the dependency archives: the shared prefix named
/// by `STP_SYS_DEPS_DIR`, then the build tree's own install prefix.
fn dependency_lib_dirs(build_tree: &Path) -> Vec<PathBuf> {
	let mut dirs = Vec::new();
	if let Some(deps) = env::var_os("STP_SYS_DEPS_DIR") {
		let deps = PathBuf::from(deps);
		dirs.push(deps.join("lib"));
		dirs.push(deps.join("lib64"));
	}
	dirs.push(build_tree.join("deps").join("install").join("lib"));
	dirs.push(build_tree.join("deps").join("install").join("lib64"));
	dirs
}

/// Finds a static archive for a bare library name (e.g. `cryptominisat5`).
fn find_archive(lib_dirs: &[PathBuf], name: &str) -> Option<PathBuf> {
	for dir in lib_dirs {
		for file in [format!("lib{name}.a"), format!("lib{name}.lib"), format!("{name}.lib")] {
			let path = dir.join(file);
			if path.is_file() {
				return Some(path);
			}
		}
	}
	None
}

/// Returns the `INTERFACE_LINK_LIBRARIES` line belonging to the `stp` target.
fn stp_link_libraries(contents: &str) -> Option<&str> {
	let mut in_stp = false;
	for line in contents.lines() {
		if line.contains("add_library(stp STATIC IMPORTED)") || line.contains("add_library(stp SHARED IMPORTED)") {
			in_stp = true;
			continue;
		}
		if in_stp && line.contains("add_library(") {
			in_stp = false;
		}
		if in_stp && line.contains("INTERFACE_LINK_LIBRARIES") {
			return Some(line);
		}
	}
	None
}

/// Extracts `$<LINK_ONLY:...>` entries from a CMake link-interface line.
fn parse_link_libraries(line: &str, build_tree: &Path, lib_dirs: &[PathBuf]) -> (Vec<PathBuf>, Vec<String>) {
	let mut archives = Vec::new();
	let mut system = Vec::new();
	let mut rest = line;

	while let Some(start) = rest.find("$<LINK_ONLY:") {
		rest = &rest[start + "$<LINK_ONLY:".len()..];
		let Some(end) = rest.find('>') else {
			break;
		};
		let token = rest[..end].trim();
		rest = &rest[end + 1..];
		if token.is_empty() {
			continue;
		}
		if token.ends_with(".a") || token.ends_with(".lib") {
			let path = Path::new(token);
			archives.push(if path.is_absolute() {
				path.to_path_buf()
			} else {
				build_tree.join(path)
			});
		} else if let Some(name) = token.strip_prefix("-l") {
			system.push(name.to_owned());
		} else if token.contains("::") {
			// A CMake target such as `Threads::Threads`; the libraries it
			// stands for are already part of the base link line.
		} else if !token.contains('/') && !token.contains('\\') {
			// A bare library name from a dependency's link interface. It may
			// be an archive in the dependency prefix (an installed
			// CryptoMiniSat exports its target that way) or a system library
			// such as `gmp`.
			if let Some(path) = find_archive(lib_dirs, token) {
				archives.push(path);
			} else {
				system.push(token.to_owned());
			}
		}
	}

	(archives, system)
}

fn dedupe(vec: &mut Vec<PathBuf>) {
	let mut seen = Vec::new();
	vec.retain(|value| {
		if seen.iter().any(|existing| existing == value) {
			false
		} else {
			seen.push(value.clone());
			true
		}
	});
}

fn dedupe_strings(vec: &mut Vec<String>) {
	let mut seen = Vec::new();
	vec.retain(|value| {
		if seen.iter().any(|existing| existing == value) {
			false
		} else {
			seen.push(value.clone());
			true
		}
	});
}

/// A vendored library whose symbols are isolated before merging.
struct VendoredLibrary {
	prefix: &'static str,
	/// Mangled-name fragments that identify the library's own symbols.
	markers: &'static [&'static str],
}

/// Whether `archive` is the CryptoMiniSat static library.
fn is_cryptominisat_archive(archive: &Path) -> bool {
	archive
		.file_name()
		.and_then(OsStr::to_str)
		.is_some_and(|name| name.to_ascii_lowercase().contains("cryptominisat"))
}

/// Identifies the vendored SAT libraries by archive name.
///
/// Order matters: `libcryptominisat5.a` contains the substring `minisat`.
fn vendored_library(archive: &Path) -> Option<VendoredLibrary> {
	let name = archive.file_name()?.to_str()?.to_ascii_lowercase();
	if name.contains("cryptominisat") {
		Some(VendoredLibrary {
			prefix: CRYPTOMINISAT_SYMBOL_PREFIX,
			markers: &["5CMSat", "6Kitten"],
		})
	} else if name.contains("cadical") {
		Some(VendoredLibrary {
			prefix: CADICAL_SYMBOL_PREFIX,
			markers: &["7CaDiCaL", "4Reap", "6Kitten"],
		})
	} else if name.contains("minisat") {
		Some(VendoredLibrary {
			prefix: MINISAT_SYMBOL_PREFIX,
			markers: &["7Minisat"],
		})
	} else if name.contains("kissat") {
		Some(VendoredLibrary {
			prefix: KISSAT_SYMBOL_PREFIX,
			markers: &[],
		})
	} else {
		None
	}
}

/// Whether `name` belongs to a vendored library rather than to the standard
/// library or compiler runtime.
///
/// Names containing one of the library's markers are always renamed: they
/// cover its C++ namespace, the standard-library instantiations over its
/// types, and the local COMDAT group signatures. The last is essential --
/// upstream CaDiCaL and STP's fork have groups with the same signature, and
/// leaving it alone makes the linker discard the renamed members.
fn is_vendored_symbol(name: &str, markers: &[&str]) -> bool {
	if name.starts_with('.')
		|| name.starts_with("DW.ref.")
		|| name.starts_with("__")
		|| name.starts_with("_GLOBAL__")
		|| name.starts_with("_Zdl")
		|| name.starts_with("_Znw")
		|| name.starts_with("_Zda")
		|| name.starts_with("_Zna")
	{
		return false;
	}
	if markers.iter().any(|marker| name.contains(marker)) {
		return true;
	}
	// Generic standard-library instantiations are shared with every other
	// library and must keep their ordinary names.
	!name.starts_with("_ZNSt")
		&& !name.starts_with("_ZNKSt")
		&& !name.starts_with("_ZSt")
		&& !name.starts_with("_ZTVSt")
		&& !name.starts_with("_ZTISt")
		&& !name.starts_with("_ZTSSt")
		&& !name.starts_with("_ZN9__gnu_cxx")
}

/// Returns the prefix for a vendor symbol that STP's own objects emit even
/// though the vendor archive does not define it -- weak RTTI objects and
/// template instantiations over the vendor's types, for instance.
///
/// STP's own `stp::*` and `stp_*` symbols are never renamed, even when their
/// mangled names mention a vendor type as a parameter.
fn stp_vendor_prefix(name: &str) -> Option<&'static str> {
	if name.starts_with('.')
		|| name.starts_with("DW.ref.")
		|| name.starts_with("__")
		|| name.starts_with("stp_")
		|| name.starts_with("_ZN3stp")
	{
		return None;
	}
	if name.contains("5CMSat") || name.starts_with("cmsat_") {
		Some(CRYPTOMINISAT_SYMBOL_PREFIX)
	} else if name.contains("7Minisat") {
		Some(MINISAT_SYMBOL_PREFIX)
	} else if name.contains("7CaDiCaL")
		|| name.contains("4Reap")
		|| name.starts_with("ccadical_")
		|| name.starts_with("cadical_")
		|| name.starts_with("ipasir_")
	{
		Some(CADICAL_SYMBOL_PREFIX)
	} else if name.starts_with("kissat_") || name == "reset_gate_hash_table" {
		Some(KISSAT_SYMBOL_PREFIX)
	} else {
		None
	}
}

/// Copies `source` to `output_name` in `work_dir`, renaming the symbols in
/// `map`.
fn rename_archive(work_dir: &Path, source: &Path, output_name: &str, map: &BTreeMap<String, String>) -> PathBuf {
	let destination = work_dir.join(output_name);
	let _ = fs::remove_file(&destination);
	if map.is_empty() {
		fs::copy(source, &destination).unwrap_or_else(|e| panic!("failed to copy {}: {e}", source.display()));
		return destination;
	}

	let map_file = work_dir.join(format!("{output_name}.map"));
	let mut contents = String::new();
	for (old, new) in map {
		contents.push_str(old);
		contents.push(' ');
		contents.push_str(new);
		contents.push('\n');
	}
	fs::write(&map_file, contents).expect("write a symbol map");

	let renamed = work_dir.join(format!("{output_name}.renamed"));
	let _ = fs::remove_file(&renamed);
	let objcopy = env::var("OBJCOPY").unwrap_or_else(|_| "objcopy".to_owned());
	run(
		Command::new(&objcopy)
			.arg("--redefine-syms")
			.arg(&map_file)
			.arg(source)
			.arg(&renamed),
		&format!("rename the symbols of {} with {objcopy}", source.display()),
	);
	fs::rename(&renamed, &destination).expect("replace the renamed archive");
	destination
}

/// Isolates one vendored archive and returns its new path and symbol map.
fn isolate_vendored_archive(
	work_dir: &Path,
	archive: &Path,
	library: &VendoredLibrary,
) -> (PathBuf, BTreeMap<String, String>) {
	let mut map = BTreeMap::new();
	for name in defined_symbols(archive) {
		if is_vendored_symbol(&name, library.markers) {
			map.insert(name.clone(), format!("{}{}", library.prefix, name));
		}
	}
	let file_name = archive.file_name().and_then(OsStr::to_str).unwrap_or("library.a");
	let output_name = format!("{}{}", library.prefix, file_name);
	(rename_archive(work_dir, archive, &output_name, &map), map)
}

/// Returns the symbols `nm` reports for `archive`.
fn symbols(archive: &Path, defined_only: bool) -> Vec<String> {
	let nm = env::var("NM").unwrap_or_else(|_| "nm".to_owned());
	let mut command = Command::new(&nm);
	if defined_only {
		command.arg("--defined-only");
	}
	let output = command
		.arg("--format=posix")
		.arg(archive)
		.stderr(Stdio::null())
		.output()
		.unwrap_or_else(|e| panic!("failed to run {nm}: {e}"));
	assert!(output.status.success(), "{nm} failed on {}", archive.display());

	let mut names = Vec::new();
	for line in String::from_utf8_lossy(&output.stdout).lines() {
		let mut fields = line.split_whitespace();
		// POSIX format is `<name> <type> <value> [<size>]`; archive member
		// headers have no type field and are skipped here.
		if let (Some(name), Some(kind)) = (fields.next(), fields.next()) {
			if kind.len() == 1 && kind.as_bytes()[0].is_ascii_alphabetic() {
				names.push(name.to_owned());
			}
		}
	}
	names
}

/// Returns the symbols `nm` reports as defined in `archive`.
fn defined_symbols(archive: &Path) -> Vec<String> {
	symbols(archive, true)
}

/// Returns every symbol `nm` reports for `archive`, defined or undefined.
fn all_symbols(archive: &Path) -> Vec<String> {
	symbols(archive, false)
}

/// Merges STP and its dependencies into a single static archive, isolates the
/// vendored SAT libraries' symbols, and strips the archive's debug info.
fn merge_archives(work_dir: &Path, stp_archive: &Path, dependencies: &[PathBuf]) -> PathBuf {
	let mut inputs = Vec::with_capacity(dependencies.len() + 1);
	let mut library_maps = Vec::new();
	for archive in dependencies {
		if let Some(library) = vendored_library(archive) {
			let (isolated, map) = isolate_vendored_archive(work_dir, archive, &library);
			inputs.push(isolated);
			library_maps.push(map);
		} else {
			inputs.push(archive.clone());
		}
	}

	// STP's own objects reference those libraries, and their headers also emit
	// weak inline copies into STP's objects; apply the same renames to
	// `libstp.a` so both sides agree.
	let stp_symbols: BTreeSet<String> = all_symbols(stp_archive).into_iter().collect();
	let mut stp_map = BTreeMap::new();
	for map in &library_maps {
		for (old, new) in map {
			if !stp_symbols.contains(old) {
				continue;
			}
			match stp_map.get(old) {
				Some(existing) => assert!(
					existing == new,
					"vendored libraries disagree on how to rename the symbol {old}"
				),
				None => {
					stp_map.insert(old.clone(), new.clone());
				}
			}
		}
	}
	// Some weak RTTI and template symbols appear only in STP's objects, not in
	// the vendor archives, so they are not in any map above.
	for name in &stp_symbols {
		if stp_map.contains_key(name) {
			continue;
		}
		if let Some(prefix) = stp_vendor_prefix(name) {
			stp_map.insert(name.clone(), format!("{prefix}{name}"));
		}
	}
	let stp_isolated = rename_archive(work_dir, stp_archive, "libstp_sys.stp.a", &stp_map);

	let combined = work_dir.join("libstp_sys.a");
	let _ = fs::remove_file(&combined);

	let mut script = String::from("create ");
	script.push_str(&combined.to_string_lossy());
	script.push('\n');
	for archive in std::iter::once(&stp_isolated).chain(inputs.iter()) {
		script.push_str("addlib ");
		script.push_str(&archive.to_string_lossy());
		script.push('\n');
	}
	script.push_str("save\nend\n");

	let ar = env::var("AR").unwrap_or_else(|_| "ar".to_owned());
	let mut child = Command::new(&ar)
		.arg("-M")
		.stdin(Stdio::piped())
		.spawn()
		.unwrap_or_else(|e| panic!("failed to run {ar}: {e}"));
	child
		.stdin
		.as_mut()
		.expect("piped stdin")
		.write_all(script.as_bytes())
		.expect("write ar script");
	let status = child.wait().expect("wait for ar");
	assert!(status.success(), "{ar} failed to merge the STP archives");

	// Debug info dwarfs everything else and is useless to the Rust linker.
	if let Ok(strip) = env::var("STRIP") {
		let _ = Command::new(strip)
			.args(["--strip-debug", "--strip-unneeded"])
			.arg(&combined)
			.status();
	} else if which("strip") {
		let _ = Command::new("strip").arg("--strip-debug").arg(&combined).status();
	}

	combined
}

// ---------------------------------------------------------------------------
// System build
// ---------------------------------------------------------------------------

fn link_system() {
	let prefix = env::var_os("STP_SYS_PREFIX").map(PathBuf::from);
	let lib_dir = env::var_os("STP_SYS_LIB_DIR")
		.map(PathBuf::from)
		.or_else(|| prefix.as_ref().map(|p| p.join("lib")));

	let static_link = env::var_os("STP_SYS_STATIC").is_some();
	if let Some(dir) = &lib_dir {
		println!("cargo:rustc-link-search=native={}", dir.display());
	}

	let kind = if static_link { "static" } else { "dylib" };
	println!("cargo:rustc-link-lib={kind}=stp");
	for lib in ["stdc++", "gmp", "pthread", "rt", "atomic", "dl", "m", "z"] {
		println!("cargo:rustc-link-lib=dylib={lib}");
	}
}

// ---------------------------------------------------------------------------
// Emitting link flags
// ---------------------------------------------------------------------------

fn emit_link_flags(combined: &Path, extra_system: &[String]) {
	let dir = combined.parent().expect("combined archive has a parent");
	println!("cargo:rustc-link-search=native={}", dir.display());
	println!("cargo:rustc-link-lib=static=stp_sys");
	// Deliberately no `rerun-if-changed` for `combined`: this build script
	// writes that archive on every run, so watching it would mark the crate
	// dirty again immediately and rebuild it on every `cargo check`.

	let mut system: Vec<String> = ["stdc++", "pthread", "rt", "atomic", "dl", "m", "z"]
		.iter()
		.map(|s| (*s).to_owned())
		.collect();
	for lib in extra_system {
		if !system.iter().any(|existing| existing == lib) {
			system.push(lib.clone());
		}
	}
	for lib in system {
		println!("cargo:rustc-link-lib=dylib={lib}");
	}
}

// ---------------------------------------------------------------------------
// Small helpers
// ---------------------------------------------------------------------------

fn run(cmd: &mut Command, what: &str) {
	let status = cmd.status().unwrap_or_else(|e| panic!("failed to {what}: {e}"));
	assert!(status.success(), "failed to {what}");
}

fn which(program: &str) -> bool {
	let Some(paths) = env::var_os("PATH") else {
		return false;
	};
	env::split_paths(&paths).any(|dir| {
		let candidate = dir.join(program);
		candidate.is_file()
	})
}
