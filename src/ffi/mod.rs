//! Raw, `unsafe` bindings to the STP 3.x C API (`stp/stp.h`).
//!
//! This module is a thin translation of the flat C99 interface exported by
//! `libstp`. Every item maps directly onto a function, type, struct or constant
//! declared by `stp.h` (or by the generated `stp/api/gen/*.h` headers it
//! includes); no memory management, lifetime checking or error handling is
//! added here. Most users should prefer the safe wrapper exposed at the crate
//! root ([`crate::Context`], [`crate::Expr`] and [`crate::Sort`]), which is
//! built on top of this module.
//!
//! # Handles and lifetimes
//!
//! The 3.x API is split into a term manager ([`stp_tm`]) and one or more
//! solvers ([`stp_solver`]). Terms ([`stp_term`]) and sorts ([`stp_sort`]) are
//! interned in a manager; a handle returned by a constructor carries one
//! reference owned by the caller. The safe wrapper keeps those references
//! alive until its context is dropped.
//!
//! Every fallible function returns a handle (`NULL` on error) or an
//! [`stp_status`]. The first error is recorded on the manager and can be read
//! with [`stp_tm_error`]; strings and buffers returned as `char*` are owned by
//! the caller and must be released with [`stp_free`].

#![allow(non_camel_case_types)]

#[cfg(test)]
mod test;

use core::ffi::{c_char, c_int, c_void};

// ---------------------------------------------------------------------------
// Opaque handles
// ---------------------------------------------------------------------------

/// A term manager: owns interned terms, sorts and the name table.
pub type stp_tm = *mut c_void;
/// A solver over a term manager: owns assertions, a stack and models.
pub type stp_solver = *mut c_void;
/// A standalone options value.
pub type stp_options = *mut c_void;
/// A detached model snapshot.
pub type stp_model = *mut c_void;
/// An interned term (the engine node itself).
pub type stp_term = *mut c_void;
/// An interned sort; sorts are pooled by the manager.
pub type stp_sort = *mut c_void;
/// A model's array value.
pub type stp_array_value = *mut c_void;
/// A model's function value.
pub type stp_fun_value = *mut c_void;
/// A statistics snapshot.
pub type stp_statistics = *mut c_void;

// ---------------------------------------------------------------------------
// Enumerations
// ---------------------------------------------------------------------------

/// `enum stp_status`: the result of a fallible operation.
pub type stp_status = c_int;
/// `enum stp_sort_kind`: the class of a sort.
pub type stp_sort_kind = c_int;
/// `enum stp_rm`: an IEEE-754 rounding mode.
pub type stp_rm = c_int;
/// `enum stp_result_kind`: a satisfiability answer.
pub type stp_result_kind = c_int;
/// `enum stp_validity`: a validity / entailment answer.
pub type stp_validity = c_int;
/// `enum stp_unknown_reason`: why a check answered unknown.
pub type stp_unknown_reason = c_int;
/// `enum stp_format`: an input or output syntax.
pub type stp_format = c_int;
/// `enum stp_parse_mode`: how an SMT-LIB script is applied.
pub type stp_parse_mode = c_int;
/// `enum stp_cnf_scope`: how a CNF relates to the assertions.
pub type stp_cnf_scope = c_int;
/// `enum stp_tier`: an option's stability tier.
pub type stp_tier = c_int;
/// `enum stp_settable`: when an option may be written.
pub type stp_settable = c_int;
/// `enum stp_option_scope`: whether an option belongs to a solver or manager.
pub type stp_option_scope = c_int;
/// `enum stp_fp_class`: the class of a floating-point value.
pub type stp_fp_class = c_int;
/// `enum stp_internal_error_policy`: what happens after an internal error.
pub type stp_internal_error_policy = c_int;
/// `enum stp_kind`: the public kind of a term.
pub type stp_kind = c_int;
/// `enum stp_error_code`: a specific API error.
pub type stp_error_code = c_int;
/// `enum stp_option`: a stable option.
pub type stp_option = c_int;

// stp_status
/// The operation succeeded.
pub const STP_OK: stp_status = 0;
/// The operation failed; see the object's error record.
pub const STP_ERROR: stp_status = 1;

// stp_sort_kind
/// Boolean sort.
pub const STP_SORT_BOOL: stp_sort_kind = 0;
/// Bit-vector sort.
pub const STP_SORT_BV: stp_sort_kind = 1;
/// Floating-point sort.
pub const STP_SORT_FP: stp_sort_kind = 2;
/// Rounding-mode sort.
pub const STP_SORT_RM: stp_sort_kind = 3;
/// Exact Real sort.
pub const STP_SORT_REAL: stp_sort_kind = 4;
/// Array sort.
pub const STP_SORT_ARRAY: stp_sort_kind = 5;
/// Function sort.
pub const STP_SORT_FUN: stp_sort_kind = 6;
/// Named uninterpreted sort.
pub const STP_SORT_UNINTERPRETED: stp_sort_kind = 7;

// stp_rm
/// Round nearest, ties to even.
pub const STP_RM_RNE: stp_rm = 0;
/// Round nearest, ties to away.
pub const STP_RM_RNA: stp_rm = 1;
/// Round toward positive.
pub const STP_RM_RTP: stp_rm = 2;
/// Round toward negative.
pub const STP_RM_RTN: stp_rm = 3;
/// Round toward zero.
pub const STP_RM_RTZ: stp_rm = 4;

// stp_result_kind
/// The assertions are satisfiable.
pub const STP_SAT: stp_result_kind = 1;
/// The assertions are unsatisfiable.
pub const STP_UNSAT: stp_result_kind = 2;
/// The check could not decide.
pub const STP_UNKNOWN: stp_result_kind = 3;

// stp_validity
/// The formula is valid.
pub const STP_VALID: stp_validity = 1;
/// The formula is not valid; a counterexample exists.
pub const STP_INVALID: stp_validity = 2;
/// Validity could not be decided.
pub const STP_UNKNOWN_VALIDITY: stp_validity = 3;

// stp_unknown_reason
/// No unknown to explain.
pub const STP_REASON_NONE: stp_unknown_reason = 0;
/// A time budget ran out.
pub const STP_REASON_TIMEOUT: stp_unknown_reason = 1;
/// A conflict budget ran out.
pub const STP_REASON_CONFLICT_LIMIT: stp_unknown_reason = 2;
/// The check was interrupted.
pub const STP_REASON_INTERRUPTED: stp_unknown_reason = 3;
/// The check stopped before an answer.
pub const STP_REASON_INCOMPLETE: stp_unknown_reason = 4;
/// A resource limit stopped the check.
pub const STP_REASON_RESOURCE_LIMIT: stp_unknown_reason = 5;
/// A declared sort's carrier was exhausted.
pub const STP_REASON_CARRIER_EXHAUSTED: stp_unknown_reason = 6;
/// An injectivity assumption could not be settled.
pub const STP_REASON_ASSUMED_INJECTIVITY: stp_unknown_reason = 7;
/// The pipeline stopped after producing a CNF.
pub const STP_REASON_STOPPED_AFTER_CNF: stp_unknown_reason = 8;
/// Another reason, with no more specific value.
pub const STP_REASON_OTHER: stp_unknown_reason = 9;

// stp_format
/// Detect the syntax automatically.
pub const STP_FORMAT_AUTO: stp_format = 0;
/// SMT-LIB 2.
pub const STP_FORMAT_SMTLIB2: stp_format = 1;
/// Graphviz DOT.
pub const STP_FORMAT_DOT: stp_format = 2;
/// GDL.
pub const STP_FORMAT_GDL: stp_format = 3;

// stp_parse_mode
/// Declare and assert, without executing commands.
pub const STP_PARSE_DECLARE_AND_ASSERT: stp_parse_mode = 0;
/// Execute the script against the solver.
pub const STP_PARSE_EXECUTE: stp_parse_mode = 1;
/// Parse only.
pub const STP_PARSE_ONLY: stp_parse_mode = 2;
/// Treat the script as a single query.
pub const STP_PARSE_SINGLE_QUERY: stp_parse_mode = 3;

// stp_cnf_scope
/// The CNF covers the whole assertion set.
pub const STP_CNF_WHOLE: stp_cnf_scope = 0;
/// The CNF covers part of the assertion set.
pub const STP_CNF_PARTIAL: stp_cnf_scope = 1;
/// The CNF is an over-approximation.
pub const STP_CNF_OVER_APPROXIMATION: stp_cnf_scope = 2;

// stp_tier
/// Stable options.
pub const STP_TIER_STABLE: stp_tier = 0;
/// Expert options.
pub const STP_TIER_EXPERT: stp_tier = 1;
/// Experimental options.
pub const STP_TIER_EXPERIMENTAL: stp_tier = 2;
/// Diagnostic options.
pub const STP_TIER_DIAGNOSTIC: stp_tier = 3;

// stp_settable
/// May be changed at any time.
pub const STP_SETTABLE_ANYTIME: stp_settable = 0;
/// Must be changed before the first check.
pub const STP_SETTABLE_BEFORE_FIRST_CHECK: stp_settable = 1;
/// Must be changed at construction time.
pub const STP_SETTABLE_CONSTRUCTION: stp_settable = 2;

// stp_option_scope
/// A solver-scoped option.
pub const STP_SCOPE_SOLVER: stp_option_scope = 0;
/// A manager-scoped option.
pub const STP_SCOPE_MANAGER: stp_option_scope = 1;

// stp_fp_class
/// A normal floating-point value.
pub const STP_FP_NORMAL: stp_fp_class = 0;
/// A subnormal floating-point value.
pub const STP_FP_SUBNORMAL: stp_fp_class = 1;
/// Zero.
pub const STP_FP_ZERO: stp_fp_class = 2;
/// Infinity.
pub const STP_FP_INFINITY: stp_fp_class = 3;
/// NaN.
pub const STP_FP_NAN: stp_fp_class = 4;

// stp_internal_error_policy
/// Poison the object after an internal error.
pub const STP_POISON: stp_internal_error_policy = 0;
/// Abort the process after an internal error.
pub const STP_ABORT: stp_internal_error_policy = 1;

