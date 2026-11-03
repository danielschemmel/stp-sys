//! Tests for arrays.

use super::*;

#[test]
fn array_read_and_write() {
	let ctx = ctx();
	let index = ctx.bv_sort(32);
	let element = ctx.bv_sort(8);
	let array = ctx.array_var("a", &index, &element);
	let i = ctx.bv_var("i", 32);
	let j = ctx.bv_var("j", 32);
	let v = ctx.bv_var("v", 8);

	let written = ctx.write(array, i, v);
	assert_valid(&ctx, ctx.eq(ctx.read(written, i), v));
	assert_valid(
		&ctx,
		ctx.eq(ctx.read(written, j), ctx.ite(ctx.eq(i, j), v, ctx.read(array, j))),
	);
}
