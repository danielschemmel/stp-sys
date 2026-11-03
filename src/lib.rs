//! Rust bindings for [STP](https://github.com/stp/stp), the Simple Theorem
//! Prover -- an SMT solver for the quantifier-free theories of bit-vectors,
//! arrays and floating point.
//!
//! This crate has two layers:
//!
//! * [`ffi`] contains the raw, `unsafe` translation of STP's 3.x C interface
//!   (`stp/stp.h`), and
//! * the crate root provides a small, safe wrapper ([`Context`], [`Sort`],
//!   [`Expr`]) built on top of it.
//!
//! # Building and linking
//!
//! By default the `vendor` feature is enabled. In that configuration the build
//! script downloads (or reuses) STP, builds its 3.x library as a **static**
//! library through the [`cmake`](https://crates.io/crates/cmake) crate, merges
//! the resulting archives plus their dependencies into one `libstp_sys.a`, and
//! links it into the crate. This requires a C++ toolchain and network access
//! the first time; see `build.rs` for the environment variables that override
//! the source, build and dependency locations.
//!
//! Disabling `vendor` (`default-features = false`) makes the build script link
//! against an STP 3.x installation already present on the system instead,
//! located with `STP_SYS_PREFIX` / `STP_SYS_LIB_DIR`.
//!
//! # Example
//!
//! ```
//! use stp_sys::{Context, Verdict};
//!
//! let ctx = Context::new();
//!
//! // Does 5 + 6 == 11 hold for all 32-bit values? (Obviously yes.)
//! let lhs = ctx.bv_add(ctx.bv_const(32, 5), ctx.bv_const(32, 6));
//! let eq = ctx.eq(lhs, ctx.bv_const(32, 11));
//! assert_eq!(ctx.query(eq), Verdict::Valid);
//!
//! // A satisfiable-but-not-valid formula, with a model.
//! let y = ctx.bv_var("y", 8);
//! let fits = ctx.bv_ult(y, ctx.bv_const(8, 10));
//! assert_eq!(ctx.query(fits), Verdict::Invalid); // not valid: y may be large
//! ```
//!
//! # Ownership model
//!
//! A [`Context`] owns a term manager and a solver over it. Every [`Sort`] and
//! [`Expr`] created from the context is interned in the manager and released
//! when the context is dropped. Handles are cheap, `Copy` tokens that borrow
//! the context through their lifetime, so they can be duplicated and passed
//! around freely. The wrappers are not thread-safe, because STP managers and
//! solvers are not.

#![allow(non_camel_case_types)]
#![warn(missing_docs)]

pub mod ffi;

use core::cell::Cell;
use core::ffi::{CStr, c_char, c_int};
use core::fmt;
use core::marker::PhantomData;
use core::ptr;

use std::ffi::CString;

/// The result of a [`Context::query`] call.
///
/// STP answers a *validity* query: [`Verdict::Valid`] means the query holds for
/// every assignment, while [`Verdict::Invalid`] means there is a counterexample
/// (a model of the query's negation), which can be read back with
/// [`Context::model_value`] and friends.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Verdict {
	/// The query is not valid; a counterexample exists.
	Invalid,
	/// The query is valid.
	Valid,
	/// An error occurred while answering the query.
	Error,
	/// The query could not be answered (for example a budget ran out).
	Unknown,
}

impl Verdict {
	/// Returns `true` for [`Verdict::Valid`].
	pub fn is_valid(self) -> bool {
		matches!(self, Verdict::Valid)
	}

	/// Returns `true` for [`Verdict::Invalid`].
	pub fn is_invalid(self) -> bool {
		matches!(self, Verdict::Invalid)
	}
}

/// Why the last query could not be answered.
///
/// Mirrors STP's `enum stp_unknown_reason`, recorded from the last
/// [`Context::query`] call.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ReasonUnknown {
	/// The last query was answered (or none has run).
	None,
	/// A wall-clock budget ran out.
	Timeout,
	/// A conflict budget ran out.
	ConflictBudget,
	/// The check was interrupted.
	Interrupted,
	/// Something stopped before an answer, with no more specific cause.
	Incomplete,
	/// A resource limit stopped the check.
	ResourceLimit,
	/// A declared sort's carrier was too narrow for the query.
	CarrierExhausted,
	/// An injectivity assumption could neither be confirmed nor retracted.
	AssumedInjectivity,
	/// The pipeline stopped after producing a CNF.
	StoppedAfterCnf,
	/// A cause this version of the bindings does not name.
	Other(c_int),
}

