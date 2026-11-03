//! Tests for basic types and handles.

use super::*;

#[test]
fn verdict_predicates() {
	assert!(Verdict::Valid.is_valid());
	assert!(!Verdict::Valid.is_invalid());
	assert!(Verdict::Invalid.is_invalid());
	assert!(!Verdict::Invalid.is_valid());
	assert!(!Verdict::Unknown.is_valid());
	assert!(!Verdict::Unknown.is_invalid());
	assert!(!Verdict::Error.is_valid());
	assert!(!Verdict::Error.is_invalid());
}

#[test]
fn version_and_build_metadata() {
	let tag = Context::version();
	assert!(!tag.is_empty());
	assert!(tag.chars().next().unwrap().is_ascii_digit(), "unexpected tag {tag:?}");
	assert_eq!(Context::version_sha().len(), 40, "git SHA should be 40 hex digits");
	assert!(!Context::compilation_env().is_empty());
}

#[test]
fn default_context_and_debug_rendering() {
	let ctx = Context::default();
	let debug = format!("{ctx:?}");
	assert!(debug.contains("Context"));
	assert!(debug.contains("stp_version"));
}

#[test]
fn raw_handle_round_trips_and_debug() {
	let ctx = ctx();
	assert!(!ctx.as_raw().is_null());
	assert!(!ctx.manager_raw().is_null());

	let sort = ctx.bv_sort(8);
	let sort_raw = sort.as_raw();
	let sort_from_raw = unsafe { Sort::from_raw(sort_raw) };
	assert_eq!(sort_from_raw.as_raw(), sort_raw);
	let sort_copy = sort_from_raw;
	#[allow(clippy::clone_on_copy)]
	let sort_clone = sort_from_raw.clone();
	assert_eq!(sort_copy.as_raw(), sort_clone.as_raw());
	assert!(format!("{sort:?}").contains("Sort"));

	let expr = ctx.bv_var("x", 8);
	let expr_raw = expr.as_raw();
	let expr_from_raw = unsafe { Expr::from_raw(expr_raw) };
	assert_eq!(expr_from_raw.as_raw(), expr_raw);
	let expr_copy = expr_from_raw;
	#[allow(clippy::clone_on_copy)]
	let expr_clone = expr_from_raw.clone();
	assert_eq!(expr_copy.as_raw(), expr_clone.as_raw());
	assert!(format!("{expr:?}").contains("Expr"));
}
