/// A Python Machine Learning Library implemented in Rust.
use pyo3::prelude::*;

pub mod dataman;
pub mod datatable;
mod dm_pybinds;
mod dt_pybinds;
mod dtree;
pub mod model;

use crate::dm_pybinds::train_test_split_py;
use crate::dt_pybinds::{ColumnPy, DataTablePy};

fn pymod_datatable(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<ColumnPy>()?;
    m.add_class::<DataTablePy>()?;
    Ok(())
}

fn pymod_dataman(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(train_test_split_py, m)?)?;
    Ok(())
}

#[pymodule]
fn ml361(m: &Bound<'_, PyModule>) -> PyResult<()> {
    let py = m.py();

    let datatable = PyModule::new(py, "ml361.datatable")?;
    pymod_datatable(&datatable)?;
    m.add_submodule(&datatable)?;

    // Explicitly add the submodule to sys.modules so that `from ml361.datatable import ...` works.
    py.import("sys")?
        .getattr("modules")?
        .set_item("ml361.datatable", &datatable)?;

    let dataman = PyModule::new(py, "ml361.dataman")?;
    pymod_dataman(&dataman)?;
    m.add_submodule(&dataman)?;

    // Explicitly add the submodule to sys.modules so that `from ml361.dataman import ...` works.
    py.import("sys")?
        .getattr("modules")?
        .set_item("ml361.dataman", &dataman)?;

    Ok(())
}
