use pyo3::{exceptions::PyException, prelude::*, wrap_pyfunction};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Hello(#[from] hello::Error),
}

impl std::convert::From<Error> for PyErr {
    fn from(err: Error) -> PyErr {
        PyException::new_err(err.to_string())
    }
}

pub fn init_module(m: &PyModule) -> PyResult<&PyModule> {
    m.add_function(wrap_pyfunction!(throw_error, m)?)?;

    Ok(m)
}

#[pyfunction]
fn throw_error() -> Result<(), Error> {
    Ok(hello::error::throw_error()?)
}
