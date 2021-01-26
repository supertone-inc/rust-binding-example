use pyo3::{prelude::*, *};

#[pymodule]
fn hello(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(greet, m)?)?;

    Ok(())
}

#[pyfunction]
fn greet() -> PyResult<()> {
    hello::greet();

    Ok(())
}
