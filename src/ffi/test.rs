//! Raw FFI surface smoke tests, generated from `src/ffi.rs`.
//!
//! Every function declared in the 3.x C API binding is called at least
//! once. Null handles are used deliberately: STP's v3 C API documents
//! them as recorded errors rather than crashes, so this checks the whole
//! ABI surface without needing a valid object for every entry point.

#![allow(clippy::too_many_lines)]

use crate::ffi;

#[test]
fn ffi_surface_00() {
	unsafe {
		let _ = ffi::stp_get_version();
		let _ = ffi::stp_capability(std::ptr::null());
		let _ = ffi::stp_capabilities();
		let _ = ffi::stp_has_sat_backend(std::ptr::null());
		let _ = ffi::stp_num_sat_backends();
		let _ = ffi::stp_sat_backend_name(0);
		let _ = ffi::stp_free(std::ptr::null_mut());
		let _ = ffi::stp_last_error();
		let _ = ffi::stp_clear_last_error();
		let _ = ffi::stp_set_internal_error_policy(0);
		let _ = ffi::stp_get_internal_error_policy();
		let _ = ffi::stp_kind_name(0);
		let _ = ffi::stp_kind_smtlib(0);
		let _ = ffi::stp_rm_name(0);
		let _ = ffi::stp_unknown_reason_name(0);
		let _ = ffi::stp_error_code_name(0);
		let _ = ffi::stp_result_kind_name(0);
		let _ = ffi::stp_validity_name(0);
		let _ = ffi::stp_tm_new(std::ptr::null_mut());
		let _ = ffi::stp_tm_new_with(false, 0, 0);
		let _ = ffi::stp_tm_copy(std::ptr::null_mut());
		let _ = ffi::stp_tm_release(std::ptr::null_mut());
		let _ = ffi::stp_tm_id(std::ptr::null_mut());
		let _ = ffi::stp_tm_simplify_enabled(std::ptr::null_mut());
		let _ = ffi::stp_tm_default_rounding_mode(std::ptr::null_mut());
	}
}

#[test]
fn ffi_surface_01() {
	unsafe {
		let _ = ffi::stp_tm_set_default_rounding_mode(std::ptr::null_mut(), 0);
		let _ = ffi::stp_tm_uf_sort_width(std::ptr::null_mut());
		let _ = ffi::stp_tm_scope_push(std::ptr::null_mut());
		let _ = ffi::stp_tm_scope_pop(std::ptr::null_mut());
		let _ = ffi::stp_tm_scope_depth(std::ptr::null_mut());
		let _ = ffi::stp_tm_release_all(std::ptr::null_mut());
		let _ = ffi::stp_tm_error(std::ptr::null_mut());
		let _ = ffi::stp_tm_error_num_terms(std::ptr::null_mut());
		let _ = ffi::stp_tm_error_term(std::ptr::null_mut(), 0);
		let _ = ffi::stp_tm_error_num_sorts(std::ptr::null_mut());
		let _ = ffi::stp_tm_error_sort(std::ptr::null_mut(), 0);
		let _ = ffi::stp_tm_clear_error(std::ptr::null_mut());
		let _ = ffi::stp_tm_set_error_callback(std::ptr::null_mut(), None, std::ptr::null_mut());
		let _ = ffi::stp_mk_bool_sort(std::ptr::null_mut());
		let _ = ffi::stp_mk_bv_sort(std::ptr::null_mut(), 0);
		let _ = ffi::stp_mk_fp_sort(std::ptr::null_mut(), 0, 0);
		let _ = ffi::stp_mk_fp16_sort(std::ptr::null_mut());
		let _ = ffi::stp_mk_fp32_sort(std::ptr::null_mut());
		let _ = ffi::stp_mk_fp64_sort(std::ptr::null_mut());
		let _ = ffi::stp_mk_fp128_sort(std::ptr::null_mut());
		let _ = ffi::stp_mk_rm_sort(std::ptr::null_mut());
		let _ = ffi::stp_mk_real_sort(std::ptr::null_mut());
		let _ = ffi::stp_mk_array_sort(std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut());
		let _ = ffi::stp_mk_fun_sort(std::ptr::null_mut(), 0, std::ptr::null(), std::ptr::null_mut());
		let _ = ffi::stp_tm_declare_sort(std::ptr::null_mut(), std::ptr::null());
	}
}

