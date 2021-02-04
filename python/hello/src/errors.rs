use pyo3::{exceptions::PyException, prelude::PyErr};

error_chain! {
    links {
        Hello(hello::errors::Error, hello::errors::ErrorKind);
    }
}

impl std::convert::From<Error> for PyErr {
    fn from(err: Error) -> PyErr {
        PyException::new_err(err.to_string())
    }
}
