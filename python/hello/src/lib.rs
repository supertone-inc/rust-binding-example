use pyo3::{prelude::*, *};

#[pymodule]
fn hello(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(to_uppercase, m)?)?;

    Ok(())
}

#[pyfunction]
fn to_uppercase(s: &str) -> String {
    hello::to_uppercase(s)
}
