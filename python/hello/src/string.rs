use pyo3::{prelude::*, wrap_pyfunction};

pub fn init_module(m: &PyModule) -> PyResult<&PyModule> {
    m.add_function(wrap_pyfunction!(to_uppercase, m)?)?;

    Ok(m)
}

#[pyfunction]
fn to_uppercase(s: &str) -> String {
    hello::string::to_uppercase(s)
}
