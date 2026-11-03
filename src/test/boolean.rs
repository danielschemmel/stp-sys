//! Tests for boolean operations.

use super::*;

#[test]
fn boolean_connectives() {
	let ctx = ctx();
	let t = ctx.bool_const(true);
	let f = ctx.bool_const(false);
	let a = ctx.bool_var("a");
	let b = ctx.bool_var("b");

	assert_invalid(&ctx, ctx.not(t));
	assert_invalid(&ctx, ctx.and(t, f));
	assert_valid(&ctx, ctx.or(t, f));
	assert_valid(&ctx, ctx.xor(t, f));
	assert_invalid(&ctx, ctx.nand(t, t));
	assert_valid(&ctx, ctx.nor(f, f));
	assert_valid(&ctx, ctx.implies(f, f));
	assert_valid(&ctx, ctx.iff(t, t));

	let chosen_then = ctx.ite(t, a, b);
	assert_valid(&ctx, ctx.eq(chosen_then, a));
	let chosen_else = ctx.ite(f, a, b);
	assert_valid(&ctx, ctx.eq(chosen_else, b));

	assert_invalid(&ctx, ctx.and_all(&[t, t, f]));
	assert_valid(&ctx, ctx.or_all(&[f, f, t]));
	let all = ctx.and_all(&[a, b]);
	assert_valid(&ctx, ctx.implies(all, a));
	let any = ctx.or_all(&[a, b]);
	assert_valid(&ctx, ctx.implies(a, any));

	let bit = ctx.bool_to_bv(t);
	assert_eq!(ctx.bv_width(bit), 1);
	assert_valid(&ctx, ctx.eq(bit, ctx.bv_const(1, 1)));
}
