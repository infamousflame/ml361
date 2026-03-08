use crate::datatable::{Column, DataTable, Value};
use pyo3::exceptions::{PyIndexError, PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::{PyBool, PyFloat, PyInt, PyString};

/// Helper to extract a `Value` from a `PyAny` object.
fn extract_value(value: Bound<'_, PyAny>) -> PyResult<Value> {
    if value.is_instance_of::<PyBool>() {
        Ok(Value::Bool(value.extract::<bool>()?))
    } else if value.is_instance_of::<PyInt>() {
        Ok(Value::Int(value.extract::<isize>()?))
    } else if value.is_instance_of::<PyFloat>() {
        Ok(Value::Float(value.extract::<f64>()?))
    } else if value.is_instance_of::<PyString>() {
        Ok(Value::Str(value.extract::<String>()?))
    } else {
        Err(PyTypeError::new_err("Unsupported value type"))
    }
}

/// Wrapper around a `Column` for use with Python bindings.
#[pyclass(name = "Column")]
pub struct ColumnPy {
    pub inner: Column,
}

/// Wrapper around a `DataTable` for use with Python bindings.
#[pyclass(name = "DataTable")]
pub struct DataTablePy {
    pub inner: DataTable,
}

#[pymethods]
impl ColumnPy {
    /// Creates a new `Column` with the given type.
    #[new]
    fn new(column_type: &str) -> PyResult<Self> {
        match Column::new(column_type) {
            Ok(column) => Ok(Self { inner: column }),
            Err(e) => Err(PyValueError::new_err(e.to_string())),
        }
    }

    /// Creates a new `Column` from a list of values.
    #[staticmethod]
    fn from_list(column_type: &str, list: Bound<'_, PyAny>) -> PyResult<Self> {
        let mut col_py = Self::new(column_type)?;
        for item in list.try_iter()? {
            col_py.append(item?)?;
        }
        Ok(col_py)
    }

    /// Appends a value to the column.
    fn append(&mut self, value: Bound<'_, PyAny>) -> PyResult<()> {
        let extracted_value = extract_value(value)?;
        if let Err(e) = self.inner.append(&extracted_value) {
            return Err(PyValueError::new_err(e.to_string()));
        }
        Ok(())
    }

    /// Removes and returns the value at the given index.
    fn pop<'py>(
        &mut self,
        py: Python<'py>,
        index: usize,
    ) -> PyResult<Bound<'py, PyAny>> {
        match self.inner.pop(index) {
            Ok(value) => {
                let result: Bound<'py, PyAny> = match value {
                    Value::Int(i) => PyInt::new(py, i).into_any(),
                    Value::Float(f) => PyFloat::new(py, f).into_any(),
                    Value::Bool(b) => PyBool::new(py, b).to_owned().into_any(),
                    Value::Str(s) => PyString::new(py, &s).into_any(),
                };
                Ok(result)
            }
            Err(e) => Err(PyIndexError::new_err(e.to_string())),
        }
    }

    /// Returns the value at the given index.
    fn __getitem__<'py>(
        &self,
        py: Python<'py>,
        index: usize,
    ) -> PyResult<Bound<'py, PyAny>> {
        match self.inner.get(index) {
            Ok(value) => {
                let result: Bound<'py, PyAny> = match value {
                    Value::Int(i) => PyInt::new(py, i).into_any(),
                    Value::Float(f) => PyFloat::new(py, f).into_any(),
                    Value::Bool(b) => PyBool::new(py, b).to_owned().into_any(),
                    Value::Str(s) => PyString::new(py, &s).into_any(),
                };
                Ok(result)
            }
            Err(e) => Err(PyIndexError::new_err(e.to_string())),
        }
    }

    /// Sets the value at the given index.
    fn __setitem__(
        &mut self,
        index: usize,
        value: Bound<'_, PyAny>,
    ) -> PyResult<()> {
        let extracted_value = extract_value(value)?;
        if let Err(e) = self.inner.set(index, &extracted_value) {
            return Err(PyIndexError::new_err(e.to_string()));
        }
        Ok(())
    }

    /// Returns the length of the column
    fn __len__(&self) -> usize {
        self.inner.len()
    }

    /// Formats the column as a string.
    fn __str__(&self) -> String {
        self.inner.format()
    }
}