#[test]
fn ffi_surface_02() {
	unsafe {
		let _ = ffi::stp_mk_fresh_sort(std::ptr::null_mut(), std::ptr::null());
		let _ = ffi::stp_sort_copy(std::ptr::null_mut());
		let _ = ffi::stp_sort_release(std::ptr::null_mut());
		let _ = ffi::stp_sort_get_kind(std::ptr::null_mut(), &mut 0);
		let _ = ffi::stp_sort_bv_size(std::ptr::null_mut(), &mut 0u32);
		let _ = ffi::stp_sort_fp_exp_size(std::ptr::null_mut(), &mut 0u32);
		let _ = ffi::stp_sort_fp_sig_size(std::ptr::null_mut(), &mut 0u32);
		let _ = ffi::stp_sort_array_index(std::ptr::null_mut());
		let _ = ffi::stp_sort_array_element(std::ptr::null_mut());
		let _ = ffi::stp_sort_fun_arity(std::ptr::null_mut(), &mut 0u32);
		let _ = ffi::stp_sort_fun_domain(std::ptr::null_mut(), 0);
		let _ = ffi::stp_sort_fun_codomain(std::ptr::null_mut());
		let _ = ffi::stp_sort_name(std::ptr::null_mut());
		let _ = ffi::stp_sort_id(std::ptr::null_mut());
		let _ = ffi::stp_sort_str(std::ptr::null_mut());
		let _ = ffi::stp_sort_manager(std::ptr::null_mut());
		let _ = ffi::stp_declare(std::ptr::null_mut(), std::ptr::null(), std::ptr::null_mut());
		let _ = ffi::stp_mk_fresh(std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null());
		let _ = ffi::stp_tm_symbol(std::ptr::null_mut(), std::ptr::null());
		let _ = ffi::stp_tm_bind_symbol(std::ptr::null_mut(), std::ptr::null(), std::ptr::null_mut());
		let _ = ffi::stp_tm_num_symbols(std::ptr::null_mut());
		let _ = ffi::stp_tm_symbol_at(std::ptr::null_mut(), 0);
		let _ = ffi::stp_tm_num_declared_sorts(std::ptr::null_mut());
		let _ = ffi::stp_tm_declared_sort_at(std::ptr::null_mut(), 0);
		let _ = ffi::stp_tm_term_from_id(std::ptr::null_mut(), 0);
	}
}

#[test]
fn ffi_surface_03() {
	unsafe {
		let _ = ffi::stp_mk_true(std::ptr::null_mut());
		let _ = ffi::stp_mk_false(std::ptr::null_mut());
		let _ = ffi::stp_mk_bool(std::ptr::null_mut(), false);
		let _ = ffi::stp_mk_bv_uint64(std::ptr::null_mut(), 0, 0);
		let _ = ffi::stp_mk_bv_int64(std::ptr::null_mut(), 0, 0);
		let _ = ffi::stp_mk_bv_str(std::ptr::null_mut(), 0, std::ptr::null(), 0);
		let _ = ffi::stp_mk_bv_limbs(std::ptr::null_mut(), 0, 0, std::ptr::null());
		let _ = ffi::stp_mk_bv_bytes(std::ptr::null_mut(), 0, 0, std::ptr::null(), false);
		let _ = ffi::stp_mk_bv_wrapped(std::ptr::null_mut(), 0, 0);
		let _ = ffi::stp_mk_bv_zero(std::ptr::null_mut(), 0);
		let _ = ffi::stp_mk_bv_ones(std::ptr::null_mut(), 0);
		let _ = ffi::stp_mk_bv_min_signed(std::ptr::null_mut(), 0);
		let _ = ffi::stp_mk_bv_max_signed(std::ptr::null_mut(), 0);
		let _ = ffi::stp_mk_fp_from_bits(std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut());
		let _ = ffi::stp_mk_fp_from_bits_str(std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null());
		let _ = ffi::stp_mk_fp(
			std::ptr::null_mut(),
			std::ptr::null_mut(),
			std::ptr::null_mut(),
			std::ptr::null_mut(),
		);
		let _ = ffi::stp_mk_fp_pos_zero(std::ptr::null_mut(), std::ptr::null_mut());
		let _ = ffi::stp_mk_fp_neg_zero(std::ptr::null_mut(), std::ptr::null_mut());
		let _ = ffi::stp_mk_fp_pos_inf(std::ptr::null_mut(), std::ptr::null_mut());
		let _ = ffi::stp_mk_fp_neg_inf(std::ptr::null_mut(), std::ptr::null_mut());
		let _ = ffi::stp_mk_fp_nan(std::ptr::null_mut(), std::ptr::null_mut());
		let _ = ffi::stp_mk_fp_double(std::ptr::null_mut(), std::ptr::null_mut(), 0, 0.0);
		let _ = ffi::stp_mk_fp_decimal(std::ptr::null_mut(), std::ptr::null_mut(), 0, std::ptr::null());
		let _ = ffi::stp_mk_rm(std::ptr::null_mut(), 0);
		let _ = ffi::stp_mk_real_int64(std::ptr::null_mut(), 0);
	}
}

