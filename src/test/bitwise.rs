//! Tests for bitwise operations, concatenation, extraction and shifts.

use super::*;

#[test]
fn bitwise_operations() {
	let ctx = ctx();
	let left = ctx.bv_const(8, 0b1100);
	let right = ctx.bv_const(8, 0b1010);

	assert_valid(&ctx, ctx.eq(ctx.bv_and(left, right), ctx.bv_const(8, 0b1000)));
	assert_valid(&ctx, ctx.eq(ctx.bv_or(left, right), ctx.bv_const(8, 0b1110)));
	assert_valid(&ctx, ctx.eq(ctx.bv_xor(left, right), ctx.bv_const(8, 0b0110)));
	assert_valid(&ctx, ctx.eq(ctx.bv_not(left), ctx.bv_const(8, 0b1111_0011)));
	assert_valid(&ctx, ctx.eq(ctx.bv_nand(left, right), ctx.bv_const(8, 0b1111_0111)));
	assert_valid(&ctx, ctx.eq(ctx.bv_nor(left, right), ctx.bv_const(8, 0b1111_0001)));
	assert_valid(&ctx, ctx.eq(ctx.bv_xnor(left, right), ctx.bv_const(8, 0b1111_1001)));

	let high = ctx.bv_const(8, 0xab);
	let low = ctx.bv_const(8, 0xcd);
	let joined = ctx.bv_concat(high, low);
	assert_eq!(ctx.bv_width(joined), 16);
	assert_valid(&ctx, ctx.eq(joined, ctx.bv_const(16, 0xabcd)));
	assert_valid(&ctx, ctx.eq(ctx.bv_extract(joined, 15, 8), high));
	assert_valid(&ctx, ctx.eq(ctx.bv_extract(joined, 7, 0), low));

	let bits = ctx.bv_const(8, 0b0000_1010);
	assert_valid(&ctx, ctx.bv_bit(bits, 1));
	assert_valid(&ctx, ctx.not(ctx.bv_bit(bits, 2)));
}

#[test]
fn extensions_and_shifts() {
	let ctx = ctx();
	let x = ctx.bv_const(8, 0x80);

	let sign_extended = ctx.bv_sign_extend(x, 16);
	assert_eq!(ctx.bv_width(sign_extended), 16);
	assert_valid(&ctx, ctx.eq(sign_extended, ctx.bv_const(16, 0xff80)));

	let zero_extended = ctx.bv_zero_extend(x, 16);
	assert_eq!(ctx.bv_width(zero_extended), 16);
	assert_valid(&ctx, ctx.eq(zero_extended, ctx.bv_const(16, 0x0080)));

	// Both extensions truncate when the requested width is smaller.
	let truncated = ctx.bv_zero_extend(ctx.bv_const(16, 0xabcd), 8);
	assert_valid(&ctx, ctx.eq(truncated, ctx.bv_const(8, 0xcd)));

	// A constant left shift widens by the shift amount.
	let shifted = ctx.bv_shl(ctx.bv_const(8, 3), 2);
	assert_eq!(ctx.bv_width(shifted), 10);
	assert_valid(&ctx, ctx.eq(shifted, ctx.bv_const(10, 12)));
	assert_valid(&ctx, ctx.eq(ctx.bv_shl(x, 0), x));

	// A constant logical right shift keeps the width and drops bits.
	assert_valid(
		&ctx,
		ctx.eq(ctx.bv_lshr(ctx.bv_const(8, 0xa0), 3), ctx.bv_const(8, 0x14)),
	);
	assert_valid(&ctx, ctx.eq(ctx.bv_lshr(x, 0), x));
	assert_valid(&ctx, ctx.eq(ctx.bv_lshr(x, 8), ctx.bv_const(8, 0)));

	// Variable shifts preserve the width.
	assert_valid(
		&ctx,
		ctx.eq(
			ctx.bv_shl_expr(ctx.bv_const(8, 1), ctx.bv_const(8, 2)),
			ctx.bv_const(8, 4),
		),
	);
	assert_valid(
		&ctx,
		ctx.eq(
			ctx.bv_lshr_expr(ctx.bv_const(8, 8), ctx.bv_const(8, 2)),
			ctx.bv_const(8, 2),
		),
	);
	assert_valid(
		&ctx,
		ctx.eq(
			ctx.bv_ashr_expr(ctx.bv_const(8, 0x80), ctx.bv_const(8, 1)),
			ctx.bv_const(8, 0xc0),
		),
	);
}
