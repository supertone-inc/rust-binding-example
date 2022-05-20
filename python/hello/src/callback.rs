use pyo3::prelude::*;

pub fn init_module(m: &PyModule) -> PyResult<&PyModule> {
    m.add_function(wrap_pyfunction!(map, m)?)?;

    Ok(m)
}

#[pyfunction]
fn map(items: Vec<PyObject>, mapper: PyObject) -> PyResult<Vec<PyObject>> {
    Python::with_gil(|py| hello::callback::map(&items, |item| mapper.call1(py, (item,))))
        .into_iter()
        .collect()
}
