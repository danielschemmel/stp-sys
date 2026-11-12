//! Unit tests for the vendored SAT backends.
//!
//! Each vendored SAT backend has a child module of the same name under
//! `test::backend` (for example `test::backend::cadical`).

#[cfg(feature = "vendor")]
mod backend;
