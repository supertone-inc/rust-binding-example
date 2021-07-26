use pyo3::{exceptions::PyException, prelude::*};
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