#[test]
fn ffi_surface_04() {
	unsafe {
		let _ = ffi::stp_mk_real_fraction(std::ptr::null_mut(), 0, 0);
		let _ = ffi::stp_mk_real_str(std::ptr::null_mut(), std::ptr::null());
		let _ = ffi::stp_mk_const_array(std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut());
		let _ = ffi::stp_array_from_bytes(std::ptr::null_mut(), 0, std::ptr::null(), 0);
		let _ = ffi::stp_mk_term(std::ptr::null_mut(), 0, 0, std::ptr::null());
		let _ = ffi::stp_mk_term_indexed(std::ptr::null_mut(), 0, 0, std::ptr::null(), 0, std::ptr::null());
		let _ = ffi::stp_mk_term_sorted(
			std::ptr::null_mut(),
			0,
			0,
			std::ptr::null(),
			0,
			std::ptr::null(),
			std::ptr::null_mut(),
		);
		let _ = ffi::stp_mk_term1(std::ptr::null_mut(), 0, std::ptr::null_mut());
		let _ = ffi::stp_mk_term2(std::ptr::null_mut(), 0, std::ptr::null_mut(), std::ptr::null_mut());
		let _ = ffi::stp_mk_term3(
			std::ptr::null_mut(),
			0,
			std::ptr::null_mut(),
			std::ptr::null_mut(),
			std::ptr::null_mut(),
		);
		let _ = ffi::stp_mk_term1_indexed1(std::ptr::null_mut(), 0, std::ptr::null_mut(), 0);
		let _ = ffi::stp_mk_term1_indexed2(std::ptr::null_mut(), 0, std::ptr::null_mut(), 0, 0);
		let _ = ffi::stp_mk_term2_indexed1(std::ptr::null_mut(), 0, std::ptr::null_mut(), std::ptr::null_mut(), 0);
		let _ = ffi::stp_mk_term2_indexed2(
			std::ptr::null_mut(),
			0,
			std::ptr::null_mut(),
			std::ptr::null_mut(),
			0,
			0,
		);
		let _ = ffi::stp_extract(std::ptr::null_mut(), 0, 0, std::ptr::null_mut());
		let _ = ffi::stp_zero_extend(std::ptr::null_mut(), 0, std::ptr::null_mut());
		let _ = ffi::stp_sign_extend(std::ptr::null_mut(), 0, std::ptr::null_mut());
		let _ = ffi::stp_repeat(std::ptr::null_mut(), 0, std::ptr::null_mut());
		let _ = ffi::stp_rotate_left(std::ptr::null_mut(), 0, std::ptr::null_mut());
		let _ = ffi::stp_rotate_right(std::ptr::null_mut(), 0, std::ptr::null_mut());
		let _ = ffi::stp_bit(std::ptr::null_mut(), std::ptr::null_mut(), 0);
		let _ = ffi::stp_bool_to_bv1(std::ptr::null_mut(), std::ptr::null_mut());
		let _ = ffi::stp_bv1_to_bool(std::ptr::null_mut(), std::ptr::null_mut());
		let _ = ffi::stp_to_fp(
			std::ptr::null_mut(),
			std::ptr::null_mut(),
			std::ptr::null_mut(),
			std::ptr::null_mut(),
		);
		let _ = ffi::stp_to_fp_unsigned(
			std::ptr::null_mut(),
			std::ptr::null_mut(),
			std::ptr::null_mut(),
			std::ptr::null_mut(),
		);
	}
}