// stp_kind
/// A literal value.
pub const STP_KIND_VALUE: stp_kind = 0;
/// A declared symbol.
pub const STP_KIND_CONSTANT: stp_kind = 1;
/// If-then-else.
pub const STP_KIND_ITE: stp_kind = 2;
/// Equality.
pub const STP_KIND_EQUAL: stp_kind = 3;
/// Distinctness.
pub const STP_KIND_DISTINCT: stp_kind = 4;
/// Function application.
pub const STP_KIND_APPLY: stp_kind = 5;
/// Logical not.
pub const STP_KIND_NOT: stp_kind = 6;
/// Logical and.
pub const STP_KIND_AND: stp_kind = 7;
/// Logical or.
pub const STP_KIND_OR: stp_kind = 8;
/// Logical xor.
pub const STP_KIND_XOR: stp_kind = 9;
/// Logical implication.
pub const STP_KIND_IMPLIES: stp_kind = 10;
/// Bitwise not.
pub const STP_KIND_BV_NOT: stp_kind = 11;
/// Bitwise and.
pub const STP_KIND_BV_AND: stp_kind = 12;
/// Bitwise or.
pub const STP_KIND_BV_OR: stp_kind = 13;
/// Bitwise xor.
pub const STP_KIND_BV_XOR: stp_kind = 14;
/// Bitwise nand.
pub const STP_KIND_BV_NAND: stp_kind = 15;
/// Bitwise nor.
pub const STP_KIND_BV_NOR: stp_kind = 16;
/// Bitwise xnor.
pub const STP_KIND_BV_XNOR: stp_kind = 17;
/// Two's-complement negation.
pub const STP_KIND_BV_NEG: stp_kind = 18;
/// Addition.
pub const STP_KIND_BV_ADD: stp_kind = 19;
/// Subtraction.
pub const STP_KIND_BV_SUB: stp_kind = 20;
/// Multiplication.
pub const STP_KIND_BV_MUL: stp_kind = 21;
/// Unsigned division.
pub const STP_KIND_BV_UDIV: stp_kind = 22;
/// Unsigned remainder.
pub const STP_KIND_BV_UREM: stp_kind = 23;
/// Signed division.
pub const STP_KIND_BV_SDIV: stp_kind = 24;
/// Signed remainder.
pub const STP_KIND_BV_SREM: stp_kind = 25;
/// Signed modulo.
pub const STP_KIND_BV_SMOD: stp_kind = 26;
/// Left shift.
pub const STP_KIND_BV_SHL: stp_kind = 27;
/// Logical right shift.
pub const STP_KIND_BV_LSHR: stp_kind = 28;
/// Arithmetic right shift.
pub const STP_KIND_BV_ASHR: stp_kind = 29;
/// Concatenation.
pub const STP_KIND_BV_CONCAT: stp_kind = 30;
/// Bit extraction.
pub const STP_KIND_BV_EXTRACT: stp_kind = 31;
/// Zero extension.
pub const STP_KIND_BV_ZERO_EXTEND: stp_kind = 32;
/// Sign extension.
pub const STP_KIND_BV_SIGN_EXTEND: stp_kind = 33;
/// Repetition.
pub const STP_KIND_BV_REPEAT: stp_kind = 34;
/// Rotate left.
pub const STP_KIND_BV_ROTATE_LEFT: stp_kind = 35;
/// Rotate right.
pub const STP_KIND_BV_ROTATE_RIGHT: stp_kind = 36;
/// Complement.
pub const STP_KIND_BV_COMP: stp_kind = 37;
/// Unsigned less-than.
pub const STP_KIND_BV_ULT: stp_kind = 38;
/// Unsigned less-or-equal.
pub const STP_KIND_BV_ULE: stp_kind = 39;
/// Unsigned greater-than.
pub const STP_KIND_BV_UGT: stp_kind = 40;
/// Unsigned greater-or-equal.
pub const STP_KIND_BV_UGE: stp_kind = 41;
/// Signed less-than.
pub const STP_KIND_BV_SLT: stp_kind = 42;
/// Signed less-or-equal.
pub const STP_KIND_BV_SLE: stp_kind = 43;
/// Signed greater-than.
pub const STP_KIND_BV_SGT: stp_kind = 44;
/// Signed greater-or-equal.
pub const STP_KIND_BV_SGE: stp_kind = 45;
/// Unsigned addition overflow.
pub const STP_KIND_BV_UADDO: stp_kind = 46;
/// Signed addition overflow.
pub const STP_KIND_BV_SADDO: stp_kind = 47;
/// Unsigned multiplication overflow.
pub const STP_KIND_BV_UMULO: stp_kind = 48;
/// Signed multiplication overflow.
pub const STP_KIND_BV_SMULO: stp_kind = 49;
/// Unsigned subtraction overflow.
pub const STP_KIND_BV_USUBO: stp_kind = 50;
/// Signed subtraction overflow.
pub const STP_KIND_BV_SSUBO: stp_kind = 51;
/// Negation overflow.
pub const STP_KIND_BV_NEGO: stp_kind = 52;
/// Signed division overflow.
pub const STP_KIND_BV_SDIVO: stp_kind = 53;
/// Bit-vector reduction and.
pub const STP_KIND_BV_REDAND: stp_kind = 54;
/// Bit-vector reduction or.
pub const STP_KIND_BV_REDOR: stp_kind = 55;
/// Array read.
pub const STP_KIND_SELECT: stp_kind = 56;
/// Array write.
pub const STP_KIND_STORE: stp_kind = 57;
/// Constant array.
pub const STP_KIND_CONST_ARRAY: stp_kind = 58;
/// Floating-point absolute value.
pub const STP_KIND_FP_ABS: stp_kind = 59;
/// Floating-point negation.
pub const STP_KIND_FP_NEG: stp_kind = 60;
/// Floating-point addition.
pub const STP_KIND_FP_ADD: stp_kind = 61;
/// Floating-point subtraction.
pub const STP_KIND_FP_SUB: stp_kind = 62;
/// Floating-point multiplication.
pub const STP_KIND_FP_MUL: stp_kind = 63;
/// Floating-point division.
pub const STP_KIND_FP_DIV: stp_kind = 64;
/// Floating-point fused multiply-add.
pub const STP_KIND_FP_FMA: stp_kind = 65;
/// Floating-point square root.
pub const STP_KIND_FP_SQRT: stp_kind = 66;
/// Floating-point remainder.
pub const STP_KIND_FP_REM: stp_kind = 67;
/// Floating-point round-to-integral.
pub const STP_KIND_FP_RTI: stp_kind = 68;
/// Floating-point minimum.
pub const STP_KIND_FP_MIN: stp_kind = 69;
/// Floating-point maximum.
pub const STP_KIND_FP_MAX: stp_kind = 70;
/// Floating-point IEEE equality.
pub const STP_KIND_FP_EQ: stp_kind = 71;
/// Floating-point less-than.
pub const STP_KIND_FP_LT: stp_kind = 72;
/// Floating-point less-or-equal.
pub const STP_KIND_FP_LEQ: stp_kind = 73;
/// Floating-point greater-than.
pub const STP_KIND_FP_GT: stp_kind = 74;
/// Floating-point greater-or-equal.
pub const STP_KIND_FP_GEQ: stp_kind = 75;
/// Floating-point is-normal predicate.
pub const STP_KIND_FP_IS_NORMAL: stp_kind = 76;
/// Floating-point is-subnormal predicate.
pub const STP_KIND_FP_IS_SUBNORMAL: stp_kind = 77;
/// Floating-point is-zero predicate.
pub const STP_KIND_FP_IS_ZERO: stp_kind = 78;
/// Floating-point is-infinite predicate.
pub const STP_KIND_FP_IS_INF: stp_kind = 79;
/// Floating-point is-NaN predicate.
pub const STP_KIND_FP_IS_NAN: stp_kind = 80;
/// Floating-point is-negative predicate.
pub const STP_KIND_FP_IS_NEG: stp_kind = 81;
/// Floating-point is-positive predicate.
pub const STP_KIND_FP_IS_POS: stp_kind = 82;
/// Floating-point construction from sign, exponent and significand.
pub const STP_KIND_FP_FP: stp_kind = 83;
/// Conversion from a bit-vector to floating point.
pub const STP_KIND_FP_TO_FP_FROM_BV: stp_kind = 84;
/// Conversion from floating point to floating point.
pub const STP_KIND_FP_TO_FP_FROM_FP: stp_kind = 85;
/// Conversion from a signed bit-vector to floating point.
pub const STP_KIND_FP_TO_FP_FROM_SBV: stp_kind = 86;
/// Conversion from an unsigned bit-vector to floating point.
pub const STP_KIND_FP_TO_FP_FROM_UBV: stp_kind = 87;
/// Conversion from Real to floating point.
pub const STP_KIND_FP_TO_FP_FROM_REAL: stp_kind = 88;
/// Conversion from floating point to unsigned bit-vector.
pub const STP_KIND_FP_TO_UBV: stp_kind = 89;
/// Conversion from floating point to signed bit-vector.
pub const STP_KIND_FP_TO_SBV: stp_kind = 90;
/// Conversion from floating point to Real.
pub const STP_KIND_FP_TO_REAL: stp_kind = 91;
/// Reinterpretation as an IEEE bit-vector.
pub const STP_KIND_FP_TO_IEEE_BV: stp_kind = 92;
/// Exact Real addition.
pub const STP_KIND_REAL_ADD: stp_kind = 93;
/// Exact Real subtraction.
pub const STP_KIND_REAL_SUB: stp_kind = 94;
/// Exact Real negation.
pub const STP_KIND_REAL_NEG: stp_kind = 95;
/// Exact Real multiplication.
pub const STP_KIND_REAL_MUL: stp_kind = 96;
/// Exact Real division.
pub const STP_KIND_REAL_DIV: stp_kind = 97;
/// Exact Real less-than.
pub const STP_KIND_REAL_LT: stp_kind = 98;
/// Exact Real less-or-equal.
pub const STP_KIND_REAL_LE: stp_kind = 99;
/// Exact Real greater-than.
pub const STP_KIND_REAL_GT: stp_kind = 100;
/// Exact Real greater-or-equal.
pub const STP_KIND_REAL_GE: stp_kind = 101;
/// Number of term kinds.
pub const STP_NUM_KINDS: stp_kind = 102;

