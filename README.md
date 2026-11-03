# stp-sys

Rust bindings for [STP](https://github.com/stp/stp), the Simple Theorem Prover:
an SMT solver for the quantifier-free theories of bit-vectors, arrays and
floating point.

***This crate is mostly AI-generated and has not undergone a major review.***

The crate exposes two layers:

- `stp_sys::ffi` -- a raw, `unsafe` translation of STP's 3.x C interface
  (`stp/stp.h`), and
- the crate root -- a small, safe wrapper ([`Context`], [`Sort`], [`Expr`])
  built on top of it.

Only the 3.x C API is used; the legacy 2.x compatibility layer is neither
built nor linked.

## Vendored static builds (default)

The `vendor` feature is enabled by default. In this configuration the build
script:

1. obtains the STP sources (downloading a pinned revision if they are not
   already present),
2. builds the 3.x `libstp.a` with the [`cmake`](https://crates.io/crates/cmake)
   crate as a **static** library (`STATICCOMPILE=ON`), plus its dependencies,
3. merges every static archive the build produces into a single
   `libstp_sys.a`, renames the symbols of the bundled SAT backends, strips
   its debug information, and
4. emits the link instructions Cargo needs.

The vendored build enables STP's **CaDiCaL**, **CryptoMiniSat**, **MiniSat**,
**simplifying-MiniSat** and **Kissat** backends. CryptoMiniSat is built with
`NOCADICAL=ON`, so it does not bring a second CaDiCaL of its own. The
library's `test` unit-test module exercises each backend by solving
`(not (fp64_eq x x))` and checking that the model is an f64 NaN; each backend
has its own `src/test/backend/<backend>.rs` file, and `tests/sat_backends.rs` verifies
that the vendored build reports the expected set of backends. Backends missing
from a build are skipped by the per-backend tests.

Each SAT library is isolated from any other copy in the final link:

- STP's **CaDiCaL** is a fork carrying STP-specific extensions. It is
  ABI-incompatible with the upstream CaDiCaL that `cvc5-sys` and
  `bitwuzla-sys` bundle, and both export the same `CaDiCaL::*`, `ccadical_*`
  and `ipasir_*` symbols.
- **CryptoMiniSat** exports `CMSat::*`, `cmsat_*` and its own bundled
  `kitten_*`/`citten_*` functions (which also exist in CaDiCaL).
- **MiniSat** exports `Minisat::*`.
- **Kissat** exports `kissat_*` (and one generic helper symbol).

The build script prefixes all of these libraries' symbols before merging them
into `libstp_sys.a`, so `stp-sys` can be linked into the same program as
`cvc5-sys`, `bitwuzla-sys`, `z3-sys`, or all of them, without duplicate-symbol
errors or one solver binding to another solver's copy.

The build needs a C++ toolchain, `pkg-config`, GMP (for CryptoMiniSat), zlib
(for MiniSat), and network access the first time it fetches STP and its
dependencies (ABC, CaDiCaL, CryptoMiniSat, MiniSat, Kissat, SymFPU, LibBF and
IMath). CMake itself is driven through the `cmake` crate rather than by
invoking the binary directly.

Useful environment variables:

| Variable              | Effect                                                        |
| --------------------- | ------------------------------------------------------------- |
| `STP_SYS_SOURCE_DIR`  | Use this STP checkout instead of downloading one.             |
| `STP_SYS_BUILD_DIR`   | Use this CMake build directory.                               |
| `STP_SYS_STP_REV`     | Git revision of `stp/stp` to download.                        |
| `STP_SYS_DEPS_DIR`    | Shared install prefix for STP's dependencies.                 |

### Symbol renaming

The vendored build is supposed to play nice with other SMT solvers (specifically: bitwuzla-sys, cvc5-sys, z3-sys), especially when all of them are vendored too.
When done naively, the symbols of transitive dependencies clash, which this crate circumvents by renaming the symbols of its own transitive dependencies where necessary.

Affected dependencies:
- CaDiCaL (also used by Bitwuzla and CVC5)
- CryptoMiniSat (exports `CMSat::*`, `cmsat_*`, and its bundled `kitten_*`/`citten_*`)
- MiniSat (exports `Minisat::*`)
- Kissat (exports `kissat_*`)

## Using a system STP

With `default-features = false` the crate links against an STP 3.x installation
already on the system instead. Point the build at it with `STP_SYS_PREFIX` or
`STP_SYS_LIB_DIR`; set `STP_SYS_STATIC` to link static archives from that
directory.

The symbol isolation described above is applied to the vendored archive only.
A static system `libstp` that bundles its own SAT libraries is used as found,
so it can still clash with `cvc5-sys`, `bitwuzla-sys` or `z3-sys`. Prefer the
vendored build (or a system STP linked against shared SAT libraries) when
combining it with another solver.

## Example

```rust
use stp_sys::{Context, Verdict};

let ctx = Context::new();

// x + 0 == x holds for every x.
let x = ctx.bv_var("x", 32);
let identity = ctx.eq(ctx.bv_add(x, ctx.bv_const(32, 0)), x);
assert_eq!(ctx.query(identity), Verdict::Valid);

// x < 10 does not; STP finds a counterexample.
let x = ctx.bv_var("x", 8);
let fits = ctx.bv_ult(x, ctx.bv_const(8, 10));
assert_eq!(ctx.query(fits), Verdict::Invalid);
let value = ctx.model_bv_u64(x).unwrap();
assert!(value >= 10);
```

## License

STP is distributed under the MIT license; these bindings are provided under the
same terms.