#[test]
fn ffi_surface_05() {
	unsafe {
		let _ = ffi::stp_to_fp_from_bits(std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut());
		let _ = ffi::stp_fp_to_ubv(std::ptr::null_mut(), 0, std::ptr::null_mut(), std::ptr::null_mut());
		let _ = ffi::stp_fp_to_sbv(std::ptr::null_mut(), 0, std::ptr::null_mut(), std::ptr::null_mut());
		let _ = ffi::stp_fp_add_rm(std::ptr::null_mut(), 0, std::ptr::null_mut(), std::ptr::null_mut());
		let _ = ffi::stp_fp_sub_rm(std::ptr::null_mut(), 0, std::ptr::null_mut(), std::ptr::null_mut());
		let _ = ffi::stp_fp_mul_rm(std::ptr::null_mut(), 0, std::ptr::null_mut(), std::ptr::null_mut());
		let _ = ffi::stp_fp_div_rm(std::ptr::null_mut(), 0, std::ptr::null_mut(), std::ptr::null_mut());
		let _ = ffi::stp_fp_fma_rm(
			std::ptr::null_mut(),
			0,
			std::ptr::null_mut(),
			std::ptr::null_mut(),
			std::ptr::null_mut(),
		);
		let _ = ffi::stp_fp_sqrt_rm(std::ptr::null_mut(), 0, std::ptr::null_mut());
		let _ = ffi::stp_fp_rti_rm(std::ptr::null_mut(), 0, std::ptr::null_mut());
		let _ = ffi::stp_to_fp_rm(std::ptr::null_mut(), std::ptr::null_mut(), 0, std::ptr::null_mut());
		let _ = ffi::stp_to_fp_unsigned_rm(std::ptr::null_mut(), std::ptr::null_mut(), 0, std::ptr::null_mut());
		let _ = ffi::stp_fp_to_ubv_rm(std::ptr::null_mut(), 0, 0, std::ptr::null_mut());
		let _ = ffi::stp_fp_to_sbv_rm(std::ptr::null_mut(), 0, 0, std::ptr::null_mut());
		let _ = ffi::stp_term_copy(std::ptr::null_mut());
		let _ = ffi::stp_term_release(std::ptr::null_mut());
		let _ = ffi::stp_term_id(std::ptr::null_mut());
		let _ = ffi::stp_term_hash(std::ptr::null_mut());
		let _ = ffi::stp_term_manager(std::ptr::null_mut());
		let _ = ffi::stp_term_get_kind(std::ptr::null_mut(), &mut 0);
		let _ = ffi::stp_term_sort(std::ptr::null_mut());
		let _ = ffi::stp_term_num_children(std::ptr::null_mut(), &mut 0usize);
		let _ = ffi::stp_term_child(std::ptr::null_mut(), 0);
		let _ = ffi::stp_term_num_indices(std::ptr::null_mut(), &mut 0usize);
		let _ = ffi::stp_term_index(std::ptr::null_mut(), 0, &mut 0u32);
	}
}

#[test]
fn ffi_surface_06() {
	unsafe {
		let _ = ffi::stp_term_is_value(std::ptr::null_mut());
		let _ = ffi::stp_term_is_defined_function(std::ptr::null_mut());
		let _ = ffi::stp_term_is_const(std::ptr::null_mut());
		let _ = ffi::stp_term_symbol(std::ptr::null_mut());
		let _ = ffi::stp_term_str(std::ptr::null_mut());
		let _ = ffi::stp_term_to_string(std::ptr::null_mut(), 0, false);
		let _ = ffi::stp_term_substitute(std::ptr::null_mut(), 0, std::ptr::null(), std::ptr::null());
		let _ = ffi::stp_tm_simplify(std::ptr::null_mut(), std::ptr::null_mut());
		let _ = ffi::stp_term_same(std::ptr::null_mut(), std::ptr::null_mut());
		let _ = ffi::stp_term_to_bool(std::ptr::null_mut(), &mut false);
		let _ = ffi::stp_term_fits_uint64(std::ptr::null_mut());
		let _ = ffi::stp_term_fits_int64(std::ptr::null_mut());
		let _ = ffi::stp_term_to_uint64(std::ptr::null_mut(), &mut 0u64);
		let _ = ffi::stp_term_to_int64(std::ptr::null_mut(), &mut 0i64);
		let _ = ffi::stp_term_to_bv_string(std::ptr::null_mut(), 0, false);
		let _ = ffi::stp_term_bv_num_limbs(std::ptr::null_mut(), &mut 0usize);
		let _ = ffi::stp_term_to_bv_limbs(std::ptr::null_mut(), 0, &mut 0u64);
		let _ = ffi::stp_term_to_bv_bytes(std::ptr::null_mut(), 0, std::ptr::null_mut(), false);
		let _ = ffi::stp_term_to_fp(std::ptr::null_mut(), &mut std::mem::zeroed());
		let _ = ffi::stp_term_fp_significand_limbs(std::ptr::null_mut(), 0, &mut 0u64);
		let _ = ffi::stp_term_fp_bits(std::ptr::null_mut());
		let _ = ffi::stp_term_fp_to_double(std::ptr::null_mut(), &mut 0.0f64);
		let _ = ffi::stp_term_fp_to_rational(
			std::ptr::null_mut(),
			&mut std::ptr::null_mut(),
			&mut std::ptr::null_mut(),
		);
		let _ = ffi::stp_term_to_rm(std::ptr::null_mut(), &mut 0);
		let _ = ffi::stp_term_real_numerator(std::ptr::null_mut());
	}
}