// stp_error_code
/// An argument is invalid.
pub const STP_ERR_INVALID_ARGUMENT: stp_error_code = 1;
/// Sorts do not match.
pub const STP_ERR_SORT_MISMATCH: stp_error_code = 2;
/// Wrong number of arguments.
pub const STP_ERR_ARITY: stp_error_code = 3;
/// An index is out of range.
pub const STP_ERR_INDEX_OUT_OF_RANGE: stp_error_code = 4;
/// A value is out of range.
pub const STP_ERR_VALUE_OUT_OF_RANGE: stp_error_code = 5;
/// A value does not fit the requested type.
pub const STP_ERR_DOES_NOT_FIT: stp_error_code = 6;
/// The term is not a value.
pub const STP_ERR_NOT_A_VALUE: stp_error_code = 7;
/// No model is available.
pub const STP_ERR_NO_MODEL: stp_error_code = 8;
/// A handle belongs to another manager.
pub const STP_ERR_FOREIGN_MANAGER: stp_error_code = 9;
/// A handle is null.
pub const STP_ERR_NULL_HANDLE: stp_error_code = 10;
/// The operation is unsupported.
pub const STP_ERR_UNSUPPORTED: stp_error_code = 11;
/// The option is unknown.
pub const STP_ERR_OPTION_UNKNOWN: stp_error_code = 12;
/// The option value is invalid.
pub const STP_ERR_OPTION_VALUE: stp_error_code = 13;
/// The option cannot be written now.
pub const STP_ERR_OPTION_TIMING: stp_error_code = 14;
/// Options conflict.
pub const STP_ERR_OPTION_CONFLICT: stp_error_code = 15;
/// An option is unavailable in this build.
pub const STP_ERR_OPTION_UNAVAILABLE: stp_error_code = 16;
/// A parse failed.
pub const STP_ERR_PARSE: stp_error_code = 17;
/// An I/O operation failed.
pub const STP_ERR_IO: stp_error_code = 18;
/// The object is in a failed state.
pub const STP_ERR_STATE: stp_error_code = 19;
/// A resource was exhausted.
pub const STP_ERR_RESOURCE: stp_error_code = 100;
/// An internal error occurred.
pub const STP_ERR_INTERNAL: stp_error_code = 101;

// stp_option
/// Produce models.
pub const STP_OPT_PRODUCE_MODELS: stp_option = 0;
/// Select the SAT backend.
pub const STP_OPT_SAT_BACKEND: stp_option = 1;
/// Random seed.
pub const STP_OPT_RANDOM_SEED: stp_option = 2;
/// Array fill rule for models.
pub const STP_OPT_MODEL_ARRAY_FILL: stp_option = 3;
/// SMT-LIB logic.
pub const STP_OPT_LOGIC: stp_option = 4;
/// Enable simplification.
pub const STP_OPT_SIMPLIFY: stp_option = 5;
/// Default rounding mode.
pub const STP_OPT_DEFAULT_ROUNDING_MODE: stp_option = 6;
/// Disable individual simplifications.
pub const STP_OPT_DISABLE_SIMPLIFICATIONS: stp_option = 7;
/// Number of threads.
pub const STP_OPT_THREADS: stp_option = 8;
/// Array equality mode.
pub const STP_OPT_ARRAY_EQUALITY: stp_option = 9;
/// Bit-vector equality abstraction.
pub const STP_OPT_BV_EQ_ABSTRACTION: stp_option = 10;
/// Bit-vector term abstraction.
pub const STP_OPT_BV_TERM_ABSTRACTION: stp_option = 11;
/// Uninterpreted functions.
pub const STP_OPT_UNINTERPRETED_FUNCTIONS: stp_option = 12;
/// Ackermannisation of uninterpreted functions.
pub const STP_OPT_UF_ACKERMANN: stp_option = 13;
/// Width of the uninterpreted-function sort.
pub const STP_OPT_UF_SORT_WIDTH: stp_option = 14;
/// Floating-point abstraction.
pub const STP_OPT_FP_ABSTRACTION: stp_option = 15;
/// CNF generation effort.
pub const STP_OPT_CNF_GENERATION_EFFORT: stp_option = 16;
/// Incremental solving.
pub const STP_OPT_INCREMENTAL: stp_option = 17;
/// When to engage incremental solving automatically.
pub const STP_OPT_INCREMENTAL_AUTO_ENGAGE_AT: stp_option = 18;
/// Maximum number of conflicts.
pub const STP_OPT_MAX_NUM_CONFL: stp_option = 19;
/// Maximum time.
pub const STP_OPT_MAX_TIME: stp_option = 20;
/// Check the engine's sanity invariants.
pub const STP_OPT_CHECK_SANITY: stp_option = 21;
/// SAT tuning.
pub const STP_OPT_SAT_TUNING: stp_option = 22;
/// Number of stable options.
pub const STP_NUM_STABLE_OPTIONS: stp_option = 23;

// ---------------------------------------------------------------------------
// By-value structs
// ---------------------------------------------------------------------------

/// A satisfiability answer and, when unknown, the reason.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct stp_result {
	/// The satisfiability answer.
	pub kind: stp_result_kind,
	/// Why the answer is unknown, or `STP_REASON_NONE`.
	pub reason: stp_unknown_reason,
}

/// A validity / entailment answer and, when unknown, the reason.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct stp_entailment {
	/// The validity answer.
	pub kind: stp_validity,
	/// Why the answer is unknown, or `STP_REASON_NONE`.
	pub reason: stp_unknown_reason,
}

/// A time and/or conflict budget. A false `has_*` means "no limit".
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct stp_budget {
	/// Whether a wall-clock limit is present.
	pub has_time: bool,
	/// The wall-clock limit in milliseconds.
	pub time_ms: u64,
	/// Whether a conflict limit is present.
	pub has_conflicts: bool,
	/// The conflict limit.
	pub conflicts: u64,
}

/// The first error recorded by a manager, solver or options value.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct stp_error {
	/// The specific error code.
	pub code: stp_error_code,
	/// Whether the object remains usable.
	pub recoverable: bool,
	/// A human-readable message owned by the recording object.
	pub message: *const c_char,
	/// The C API function that refused.
	pub function: *const c_char,
	/// The zero-based argument index, or -1.
	pub argument_index: c_int,
	/// The option name for `STP_ERR_OPTION_*` errors.
	pub option: *const c_char,
	/// The one-based parse line, or 0.
	pub line: c_int,
	/// The one-based parse column, or 0.
	pub column: c_int,
}

/// The format and classification of a floating-point value.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct stp_float_value {
	/// Number of exponent bits.
	pub exp_size: u32,
	/// Number of significand bits, including the hidden bit.
	pub sig_size: u32,
	/// The sign bit.
	pub sign: bool,
	/// The biased exponent.
	pub biased_exponent: u64,
	/// The value's class.
	pub cls: stp_fp_class,
}

/// The version of the linked STP build. All strings are static.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct stp_version {
	/// Major version number.
	pub major: c_int,
	/// Minor version number.
	pub minor: c_int,
	/// Patch version number.
	pub patch: c_int,
	/// The version string.
	pub string: *const c_char,
	/// The git commit SHA.
	pub git_sha: *const c_char,
	/// The git tag.
	pub git_tag: *const c_char,
	/// A description of the build environment.
	pub build_info: *const c_char,
}

// ---------------------------------------------------------------------------
// Callback types
// ---------------------------------------------------------------------------

/// Called with every error recorded by a manager.
pub type stp_error_callback = Option<unsafe extern "C" fn(*const stp_error, *mut c_void)>;
/// Called inside a check; returning `true` stops it.
pub type stp_terminate_callback = Option<unsafe extern "C" fn(*mut c_void) -> bool>;
/// Receives text produced by the solver.
pub type stp_text_sink = Option<unsafe extern "C" fn(*const c_char, usize, *mut c_void)>;
/// Fills a buffer for the parser.
pub type stp_text_source = Option<unsafe extern "C" fn(*mut c_char, usize, *mut c_void) -> usize>;
/// Receives an engine fatal error.
pub type stp_fatal_error_handler = Option<unsafe extern "C" fn(*const c_char, *mut c_void)>;
/// Receives a CNF in DIMACS form.
pub type stp_cnf_sink = Option<unsafe extern "C" fn(*const c_char, usize, stp_cnf_scope, *mut c_void)>;