impl ReasonUnknown {
	fn from_raw(raw: ffi::stp_unknown_reason) -> ReasonUnknown {
		match raw {
			ffi::STP_REASON_NONE => ReasonUnknown::None,
			ffi::STP_REASON_TIMEOUT => ReasonUnknown::Timeout,
			ffi::STP_REASON_CONFLICT_LIMIT => ReasonUnknown::ConflictBudget,
			ffi::STP_REASON_INTERRUPTED => ReasonUnknown::Interrupted,
			ffi::STP_REASON_INCOMPLETE => ReasonUnknown::Incomplete,
			ffi::STP_REASON_RESOURCE_LIMIT => ReasonUnknown::ResourceLimit,
			ffi::STP_REASON_CARRIER_EXHAUSTED => ReasonUnknown::CarrierExhausted,
			ffi::STP_REASON_ASSUMED_INJECTIVITY => ReasonUnknown::AssumedInjectivity,
			ffi::STP_REASON_STOPPED_AFTER_CNF => ReasonUnknown::StoppedAfterCnf,
			other => ReasonUnknown::Other(other),
		}
	}
}

/// A sort (type) belonging to a [`Context`].
///
/// Sorts are pooled by their context's manager, so this is a `Copy` token whose
/// lifetime is tied to the context that created it.
pub struct Sort<'ctx> {
	raw: ffi::stp_sort,
	_marker: PhantomData<&'ctx Context>,
}

impl Clone for Sort<'_> {
	fn clone(&self) -> Self {
		*self
	}
}
impl Copy for Sort<'_> {}

impl fmt::Debug for Sort<'_> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("Sort").field("raw", &self.raw).finish()
	}
}

impl<'ctx> Sort<'ctx> {
	/// Wraps a raw STP sort handle.
	///
	/// # Safety
	///
	/// `raw` must be a non-null sort owned by `ctx`.
	pub unsafe fn from_raw(raw: ffi::stp_sort) -> Sort<'ctx> {
		Sort {
			raw,
			_marker: PhantomData,
		}
	}

	/// Returns the raw STP handle.
	pub fn as_raw(&self) -> ffi::stp_sort {
		self.raw
	}
}

/// An expression (term or formula) belonging to a [`Context`].
///
/// Like [`Sort`], an expression is interned in its context's manager, so this
/// is a `Copy` token with the context's lifetime.
pub struct Expr<'ctx> {
	raw: ffi::stp_term,
	_marker: PhantomData<&'ctx Context>,
}

impl Clone for Expr<'_> {
	fn clone(&self) -> Self {
		*self
	}
}
impl Copy for Expr<'_> {}

impl fmt::Debug for Expr<'_> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("Expr").field("raw", &self.raw).finish()
	}
}

impl<'ctx> Expr<'ctx> {
	/// Wraps a raw STP term handle.
	///
	/// # Safety
	///
	/// `raw` must be a non-null term owned by `ctx`.
	pub unsafe fn from_raw(raw: ffi::stp_term) -> Expr<'ctx> {
		Expr {
			raw,
			_marker: PhantomData,
		}
	}

	/// Returns the raw STP handle.
	pub fn as_raw(&self) -> ffi::stp_term {
		self.raw
	}
}

/// An STP solving context.
///
/// A context owns a term manager and a solver over it. It is the entry point
/// for the whole API: it creates sorts, expressions, assertions and queries,
/// and owns everything created from it. It is released by [`Drop`], which
/// deletes the solver and releases the manager.
pub struct Context {
	manager: ffi::stp_tm,
	solver: ffi::stp_solver,
	last_reason: Cell<ffi::stp_unknown_reason>,
}

impl Context {
	/// Creates a fresh STP context.
	pub fn new() -> Context {
		// Simplification is disabled at construction so that `expr_kind`
		// reports the structural kind of every term; `Context::simplify`
		// still performs the local rewrites on demand.
		let manager = unsafe { ffi::stp_tm_new_with(false, ffi::STP_RM_RNE, 16) };
		assert!(!manager.is_null(), "stp_tm_new_with returned null");
		let solver = unsafe { ffi::stp_solver_new(manager, ptr::null_mut()) };
		if solver.is_null() {
			unsafe { ffi::stp_tm_release(manager) };
			panic!("stp_solver_new returned null");
		}
		Context {
			manager,
			solver,
			last_reason: Cell::new(ffi::STP_REASON_NONE),
		}
	}

	/// Returns the raw STP solver handle.
	pub fn as_raw(&self) -> ffi::stp_solver {
		self.solver
	}

	/// Returns the raw STP term-manager handle.
	pub fn manager_raw(&self) -> ffi::stp_tm {
		self.manager
	}

	/// Returns the git tag (or version string) of the linked STP build.
	pub fn version() -> String {
		let version = unsafe { ffi::stp_get_version() };
		let tag = unsafe { c_ptr_to_string(version.git_tag) };
		if tag.is_empty() {
			unsafe { c_ptr_to_string(version.string) }
		} else {
			tag
		}
	}

	/// Returns the git SHA of the linked STP build.
	pub fn version_sha() -> String {
		let version = unsafe { ffi::stp_get_version() };
		unsafe { c_ptr_to_string(version.git_sha) }
	}

	/// Returns a description of the environment STP was compiled in.
	pub fn compilation_env() -> String {
		let version = unsafe { ffi::stp_get_version() };
		unsafe { c_ptr_to_string(version.build_info) }
	}

	// -- Sorts --------------------------------------------------------------