#[test]
fn ffi_surface_07() {
	unsafe {
		let _ = ffi::stp_term_real_denominator(std::ptr::null_mut());
		let _ = ffi::stp_term_real_fits_int64(std::ptr::null_mut());
		let _ = ffi::stp_term_real_to_int64(std::ptr::null_mut(), &mut 0i64, &mut 0i64);
		let _ = ffi::stp_term_real_to_double(std::ptr::null_mut(), &mut 0.0f64);
		let _ = ffi::stp_term_to_uninterpreted_index(std::ptr::null_mut(), &mut 0u64);
		let _ = ffi::stp_options_new();
		let _ = ffi::stp_options_copy(std::ptr::null_mut());
		let _ = ffi::stp_options_delete(std::ptr::null_mut());
		let _ = ffi::stp_options_error(std::ptr::null_mut());
		let _ = ffi::stp_options_clear_error(std::ptr::null_mut());
		let _ = ffi::stp_options_set_str(std::ptr::null_mut(), std::ptr::null(), std::ptr::null());
		let _ = ffi::stp_options_set_bool(std::ptr::null_mut(), std::ptr::null(), false);
		let _ = ffi::stp_options_set_int64(std::ptr::null_mut(), std::ptr::null(), 0);
		let _ = ffi::stp_options_set_uint64(std::ptr::null_mut(), std::ptr::null(), 0);
		let _ = ffi::stp_options_set_duration_ms(std::ptr::null_mut(), std::ptr::null(), 0);
		let _ = ffi::stp_options_set_names(std::ptr::null_mut(), std::ptr::null(), 0, std::ptr::null());
		let _ = ffi::stp_options_set_bool_e(std::ptr::null_mut(), 0, false);
		let _ = ffi::stp_options_set_int64_e(std::ptr::null_mut(), 0, 0);
		let _ = ffi::stp_options_set_uint64_e(std::ptr::null_mut(), 0, 0);
		let _ = ffi::stp_options_set_str_e(std::ptr::null_mut(), 0, std::ptr::null());
		let _ = ffi::stp_options_set_duration_ms_e(std::ptr::null_mut(), 0, 0);
		let _ = ffi::stp_options_set_args(std::ptr::null_mut(), 0, std::ptr::null());
		let _ = ffi::stp_options_get_str(std::ptr::null_mut(), std::ptr::null());
		let _ = ffi::stp_options_get_bool(std::ptr::null_mut(), std::ptr::null(), &mut false);
		let _ = ffi::stp_options_get_int64(std::ptr::null_mut(), std::ptr::null(), &mut 0i64);
	}
}

#[test]
fn ffi_surface_08() {
	unsafe {
		let _ = ffi::stp_options_get_uint64(std::ptr::null_mut(), std::ptr::null(), &mut 0u64);
		let _ = ffi::stp_options_get_duration_ms(std::ptr::null_mut(), std::ptr::null(), &mut 0u64);
		let _ = ffi::stp_options_resolved_str(std::ptr::null_mut(), std::ptr::null());
		let _ = ffi::stp_options_is_set(std::ptr::null_mut(), std::ptr::null());
		let _ = ffi::stp_options_reset(std::ptr::null_mut(), std::ptr::null());
		let _ = ffi::stp_options_reset_all(std::ptr::null_mut());
		let _ = ffi::stp_options_resolve(std::ptr::null_mut());
		let _ = ffi::stp_options_num_names(0);
		let _ = ffi::stp_options_name(0, 0);
		let _ = ffi::stp_options_help(0);
		let _ = ffi::stp_option_name(0);
		let _ = ffi::stp_option_from_name(std::ptr::null(), std::ptr::null_mut());
		let _ = ffi::stp_option_info_type(std::ptr::null());
		let _ = ffi::stp_option_info_python_key(std::ptr::null());
		let _ = ffi::stp_option_info_default(std::ptr::null());
		let _ = ffi::stp_option_info_tier(std::ptr::null());
		let _ = ffi::stp_option_info_settable(std::ptr::null());
		let _ = ffi::stp_option_info_scope(std::ptr::null());
		let _ = ffi::stp_option_info_category(std::ptr::null());
		let _ = ffi::stp_option_info_help(std::ptr::null());
		let _ = ffi::stp_option_info_supported(std::ptr::null());
		let _ = ffi::stp_option_info_range(std::ptr::null(), &mut false, &mut 0i64, &mut false, &mut 0i64);
		let _ = ffi::stp_option_info_num_values(std::ptr::null());
		let _ = ffi::stp_option_info_value(std::ptr::null(), 0);
		let _ = ffi::stp_option_info_num_aliases(std::ptr::null());
	}
}