unsafe extern "C" {
	// -----------------------------------------------------------------------
	// Library
	// -----------------------------------------------------------------------

	/// Returns the version of the linked STP build.
	pub fn stp_get_version() -> stp_version;
	/// Returns the value of one capability, or `NULL` if unknown.
	pub fn stp_capability(key: *const c_char) -> *mut c_char;
	/// Returns every capability as `key=value` lines.
	pub fn stp_capabilities() -> *mut c_char;
	/// Whether this build has the named SAT backend.
	pub fn stp_has_sat_backend(name: *const c_char) -> bool;
	/// Number of SAT backends in this build.
	pub fn stp_num_sat_backends() -> usize;
	/// Name of the `i`-th SAT backend.
	pub fn stp_sat_backend_name(i: usize) -> *const c_char;
	/// Releases a buffer returned by the library.
	pub fn stp_free(p: *mut c_void);

	// -----------------------------------------------------------------------
	// Errors and names
	// -----------------------------------------------------------------------

	/// The thread-local error record.
	pub fn stp_last_error() -> *const stp_error;
	/// Clears the thread-local error record.
	pub fn stp_clear_last_error();
	/// Sets the process-wide internal-error policy.
	pub fn stp_set_internal_error_policy(policy: stp_internal_error_policy);
	/// Returns the process-wide internal-error policy.
	pub fn stp_get_internal_error_policy() -> stp_internal_error_policy;
	/// Returns the name of a term kind.
	pub fn stp_kind_name(kind: stp_kind) -> *const c_char;
	/// Returns the SMT-LIB name of a term kind.
	pub fn stp_kind_smtlib(kind: stp_kind) -> *const c_char;
	/// Returns the name of a rounding mode.
	pub fn stp_rm_name(rm: stp_rm) -> *const c_char;
	/// Returns the name of an unknown reason.
	pub fn stp_unknown_reason_name(reason: stp_unknown_reason) -> *const c_char;
	/// Returns the name of an error code.
	pub fn stp_error_code_name(code: stp_error_code) -> *const c_char;
	/// Returns the name of a result kind.
	pub fn stp_result_kind_name(kind: stp_result_kind) -> *const c_char;
	/// Returns the name of a validity value.
	pub fn stp_validity_name(kind: stp_validity) -> *const c_char;

	// -----------------------------------------------------------------------
	// Term manager
	// -----------------------------------------------------------------------

	/// Creates a term manager with the given options (or defaults).
	pub fn stp_tm_new(manager_options: stp_options) -> stp_tm;
	/// Creates a manager from the three construction-time options.
	pub fn stp_tm_new_with(simplify: bool, default_rounding_mode: stp_rm, uf_sort_width: u32) -> stp_tm;
	/// Returns another handle to the same manager.
	pub fn stp_tm_copy(tm: stp_tm) -> stp_tm;
	/// Releases one manager handle.
	pub fn stp_tm_release(tm: stp_tm);
	/// Returns the process-unique manager id.
	pub fn stp_tm_id(tm: stp_tm) -> u64;
	/// Whether construction-time simplification is enabled.
	pub fn stp_tm_simplify_enabled(tm: stp_tm) -> bool;
	/// Returns the manager's default rounding mode.
	pub fn stp_tm_default_rounding_mode(tm: stp_tm) -> stp_rm;
	/// Sets the manager's default rounding mode.
	pub fn stp_tm_set_default_rounding_mode(tm: stp_tm, rm: stp_rm) -> stp_status;
	/// Returns the manager's uninterpreted-function sort width.
	pub fn stp_tm_uf_sort_width(tm: stp_tm) -> u32;
	/// Opens a term scope.
	pub fn stp_tm_scope_push(tm: stp_tm);
	/// Closes a term scope, releasing terms exported inside it.
	pub fn stp_tm_scope_pop(tm: stp_tm);
	/// Number of open term scopes.
	pub fn stp_tm_scope_depth(tm: stp_tm) -> usize;
	/// Releases every term reference held by the manager.
	pub fn stp_tm_release_all(tm: stp_tm);
	/// Returns the manager's first error, or `NULL`.
	pub fn stp_tm_error(tm: stp_tm) -> *const stp_error;
	/// Number of terms named by the recorded error.
	pub fn stp_tm_error_num_terms(tm: stp_tm) -> usize;
	/// Returns the `i`-th term named by the recorded error.
	pub fn stp_tm_error_term(tm: stp_tm, i: usize) -> stp_term;
	/// Number of sorts named by the recorded error.
	pub fn stp_tm_error_num_sorts(tm: stp_tm) -> usize;
	/// Returns the `i`-th sort named by the recorded error.
	pub fn stp_tm_error_sort(tm: stp_tm, i: usize) -> stp_sort;
	/// Clears the manager's error record.
	pub fn stp_tm_clear_error(tm: stp_tm);
	/// Installs a callback that sees every error.
	pub fn stp_tm_set_error_callback(tm: stp_tm, cb: stp_error_callback, user: *mut c_void);

	// -----------------------------------------------------------------------
	// Sorts
	// -----------------------------------------------------------------------

	/// Returns the Boolean sort.
	pub fn stp_mk_bool_sort(tm: stp_tm) -> stp_sort;
	/// Returns a bit-vector sort of `width` bits.
	pub fn stp_mk_bv_sort(tm: stp_tm, width: u32) -> stp_sort;
	/// Returns a floating-point sort.
	pub fn stp_mk_fp_sort(tm: stp_tm, exp_size: u32, sig_size: u32) -> stp_sort;
	/// Returns the binary16 sort.
	pub fn stp_mk_fp16_sort(tm: stp_tm) -> stp_sort;
	/// Returns the binary32 sort.
	pub fn stp_mk_fp32_sort(tm: stp_tm) -> stp_sort;
	/// Returns the binary64 sort.
	pub fn stp_mk_fp64_sort(tm: stp_tm) -> stp_sort;
	/// Returns the binary128 sort.
	pub fn stp_mk_fp128_sort(tm: stp_tm) -> stp_sort;
	/// Returns the rounding-mode sort.
	pub fn stp_mk_rm_sort(tm: stp_tm) -> stp_sort;
	/// Returns the exact Real sort.
	pub fn stp_mk_real_sort(tm: stp_tm) -> stp_sort;
	/// Returns an array sort.
	pub fn stp_mk_array_sort(tm: stp_tm, index: stp_sort, element: stp_sort) -> stp_sort;
	/// Returns a function sort.
	pub fn stp_mk_fun_sort(tm: stp_tm, arity: usize, domain: *const stp_sort, codomain: stp_sort) -> stp_sort;
	/// Declares a named uninterpreted sort.
	pub fn stp_tm_declare_sort(tm: stp_tm, name: *const c_char) -> stp_sort;
	/// Returns a fresh anonymous sort.
	pub fn stp_mk_fresh_sort(tm: stp_tm, prefix: *const c_char) -> stp_sort;
	/// Returns another handle to the same sort.
	pub fn stp_sort_copy(sort: stp_sort) -> stp_sort;
	/// Releases a sort handle (a no-op; sorts are pooled).
	pub fn stp_sort_release(sort: stp_sort);
	/// Returns the sort's class.
	pub fn stp_sort_get_kind(sort: stp_sort, out: *mut stp_sort_kind) -> stp_status;
	/// Returns a bit-vector sort's width.
	pub fn stp_sort_bv_size(sort: stp_sort, out: *mut u32) -> stp_status;
	/// Returns a floating-point sort's exponent width.
	pub fn stp_sort_fp_exp_size(sort: stp_sort, out: *mut u32) -> stp_status;
	/// Returns a floating-point sort's significand width.
	pub fn stp_sort_fp_sig_size(sort: stp_sort, out: *mut u32) -> stp_status;
	/// Returns an array sort's index sort.
	pub fn stp_sort_array_index(sort: stp_sort) -> stp_sort;
	/// Returns an array sort's element sort.
	pub fn stp_sort_array_element(sort: stp_sort) -> stp_sort;
	/// Returns a function sort's arity.
	pub fn stp_sort_fun_arity(sort: stp_sort, out: *mut u32) -> stp_status;
	/// Returns a function sort's `i`-th domain sort.
	pub fn stp_sort_fun_domain(sort: stp_sort, i: u32) -> stp_sort;
	/// Returns a function sort's codomain sort.
	pub fn stp_sort_fun_codomain(sort: stp_sort) -> stp_sort;
	/// Returns an uninterpreted sort's name.
	pub fn stp_sort_name(sort: stp_sort) -> *mut c_char;
	/// Returns the manager-unique sort id.
	pub fn stp_sort_id(sort: stp_sort) -> u64;
	/// Returns the sort's SMT-LIB rendering.
	pub fn stp_sort_str(sort: stp_sort) -> *mut c_char;
	/// Returns another manager handle.
	pub fn stp_sort_manager(sort: stp_sort) -> stp_tm;

	// -----------------------------------------------------------------------
	// Symbols and values
	// -----------------------------------------------------------------------

	/// Declares (or looks up) a symbol by name and sort.
	pub fn stp_declare(tm: stp_tm, name: *const c_char, sort: stp_sort) -> stp_term;
	/// Returns a fresh anonymous symbol.
	pub fn stp_mk_fresh(tm: stp_tm, sort: stp_sort, prefix: *const c_char) -> stp_term;
	/// Looks up a symbol by name.
	pub fn stp_tm_symbol(tm: stp_tm, name: *const c_char) -> stp_term;
	/// Binds an existing symbol to a name.
	pub fn stp_tm_bind_symbol(tm: stp_tm, name: *const c_char, term: stp_term) -> stp_status;
	/// Number of symbols in the manager's name table.
	pub fn stp_tm_num_symbols(tm: stp_tm) -> usize;
	/// Returns the `i`-th symbol in the name table.
	pub fn stp_tm_symbol_at(tm: stp_tm, i: usize) -> stp_term;
	/// Number of declared sorts.
	pub fn stp_tm_num_declared_sorts(tm: stp_tm) -> usize;
	/// Returns the `i`-th declared sort.
	pub fn stp_tm_declared_sort_at(tm: stp_tm, i: usize) -> stp_sort;
	/// Returns a live term by id.
	pub fn stp_tm_term_from_id(tm: stp_tm, id: u64) -> stp_term;
	/// Returns the constant `true`.
	pub fn stp_mk_true(tm: stp_tm) -> stp_term;
	/// Returns the constant `false`.
	pub fn stp_mk_false(tm: stp_tm) -> stp_term;
	/// Returns a Boolean constant.
	pub fn stp_mk_bool(tm: stp_tm, value: bool) -> stp_term;
	/// Returns a bit-vector constant from an unsigned 64-bit value.
	pub fn stp_mk_bv_uint64(tm: stp_tm, width: u32, value: u64) -> stp_term;
	/// Returns a bit-vector constant from a signed 64-bit value.
	pub fn stp_mk_bv_int64(tm: stp_tm, width: u32, value: i64) -> stp_term;
	/// Returns a bit-vector constant parsed from a string.
	pub fn stp_mk_bv_str(tm: stp_tm, width: u32, digits: *const c_char, base: c_int) -> stp_term;
	/// Returns a bit-vector constant from little-endian limbs.
	pub fn stp_mk_bv_limbs(tm: stp_tm, width: u32, n: usize, lsb_first: *const u64) -> stp_term;
	/// Returns a bit-vector constant from bytes.
	pub fn stp_mk_bv_bytes(tm: stp_tm, width: u32, n: usize, bytes: *const u8, little_endian: bool) -> stp_term;
	/// Returns a bit-vector constant, wrapping the value modulo `2^width`.
	pub fn stp_mk_bv_wrapped(tm: stp_tm, width: u32, value: u64) -> stp_term;
	/// Returns a bit-vector zero.
	pub fn stp_mk_bv_zero(tm: stp_tm, width: u32) -> stp_term;
	/// Returns a bit-vector of all ones.
	pub fn stp_mk_bv_ones(tm: stp_tm, width: u32) -> stp_term;
	/// Returns the smallest signed bit-vector value.
	pub fn stp_mk_bv_min_signed(tm: stp_tm, width: u32) -> stp_term;
	/// Returns the largest signed bit-vector value.
	pub fn stp_mk_bv_max_signed(tm: stp_tm, width: u32) -> stp_term;
	/// Returns a floating-point value from its bit-vector representation.
	pub fn stp_mk_fp_from_bits(tm: stp_tm, fp: stp_sort, bv_value: stp_term) -> stp_term;
	/// Returns a floating-point value from a bit string.
	pub fn stp_mk_fp_from_bits_str(tm: stp_tm, fp: stp_sort, bits: *const c_char) -> stp_term;
	/// Returns `(fp sign exponent significand)`.
	pub fn stp_mk_fp(tm: stp_tm, sign: stp_term, exponent: stp_term, significand: stp_term) -> stp_term;
	/// Returns positive zero.
	pub fn stp_mk_fp_pos_zero(tm: stp_tm, fp: stp_sort) -> stp_term;
	/// Returns negative zero.
	pub fn stp_mk_fp_neg_zero(tm: stp_tm, fp: stp_sort) -> stp_term;
	/// Returns positive infinity.
	pub fn stp_mk_fp_pos_inf(tm: stp_tm, fp: stp_sort) -> stp_term;
	/// Returns negative infinity.
	pub fn stp_mk_fp_neg_inf(tm: stp_tm, fp: stp_sort) -> stp_term;
	/// Returns the canonical quiet NaN.
	pub fn stp_mk_fp_nan(tm: stp_tm, fp: stp_sort) -> stp_term;
	/// Returns a floating-point value from a double.
	pub fn stp_mk_fp_double(tm: stp_tm, fp: stp_sort, rm: stp_rm, value: f64) -> stp_term;
	/// Returns a floating-point value from a decimal literal.
	pub fn stp_mk_fp_decimal(tm: stp_tm, fp: stp_sort, rm: stp_rm, literal: *const c_char) -> stp_term;
	/// Returns a rounding-mode value.
	pub fn stp_mk_rm(tm: stp_tm, rm: stp_rm) -> stp_term;
	/// Returns an exact Real integer.
	pub fn stp_mk_real_int64(tm: stp_tm, value: i64) -> stp_term;
	/// Returns an exact Real fraction.
	pub fn stp_mk_real_fraction(tm: stp_tm, numerator: i64, denominator: i64) -> stp_term;
	/// Returns an exact Real from a literal.
	pub fn stp_mk_real_str(tm: stp_tm, literal: *const c_char) -> stp_term;
	/// Returns a constant array.
	pub fn stp_mk_const_array(tm: stp_tm, array_sort: stp_sort, element: stp_term) -> stp_term;
	/// Returns an array from bytes.
	pub fn stp_array_from_bytes(tm: stp_tm, n: usize, bytes: *const u8, index_width: u32) -> stp_term;

	// -----------------------------------------------------------------------
	// Generic construction
	// -----------------------------------------------------------------------

	/// Builds a term from a kind and arguments.
	pub fn stp_mk_term(tm: stp_tm, kind: stp_kind, n: usize, args: *const stp_term) -> stp_term;
	/// Builds an indexed term from a kind and arguments.
	pub fn stp_mk_term_indexed(
		tm: stp_tm,
		kind: stp_kind,
		n: usize,
		args: *const stp_term,
		m: usize,
		idx: *const u32,
	) -> stp_term;
	/// Builds a sorted indexed term.
	pub fn stp_mk_term_sorted(
		tm: stp_tm,
		kind: stp_kind,
		n: usize,
		args: *const stp_term,
		m: usize,
		idx: *const u32,
		result: stp_sort,
	) -> stp_term;
	/// Builds a unary term.
	pub fn stp_mk_term1(tm: stp_tm, kind: stp_kind, a: stp_term) -> stp_term;
	/// Builds a binary term.
	pub fn stp_mk_term2(tm: stp_tm, kind: stp_kind, a: stp_term, b: stp_term) -> stp_term;
	/// Builds a ternary term.
	pub fn stp_mk_term3(tm: stp_tm, kind: stp_kind, a: stp_term, b: stp_term, c: stp_term) -> stp_term;
	/// Builds a unary term with one index.
	pub fn stp_mk_term1_indexed1(tm: stp_tm, kind: stp_kind, a: stp_term, i: u32) -> stp_term;
	/// Builds a unary term with two indices.
	pub fn stp_mk_term1_indexed2(tm: stp_tm, kind: stp_kind, a: stp_term, i: u32, j: u32) -> stp_term;
	/// Builds a binary term with one index.
	pub fn stp_mk_term2_indexed1(tm: stp_tm, kind: stp_kind, a: stp_term, b: stp_term, i: u32) -> stp_term;
	/// Builds a binary term with two indices.
	pub fn stp_mk_term2_indexed2(tm: stp_tm, kind: stp_kind, a: stp_term, b: stp_term, i: u32, j: u32) -> stp_term;

	// -----------------------------------------------------------------------
	// Named constructors
	// -----------------------------------------------------------------------

	/// Extracts bits `hi..=lo`.
	pub fn stp_extract(tm: stp_tm, hi: u32, lo: u32, t: stp_term) -> stp_term;
	/// Zero-extends `t` by `k` bits.
	pub fn stp_zero_extend(tm: stp_tm, k: u32, t: stp_term) -> stp_term;
	/// Sign-extends `t` by `k` bits.
	pub fn stp_sign_extend(tm: stp_tm, k: u32, t: stp_term) -> stp_term;
	/// Repeats `t` `k` times.
	pub fn stp_repeat(tm: stp_tm, k: u32, t: stp_term) -> stp_term;
	/// Rotates `t` left by `k` bits.
	pub fn stp_rotate_left(tm: stp_tm, k: u32, t: stp_term) -> stp_term;
	/// Rotates `t` right by `k` bits.
	pub fn stp_rotate_right(tm: stp_tm, k: u32, t: stp_term) -> stp_term;
	/// Returns `(= ((_ extract i i) bv) #b1)`.
	pub fn stp_bit(tm: stp_tm, bv: stp_term, i: u32) -> stp_term;
	/// Converts a Boolean to a one-bit bit-vector.
	pub fn stp_bool_to_bv1(tm: stp_tm, b: stp_term) -> stp_term;
	/// Converts a one-bit bit-vector to a Boolean.
	pub fn stp_bv1_to_bool(tm: stp_tm, bv1: stp_term) -> stp_term;
	/// Converts a value to floating point.
	pub fn stp_to_fp(tm: stp_tm, fp: stp_sort, rm: stp_term, value: stp_term) -> stp_term;
	/// Converts an unsigned bit-vector to floating point.
	pub fn stp_to_fp_unsigned(tm: stp_tm, fp: stp_sort, rm: stp_term, bv: stp_term) -> stp_term;
	/// Reinterprets a bit-vector as floating point.
	pub fn stp_to_fp_from_bits(tm: stp_tm, fp: stp_sort, bv: stp_term) -> stp_term;
	/// Converts floating point to an unsigned bit-vector.
	pub fn stp_fp_to_ubv(tm: stp_tm, m: u32, rm: stp_term, value: stp_term) -> stp_term;
	/// Converts floating point to a signed bit-vector.
	pub fn stp_fp_to_sbv(tm: stp_tm, m: u32, rm: stp_term, value: stp_term) -> stp_term;
	/// Floating-point addition with an explicit rounding mode.
	pub fn stp_fp_add_rm(tm: stp_tm, rm: stp_rm, a: stp_term, b: stp_term) -> stp_term;
	/// Floating-point subtraction with an explicit rounding mode.
	pub fn stp_fp_sub_rm(tm: stp_tm, rm: stp_rm, a: stp_term, b: stp_term) -> stp_term;
	/// Floating-point multiplication with an explicit rounding mode.
	pub fn stp_fp_mul_rm(tm: stp_tm, rm: stp_rm, a: stp_term, b: stp_term) -> stp_term;
	/// Floating-point division with an explicit rounding mode.
	pub fn stp_fp_div_rm(tm: stp_tm, rm: stp_rm, a: stp_term, b: stp_term) -> stp_term;
	/// Floating-point fused multiply-add with an explicit rounding mode.
	pub fn stp_fp_fma_rm(tm: stp_tm, rm: stp_rm, a: stp_term, b: stp_term, c: stp_term) -> stp_term;
	/// Floating-point square root with an explicit rounding mode.
	pub fn stp_fp_sqrt_rm(tm: stp_tm, rm: stp_rm, a: stp_term) -> stp_term;
	/// Floating-point round-to-integral with an explicit rounding mode.
	pub fn stp_fp_rti_rm(tm: stp_tm, rm: stp_rm, a: stp_term) -> stp_term;
	/// Converts a value to floating point with an explicit rounding mode.
	pub fn stp_to_fp_rm(tm: stp_tm, fp: stp_sort, rm: stp_rm, value: stp_term) -> stp_term;
	/// Converts an unsigned bit-vector to floating point with an explicit rounding mode.
	pub fn stp_to_fp_unsigned_rm(tm: stp_tm, fp: stp_sort, rm: stp_rm, bv: stp_term) -> stp_term;
	/// Converts floating point to an unsigned bit-vector with an explicit rounding mode.
	pub fn stp_fp_to_ubv_rm(tm: stp_tm, m: u32, rm: stp_rm, value: stp_term) -> stp_term;
	/// Converts floating point to a signed bit-vector with an explicit rounding mode.
	pub fn stp_fp_to_sbv_rm(tm: stp_tm, m: u32, rm: stp_rm, value: stp_term) -> stp_term;

	// -----------------------------------------------------------------------
	// Terms: identity, introspection, readers
	// -----------------------------------------------------------------------

	/// Returns another reference to the same term.
	pub fn stp_term_copy(t: stp_term) -> stp_term;
	/// Releases a term reference.
	pub fn stp_term_release(t: stp_term) -> stp_status;
	/// Returns the term id.
	pub fn stp_term_id(t: stp_term) -> u64;
	/// Returns the term hash.
	pub fn stp_term_hash(t: stp_term) -> u64;
	/// Returns another manager handle.
	pub fn stp_term_manager(t: stp_term) -> stp_tm;
	/// Returns the term's public kind.
	pub fn stp_term_get_kind(t: stp_term, out: *mut stp_kind) -> stp_status;
	/// Returns the term's sort.
	pub fn stp_term_sort(t: stp_term) -> stp_sort;
	/// Number of children.
	pub fn stp_term_num_children(t: stp_term, out: *mut usize) -> stp_status;
	/// Returns the `i`-th child.
	pub fn stp_term_child(t: stp_term, i: usize) -> stp_term;
	/// Number of indices.
	pub fn stp_term_num_indices(t: stp_term, out: *mut usize) -> stp_status;
	/// Returns the `i`-th index.
	pub fn stp_term_index(t: stp_term, i: usize, out: *mut u32) -> stp_status;
	/// Whether the term is a literal value.
	pub fn stp_term_is_value(t: stp_term) -> bool;
	/// Whether the term is a parameterized define-fun.
	pub fn stp_term_is_defined_function(t: stp_term) -> bool;
	/// Whether the term is a declared symbol.
	pub fn stp_term_is_const(t: stp_term) -> bool;
	/// Returns a symbol's name.
	pub fn stp_term_symbol(t: stp_term) -> *mut c_char;
	/// Returns the term's SMT-LIB rendering.
	pub fn stp_term_str(t: stp_term) -> *mut c_char;
	/// Returns the term in the requested format.
	pub fn stp_term_to_string(t: stp_term, format: stp_format, share_subterms: bool) -> *mut c_char;
	/// Substitutes terms.
	pub fn stp_term_substitute(t: stp_term, n: usize, from: *const stp_term, to: *const stp_term) -> stp_term;
	/// Simplifies a term with local rewrites.
	pub fn stp_tm_simplify(tm: stp_tm, t: stp_term) -> stp_term;
	/// Whether two handles are the same node.
	pub fn stp_term_same(a: stp_term, b: stp_term) -> bool;
	/// Reads a Boolean value.
	pub fn stp_term_to_bool(t: stp_term, out: *mut bool) -> stp_status;
	/// Whether the term is a bit-vector value that fits in a `uint64_t`.
	pub fn stp_term_fits_uint64(t: stp_term) -> bool;
	/// Whether the term is a bit-vector value that fits in an `int64_t`.
	pub fn stp_term_fits_int64(t: stp_term) -> bool;
	/// Reads an unsigned 64-bit value.
	pub fn stp_term_to_uint64(t: stp_term, out: *mut u64) -> stp_status;
	/// Reads a signed 64-bit value.
	pub fn stp_term_to_int64(t: stp_term, out: *mut i64) -> stp_status;
	/// Renders a bit-vector value in the given base.
	pub fn stp_term_to_bv_string(t: stp_term, base: c_int, pad: bool) -> *mut c_char;
	/// Number of 64-bit limbs in a bit-vector value.
	pub fn stp_term_bv_num_limbs(t: stp_term, out: *mut usize) -> stp_status;
	/// Reads a bit-vector value into limbs.
	pub fn stp_term_to_bv_limbs(t: stp_term, n: usize, out_lsb_first: *mut u64) -> stp_status;
	/// Reads a bit-vector value into bytes.
	pub fn stp_term_to_bv_bytes(t: stp_term, n: usize, out: *mut u8, little_endian: bool) -> stp_status;
	/// Reads a floating-point value.
	pub fn stp_term_to_fp(t: stp_term, out: *mut stp_float_value) -> stp_status;
	/// Reads the significand limbs of a floating-point value.
	pub fn stp_term_fp_significand_limbs(t: stp_term, n: usize, out_lsb_first: *mut u64) -> stp_status;
	/// Returns the IEEE interchange bits of a floating-point value.
	pub fn stp_term_fp_bits(t: stp_term) -> *mut c_char;
	/// Converts a floating-point value to a double.
	pub fn stp_term_fp_to_double(t: stp_term, out: *mut f64) -> stp_status;
	/// Converts a floating-point value to a rational.
	pub fn stp_term_fp_to_rational(t: stp_term, numerator: *mut *mut c_char, denominator: *mut *mut c_char)
	-> stp_status;
	/// Reads a rounding-mode value.
	pub fn stp_term_to_rm(t: stp_term, out: *mut stp_rm) -> stp_status;
	/// Returns a Real value's numerator.
	pub fn stp_term_real_numerator(t: stp_term) -> *mut c_char;
	/// Returns a Real value's denominator.
	pub fn stp_term_real_denominator(t: stp_term) -> *mut c_char;
	/// Whether a Real value fits in an `int64_t` pair.
	pub fn stp_term_real_fits_int64(t: stp_term) -> bool;
	/// Reads a Real value as a numerator/denominator pair.
	pub fn stp_term_real_to_int64(t: stp_term, num: *mut i64, den: *mut i64) -> stp_status;
	/// Converts a Real value to a double.
	pub fn stp_term_real_to_double(t: stp_term, out: *mut f64) -> stp_status;
	/// Reads an uninterpreted sort value's index.
	pub fn stp_term_to_uninterpreted_index(t: stp_term, out: *mut u64) -> stp_status;

	// -----------------------------------------------------------------------
	// Options
	// -----------------------------------------------------------------------

	/// Creates an options value with every entry at its default.
	pub fn stp_options_new() -> stp_options;
	/// Copies an options value.
	pub fn stp_options_copy(options: stp_options) -> stp_options;
	/// Deletes an options value.
	pub fn stp_options_delete(options: stp_options);
	/// Returns the options value's error record.
	pub fn stp_options_error(options: stp_options) -> *const stp_error;
	/// Clears the options value's error record.
	pub fn stp_options_clear_error(options: stp_options);
	/// Sets an option by name from a string.
	pub fn stp_options_set_str(options: stp_options, name: *const c_char, value: *const c_char) -> stp_status;
	/// Sets a Boolean option by name.
	pub fn stp_options_set_bool(options: stp_options, name: *const c_char, value: bool) -> stp_status;
	/// Sets an integer option by name.
	pub fn stp_options_set_int64(options: stp_options, name: *const c_char, value: i64) -> stp_status;
	/// Sets an unsigned option by name.
	pub fn stp_options_set_uint64(options: stp_options, name: *const c_char, value: u64) -> stp_status;
	/// Sets a duration option by name.
	pub fn stp_options_set_duration_ms(options: stp_options, name: *const c_char, ms: u64) -> stp_status;
	/// Sets a set-typed option by name.
	pub fn stp_options_set_names(
		options: stp_options,
		name: *const c_char,
		n: usize,
		members: *const *const c_char,
	) -> stp_status;
	/// Sets a Boolean stable option.
	pub fn stp_options_set_bool_e(options: stp_options, option: stp_option, value: bool) -> stp_status;
	/// Sets an integer stable option.
	pub fn stp_options_set_int64_e(options: stp_options, option: stp_option, value: i64) -> stp_status;
	/// Sets an unsigned stable option.
	pub fn stp_options_set_uint64_e(options: stp_options, option: stp_option, value: u64) -> stp_status;
	/// Sets a string stable option.
	pub fn stp_options_set_str_e(options: stp_options, option: stp_option, value: *const c_char) -> stp_status;
	/// Sets a duration stable option.
	pub fn stp_options_set_duration_ms_e(options: stp_options, option: stp_option, ms: u64) -> stp_status;
	/// Applies command-line arguments.
	pub fn stp_options_set_args(options: stp_options, argc: c_int, argv: *const *const c_char) -> stp_status;
	/// Gets an option as a string.
	pub fn stp_options_get_str(options: stp_options, name: *const c_char) -> *mut c_char;
	/// Gets a Boolean option.
	pub fn stp_options_get_bool(options: stp_options, name: *const c_char, out: *mut bool) -> stp_status;
	/// Gets an integer option.
	pub fn stp_options_get_int64(options: stp_options, name: *const c_char, out: *mut i64) -> stp_status;
	/// Gets an unsigned option.
	pub fn stp_options_get_uint64(options: stp_options, name: *const c_char, out: *mut u64) -> stp_status;
	/// Gets a duration option.
	pub fn stp_options_get_duration_ms(options: stp_options, name: *const c_char, out: *mut u64) -> stp_status;
	/// Gets an option's resolved string value.
	pub fn stp_options_resolved_str(options: stp_options, name: *const c_char) -> *mut c_char;
	/// Whether an option was explicitly set.
	pub fn stp_options_is_set(options: stp_options, name: *const c_char) -> bool;
	/// Resets an option to its default.
	pub fn stp_options_reset(options: stp_options, name: *const c_char) -> stp_status;
	/// Resets every option.
	pub fn stp_options_reset_all(options: stp_options);
	/// Resolves option implications.
	pub fn stp_options_resolve(options: stp_options) -> stp_status;
	/// Number of option names in a tier.
	pub fn stp_options_num_names(tier: c_int) -> usize;
	/// Returns the `i`-th option name in a tier.
	pub fn stp_options_name(tier: c_int, i: usize) -> *const c_char;
	/// Returns help for a tier.
	pub fn stp_options_help(tier: c_int) -> *mut c_char;
	/// Returns the name of a stable option.
	pub fn stp_option_name(option: stp_option) -> *const c_char;
	/// Looks up a stable option by name.
	pub fn stp_option_from_name(name: *const c_char, out: *mut stp_option) -> stp_status;
	/// Returns an option's type.
	pub fn stp_option_info_type(name: *const c_char) -> *const c_char;
	/// Returns an option's Python key.
	pub fn stp_option_info_python_key(name: *const c_char) -> *const c_char;
	/// Returns an option's default value.
	pub fn stp_option_info_default(name: *const c_char) -> *mut c_char;
	/// Returns an option's tier.
	pub fn stp_option_info_tier(name: *const c_char) -> stp_tier;
	/// Returns when an option may be set.
	pub fn stp_option_info_settable(name: *const c_char) -> stp_settable;
	/// Returns an option's scope.
	pub fn stp_option_info_scope(name: *const c_char) -> stp_option_scope;
	/// Returns an option's category.
	pub fn stp_option_info_category(name: *const c_char) -> *const c_char;
	/// Returns an option's help text.
	pub fn stp_option_info_help(name: *const c_char) -> *const c_char;
	/// Whether this build supports an option.
	pub fn stp_option_info_supported(name: *const c_char) -> bool;
	/// Returns an option's numeric range.
	pub fn stp_option_info_range(
		name: *const c_char,
		has_min: *mut bool,
		min: *mut i64,
		has_max: *mut bool,
		max: *mut i64,
	) -> stp_status;
	/// Number of values of an enum option.
	pub fn stp_option_info_num_values(name: *const c_char) -> usize;
	/// Returns one value of an enum option.
	pub fn stp_option_info_value(name: *const c_char, i: usize) -> *const c_char;
	/// Number of aliases of an option.
	pub fn stp_option_info_num_aliases(name: *const c_char) -> usize;
	/// Returns one alias of an option.
	pub fn stp_option_info_alias(name: *const c_char, i: usize) -> *const c_char;
	/// Returns an option's short name.
	pub fn stp_option_info_short(name: *const c_char) -> *const c_char;
	/// Returns an option's negation.
	pub fn stp_option_info_negation(name: *const c_char) -> *const c_char;

	// -----------------------------------------------------------------------
	// Solver
	// -----------------------------------------------------------------------

	/// Creates a solver over a manager.
	pub fn stp_solver_new(tm: stp_tm, options: stp_options) -> stp_solver;
	/// Deletes a solver.
	pub fn stp_solver_delete(solver: stp_solver);
	/// Returns the failure that put the solver in its failed state.
	pub fn stp_solver_failed(solver: stp_solver) -> *const stp_error;
	/// Number of terms named by the solver failure.
	pub fn stp_solver_failed_num_terms(solver: stp_solver) -> usize;
	/// Returns the `i`-th term named by the solver failure.
	pub fn stp_solver_failed_term(solver: stp_solver, i: usize) -> stp_term;
	/// Number of sorts named by the solver failure.
	pub fn stp_solver_failed_num_sorts(solver: stp_solver) -> usize;
	/// Returns the `i`-th sort named by the solver failure.
	pub fn stp_solver_failed_sort(solver: stp_solver, i: usize) -> stp_sort;
	/// Leaves the solver's failed state.
	pub fn stp_solver_clear_error(solver: stp_solver);
	/// Returns another manager handle.
	pub fn stp_solver_manager(solver: stp_solver) -> stp_tm;
	/// Sets a live solver option by name from a string.
	pub fn stp_solver_set_str(solver: stp_solver, name: *const c_char, value: *const c_char) -> stp_status;
	/// Sets a live Boolean solver option by name.
	pub fn stp_solver_set_bool(solver: stp_solver, name: *const c_char, value: bool) -> stp_status;
	/// Sets a live integer solver option by name.
	pub fn stp_solver_set_int64(solver: stp_solver, name: *const c_char, value: i64) -> stp_status;
	/// Sets a live unsigned solver option by name.
	pub fn stp_solver_set_uint64(solver: stp_solver, name: *const c_char, value: u64) -> stp_status;
	/// Sets a live duration solver option by name.
	pub fn stp_solver_set_duration_ms(solver: stp_solver, name: *const c_char, ms: u64) -> stp_status;
	/// Sets a live set-typed solver option by name.
	pub fn stp_solver_set_names(
		solver: stp_solver,
		name: *const c_char,
		n: usize,
		members: *const *const c_char,
	) -> stp_status;
	/// Sets a live Boolean stable solver option.
	pub fn stp_solver_set_bool_e(solver: stp_solver, option: stp_option, value: bool) -> stp_status;
	/// Sets a live integer stable solver option.
	pub fn stp_solver_set_int64_e(solver: stp_solver, option: stp_option, value: i64) -> stp_status;
	/// Sets a live unsigned stable solver option.
	pub fn stp_solver_set_uint64_e(solver: stp_solver, option: stp_option, value: u64) -> stp_status;
	/// Sets a live string stable solver option.
	pub fn stp_solver_set_str_e(solver: stp_solver, option: stp_option, value: *const c_char) -> stp_status;
	/// Sets a live duration stable solver option.
	pub fn stp_solver_set_duration_ms_e(solver: stp_solver, option: stp_option, ms: u64) -> stp_status;
	/// Applies command-line arguments to a solver.
	pub fn stp_solver_set_args(solver: stp_solver, argc: c_int, argv: *const *const c_char) -> stp_status;
	/// Gets a live solver option as a string.
	pub fn stp_solver_get_str(solver: stp_solver, name: *const c_char) -> *mut c_char;
	/// Gets a live Boolean solver option.
	pub fn stp_solver_get_bool(solver: stp_solver, name: *const c_char, out: *mut bool) -> stp_status;
	/// Gets a live integer solver option.
	pub fn stp_solver_get_int64(solver: stp_solver, name: *const c_char, out: *mut i64) -> stp_status;
	/// Gets a live unsigned solver option.
	pub fn stp_solver_get_uint64(solver: stp_solver, name: *const c_char, out: *mut u64) -> stp_status;
	/// Gets a live duration solver option.
	pub fn stp_solver_get_duration_ms(solver: stp_solver, name: *const c_char, out: *mut u64) -> stp_status;
	/// Gets a live solver option's resolved string value.
	pub fn stp_solver_resolved_str(solver: stp_solver, name: *const c_char) -> *mut c_char;
	/// Whether a live solver option was explicitly set.
	pub fn stp_solver_option_is_set(solver: stp_solver, name: *const c_char) -> bool;
	/// Resets a live solver option to its default.
	pub fn stp_solver_reset_option(solver: stp_solver, name: *const c_char) -> stp_status;
	/// Resets every live solver option.
	pub fn stp_solver_reset_all_options(solver: stp_solver) -> stp_status;
	/// Resolves live solver option implications.
	pub fn stp_solver_resolve_options(solver: stp_solver) -> stp_status;
	/// Copies a solver's live options.
	pub fn stp_solver_options_copy(solver: stp_solver) -> stp_options;
	/// Asserts a Boolean formula.
	pub fn stp_solver_assert(solver: stp_solver, term: stp_term) -> stp_status;
	/// Opens `n` assertion scopes.
	pub fn stp_solver_push(solver: stp_solver, n: u32) -> stp_status;
	/// Closes `n` assertion scopes.
	pub fn stp_solver_pop(solver: stp_solver, n: u32) -> stp_status;
	/// Returns the current assertion level.
	pub fn stp_solver_level(solver: stp_solver) -> u32;
	/// Returns the logic named by the last successful parse.
	pub fn stp_solver_declared_logic(solver: stp_solver) -> *mut c_char;
	/// Number of assertions.
	pub fn stp_solver_num_assertions(solver: stp_solver) -> usize;
	/// Returns the `i`-th assertion.
	pub fn stp_solver_assertion(solver: stp_solver, i: usize) -> stp_term;
	/// Removes every assertion, keeping options.
	pub fn stp_solver_reset_assertions(solver: stp_solver) -> stp_status;
	/// Resets the solver completely.
	pub fn stp_solver_reset(solver: stp_solver) -> stp_status;
	/// Checks satisfiability.
	pub fn stp_solver_check_sat(solver: stp_solver, out: *mut stp_result) -> stp_status;
	/// Checks satisfiability under assumptions.
	pub fn stp_solver_check_sat_assuming(
		solver: stp_solver,
		n: usize,
		assumptions: *const stp_term,
		out: *mut stp_result,
	) -> stp_status;
	/// Checks satisfiability under assumptions and a budget.
	pub fn stp_solver_check_sat_budget(
		solver: stp_solver,
		n: usize,
		assumptions: *const stp_term,
		budget: *const stp_budget,
		out: *mut stp_result,
	) -> stp_status;
	/// Checks whether the assertions entail a formula.
	pub fn stp_solver_entails(
		solver: stp_solver,
		formula: stp_term,
		budget: *const stp_budget,
		out: *mut stp_entailment,
	) -> stp_status;
	/// Returns the message behind the last result's reason.
	pub fn stp_solver_last_reason_message(solver: stp_solver) -> *mut c_char;
	/// Number of failed assumptions after an unsatisfiable check.
	pub fn stp_solver_num_unsat_assumptions(solver: stp_solver) -> usize;
	/// Returns the `i`-th failed assumption.
	pub fn stp_solver_unsat_assumption(solver: stp_solver, i: usize) -> stp_term;
	/// Returns the model of the last satisfiable check.
	pub fn stp_solver_model(solver: stp_solver) -> stp_model;
	/// Returns the candidate model, if any.
	pub fn stp_solver_candidate_model(solver: stp_solver) -> stp_model;
	/// Looks up a term's value in the shared model snapshot.
	pub fn stp_solver_value(solver: stp_solver, term: stp_term) -> stp_term;
	/// Requests that the next check stop.
	pub fn stp_solver_interrupt(solver: stp_solver);
	/// Discards a pending interrupt.
	pub fn stp_solver_clear_interrupt(solver: stp_solver);
	/// Whether an interrupt is pending.
	pub fn stp_solver_interrupt_pending(solver: stp_solver) -> bool;
	/// Installs a terminator callback.
	pub fn stp_solver_set_terminator(solver: stp_solver, cb: stp_terminate_callback, user: *mut c_void) -> stp_status;
	/// Returns a statistics snapshot.
	pub fn stp_solver_statistics(solver: stp_solver) -> stp_statistics;
	/// Looks up a symbol known to the solver.
	pub fn stp_solver_symbol(solver: stp_solver, name: *const c_char) -> stp_term;
	/// Parses an SMT-LIB 2 script.
	pub fn stp_solver_parse_smt2(solver: stp_solver, script: *const c_char, mode: stp_parse_mode) -> stp_status;
	/// Parses text in the given format.
	pub fn stp_solver_parse(solver: stp_solver, text: *const c_char, format: stp_format) -> stp_status;
	/// Parses a file in the given format.
	pub fn stp_solver_parse_file(solver: stp_solver, path: *const c_char, format: stp_format) -> stp_status;
	/// Parses a single SMT-LIB 2 term.
	pub fn stp_solver_parse_term(solver: stp_solver, smt2_term: *const c_char) -> stp_term;
	/// Renders the solver's assertions as SMT-LIB 2.
	pub fn stp_solver_to_smt2(solver: stp_solver, with_check_sat: bool) -> *mut c_char;
	/// Renders the solver in the given format.
	pub fn stp_solver_to_string(solver: stp_solver, format: stp_format) -> *mut c_char;
	/// Writes the assertions' CNF to a sink.
	pub fn stp_solver_write_cnf(
		solver: stp_solver,
		sink: stp_text_sink,
		user: *mut c_void,
		scope: *mut stp_cnf_scope,
	) -> stp_status;
	/// Installs a diagnostic-text sink.
	pub fn stp_solver_set_diagnostic_sink(solver: stp_solver, sink: stp_text_sink, user: *mut c_void);
	/// Parses from a text source.
	pub fn stp_solver_parse_source(
		solver: stp_solver,
		source: stp_text_source,
		user: *mut c_void,
		format: stp_format,
		mode: stp_parse_mode,
	) -> stp_status;
	/// Installs an output-text sink.
	pub fn stp_solver_set_output_sink(solver: stp_solver, sink: stp_text_sink, user: *mut c_void);
	/// Installs a fatal-error handler.
	pub fn stp_solver_set_fatal_error_handler(solver: stp_solver, handler: stp_fatal_error_handler, user: *mut c_void);
	/// Installs a CNF sink.
	pub fn stp_solver_set_cnf_sink(solver: stp_solver, sink: stp_cnf_sink, user: *mut c_void);

	// -----------------------------------------------------------------------
	// Model
	// -----------------------------------------------------------------------

	/// Copies a model handle.
	pub fn stp_model_copy(model: stp_model) -> stp_model;
	/// Releases a model handle.
	pub fn stp_model_release(model: stp_model);
	/// Returns another manager handle.
	pub fn stp_model_manager(model: stp_model) -> stp_tm;
	/// Returns a term's value in the model.
	pub fn stp_model_value(model: stp_model, term: stp_term) -> stp_term;
	/// Returns a term's value without completing it.
	pub fn stp_model_try_value(model: stp_model, term: stp_term) -> stp_term;
	/// Reads several term values at once.
	pub fn stp_model_values(model: stp_model, n: usize, input: *const stp_term, out: *mut stp_term) -> stp_status;
	/// Reads a Boolean value.
	pub fn stp_model_bool(model: stp_model, term: stp_term, out: *mut bool) -> stp_status;
	/// Reads an unsigned 64-bit value.
	pub fn stp_model_uint64(model: stp_model, term: stp_term, out: *mut u64) -> stp_status;
	/// Reads a signed 64-bit value.
	pub fn stp_model_int64(model: stp_model, term: stp_term, out: *mut i64) -> stp_status;
	/// Renders a bit-vector value in the given base.
	pub fn stp_model_bv_string(model: stp_model, term: stp_term, base: c_int, pad: bool) -> *mut c_char;
	/// Number of 64-bit limbs in a bit-vector value.
	pub fn stp_model_bv_num_limbs(model: stp_model, term: stp_term, out: *mut usize) -> stp_status;
	/// Reads a bit-vector value into limbs.
	pub fn stp_model_bv_limbs(model: stp_model, term: stp_term, n: usize, out_lsb_first: *mut u64) -> stp_status;
	/// Reads a bit-vector value into bytes.
	pub fn stp_model_bv_bytes(
		model: stp_model,
		term: stp_term,
		n: usize,
		out: *mut u8,
		little_endian: bool,
	) -> stp_status;
	/// Reads a floating-point value.
	pub fn stp_model_fp(model: stp_model, term: stp_term, out: *mut stp_float_value) -> stp_status;
	/// Reads the significand limbs of a floating-point value.
	pub fn stp_model_fp_significand_limbs(
		model: stp_model,
		term: stp_term,
		n: usize,
		out_lsb_first: *mut u64,
	) -> stp_status;
	/// Converts a floating-point value to a double.
	pub fn stp_model_fp_to_double(model: stp_model, term: stp_term, out: *mut f64) -> stp_status;
	/// Reads a rounding-mode value.
	pub fn stp_model_rm(model: stp_model, term: stp_term, out: *mut stp_rm) -> stp_status;
	/// Returns a Real value's numerator.
	pub fn stp_model_real_numerator(model: stp_model, term: stp_term) -> *mut c_char;
	/// Returns a Real value's denominator.
	pub fn stp_model_real_denominator(model: stp_model, term: stp_term) -> *mut c_char;
	/// Reads an uninterpreted sort value's index.
	pub fn stp_model_uninterpreted_index(model: stp_model, term: stp_term, out: *mut u64) -> stp_status;
	/// Returns an array value.
	pub fn stp_model_array_value(model: stp_model, array: stp_term) -> stp_array_value;
	/// Returns a function value.
	pub fn stp_model_fun_value(model: stp_model, fun: stp_term) -> stp_fun_value;
	/// Reads a dense range of array bytes.
	pub fn stp_model_array_bytes(
		model: stp_model,
		array: stp_term,
		first_index: u64,
		count: usize,
		out: *mut u8,
	) -> stp_status;
	/// Number of symbols in the model core.
	pub fn stp_model_num_symbols(model: stp_model) -> usize;
	/// Returns the `i`-th symbol in the model core.
	pub fn stp_model_symbol(model: stp_model, i: usize) -> stp_term;
	/// Whether a symbol is in the model core.
	pub fn stp_model_in_core(model: stp_model, symbol: stp_term) -> bool;
	/// Renders the whole model as SMT-LIB 2.
	pub fn stp_model_to_smt2(model: stp_model) -> *mut c_char;

	// -----------------------------------------------------------------------
	// Array and function values
	// -----------------------------------------------------------------------

	/// Releases an array value.
	pub fn stp_array_value_release(value: stp_array_value);
	/// Returns an array value's sort.
	pub fn stp_array_value_sort(value: stp_array_value) -> stp_sort;
	/// Returns an array value's default element.
	pub fn stp_array_value_default(value: stp_array_value) -> stp_term;
	/// Number of explicit entries.
	pub fn stp_array_value_size(value: stp_array_value) -> usize;
	/// Reads an explicit entry.
	pub fn stp_array_value_entry(
		value: stp_array_value,
		i: usize,
		index: *mut stp_term,
		element: *mut stp_term,
	) -> stp_status;
	/// Reads an array value at an index.
	pub fn stp_array_value_at(value: stp_array_value, index_value: stp_term) -> stp_term;
	/// Renders an array value as a term.
	pub fn stp_array_value_as_term(value: stp_array_value) -> stp_term;
	/// Releases a function value.
	pub fn stp_fun_value_release(value: stp_fun_value);
	/// Returns a function value's sort.
	pub fn stp_fun_value_sort(value: stp_fun_value) -> stp_sort;
	/// Returns a function value's arity.
	pub fn stp_fun_value_arity(value: stp_fun_value) -> u32;
	/// Whether a function value is tabular.
	pub fn stp_fun_value_is_tabular(value: stp_fun_value) -> bool;
	/// Returns a function value's else branch.
	pub fn stp_fun_value_else(value: stp_fun_value) -> stp_term;
	/// Number of entries in a function value.
	pub fn stp_fun_value_size(value: stp_fun_value) -> usize;
	/// Reads an entry from a function value.
	pub fn stp_fun_value_entry(
		value: stp_fun_value,
		i: usize,
		n: usize,
		args_out: *mut stp_term,
		result: *mut stp_term,
	) -> stp_status;
	/// Applies a function value.
	pub fn stp_fun_value_apply(value: stp_fun_value, n: usize, arg_values: *const stp_term) -> stp_term;
	/// Renders a function value as an if-then-else.
	pub fn stp_fun_value_as_ite(value: stp_fun_value, n: usize, formals: *const stp_term) -> stp_term;

	// -----------------------------------------------------------------------
	// Statistics
	// -----------------------------------------------------------------------

	/// Releases a statistics snapshot.
	pub fn stp_statistics_release(stats: stp_statistics);
	/// Number of statistics.
	pub fn stp_statistics_size(stats: stp_statistics) -> usize;
	/// Returns the `i`-th statistic name.
	pub fn stp_statistics_name(stats: stp_statistics, i: usize) -> *const c_char;
	/// Whether a statistic is an unsigned integer.
	pub fn stp_statistics_is_uint64(stats: stp_statistics, name: *const c_char) -> bool;
	/// Whether a statistic is a double.
	pub fn stp_statistics_is_double(stats: stp_statistics, name: *const c_char) -> bool;
	/// Reads an unsigned statistic.
	pub fn stp_statistics_uint64(stats: stp_statistics, name: *const c_char, out: *mut u64) -> stp_status;
	/// Reads a double statistic.
	pub fn stp_statistics_double(stats: stp_statistics, name: *const c_char, out: *mut f64) -> stp_status;
	/// Reads a statistic as text.
	pub fn stp_statistics_str(stats: stp_statistics, name: *const c_char) -> *mut c_char;
	/// Returns a statistic's tier.
	pub fn stp_statistics_tier(name: *const c_char) -> stp_tier;
}
