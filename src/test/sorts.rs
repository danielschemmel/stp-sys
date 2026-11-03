//! Tests for sorts, variables and constants.

use super::*;

#[test]
fn sorts_and_variables_are_inspectable() {
	let ctx = ctx();
	let bool_sort = ctx.bool_sort();
	let bv_sort = ctx.bv_sort(16);
	let index = ctx.bv_sort(32);
	let element = ctx.bv_sort(8);
	let array_sort = ctx.array_sort(&index, &element);

	assert!(!ctx.sort_to_string(bool_sort).is_empty());
	assert!(!ctx.sort_to_string(bv_sort).is_empty());
	assert!(!ctx.sort_to_string(array_sort).is_empty());

	let b = ctx.bool_var("b");
	let x = ctx.bv_var("x", 16);
	let a = ctx.array_var("a", &index, &element);

	assert!(ctx.is_bool(b));
	assert!(!ctx.is_bool(x));
	assert_eq!(ctx.bv_width(x), 16);
	assert_eq!(ctx.sort_of(b).as_raw(), bool_sort.as_raw());
	assert_eq!(ctx.sort_of(x).as_raw(), bv_sort.as_raw());
	assert_eq!(ctx.sort_of(a).as_raw(), array_sort.as_raw());
	assert_eq!(ctx.array_index_width(array_sort), 32);
	assert_eq!(ctx.array_element_width(array_sort), 8);
	assert_eq!(ctx.array_index_width(bv_sort), 0);
	assert_eq!(ctx.array_element_width(bv_sort), 0);
}

#[test]
fn constants_and_rendering() {
	let ctx = ctx();
	let t = ctx.bool_const(true);
	let f = ctx.bool_const(false);
	assert_valid(&ctx, t);
	assert_invalid(&ctx, f);

	// Values are wrapped modulo 2^width.
	let wrapped = ctx.bv_const(8, 300);
	assert_valid(&ctx, ctx.eq(wrapped, ctx.bv_const(8, 44)));

	let binary = ctx.bv_const_binary("1010");
	assert_eq!(ctx.bv_width(binary), 4);
	assert_valid(&ctx, ctx.eq(binary, ctx.bv_const(4, 10)));

	let decimal = ctx.bv_const_decimal(8, "200");
	assert_valid(&ctx, ctx.eq(decimal, ctx.bv_const(8, 200)));

	let text = ctx.to_smtlib2(decimal);
	assert!(!text.is_empty());
	assert_eq!(ctx.expr_kind(decimal), ffi::STP_KIND_VALUE);
	assert_eq!(ctx.expr_kind(ctx.bv_var("constant_symbol", 8)), ffi::STP_KIND_CONSTANT);
}
