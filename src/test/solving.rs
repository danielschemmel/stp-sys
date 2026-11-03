//! Tests for solving, scopes, simplification and timeouts.

use super::*;

#[test]
fn solving_scopes_simplification_and_timeouts() {
	let ctx = ctx();
	let x = ctx.bv_var("x", 8);
	let five = ctx.bv_const(8, 5);
	let x_is_five = ctx.eq(x, five);

	ctx.push();
	ctx.assert(x_is_five);
	assert_valid(&ctx, x_is_five);
	ctx.pop();
	assert_invalid(&ctx, x_is_five);

	assert_eq!(ctx.query_with_timeout(x_is_five, -1, 60), Verdict::Invalid);
	assert_eq!(ctx.query_with_timeout(ctx.eq(x, x), -1, -1), Verdict::Valid);
	assert_eq!(ctx.reason_unknown(), ReasonUnknown::None);

	let sum = ctx.bv_add(five, ctx.bv_const(8, 6));
	let simplified = ctx.simplify(sum);
	assert_eq!(ctx.expr_kind(simplified), ffi::STP_KIND_VALUE);
	assert_valid(&ctx, ctx.eq(simplified, ctx.bv_const(8, 11)));
}
