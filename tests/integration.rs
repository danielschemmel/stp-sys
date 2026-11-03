//! Integration tests exercising the safe wrapper against a real STP solver.
//!
//! These tests link the vendored (or system) STP build and run actual queries.

use stp_sys::{Context, ReasonUnknown, Verdict};

#[test]
fn addition_identity_is_valid() {
	let ctx = Context::new();
	let x = ctx.bv_var("x", 32);
	let zero = ctx.bv_const(32, 0);
	let lhs = ctx.bv_add(x, zero);
	assert_eq!(ctx.query(ctx.eq(lhs, x)), Verdict::Valid);
}

#[test]
fn multiplication_is_commutative() {
	let ctx = Context::new();
	let a = ctx.bv_var("a", 16);
	let b = ctx.bv_var("b", 16);
	let ab = ctx.bv_mul(a, b);
	let ba = ctx.bv_mul(b, a);
	assert_eq!(ctx.query(ctx.eq(ab, ba)), Verdict::Valid);
}

#[test]
fn unsigned_wraparound() {
	let ctx = Context::new();
	// In 8 bits, 255 + 1 == 0.
	let lhs = ctx.bv_add(ctx.bv_const(8, 255), ctx.bv_const(8, 1));
	assert_eq!(ctx.query(ctx.eq(lhs, ctx.bv_const(8, 0))), Verdict::Valid);
}

#[test]
fn finds_counterexample_and_reads_model() {
	let ctx = Context::new();
	let x = ctx.bv_var("x", 8);
	let fits = ctx.bv_ult(x, ctx.bv_const(8, 10));
	assert_eq!(ctx.query(fits), Verdict::Invalid);
	let value = ctx.model_bv_u64(x).expect("a counterexample should exist");
	assert!(value >= 10, "counterexample {value} should violate x < 10");
}

#[test]
fn signed_and_unsigned_ordering_differ() {
	let ctx = Context::new();
	// The 8-bit pattern 0xff is -1 signed and 255 unsigned.
	let minus_one = ctx.bv_const(8, 0xff);
	let zero = ctx.bv_const(8, 0);
	assert_eq!(ctx.query(ctx.bv_slt(minus_one, zero)), Verdict::Valid);
	assert_eq!(ctx.query(ctx.bv_ult(minus_one, zero)), Verdict::Invalid);
}

#[test]
fn assumptions_are_scoped_by_push_pop() {
	let ctx = Context::new();
	let x = ctx.bv_var("x", 8);
	let five = ctx.bv_const(8, 5);
	let x_is_five = ctx.eq(x, five);

	ctx.push();
	ctx.assert(x_is_five);
	assert_eq!(ctx.query(x_is_five), Verdict::Valid);
	ctx.pop();

	// Without the assumption, x == 5 is not valid.
	assert_eq!(ctx.query(x_is_five), Verdict::Invalid);
}

#[test]
fn nested_assumptions_retract_lifo() {
	let ctx = Context::new();
	let x = ctx.bv_var("x", 8);
	let five = ctx.bv_const(8, 5);
	let six = ctx.bv_const(8, 6);

	ctx.push();
	ctx.assert(ctx.eq(x, five));
	ctx.push();
	ctx.assert(ctx.eq(x, six));
	// x == 5 and x == 6 are inconsistent, so anything follows.
	assert_eq!(ctx.query(ctx.bool_const(false)), Verdict::Valid);
	ctx.pop();
	// Still x == 5.
	assert_eq!(ctx.query(ctx.eq(x, five)), Verdict::Valid);
	ctx.pop();
	assert_eq!(ctx.query(ctx.eq(x, five)), Verdict::Invalid);
}

#[test]
fn array_read_over_write() {
	let ctx = Context::new();
	let index = ctx.bv_sort(32);
	let element = ctx.bv_sort(8);
	let array = ctx.array_var("a", &index, &element);
	let i = ctx.bv_var("i", 32);
	let v = ctx.bv_var("v", 8);
	let written = ctx.write(array, i, v);
	// read(write(a, i, v), i) == v
	assert_eq!(ctx.query(ctx.eq(ctx.read(written, i), v)), Verdict::Valid);
}

