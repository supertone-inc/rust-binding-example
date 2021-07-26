mod error;

use error::Error;
use pyo3::{prelude::*, wrap_pyfunction};

#[pymodule]
fn hello(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(to_uppercase, m)?)?;
    m.add_function(wrap_pyfunction!(concat, m)?)?;
    m.add_function(wrap_pyfunction!(raise_error, m)?)?;

    Ok(())
}

#[pyfunction]
fn to_uppercase(s: &str) -> String {
    hello::to_uppercase(s)
}

#[pyfunction]
fn concat(a: Vec<f32>, b: Vec<f32>) -> Vec<f32> {
    hello::concat(&a, &b)
}

#[pyfunction]
pub fn raise_error() -> Result<(), Error> {
    Ok(hello::raise_error()?)
}
