use pyo3::prelude::*;

pub fn init_module(m: &PyModule) -> PyResult<&PyModule> {
    m.add_function(wrap_pyfunction!(map, m)?)?;

    Ok(m)
}

#[pyfunction]
fn map(items: Vec<PyObject>, mapper: PyObject) -> Vec<PyObject> {
    hello::callback::map(&items, move |item| {
        Python::with_gil(|py| mapper.call1(py, (item,))).unwrap()
    })
}
