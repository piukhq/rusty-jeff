mod rustyjeff;

use pyo3::prelude::*;
use pyo3::{exceptions, wrap_pyfunction};

#[pyfunction]
fn rsa_decrypt_base64(priv_key_pem: &str, base64_text: &str) -> PyResult<String> {
    match rustyjeff::rsa_decrypt_base64(priv_key_pem, base64_text) {
        Ok(result) => Ok(result),
        Err(msg) => exceptions::ValueError::into(msg),
    }
}

#[pymodule]
fn rustyjeff(_: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(rsa_decrypt_base64))?;

    Ok(())
}
