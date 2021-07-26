mod array;
mod error;
mod string;

use pyo3::prelude::*;

#[pymodule]
fn hello(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_submodule(array::init_module(PyModule::new(py, "array")?)?)?;
    m.add_submodule(error::init_module(PyModule::new(py, "error")?)?)?;
    m.add_submodule(string::init_module(PyModule::new(py, "string")?)?)?;

    Ok(())
}
