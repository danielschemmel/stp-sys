//! Tests for bit-vector arithmetic.

use super::*;

#[test]
fn bitvector_arithmetic() {
	let ctx = ctx();
	let one = ctx.bv_const(8, 1);
	let two = ctx.bv_const(8, 2);
	let three = ctx.bv_const(8, 3);
	let five = ctx.bv_const(8, 5);
	let six = ctx.bv_const(8, 6);
	let eleven = ctx.bv_const(8, 11);
	let seventeen = ctx.bv_const(8, 17);

	assert_valid(&ctx, ctx.eq(ctx.bv_add(five, six), eleven));
	assert_valid(&ctx, ctx.eq(ctx.bv_add_width(8, five, six), eleven));
	assert_valid(&ctx, ctx.eq(ctx.bv_add_all(8, &[one, two, three]), six));
	assert_valid(&ctx, ctx.eq(ctx.bv_sub(eleven, six), five));
	assert_valid(&ctx, ctx.eq(ctx.bv_mul(five, six), ctx.bv_const(8, 30)));
	assert_valid(&ctx, ctx.eq(ctx.bv_udiv(seventeen, five), three));
	assert_valid(&ctx, ctx.eq(ctx.bv_umod(seventeen, five), two));
	assert_valid(&ctx, ctx.eq(ctx.bv_urem(seventeen, five), two));
	assert_valid(&ctx, ctx.eq(ctx.bv_sdiv(seventeen, five), three));
	assert_valid(&ctx, ctx.eq(ctx.bv_smod(seventeen, five), two));
	assert_valid(&ctx, ctx.eq(ctx.bv_srem(seventeen, five), two));
	assert_valid(&ctx, ctx.eq(ctx.bv_neg(five), ctx.bv_const(8, 251)));
}
