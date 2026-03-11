/// A Python Machine Learning Library implemented in Rust.
use pyo3::prelude::*;

pub mod dataman;
pub mod datatable;
mod dt_pybinds;
pub mod model;

use crate::dt_pybinds::{ColumnPy, DataTablePy};

/// A Python module implemented in Rust.
// #[pymodule(name="datatable")]
fn pymod_datatable(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<ColumnPy>()?;
    m.add_class::<DataTablePy>()?;
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

    Ok(())
}
