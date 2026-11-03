//! Unit tests for the vendored CryptoMiniSat SAT backend.

use super::sat_backends::assert_backend_yields_nan_model;

#[test]
fn nan_model() {
	assert_backend_yields_nan_model("cryptominisat");
}
