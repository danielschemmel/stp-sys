//! Unit tests for the safe STP wrapper and the vendored SAT backends.
//!
//! The wrapper tests are grouped by API area and deliberately call every
//! public function of [`crate`] at least once. Each vendored SAT backend has a
//! child module of the same name under `test::backend` (for example
//! `test::backend::cadical`).

use super::*;

fn ctx() -> Context {
	Context::new()
}

fn assert_valid(ctx: &Context, formula: Expr<'_>) {
	assert_eq!(ctx.query(formula), Verdict::Valid);
}

fn assert_invalid(ctx: &Context, formula: Expr<'_>) {
	assert_eq!(ctx.query(formula), Verdict::Invalid);
}

// ---------------------------------------------------------------------------
// Safe wrapper tests, grouped by API area
// ---------------------------------------------------------------------------

mod arrays;
mod basic;
mod bitvector_arithmetic;
mod bitwise;
mod boolean;
mod comparisons;
mod inspection;
mod models;
mod solving;
mod sorts;

// ---------------------------------------------------------------------------
// Vendored SAT backends
// ---------------------------------------------------------------------------

#[cfg(feature = "vendor")]
mod backend;