#[test]
fn array_extensionality_axiom() {
	let ctx = Context::new();
	let index = ctx.bv_sort(32);
	let element = ctx.bv_sort(8);
	let array = ctx.array_var("a", &index, &element);
	let i = ctx.bv_var("i", 32);
	let j = ctx.bv_var("j", 32);
	let v = ctx.bv_var("v", 8);

	let written = ctx.write(array, i, v);
	// read(write(a, i, v), j) == ite(i == j, v, read(a, j))
	let lhs = ctx.read(written, j);
	let rhs = ctx.ite(ctx.eq(i, j), v, ctx.read(array, j));
	assert_eq!(ctx.query(ctx.eq(lhs, rhs)), Verdict::Valid);
}

#[test]
fn if_then_else_selects_branch() {
	let ctx = Context::new();
	let a = ctx.bv_var("a", 8);
	let b = ctx.bv_var("b", 8);
	let chosen = ctx.ite(ctx.bool_const(true), a, b);
	assert_eq!(ctx.query(ctx.eq(chosen, a)), Verdict::Valid);
	let chosen = ctx.ite(ctx.bool_const(false), a, b);
	assert_eq!(ctx.query(ctx.eq(chosen, b)), Verdict::Valid);
}

#[test]
fn concat_and_extract_round_trip() {
	let ctx = Context::new();
	let high = ctx.bv_const(8, 0xab);
	let low = ctx.bv_const(8, 0xcd);
	let joined = ctx.bv_concat(high, low);
	assert_eq!(ctx.bv_width(joined), 16);
	// Extract the two halves back out.
	assert_eq!(ctx.query(ctx.eq(ctx.bv_extract(joined, 15, 8), high)), Verdict::Valid);
	assert_eq!(ctx.query(ctx.eq(ctx.bv_extract(joined, 7, 0), low)), Verdict::Valid);
}

#[test]
fn zero_extension_and_shifts() {
	let ctx = Context::new();
	let x = ctx.bv_var("x", 8);

	// Zero-extending to a wider sort preserves the value.
	let wide = ctx.bv_zero_extend(x, 32);
	assert_eq!(ctx.bv_width(wide), 32);
	// Zero-extending to the same width is the identity.
	let same = ctx.bv_zero_extend(x, 8);
	assert_eq!(ctx.query(ctx.eq(same, x)), Verdict::Valid);

	// A constant left shift concatenates zeros, so the result is wider than
	// the operand by the shift amount.
	let three = ctx.bv_const(8, 3);
	let shifted = ctx.bv_shl(three, 2);
	assert_eq!(ctx.bv_width(shifted), 10);
	assert_eq!(ctx.query(ctx.eq(shifted, ctx.bv_const(10, 12))), Verdict::Valid);

	// A (logical) right shift keeps the width.
	let value = ctx.bv_const(8, 0b1010_0000);
	let shifted_right = ctx.bv_lshr(value, 3);
	assert_eq!(ctx.bv_width(shifted_right), 8);
	assert_eq!(
		ctx.query(ctx.eq(shifted_right, ctx.bv_const(8, 0b0001_0100))),
		Verdict::Valid
	);

	// A shift by a bit-vector amount keeps the width too.
	let amount = ctx.bv_const(8, 0);
	let shifted_expr = ctx.bv_shl_expr(x, amount);
	assert_eq!(ctx.bv_width(shifted_expr), 8);
	assert_eq!(ctx.query(ctx.eq(shifted_expr, x)), Verdict::Valid);
}

#[test]
fn overflow_predicates() {
	let ctx = Context::new();
	// 255 + 1 overflows 8 bits.
	let overflow = ctx.bv_uaddo(ctx.bv_const(8, 255), ctx.bv_const(8, 1));
	assert_eq!(ctx.query(overflow), Verdict::Valid);
	// 1 + 2 does not.
	let no_overflow = ctx.bv_uaddo(ctx.bv_const(8, 1), ctx.bv_const(8, 2));
	assert_eq!(ctx.query(ctx.not(no_overflow)), Verdict::Valid);
}

#[test]
fn boolean_connectives() {
	let ctx = Context::new();
	let a = ctx.bool_var("a");
	let b = ctx.bool_var("b");

	// a => (a or b)
	let formula = ctx.implies(a, ctx.or(a, b));
	assert_eq!(ctx.query(formula), Verdict::Valid);

	// (a and b) <=> (b and a)
	let lhs = ctx.and(a, b);
	let rhs = ctx.and(b, a);
	assert_eq!(ctx.query(ctx.iff(lhs, rhs)), Verdict::Valid);
}

