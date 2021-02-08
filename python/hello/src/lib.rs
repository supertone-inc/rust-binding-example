use pyo3::{exceptions::PyException, prelude::*, wrap_pyfunction};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum Error {
    #[error(transparent)]
    Hello(#[from] hello::Error),
}

impl std::convert::From<Error> for PyErr {
    fn from(err: Error) -> PyErr {
        PyException::new_err(err.to_string())
    }
}

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
fn raise_error() -> Result<(), Error> {
    Ok(hello::raise_error()?)
}