#[test]
fn ffi_surface_09() {
	unsafe {
		let _ = ffi::stp_option_info_alias(std::ptr::null(), 0);
		let _ = ffi::stp_option_info_short(std::ptr::null());
		let _ = ffi::stp_option_info_negation(std::ptr::null());
		let _ = ffi::stp_solver_new(std::ptr::null_mut(), std::ptr::null_mut());
		let _ = ffi::stp_solver_delete(std::ptr::null_mut());
		let _ = ffi::stp_solver_failed(std::ptr::null_mut());
		let _ = ffi::stp_solver_failed_num_terms(std::ptr::null_mut());
		let _ = ffi::stp_solver_failed_term(std::ptr::null_mut(), 0);
		let _ = ffi::stp_solver_failed_num_sorts(std::ptr::null_mut());
		let _ = ffi::stp_solver_failed_sort(std::ptr::null_mut(), 0);
		let _ = ffi::stp_solver_clear_error(std::ptr::null_mut());
		let _ = ffi::stp_solver_manager(std::ptr::null_mut());
		let _ = ffi::stp_solver_set_str(std::ptr::null_mut(), std::ptr::null(), std::ptr::null());
		let _ = ffi::stp_solver_set_bool(std::ptr::null_mut(), std::ptr::null(), false);
		let _ = ffi::stp_solver_set_int64(std::ptr::null_mut(), std::ptr::null(), 0);
		let _ = ffi::stp_solver_set_uint64(std::ptr::null_mut(), std::ptr::null(), 0);
		let _ = ffi::stp_solver_set_duration_ms(std::ptr::null_mut(), std::ptr::null(), 0);
		let _ = ffi::stp_solver_set_names(std::ptr::null_mut(), std::ptr::null(), 0, std::ptr::null());
		let _ = ffi::stp_solver_set_bool_e(std::ptr::null_mut(), 0, false);
		let _ = ffi::stp_solver_set_int64_e(std::ptr::null_mut(), 0, 0);
		let _ = ffi::stp_solver_set_uint64_e(std::ptr::null_mut(), 0, 0);
		let _ = ffi::stp_solver_set_str_e(std::ptr::null_mut(), 0, std::ptr::null());
		let _ = ffi::stp_solver_set_duration_ms_e(std::ptr::null_mut(), 0, 0);
		let _ = ffi::stp_solver_set_args(std::ptr::null_mut(), 0, std::ptr::null());
		let _ = ffi::stp_solver_get_str(std::ptr::null_mut(), std::ptr::null());
	}
}

#[test]
fn ffi_surface_10() {
	unsafe {
		let _ = ffi::stp_solver_get_bool(std::ptr::null_mut(), std::ptr::null(), &mut false);
		let _ = ffi::stp_solver_get_int64(std::ptr::null_mut(), std::ptr::null(), &mut 0i64);
		let _ = ffi::stp_solver_get_uint64(std::ptr::null_mut(), std::ptr::null(), &mut 0u64);
		let _ = ffi::stp_solver_get_duration_ms(std::ptr::null_mut(), std::ptr::null(), &mut 0u64);
		let _ = ffi::stp_solver_resolved_str(std::ptr::null_mut(), std::ptr::null());
		let _ = ffi::stp_solver_option_is_set(std::ptr::null_mut(), std::ptr::null());
		let _ = ffi::stp_solver_reset_option(std::ptr::null_mut(), std::ptr::null());
		let _ = ffi::stp_solver_reset_all_options(std::ptr::null_mut());
		let _ = ffi::stp_solver_resolve_options(std::ptr::null_mut());
		let _ = ffi::stp_solver_options_copy(std::ptr::null_mut());
		let _ = ffi::stp_solver_assert(std::ptr::null_mut(), std::ptr::null_mut());
		let _ = ffi::stp_solver_push(std::ptr::null_mut(), 0);
		let _ = ffi::stp_solver_pop(std::ptr::null_mut(), 0);
		let _ = ffi::stp_solver_level(std::ptr::null_mut());
		let _ = ffi::stp_solver_declared_logic(std::ptr::null_mut());
		let _ = ffi::stp_solver_num_assertions(std::ptr::null_mut());
		let _ = ffi::stp_solver_assertion(std::ptr::null_mut(), 0);
		let _ = ffi::stp_solver_reset_assertions(std::ptr::null_mut());
		let _ = ffi::stp_solver_reset(std::ptr::null_mut());
		let _ = ffi::stp_solver_check_sat(std::ptr::null_mut(), &mut std::mem::zeroed());
		let _ = ffi::stp_solver_check_sat_assuming(std::ptr::null_mut(), 0, std::ptr::null(), &mut std::mem::zeroed());
		let _ = ffi::stp_solver_check_sat_budget(
			std::ptr::null_mut(),
			0,
			std::ptr::null(),
			std::ptr::null(),
			&mut std::mem::zeroed(),
		);
		let _ = ffi::stp_solver_entails(
			std::ptr::null_mut(),
			std::ptr::null_mut(),
			std::ptr::null(),
			&mut std::mem::zeroed(),
		);
		let _ = ffi::stp_solver_last_reason_message(std::ptr::null_mut());
		let _ = ffi::stp_solver_num_unsat_assumptions(std::ptr::null_mut());
	}
}

