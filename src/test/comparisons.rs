//! Tests for comparisons and overflow predicates.

use super::*;

#[test]
fn comparisons() {
	let ctx = ctx();
	let one = ctx.bv_const(8, 1);
	let two = ctx.bv_const(8, 2);
	let minus_one = ctx.bv_const(8, 0xff);

	assert_valid(&ctx, ctx.bv_ult(one, two));
	assert_valid(&ctx, ctx.not(ctx.bv_ult(two, one)));
	assert_valid(&ctx, ctx.bv_ule(one, one));
	assert_valid(&ctx, ctx.bv_ugt(two, one));
	assert_valid(&ctx, ctx.bv_uge(one, one));
	assert_valid(&ctx, ctx.bv_slt(minus_one, one));
	assert_valid(&ctx, ctx.bv_sle(minus_one, minus_one));
	assert_valid(&ctx, ctx.bv_sgt(one, minus_one));
	assert_valid(&ctx, ctx.bv_sge(one, one));
}

#[test]
fn overflow_predicates() {
	let ctx = ctx();
	let max = ctx.bv_const(8, 255);
	let one = ctx.bv_const(8, 1);
	let zero = ctx.bv_const(8, 0);
	let signed_max = ctx.bv_const(8, 127);
	let signed_min = ctx.bv_const(8, 0x80);
	let sixteen = ctx.bv_const(8, 16);

	assert_valid(&ctx, ctx.bv_uaddo(max, one));
	assert_valid(&ctx, ctx.bv_saddo(signed_max, one));
	assert_valid(&ctx, ctx.bv_usubo(zero, one));
	assert_valid(&ctx, ctx.bv_ssubo(signed_min, one));
	assert_valid(&ctx, ctx.bv_umulo(sixteen, sixteen));
	assert_valid(&ctx, ctx.bv_smulo(signed_max, ctx.bv_const(8, 2)));
}
