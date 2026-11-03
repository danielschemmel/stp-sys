//! Tests for inspection and edge cases.

use super::*;

#[test]
fn inspection_helpers() {
	let ctx = ctx();
	let x = ctx.bv_var("x", 24);
	let b = ctx.bool_var("b");
	let sort = ctx.bv_sort(24);

	assert_eq!(ctx.bv_width(x), 24);
	assert_eq!(ctx.sort_of(x).as_raw(), sort.as_raw());
	assert!(ctx.to_smtlib2(x).contains('x'));
	assert!(!ctx.sort_to_string(sort).is_empty());
	assert!(!ctx.is_bool(x));
	assert!(ctx.is_bool(b));
	assert_eq!(ctx.expr_kind(x), ffi::STP_KIND_CONSTANT);
}

#[test]
fn empty_nary_helpers_have_identities() {
	let ctx = ctx();
	assert_valid(&ctx, ctx.and_all(&[]));
	assert_valid(&ctx, ctx.not(ctx.or_all(&[])));
	let zero = ctx.bv_add_all(8, &[]);
	assert_valid(&ctx, ctx.eq(zero, ctx.bv_const(8, 0)));
}

#[test]
#[should_panic(expected = "expression is not a bit-vector")]
fn bv_width_rejects_non_bitvectors() {
	let ctx = ctx();
	let b = ctx.bool_var("b");
	let _ = ctx.bv_width(b);
}

#[test]
#[should_panic(expected = "must be positive")]
fn sign_extend_rejects_zero_width() {
	let ctx = ctx();
	let _ = ctx.bv_sign_extend(ctx.bv_const(8, 1), 0);
}

#[test]
#[should_panic(expected = "must be positive")]
fn zero_extend_rejects_zero_width() {
	let ctx = ctx();
	let _ = ctx.bv_zero_extend(ctx.bv_const(8, 1), 0);
}
