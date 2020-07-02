mod rustyjeff;

use pyo3::prelude::*;
use pyo3::{exceptions, wrap_pyfunction};

#[pyfunction]
fn rsa_decrypt_base64(priv_key_pem: &str, field_values: Vec<&str>) -> PyResult<Vec<String>> {
    match rustyjeff::rsa_decrypt_base64(priv_key_pem, field_values) {
        Ok(result) => Ok(result),
        Err(msg) => exceptions::ValueError::into(msg),
    }
}

#[pymodule]
fn rustyjeff(_: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(rsa_decrypt_base64))?;

    Ok(())
}