#[test]
fn ffi_surface_11() {
	unsafe {
		let _ = ffi::stp_solver_unsat_assumption(std::ptr::null_mut(), 0);
		let _ = ffi::stp_solver_model(std::ptr::null_mut());
		let _ = ffi::stp_solver_candidate_model(std::ptr::null_mut());
		let _ = ffi::stp_solver_value(std::ptr::null_mut(), std::ptr::null_mut());
		let _ = ffi::stp_solver_interrupt(std::ptr::null_mut());
		let _ = ffi::stp_solver_clear_interrupt(std::ptr::null_mut());
		let _ = ffi::stp_solver_interrupt_pending(std::ptr::null_mut());
		let _ = ffi::stp_solver_set_terminator(std::ptr::null_mut(), None, std::ptr::null_mut());
		let _ = ffi::stp_solver_statistics(std::ptr::null_mut());
		let _ = ffi::stp_solver_symbol(std::ptr::null_mut(), std::ptr::null());
		let _ = ffi::stp_solver_parse_smt2(std::ptr::null_mut(), std::ptr::null(), 0);
		let _ = ffi::stp_solver_parse(std::ptr::null_mut(), std::ptr::null(), 0);
		let _ = ffi::stp_solver_parse_file(std::ptr::null_mut(), std::ptr::null(), 0);
		let _ = ffi::stp_solver_parse_term(std::ptr::null_mut(), std::ptr::null());
		let _ = ffi::stp_solver_to_smt2(std::ptr::null_mut(), false);
		let _ = ffi::stp_solver_to_string(std::ptr::null_mut(), 0);
		let _ = ffi::stp_solver_write_cnf(std::ptr::null_mut(), None, std::ptr::null_mut(), &mut 0);
		let _ = ffi::stp_solver_set_diagnostic_sink(std::ptr::null_mut(), None, std::ptr::null_mut());
		let _ = ffi::stp_solver_parse_source(std::ptr::null_mut(), None, std::ptr::null_mut(), 0, 0);
		let _ = ffi::stp_solver_set_output_sink(std::ptr::null_mut(), None, std::ptr::null_mut());
		let _ = ffi::stp_solver_set_fatal_error_handler(std::ptr::null_mut(), None, std::ptr::null_mut());
		let _ = ffi::stp_solver_set_cnf_sink(std::ptr::null_mut(), None, std::ptr::null_mut());
		let _ = ffi::stp_model_copy(std::ptr::null_mut());
		let _ = ffi::stp_model_release(std::ptr::null_mut());
		let _ = ffi::stp_model_manager(std::ptr::null_mut());
	}
}

