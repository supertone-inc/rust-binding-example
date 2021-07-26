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

pub fn init_module(py: Python) -> PyResult<&PyModule> {
    let m = PyModule::new(py, "error")?;

    m.add_function(wrap_pyfunction!(raise_error, m)?)?;

    Ok(m)
}

#[pyfunction]
fn raise_error() -> Result<(), Error> {
    Ok(hello::error::raise_error()?)
}
