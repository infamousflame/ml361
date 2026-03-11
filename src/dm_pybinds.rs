use pyo3::exceptions::PyValueError;
/// Contains Python bindings for the `dataman` module.
use pyo3::prelude::*;

use crate::dataman::train_test_split;
use crate::dt_pybinds::DataTablePy;

#[pyfunction(name = "train_test_split")]
pub fn train_test_split_py(
    data: &DataTablePy,
    test_size: f64,
    seed: Option<u64>,
) -> PyResult<(DataTablePy, DataTablePy)> {
    let (train, test) = match train_test_split(&data.inner, test_size, seed) {
        Ok((tr, te)) => (tr, te),
        Err(e) => return Err(PyValueError::new_err(e.to_string())),
    };
    Ok((DataTablePy { inner: train }, DataTablePy { inner: test }))
}
