mod error;
mod string;

use pyo3::{prelude::*, wrap_pyfunction};

#[pymodule]
fn hello(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(concat, m)?)?;

    m.add_submodule(error::init_module(PyModule::new(py, "error")?)?)?;
    m.add_submodule(string::init_module(PyModule::new(py, "string")?)?)?;

    Ok(())
}

#[pyfunction]
fn concat(a: Vec<f32>, b: Vec<f32>) -> Vec<f32> {
    hello::concat(&a, &b)
}
