/// A Python Machine Learning Library implemented in Rust.

use pyo3::prelude::*;

pub mod datatable;
pub mod model;
mod dt_pybinds;

use crate::dt_pybinds::{ColumnPy, DataTablePy};

/// A Python module implemented in Rust.
#[pymodule]
fn ml361(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<ColumnPy>()?;
    m.add_class::<DataTablePy>()?;
    Ok(())
}
