//! Tests for models.

use super::*;

#[test]
fn models_and_counterexample_size() {
	let ctx = ctx();
	let x = ctx.bv_var("x", 8);
	let fits = ctx.bv_ult(x, ctx.bv_const(8, 10));

	assert_invalid(&ctx, fits);
	let value = ctx.model_bv_u64(x).expect("a counterexample should exist");
	assert!(value >= 10, "counterexample {value} should violate x < 10");
	let value_expr = ctx.model_value(x).expect("the model value should be available");
	assert_eq!(ctx.bv_width(value_expr), 8);
	assert!(ctx.counterexample_size() >= 1);

	let b = ctx.bool_var("b");
	assert_invalid(&ctx, b);
	assert_eq!(ctx.model_bool(b), Some(false));

	// A valid query has no counterexample, so every model accessor is empty.
	let ctx2 = Context::new();
	let y = ctx2.bv_var("y", 8);
	assert_valid(&ctx2, ctx2.eq(y, y));
	assert!(ctx2.model_value(y).is_none());
	assert_eq!(ctx2.model_bv_u64(y), None);
	assert_eq!(ctx2.model_bool(y), None);
	assert_eq!(ctx2.counterexample_size(), 0);
}
