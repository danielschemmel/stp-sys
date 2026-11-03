//! Runtime checks that the vendored STP build reports the expected set of SAT
//! backends.
//!
//! The tests that exercise each backend's actual model live in the library's
//! `tests` unit-test module (`src/tests.rs`).
#![cfg(feature = "vendor")]

use std::ffi::CStr;

use stp_sys::ffi;

const EXPECTED_BACKENDS: [&str; 5] = ["cadical", "cryptominisat", "minisat", "simplifying-minisat", "kissat"];

#[test]
fn vendored_build_reports_every_enabled_sat_backend() {
	let count = unsafe { ffi::stp_num_sat_backends() };
	let names: Vec<String> = (0..count)
		.map(|i| {
			let name = unsafe { ffi::stp_sat_backend_name(i) };
			assert!(!name.is_null(), "stp_sat_backend_name({i}) returned null");
			unsafe { CStr::from_ptr(name) }
				.to_str()
				.expect("backend name is UTF-8")
				.to_owned()
		})
		.collect();

	for expected in EXPECTED_BACKENDS {
		assert!(
			names.iter().any(|name| name == expected),
			"vendored STP does not report the {expected} SAT backend (reported: {names:?})"
		);
	}
}
