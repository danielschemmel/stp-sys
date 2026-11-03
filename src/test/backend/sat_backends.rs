//! Shared helper for the vendored SAT backend unit tests.

use std::ffi::CString;

use crate::ffi;

/// Checks that `name` is compiled in, then solves `(not (fp64_eq x x))`:
/// the only way that assertion can hold is for `x` to be NaN, so the model
/// must give `x` an f64 NaN value. Backends missing from this build are
/// skipped.
pub(super) fn assert_backend_yields_nan_model(name: &str) {
	let backend = CString::new(name).expect("backend names contain no NUL");
	if !unsafe { ffi::stp_has_sat_backend(backend.as_ptr()) } {
		eprintln!("skipping the {name} SAT backend test: not present in this build");
		return;
	}

	let option_name = CString::new("sat-backend").expect("option name contains no NUL");
	let options = unsafe { ffi::stp_options_new() };
	assert!(!options.is_null(), "stp_options_new returned null");
	let status = unsafe { ffi::stp_options_set_str(options, option_name.as_ptr(), backend.as_ptr()) };
	assert_eq!(status, ffi::STP_OK, "could not select the {name} SAT backend");

	let tm = unsafe { ffi::stp_tm_new(std::ptr::null_mut()) };
	assert!(!tm.is_null(), "stp_tm_new returned null");
	let solver = unsafe { ffi::stp_solver_new(tm, options) };
	unsafe { ffi::stp_options_delete(options) };
	assert!(
		!solver.is_null(),
		"could not create a solver with the {name} SAT backend"
	);

	// A single symbol `x` of sort Float64, and the single assertion
	// `(not (fp64_eq x x))`.
	let x_name = CString::new("x").expect("symbol name contains no NUL");
	let fp64 = unsafe { ffi::stp_mk_fp64_sort(tm) };
	let x = unsafe { ffi::stp_declare(tm, x_name.as_ptr(), fp64) };
	let eq_args = [x, x];
	let x_eq_x = unsafe { ffi::stp_mk_term(tm, ffi::STP_KIND_FP_EQ, eq_args.len(), eq_args.as_ptr()) };
	let not_args = [x_eq_x];
	let assertion = unsafe { ffi::stp_mk_term(tm, ffi::STP_KIND_NOT, not_args.len(), not_args.as_ptr()) };
	assert_eq!(unsafe { ffi::stp_solver_assert(solver, assertion) }, ffi::STP_OK);

	let mut result: ffi::stp_result = unsafe { std::mem::zeroed() };
	let status = unsafe { ffi::stp_solver_check_sat(solver, &mut result) };
	assert_eq!(status, ffi::STP_OK, "the {name} SAT backend failed to answer");
	assert_eq!(
		result.kind,
		ffi::STP_SAT,
		"the {name} SAT backend should find a NaN model"
	);

	let model = unsafe { ffi::stp_solver_model(solver) };
	assert!(!model.is_null(), "the {name} SAT backend produced no model");

	let mut value: ffi::stp_float_value = unsafe { std::mem::zeroed() };
	let fp_status = unsafe { ffi::stp_model_fp(model, x, &mut value) };
	let mut double = 0.0_f64;
	let double_status = unsafe { ffi::stp_model_fp_to_double(model, x, &mut double) };
	unsafe {
		ffi::stp_model_release(model);
		ffi::stp_solver_delete(solver);
		ffi::stp_tm_release(tm);
	}

	assert_eq!(fp_status, ffi::STP_OK, "could not read the {name} model value");
	assert_eq!(
		double_status,
		ffi::STP_OK,
		"could not convert the {name} model value to f64"
	);
	assert_eq!(
		(value.exp_size, value.sig_size),
		(11, 53),
		"the {name} model value is not Float64"
	);
	assert_eq!(
		value.biased_exponent, 0x7ff,
		"the {name} model value does not have an all-ones exponent"
	);
	assert_eq!(value.cls, ffi::STP_FP_NAN, "the {name} model value is not NaN");
	assert!(double.is_nan(), "the {name} model value does not convert to a NaN f64");
}