	/// Returns the Boolean sort.
	pub fn bool_sort(&self) -> Sort<'_> {
		self.wrap_sort(unsafe { ffi::stp_mk_bool_sort(self.manager) })
	}

	/// Returns a bit-vector sort of `width` bits.
	pub fn bv_sort(&self, width: u32) -> Sort<'_> {
		self.wrap_sort(unsafe { ffi::stp_mk_bv_sort(self.manager, width) })
	}

	/// Returns an array sort with the given index and element sorts.
	pub fn array_sort(&self, index: &Sort<'_>, element: &Sort<'_>) -> Sort<'_> {
		self.wrap_sort(unsafe { ffi::stp_mk_array_sort(self.manager, index.raw, element.raw) })
	}

	// -- Symbolic variables -------------------------------------------------

	/// Declares a Boolean variable named `name`.
	pub fn bool_var(&self, name: &str) -> Expr<'_> {
		let cname = cstring(name);
		let sort = self.bool_sort();
		self.wrap_expr(unsafe { ffi::stp_declare(self.manager, cname.as_ptr(), sort.raw) })
	}

	/// Declares a bit-vector variable of `width` bits named `name`.
	pub fn bv_var(&self, name: &str, width: u32) -> Expr<'_> {
		let cname = cstring(name);
		let sort = self.bv_sort(width);
		self.wrap_expr(unsafe { ffi::stp_declare(self.manager, cname.as_ptr(), sort.raw) })
	}

	/// Declares an array variable named `name`.
	pub fn array_var(&self, name: &str, index: &Sort<'_>, element: &Sort<'_>) -> Expr<'_> {
		let cname = cstring(name);
		let sort = self.array_sort(index, element);
		self.wrap_expr(unsafe { ffi::stp_declare(self.manager, cname.as_ptr(), sort.raw) })
	}

	// -- Constants ----------------------------------------------------------

	/// Returns the Boolean constant `true` or `false`.
	pub fn bool_const(&self, value: bool) -> Expr<'_> {
		self.wrap_expr(unsafe { ffi::stp_mk_bool(self.manager, value) })
	}

	/// Returns a `width`-bit bit-vector constant from a 64-bit value.
	///
	/// The value is wrapped modulo `2^width`, matching STP's bit-vector
	/// constant semantics.
	pub fn bv_const(&self, width: u32, value: u64) -> Expr<'_> {
		self.wrap_expr(unsafe { ffi::stp_mk_bv_wrapped(self.manager, width, value) })
	}

	/// Returns a bit-vector constant parsed from a binary string such as
	/// `"1011"`. The string's length determines the width.
	pub fn bv_const_binary(&self, bits: &str) -> Expr<'_> {
		let cbits = cstring(bits);
		let width = u32::try_from(bits.len()).expect("bit-vector literal is too wide");
		self.wrap_expr(unsafe { ffi::stp_mk_bv_str(self.manager, width, cbits.as_ptr(), 2) })
	}

	/// Returns a `width`-bit bit-vector constant from a decimal string.
	pub fn bv_const_decimal(&self, width: u32, decimal: &str) -> Expr<'_> {
		let cdec = cstring(decimal);
		self.wrap_expr(unsafe { ffi::stp_mk_bv_str(self.manager, width, cdec.as_ptr(), 10) })
	}

	// -- Boolean operations -------------------------------------------------

	/// Equality of two expressions of the same sort.
	pub fn eq(&self, left: Expr<'_>, right: Expr<'_>) -> Expr<'_> {
		self.term2(ffi::STP_KIND_EQUAL, left, right)
	}

	/// Logical negation.
	pub fn not(&self, child: Expr<'_>) -> Expr<'_> {
		self.term1(ffi::STP_KIND_NOT, child)
	}

	/// Logical conjunction.
	pub fn and(&self, left: Expr<'_>, right: Expr<'_>) -> Expr<'_> {
		self.term2(ffi::STP_KIND_AND, left, right)
	}

	/// Logical conjunction of a slice of formulas.
	pub fn and_all(&self, children: &[Expr<'_>]) -> Expr<'_> {
		let mut iter = children.iter().copied();
		let Some(first) = iter.next() else {
			return self.bool_const(true);
		};
		let mut acc = self.wrap_expr(first.raw);
		for child in iter {
			acc = self.and(acc, child);
		}
		acc
	}

	/// Logical disjunction.
	pub fn or(&self, left: Expr<'_>, right: Expr<'_>) -> Expr<'_> {
		self.term2(ffi::STP_KIND_OR, left, right)
	}

	/// Logical disjunction of a slice of formulas.
	pub fn or_all(&self, children: &[Expr<'_>]) -> Expr<'_> {
		let mut iter = children.iter().copied();
		let Some(first) = iter.next() else {
			return self.bool_const(false);
		};
		let mut acc = self.wrap_expr(first.raw);
		for child in iter {
			acc = self.or(acc, child);
		}
		acc
	}

	/// Logical exclusive-or.
	pub fn xor(&self, left: Expr<'_>, right: Expr<'_>) -> Expr<'_> {
		self.term2(ffi::STP_KIND_XOR, left, right)
	}

	/// Logical not-and.
	pub fn nand(&self, left: Expr<'_>, right: Expr<'_>) -> Expr<'_> {
		self.not(self.and(left, right))
	}

	/// Logical not-or.
	pub fn nor(&self, left: Expr<'_>, right: Expr<'_>) -> Expr<'_> {
		self.not(self.or(left, right))
	}

	/// Logical implication (`hyp` implies `conc`).
	pub fn implies(&self, hyp: Expr<'_>, conc: Expr<'_>) -> Expr<'_> {
		self.term2(ffi::STP_KIND_IMPLIES, hyp, conc)
	}

	/// Logical if-and-only-if.
	pub fn iff(&self, left: Expr<'_>, right: Expr<'_>) -> Expr<'_> {
		self.eq(left, right)
	}

	/// If-then-else over expressions of a common sort.
	pub fn ite(&self, condition: Expr<'_>, then_expr: Expr<'_>, else_expr: Expr<'_>) -> Expr<'_> {
		self.term3(ffi::STP_KIND_ITE, condition, then_expr, else_expr)
	}

	/// Converts a Boolean formula into a one-bit bit-vector.
	pub fn bool_to_bv(&self, formula: Expr<'_>) -> Expr<'_> {
		self.wrap_expr(unsafe { ffi::stp_bool_to_bv1(self.manager, formula.raw) })
	}

	// -- Arrays -------------------------------------------------------------

	/// Array read: `array[index]`.
	pub fn read(&self, array: Expr<'_>, index: Expr<'_>) -> Expr<'_> {
		self.term2(ffi::STP_KIND_SELECT, array, index)
	}

	/// Array write: `array[index := value]`.
	pub fn write(&self, array: Expr<'_>, index: Expr<'_>, value: Expr<'_>) -> Expr<'_> {
		self.term3(ffi::STP_KIND_STORE, array, index, value)
	}

	// -- Bit-vector arithmetic ----------------------------------------------

	/// Bit-vector addition, at the operands' width.
	pub fn bv_add(&self, left: Expr<'_>, right: Expr<'_>) -> Expr<'_> {
		self.term2(ffi::STP_KIND_BV_ADD, left, right)
	}

	/// Bit-vector addition at an explicit width.
	pub fn bv_add_width(&self, _width: u32, left: Expr<'_>, right: Expr<'_>) -> Expr<'_> {
		self.bv_add(left, right)
	}

	/// N-ary bit-vector addition at an explicit width.
	pub fn bv_add_all(&self, width: u32, children: &[Expr<'_>]) -> Expr<'_> {
		let mut iter = children.iter().copied();
		let Some(first) = iter.next() else {
			return self.bv_const(width, 0);
		};
		let mut acc = self.wrap_expr(first.raw);
		for child in iter {
			acc = self.bv_add_width(width, acc, child);
		}
		acc
	}

	/// Bit-vector subtraction, at the operands' width.
	pub fn bv_sub(&self, left: Expr<'_>, right: Expr<'_>) -> Expr<'_> {
		self.term2(ffi::STP_KIND_BV_SUB, left, right)
	}

	/// Bit-vector multiplication, at the operands' width.
	pub fn bv_mul(&self, left: Expr<'_>, right: Expr<'_>) -> Expr<'_> {
		self.term2(ffi::STP_KIND_BV_MUL, left, right)
	}

	/// Unsigned bit-vector division, at the operands' width.
	pub fn bv_udiv(&self, dividend: Expr<'_>, divisor: Expr<'_>) -> Expr<'_> {
		self.term2(ffi::STP_KIND_BV_UDIV, dividend, divisor)
	}

	/// Unsigned bit-vector modulo, at the operands' width.
	pub fn bv_umod(&self, dividend: Expr<'_>, divisor: Expr<'_>) -> Expr<'_> {
		self.term2(ffi::STP_KIND_BV_UREM, dividend, divisor)
	}

	/// Unsigned bit-vector remainder, at the operands' width.
	pub fn bv_urem(&self, dividend: Expr<'_>, divisor: Expr<'_>) -> Expr<'_> {
		self.term2(ffi::STP_KIND_BV_UREM, dividend, divisor)
	}

	/// Signed bit-vector division, at the operands' width.
	pub fn bv_sdiv(&self, dividend: Expr<'_>, divisor: Expr<'_>) -> Expr<'_> {
		self.term2(ffi::STP_KIND_BV_SDIV, dividend, divisor)
	}

	/// Signed bit-vector modulo, at the operands' width.
	pub fn bv_smod(&self, dividend: Expr<'_>, divisor: Expr<'_>) -> Expr<'_> {
		self.term2(ffi::STP_KIND_BV_SMOD, dividend, divisor)
	}

	/// Signed bit-vector remainder, at the operands' width.
	pub fn bv_srem(&self, dividend: Expr<'_>, divisor: Expr<'_>) -> Expr<'_> {
		self.term2(ffi::STP_KIND_BV_SREM, dividend, divisor)
	}

	/// Bit-vector two's-complement negation.
	pub fn bv_neg(&self, child: Expr<'_>) -> Expr<'_> {
		self.term1(ffi::STP_KIND_BV_NEG, child)
	}

	// -- Bitwise operations -------------------------------------------------

	/// Bitwise and.
	pub fn bv_and(&self, left: Expr<'_>, right: Expr<'_>) -> Expr<'_> {
		self.term2(ffi::STP_KIND_BV_AND, left, right)
	}

	/// Bitwise or.
	pub fn bv_or(&self, left: Expr<'_>, right: Expr<'_>) -> Expr<'_> {
		self.term2(ffi::STP_KIND_BV_OR, left, right)
	}

	/// Bitwise xor.
	pub fn bv_xor(&self, left: Expr<'_>, right: Expr<'_>) -> Expr<'_> {
		self.term2(ffi::STP_KIND_BV_XOR, left, right)
	}

	/// Bitwise not.
	pub fn bv_not(&self, child: Expr<'_>) -> Expr<'_> {
		self.term1(ffi::STP_KIND_BV_NOT, child)
	}

	/// Bitwise nand.
	pub fn bv_nand(&self, left: Expr<'_>, right: Expr<'_>) -> Expr<'_> {
		self.term2(ffi::STP_KIND_BV_NAND, left, right)
	}

	/// Bitwise nor.
	pub fn bv_nor(&self, left: Expr<'_>, right: Expr<'_>) -> Expr<'_> {
		self.term2(ffi::STP_KIND_BV_NOR, left, right)
	}

	/// Bitwise xnor.
	pub fn bv_xnor(&self, left: Expr<'_>, right: Expr<'_>) -> Expr<'_> {
		self.term2(ffi::STP_KIND_BV_XNOR, left, right)
	}

	/// Concatenates two bit-vectors; `left` becomes the high bits.
	pub fn bv_concat(&self, left: Expr<'_>, right: Expr<'_>) -> Expr<'_> {
		self.term2(ffi::STP_KIND_BV_CONCAT, left, right)
	}

	/// Extracts bits `high..=low` (inclusive), producing a `high - low + 1`-bit
	/// vector.
	pub fn bv_extract(&self, child: Expr<'_>, high: u32, low: u32) -> Expr<'_> {
		self.wrap_expr(unsafe { ffi::stp_extract(self.manager, high, low, child.raw) })
	}

	/// Tests whether bit `bit` of `child` is set, as a Boolean formula.
	pub fn bv_bit(&self, child: Expr<'_>, bit: u32) -> Expr<'_> {
		self.wrap_expr(unsafe { ffi::stp_bit(self.manager, child.raw, bit) })
	}

	/// Sign-extends (or truncates) `child` to `new_width` bits.
	pub fn bv_sign_extend(&self, child: Expr<'_>, new_width: u32) -> Expr<'_> {
		assert!(new_width > 0, "the new bit-vector width must be positive");
		let width = self.bv_width(child);
		if new_width >= width {
			self.wrap_expr(unsafe { ffi::stp_sign_extend(self.manager, new_width - width, child.raw) })
		} else {
			self.bv_extract(child, new_width - 1, 0)
		}
	}

	/// Zero-extends (or truncates) `child` to `new_width` bits.
	pub fn bv_zero_extend(&self, child: Expr<'_>, new_width: u32) -> Expr<'_> {
		assert!(new_width > 0, "the new bit-vector width must be positive");
		let width = self.bv_width(child);
		if new_width >= width {
			self.wrap_expr(unsafe { ffi::stp_zero_extend(self.manager, new_width - width, child.raw) })
		} else {
			self.bv_extract(child, new_width - 1, 0)
		}
	}

	/// Left shift by a constant amount.
	///
	/// STP implements this by concatenating `amount` zero bits below the
	/// operand, so the result is `amount` bits **wider** than `child`. Use
	/// [`Context::bv_shl_expr`] for a shift that preserves the width.
	pub fn bv_shl(&self, child: Expr<'_>, amount: u32) -> Expr<'_> {
		if amount == 0 {
			self.wrap_expr(child.raw)
		} else {
			let zeros = self.bv_const(amount, 0);
			self.bv_concat(child, zeros)
		}
	}

	/// Logical right shift by a constant amount. The result keeps `child`'s
	/// width and its top `amount` bits become zero.
	pub fn bv_lshr(&self, child: Expr<'_>, amount: u32) -> Expr<'_> {
		let width = self.bv_width(child);
		if amount == 0 {
			return self.wrap_expr(child.raw);
		}
		if amount >= width {
			return self.bv_const(width, 0);
		}
		let shift = self.bv_const(width, u64::from(amount));
		self.term2(ffi::STP_KIND_BV_LSHR, child, shift)
	}

	/// Left shift by a bit-vector amount. The result has `child`'s width.
	pub fn bv_shl_expr(&self, child: Expr<'_>, amount: Expr<'_>) -> Expr<'_> {
		self.term2(ffi::STP_KIND_BV_SHL, child, amount)
	}

	/// Logical right shift by a bit-vector amount. The result has `child`'s
	/// width.
	pub fn bv_lshr_expr(&self, child: Expr<'_>, amount: Expr<'_>) -> Expr<'_> {
		self.term2(ffi::STP_KIND_BV_LSHR, child, amount)
	}

	/// Arithmetic (signed) right shift by a bit-vector amount.
	pub fn bv_ashr_expr(&self, child: Expr<'_>, amount: Expr<'_>) -> Expr<'_> {
		self.term2(ffi::STP_KIND_BV_ASHR, child, amount)
	}

	// -- Bit-vector comparisons ---------------------------------------------

	/// Unsigned `left < right`.
	pub fn bv_ult(&self, left: Expr<'_>, right: Expr<'_>) -> Expr<'_> {
		self.term2(ffi::STP_KIND_BV_ULT, left, right)
	}

	/// Unsigned `left <= right`.
	pub fn bv_ule(&self, left: Expr<'_>, right: Expr<'_>) -> Expr<'_> {
		self.term2(ffi::STP_KIND_BV_ULE, left, right)
	}

	/// Unsigned `left > right`.
	pub fn bv_ugt(&self, left: Expr<'_>, right: Expr<'_>) -> Expr<'_> {
		self.term2(ffi::STP_KIND_BV_UGT, left, right)
	}

	/// Unsigned `left >= right`.
	pub fn bv_uge(&self, left: Expr<'_>, right: Expr<'_>) -> Expr<'_> {
		self.term2(ffi::STP_KIND_BV_UGE, left, right)
	}

	/// Signed `left < right`.
	pub fn bv_slt(&self, left: Expr<'_>, right: Expr<'_>) -> Expr<'_> {
		self.term2(ffi::STP_KIND_BV_SLT, left, right)
	}

	/// Signed `left <= right`.
	pub fn bv_sle(&self, left: Expr<'_>, right: Expr<'_>) -> Expr<'_> {
		self.term2(ffi::STP_KIND_BV_SLE, left, right)
	}

	/// Signed `left > right`.
	pub fn bv_sgt(&self, left: Expr<'_>, right: Expr<'_>) -> Expr<'_> {
		self.term2(ffi::STP_KIND_BV_SGT, left, right)
	}

	/// Signed `left >= right`.
	pub fn bv_sge(&self, left: Expr<'_>, right: Expr<'_>) -> Expr<'_> {
		self.term2(ffi::STP_KIND_BV_SGE, left, right)
	}

	/// Unsigned addition overflow predicate.
	pub fn bv_uaddo(&self, left: Expr<'_>, right: Expr<'_>) -> Expr<'_> {
		self.term2(ffi::STP_KIND_BV_UADDO, left, right)
	}

	/// Signed addition overflow predicate.
	pub fn bv_saddo(&self, left: Expr<'_>, right: Expr<'_>) -> Expr<'_> {
		self.term2(ffi::STP_KIND_BV_SADDO, left, right)
	}

	/// Unsigned subtraction overflow predicate.
	pub fn bv_usubo(&self, left: Expr<'_>, right: Expr<'_>) -> Expr<'_> {
		self.term2(ffi::STP_KIND_BV_USUBO, left, right)
	}

	/// Signed subtraction overflow predicate.
	pub fn bv_ssubo(&self, left: Expr<'_>, right: Expr<'_>) -> Expr<'_> {
		self.term2(ffi::STP_KIND_BV_SSUBO, left, right)
	}

	/// Unsigned multiplication overflow predicate.
	pub fn bv_umulo(&self, left: Expr<'_>, right: Expr<'_>) -> Expr<'_> {
		self.term2(ffi::STP_KIND_BV_UMULO, left, right)
	}

	/// Signed multiplication overflow predicate.
	pub fn bv_smulo(&self, left: Expr<'_>, right: Expr<'_>) -> Expr<'_> {
		self.term2(ffi::STP_KIND_BV_SMULO, left, right)
	}

	// -- Solving ------------------------------------------------------------

	/// Asserts that `formula` (which must be Boolean) holds.
	pub fn assert(&self, formula: Expr<'_>) {
		let status = unsafe { ffi::stp_solver_assert(self.solver, formula.raw) };
		self.ensure_status(status, "stp_solver_assert");
	}

	/// Simplifies `expression` with respect to the current assertions.
	pub fn simplify(&self, expression: Expr<'_>) -> Expr<'_> {
		self.wrap_expr(unsafe { ffi::stp_tm_simplify(self.manager, expression.raw) })
	}

	/// Checks whether `query` (which must be Boolean) is valid, with no budget.
	pub fn query(&self, query: Expr<'_>) -> Verdict {
		self.entails(query, ptr::null())
	}

	/// Checks whether `query` is valid within the given conflict and
	/// wall-clock (seconds) budgets. A budget of `-1` means "no limit".
	pub fn query_with_timeout(&self, query: Expr<'_>, max_conflicts: i32, max_time_secs: i32) -> Verdict {
		if max_conflicts < 0 && max_time_secs < 0 {
			return self.query(query);
		}
		let budget = ffi::stp_budget {
			has_time: max_time_secs >= 0,
			time_ms: (max_time_secs.max(0) as u64).saturating_mul(1000),
			has_conflicts: max_conflicts >= 0,
			conflicts: max_conflicts.max(0) as u64,
		};
		self.entails(query, &budget)
	}

	fn entails(&self, query: Expr<'_>, budget: *const ffi::stp_budget) -> Verdict {
		let mut answer = ffi::stp_entailment {
			kind: ffi::STP_UNKNOWN_VALIDITY,
			reason: ffi::STP_REASON_NONE,
		};
		let status = unsafe { ffi::stp_solver_entails(self.solver, query.raw, budget, &mut answer) };
		self.last_reason.set(answer.reason);
		if status != ffi::STP_OK {
			return Verdict::Error;
		}
		match answer.kind {
			ffi::STP_VALID => Verdict::Valid,
			ffi::STP_INVALID => Verdict::Invalid,
			_ => Verdict::Unknown,
		}
	}

	/// Returns why the last [`Context::query`] could not be answered.
	pub fn reason_unknown(&self) -> ReasonUnknown {
		ReasonUnknown::from_raw(self.last_reason.get())
	}

	/// Opens a new assertion scope; [`Context::pop`] retracts it.
	pub fn push(&self) {
		let status = unsafe { ffi::stp_solver_push(self.solver, 1) };
		self.ensure_status(status, "stp_solver_push");
	}

	/// Retracts every assertion added since the matching [`Context::push`].
	pub fn pop(&self) {
		let status = unsafe { ffi::stp_solver_pop(self.solver, 1) };
		self.ensure_status(status, "stp_solver_pop");
	}

	/// Returns the value of `expression` in the counterexample of the last
	/// query, or `None` if there is no model to read.
	pub fn model_value(&self, expression: Expr<'_>) -> Option<Expr<'_>> {
		let model = unsafe { ffi::stp_solver_model(self.solver) };
		if model.is_null() {
			return None;
		}
		let raw = unsafe { ffi::stp_model_value(model, expression.raw) };
		unsafe { ffi::stp_model_release(model) };
		if raw.is_null() { None } else { Some(self.wrap_expr(raw)) }
	}

	/// Returns the number of assignments in the current counterexample.
	pub fn counterexample_size(&self) -> u32 {
		let model = unsafe { ffi::stp_solver_model(self.solver) };
		if model.is_null() {
			return 0;
		}
		let size = unsafe { ffi::stp_model_num_symbols(model) };
		unsafe { ffi::stp_model_release(model) };
		u32::try_from(size).unwrap_or(u32::MAX)
	}

	/// Reads a bit-vector variable's value from the last counterexample as a
	/// `u64`, or `None` if there is no model or the value does not fit.
	pub fn model_bv_u64(&self, expression: Expr<'_>) -> Option<u64> {
		let model = unsafe { ffi::stp_solver_model(self.solver) };
		if model.is_null() {
			return None;
		}
		let mut value = 0u64;
		let status = unsafe { ffi::stp_model_uint64(model, expression.raw, &mut value) };
		unsafe { ffi::stp_model_release(model) };
		(status == ffi::STP_OK).then_some(value)
	}

	/// Reads a Boolean variable's value from the last counterexample, or
	/// `None` if there is no model for it.
	pub fn model_bool(&self, expression: Expr<'_>) -> Option<bool> {
		let model = unsafe { ffi::stp_solver_model(self.solver) };
		if model.is_null() {
			return None;
		}
		let mut value = false;
		let status = unsafe { ffi::stp_model_bool(model, expression.raw, &mut value) };
		unsafe { ffi::stp_model_release(model) };
		(status == ffi::STP_OK).then_some(value)
	}

	// -- Inspection and printing --------------------------------------------

	/// Returns the sort of `expression`.
	pub fn sort_of(&self, expression: Expr<'_>) -> Sort<'_> {
		self.wrap_sort(unsafe { ffi::stp_term_sort(expression.raw) })
	}

	/// Returns the width of a bit-vector expression.
	///
	/// # Panics
	///
	/// Panics if `expression` is not a bit-vector.
	pub fn bv_width(&self, expression: Expr<'_>) -> u32 {
		let sort = self.sort_of(expression);
		let mut width = 0u32;
		let status = unsafe { ffi::stp_sort_bv_size(sort.raw, &mut width) };
		assert!(status == ffi::STP_OK, "expression is not a bit-vector");
		width
	}

	/// Returns the index width of an array sort.
	pub fn array_index_width(&self, sort: Sort<'_>) -> u32 {
		let index = unsafe { ffi::stp_sort_array_index(sort.raw) };
		if index.is_null() {
			return 0;
		}
		let mut width = 0u32;
		let status = unsafe { ffi::stp_sort_bv_size(index, &mut width) };
		if status == ffi::STP_OK { width } else { 0 }
	}

	/// Returns the element width of a bit-vector-element array sort.
	pub fn array_element_width(&self, sort: Sort<'_>) -> u32 {
		let element = unsafe { ffi::stp_sort_array_element(sort.raw) };
		if element.is_null() {
			return 0;
		}
		let mut width = 0u32;
		let status = unsafe { ffi::stp_sort_bv_size(element, &mut width) };
		if status == ffi::STP_OK { width } else { 0 }
	}

	/// Returns whether `expression` is Boolean.
	pub fn is_bool(&self, expression: Expr<'_>) -> bool {
		let sort = unsafe { ffi::stp_term_sort(expression.raw) };
		if sort.is_null() {
			return false;
		}
		let mut kind = ffi::STP_SORT_BOOL;
		let status = unsafe { ffi::stp_sort_get_kind(sort, &mut kind) };
		status == ffi::STP_OK && kind == ffi::STP_SORT_BOOL
	}

	/// Returns the raw kind of `expression` (see the `ffi::STP_KIND_*`
	/// constants).
	pub fn expr_kind(&self, expression: Expr<'_>) -> ffi::stp_kind {
		let mut kind = ffi::STP_KIND_VALUE;
		let status = unsafe { ffi::stp_term_get_kind(expression.raw, &mut kind) };
		self.ensure_status(status, "stp_term_get_kind");
		kind
	}

	/// Renders `expression` as an SMT-LIB 2 term.
	pub fn to_smtlib2(&self, expression: Expr<'_>) -> String {
		let raw = unsafe { ffi::stp_term_str(expression.raw) };
		if raw.is_null() {
			return String::new();
		}
		let string = unsafe { c_ptr_to_string(raw) };
		unsafe { ffi::stp_free(raw.cast()) };
		string
	}

	/// Renders a sort as an SMT-LIB 2 sort.
	pub fn sort_to_string(&self, sort: Sort<'_>) -> String {
		let raw = unsafe { ffi::stp_sort_str(sort.raw) };
		if raw.is_null() {
			return String::new();
		}
		let string = unsafe { c_ptr_to_string(raw) };
		unsafe { ffi::stp_free(raw.cast()) };
		string
	}

	// -- Internal helpers ---------------------------------------------------

	fn term1(&self, kind: ffi::stp_kind, a: Expr<'_>) -> Expr<'_> {
		self.wrap_expr(unsafe { ffi::stp_mk_term1(self.manager, kind, a.raw) })
	}

	fn term2(&self, kind: ffi::stp_kind, a: Expr<'_>, b: Expr<'_>) -> Expr<'_> {
		self.wrap_expr(unsafe { ffi::stp_mk_term2(self.manager, kind, a.raw, b.raw) })
	}

	fn term3(&self, kind: ffi::stp_kind, a: Expr<'_>, b: Expr<'_>, c: Expr<'_>) -> Expr<'_> {
		self.wrap_expr(unsafe { ffi::stp_mk_term3(self.manager, kind, a.raw, b.raw, c.raw) })
	}

	fn ensure_status(&self, status: ffi::stp_status, what: &str) {
		if status != ffi::STP_OK {
			panic!("{what} failed: {}", self.error_message());
		}
	}

	fn error_message(&self) -> String {
		let mut record = unsafe { ffi::stp_tm_error(self.manager) };
		if record.is_null() {
			record = unsafe { ffi::stp_solver_failed(self.solver) };
		}
		if record.is_null() {
			return "unknown STP error".to_owned();
		}
		let error = unsafe { &*record };
		let function = unsafe { c_ptr_to_string(error.function) };
		let message = unsafe { c_ptr_to_string(error.message) };
		format!("{function}: {message} (code {})", error.code)
	}

	fn wrap_expr<'ctx>(&'ctx self, raw: ffi::stp_term) -> Expr<'ctx> {
		if raw.is_null() {
			panic!("STP returned a null expression: {}", self.error_message());
		}
		Expr {
			raw,
			_marker: PhantomData,
		}
	}

	fn wrap_sort<'ctx>(&'ctx self, raw: ffi::stp_sort) -> Sort<'ctx> {
		if raw.is_null() {
			panic!("STP returned a null sort: {}", self.error_message());
		}
		Sort {
			raw,
			_marker: PhantomData,
		}
	}
}

impl Default for Context {
	fn default() -> Context {
		Context::new()
	}
}

impl Drop for Context {
	fn drop(&mut self) {
		unsafe {
			ffi::stp_solver_delete(self.solver);
			ffi::stp_tm_release(self.manager);
		}
	}
}

impl fmt::Debug for Context {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("Context")
			.field("stp_version", &Context::version())
			.finish()
	}
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Converts an STP-owned C string to a Rust `String`.
///
/// # Safety
///
/// `ptr` must be null or point to a valid NUL-terminated string.
unsafe fn c_ptr_to_string(ptr: *const c_char) -> String {
	if ptr.is_null() {
		return String::new();
	}
	unsafe { CStr::from_ptr(ptr) }.to_string_lossy().into_owned()
}

fn cstring(value: &str) -> CString {
	CString::new(value).expect("name contains an interior NUL byte")
}

#[cfg(test)]
mod test;