#[test]
fn ffi_surface_12() {
	unsafe {
		let _ = ffi::stp_model_value(std::ptr::null_mut(), std::ptr::null_mut());
		let _ = ffi::stp_model_try_value(std::ptr::null_mut(), std::ptr::null_mut());
		let _ = ffi::stp_model_values(std::ptr::null_mut(), 0, std::ptr::null(), &mut std::ptr::null_mut());
		let _ = ffi::stp_model_bool(std::ptr::null_mut(), std::ptr::null_mut(), &mut false);
		let _ = ffi::stp_model_uint64(std::ptr::null_mut(), std::ptr::null_mut(), &mut 0u64);
		let _ = ffi::stp_model_int64(std::ptr::null_mut(), std::ptr::null_mut(), &mut 0i64);
		let _ = ffi::stp_model_bv_string(std::ptr::null_mut(), std::ptr::null_mut(), 0, false);
		let _ = ffi::stp_model_bv_num_limbs(std::ptr::null_mut(), std::ptr::null_mut(), &mut 0usize);
		let _ = ffi::stp_model_bv_limbs(std::ptr::null_mut(), std::ptr::null_mut(), 0, &mut 0u64);
		let _ = ffi::stp_model_bv_bytes(
			std::ptr::null_mut(),
			std::ptr::null_mut(),
			0,
			std::ptr::null_mut(),
			false,
		);
		let _ = ffi::stp_model_fp(std::ptr::null_mut(), std::ptr::null_mut(), &mut std::mem::zeroed());
		let _ = ffi::stp_model_fp_significand_limbs(std::ptr::null_mut(), std::ptr::null_mut(), 0, &mut 0u64);
		let _ = ffi::stp_model_fp_to_double(std::ptr::null_mut(), std::ptr::null_mut(), &mut 0.0f64);
		let _ = ffi::stp_model_rm(std::ptr::null_mut(), std::ptr::null_mut(), &mut 0);
		let _ = ffi::stp_model_real_numerator(std::ptr::null_mut(), std::ptr::null_mut());
		let _ = ffi::stp_model_real_denominator(std::ptr::null_mut(), std::ptr::null_mut());
		let _ = ffi::stp_model_uninterpreted_index(std::ptr::null_mut(), std::ptr::null_mut(), &mut 0u64);
		let _ = ffi::stp_model_array_value(std::ptr::null_mut(), std::ptr::null_mut());
		let _ = ffi::stp_model_fun_value(std::ptr::null_mut(), std::ptr::null_mut());
		let _ = ffi::stp_model_array_bytes(std::ptr::null_mut(), std::ptr::null_mut(), 0, 0, std::ptr::null_mut());
		let _ = ffi::stp_model_num_symbols(std::ptr::null_mut());
		let _ = ffi::stp_model_symbol(std::ptr::null_mut(), 0);
		let _ = ffi::stp_model_in_core(std::ptr::null_mut(), std::ptr::null_mut());
		let _ = ffi::stp_model_to_smt2(std::ptr::null_mut());
		let _ = ffi::stp_array_value_release(std::ptr::null_mut());
	}
}

#[test]
fn ffi_surface_13() {
	unsafe {
		let _ = ffi::stp_array_value_sort(std::ptr::null_mut());
		let _ = ffi::stp_array_value_default(std::ptr::null_mut());
		let _ = ffi::stp_array_value_size(std::ptr::null_mut());
		let _ = ffi::stp_array_value_entry(
			std::ptr::null_mut(),
			0,
			&mut std::ptr::null_mut(),
			&mut std::ptr::null_mut(),
		);
		let _ = ffi::stp_array_value_at(std::ptr::null_mut(), std::ptr::null_mut());
		let _ = ffi::stp_array_value_as_term(std::ptr::null_mut());
		let _ = ffi::stp_fun_value_release(std::ptr::null_mut());
		let _ = ffi::stp_fun_value_sort(std::ptr::null_mut());
		let _ = ffi::stp_fun_value_arity(std::ptr::null_mut());
		let _ = ffi::stp_fun_value_is_tabular(std::ptr::null_mut());
		let _ = ffi::stp_fun_value_else(std::ptr::null_mut());
		let _ = ffi::stp_fun_value_size(std::ptr::null_mut());
		let _ = ffi::stp_fun_value_entry(
			std::ptr::null_mut(),
			0,
			0,
			&mut std::ptr::null_mut(),
			&mut std::ptr::null_mut(),
		);
		let _ = ffi::stp_fun_value_apply(std::ptr::null_mut(), 0, std::ptr::null());
		let _ = ffi::stp_fun_value_as_ite(std::ptr::null_mut(), 0, std::ptr::null());
		let _ = ffi::stp_statistics_release(std::ptr::null_mut());
		let _ = ffi::stp_statistics_size(std::ptr::null_mut());
		let _ = ffi::stp_statistics_name(std::ptr::null_mut(), 0);
		let _ = ffi::stp_statistics_is_uint64(std::ptr::null_mut(), std::ptr::null());
		let _ = ffi::stp_statistics_is_double(std::ptr::null_mut(), std::ptr::null());
		let _ = ffi::stp_statistics_uint64(std::ptr::null_mut(), std::ptr::null(), &mut 0u64);
		let _ = ffi::stp_statistics_double(std::ptr::null_mut(), std::ptr::null(), &mut 0.0f64);
		let _ = ffi::stp_statistics_str(std::ptr::null_mut(), std::ptr::null());
		let _ = ffi::stp_statistics_tier(std::ptr::null());
	}
}