#[pymethods]
impl DataTablePy {
    /// Creates a new `DataTable` with the given schema.
    #[new]
    fn new(
        column_names: Vec<String>,
        column_types: Vec<String>,
    ) -> PyResult<Self> {
        let names: Vec<&str> =
            column_names.iter().map(|s| s.as_str()).collect();
        let types: Vec<&str> =
            column_types.iter().map(|s| s.as_str()).collect();
        match DataTable::new(&names, &types) {
            Ok(inner) => Ok(DataTablePy { inner }),
            Err(e) => Err(PyValueError::new_err(e.to_string())),
        }
    }

    /// Creates a new `DataTable` from lists of values.
    #[staticmethod]
    fn from_lists(
        column_names: Vec<String>,
        column_types: Vec<String>,
        data: Bound<'_, PyAny>,
    ) -> PyResult<Self> {
        let mut dt_py = Self::new(column_names, column_types)?;
        for row in data.try_iter()? {
            let row_vec: Vec<Bound<'_, PyAny>> = row?.extract()?;
            dt_py.append(row_vec)?;
        }
        Ok(dt_py)
    }

    /// Appends a row to the table.
    fn append(&mut self, row: Vec<Bound<'_, PyAny>>) -> PyResult<()> {
        let mut extracted_values: Vec<Value> = Vec::new();
        for v in row {
            extracted_values.push(extract_value(v)?);
        }
        if let Err(e) = self.inner.append(extracted_values) {
            return Err(PyValueError::new_err(e.to_string()));
        }
        Ok(())
    }

    /// Removes and returns a row from the table.
    fn pop<'py>(
        &mut self,
        py: Python<'py>,
        index: usize,
    ) -> PyResult<Vec<Bound<'py, PyAny>>> {
        match self.inner.pop(index) {
            Ok(row) => {
                let values: Vec<Bound<'py, PyAny>> = row
                    .into_iter()
                    .map(|v| match v {
                        Value::Bool(b) => {
                            PyBool::new(py, b).to_owned().into_any()
                        }
                        Value::Int(i) => PyInt::new(py, i).into_any(),
                        Value::Float(f) => PyFloat::new(py, f).into_any(),
                        Value::Str(s) => PyString::new(py, &s).into_any(),
                    })
                    .collect();
                Ok(values)
            }
            Err(e) => Err(PyIndexError::new_err(e.to_string())),
        }
    }

    /// Returns a row from the DataTable as a tuple of values
    fn get_row<'py>(
        &self,
        py: Python<'py>,
        row_index: usize,
    ) -> PyResult<Vec<Bound<'py, PyAny>>> {
        match self.inner.get_row(row_index) {
            Ok(row) => {
                let values: Vec<Bound<'py, PyAny>> = row
                    .into_iter()
                    .map(|v| match v {
                        Value::Bool(b) => {
                            PyBool::new(py, b).to_owned().into_any()
                        }
                        Value::Int(i) => PyInt::new(py, i).into_any(),
                        Value::Float(f) => PyFloat::new(py, f).into_any(),
                        Value::Str(s) => PyString::new(py, &s).into_any(),
                    })
                    .collect();
                Ok(values)
            }
            Err(e) => Err(PyIndexError::new_err(e.to_string())),
        }
    }

    /// Returns the number of rows in the table.
    fn __len__(&self) -> usize {
        self.inner.len()
    }

    /// Returns an element from the DataTable as a Python value
    fn __getitem__<'py>(
        &self,
        py: Python<'py>,
        key: Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        // Expecting key to be (col_name, row_index)
        let (col_name, row_index): (String, usize) = key.extract()?;
        match self.inner.get(&col_name, row_index) {
            Ok(value) => {
                let result = match value {
                    Value::Bool(b) => PyBool::new(py, b).to_owned().into_any(),
                    Value::Int(i) => PyInt::new(py, i).into_any(),
                    Value::Float(f) => PyFloat::new(py, f).into_any(),
                    Value::Str(s) => PyString::new(py, &s).into_any(),
                };
                Ok(result)
            }
            Err(e) => Err(PyIndexError::new_err(e.to_string())),
        }
    }

    /// Sets an element in the DataTable
    fn __setitem__(
        &mut self,
        key: Bound<'_, PyAny>,
        value: Bound<'_, PyAny>,
    ) -> PyResult<()> {
        let (col_name, row_index): (String, usize) = key.extract()?;
        let extracted_value = extract_value(value)?;
        if let Err(e) = self.inner.set(&col_name, row_index, &extracted_value) {
            Err(PyValueError::new_err(e.to_string()))
        } else {
            Ok(())
        }
    }

    /// Returns a string representation of the DataTable.
    fn __str__(&self) -> String {
        self.inner.format()
    }
}
