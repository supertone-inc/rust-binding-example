use pyo3::prelude::*;

pub fn init_module(m: &PyModule) -> PyResult<&PyModule> {
    m.add_function(wrap_pyfunction!(concat, m)?)?;

    Ok(m)
}

#[pyfunction]
fn concat(a: Vec<f32>, b: Vec<f32>) -> Vec<f32> {
    hello::array::concat(&a, &b)
}
