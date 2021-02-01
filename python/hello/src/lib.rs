use pyo3::{prelude::*, *};

#[pymodule]
fn hello(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(print_string, m)?)?;
    m.add_function(wrap_pyfunction!(get_string, m)?)?;

    Ok(())
}

#[pyfunction]
fn print_string(name: &str) {
    hello::print_string(name);
}

#[pyfunction]
fn get_string(name: &str) -> String {
    hello::get_string(name)
}