#[test]
fn nary_conjunction_and_disjunction() {
	let ctx = Context::new();
	let a = ctx.bool_var("a");
	let b = ctx.bool_var("b");
	let c = ctx.bool_var("c");

	let all = ctx.and_all(&[a, b, c]);
	let any = ctx.or_all(&[a, b, c]);
	// (a and b and c) => (a or b or c)
	assert_eq!(ctx.query(ctx.implies(all, any)), Verdict::Valid);
}

#[test]
fn simplify_folds_constants() {
	let ctx = Context::new();
	let five = ctx.bv_const(16, 5);
	let six = ctx.bv_const(16, 6);
	let sum = ctx.bv_add(five, six);
	let simplified = ctx.simplify(sum);
	// The simplifier should fold the sum to the constant 11.
	assert_eq!(ctx.expr_kind(simplified), stp_sys::ffi::STP_KIND_VALUE);
	assert_eq!(ctx.query(ctx.eq(simplified, ctx.bv_const(16, 11))), Verdict::Valid);
}

#[test]
fn division_and_modulo() {
	let ctx = Context::new();
	// 17 / 5 == 3 and 17 % 5 == 2 (unsigned).
	let q = ctx.bv_udiv(ctx.bv_const(8, 17), ctx.bv_const(8, 5));
	let r = ctx.bv_umod(ctx.bv_const(8, 17), ctx.bv_const(8, 5));
	assert_eq!(ctx.query(ctx.eq(q, ctx.bv_const(8, 3))), Verdict::Valid);
	assert_eq!(ctx.query(ctx.eq(r, ctx.bv_const(8, 2))), Verdict::Valid);
}

#[test]
fn model_for_boolean_variable() {
	let ctx = Context::new();
	let b = ctx.bool_var("b");
	// b is not valid; a counterexample must exist (b = false).
	assert_eq!(ctx.query(b), Verdict::Invalid);
	assert_eq!(ctx.model_bool(b), Some(false));
}

#[test]
fn smtlib2_rendering_is_nonempty() {
	let ctx = Context::new();
	let x = ctx.bv_var("x", 8);
	let formula = ctx.eq(x, ctx.bv_const(8, 0));
	let text = ctx.to_smtlib2(formula);
	assert!(!text.is_empty());
	assert!(text.contains('x'), "rendering should mention the symbol x: {text}");
}

#[test]
fn query_with_timeout_answers_easy_queries() {
	let ctx = Context::new();
	let x = ctx.bv_var("x", 8);
	let formula = ctx.eq(x, x);
	// A generous budget must still return Valid.
	assert_eq!(ctx.query_with_timeout(formula, -1, 60), Verdict::Valid);
}

#[test]
fn reason_unknown_is_none_after_a_decided_query() {
	let ctx = Context::new();
	let x = ctx.bv_var("x", 8);
	let _ = ctx.query(ctx.eq(x, x));
	assert_eq!(ctx.reason_unknown(), ReasonUnknown::None);
}

#[test]
fn sorts_are_inspectable() {
	let ctx = Context::new();
	let bv = ctx.bv_sort(24);
	let index = ctx.bv_sort(32);
	let element = ctx.bv_sort(8);
	let array = ctx.array_sort(&index, &element);

	assert_eq!(ctx.array_index_width(array), 32);
	assert_eq!(ctx.array_element_width(array), 8);
	assert_eq!(ctx.array_index_width(bv), 0);
}

#[test]
fn counterexample_size_is_available() {
	let ctx = Context::new();
	let x = ctx.bv_var("x", 8);
	let y = ctx.bv_var("y", 8);
	let neq = ctx.not(ctx.eq(x, y));
	assert_eq!(ctx.query(neq), Verdict::Invalid);
	assert!(ctx.counterexample_size() >= 1);
}

#[test]
fn bv_const_binary_and_decimal() {
	let ctx = Context::new();
	let binary = ctx.bv_const_binary("1010");
	assert_eq!(ctx.bv_width(binary), 4);
	let decimal = ctx.bv_const_decimal(8, "200");
	assert_eq!(ctx.query(ctx.eq(decimal, ctx.bv_const(8, 200))), Verdict::Valid);
}
