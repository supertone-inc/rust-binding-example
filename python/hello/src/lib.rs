#[macro_use]
extern crate error_chain;

mod errors;
use errors::*;

use pyo3::{prelude::*, *};

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
fn raise_error() -> Result<()> {
    hello::raise_error().map_err(|err| err.into())
}
