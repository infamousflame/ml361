/// Contains structures used to store data.

use std::collections::HashMap;

/// A value to be stored or retrieved from a DataTable column
#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Int(isize),
    Float(f64),
    Bool(bool),
    Str(String),
}

/// A column in a DataTable, containing values of a specific type
#[derive(Debug, PartialEq)]
pub enum Column {
    Int(Vec<isize>),
    Float(Vec<f64>),
    Bool(Vec<bool>),
    Str(Vec<String>),
}

/// A DataTable containing columns of various types
pub struct DataTable {
    columns: Vec<(String, Column)>, // Stores the column name and values
    name_map: HashMap<String, usize>, // Maps column names to their index
}

impl Column {
    /// Creates a new column of the specified type
    pub fn new(column_type: &str) -> Result<Self, &'static str> {
        match column_type {
            "int" => Ok(Column::Int(Vec::new())),
            "float" => Ok(Column::Float(Vec::new())),
            "bool" => Ok(Column::Bool(Vec::new())),
            "str" => Ok(Column::Str(Vec::new())),
            _ => Err("Invalid column type"),
        }
    }

    /// Creates a new column from an array of values
    pub fn from_array(
        column_type: &str,
        values: &[Value],
    ) -> Result<Self, &'static str> {
        let mut col = Column::new(column_type)?;
        for value in values {
            col.append(value).map_err(|_| "Value type mismatch")?;
        }
        Ok(col)
    }

    /// Appends a value to the column
    pub fn append(&mut self, value: &Value) -> Result<(), &str> {
        match self {
            Column::Int(values) => {
                if let Value::Int(v) = value {
                    values.push(*v);
                    Ok(())
                } else {
                    Err("Value type mismatch: expected int")
                }
            }
            Column::Float(values) => {
                if let Value::Float(v) = value {
                    values.push(*v);
                    Ok(())
                } else {
                    Err("Value type mismatch: expected float")
                }
            }
            Column::Bool(values) => {
                if let Value::Bool(v) = value {
                    values.push(*v);
                    Ok(())
                } else {
                    Err("Value type mismatch: expected bool")
                }
            }
            Column::Str(values) => {
                if let Value::Str(v) = value {
                    values.push(v.clone());
                    Ok(())
                } else {
                    Err("Value type mismatch: expected string")
                }
            }
        }
    }

    /// Removes and returns a value from the column at the specified index
    pub fn pop(&mut self, index: usize) -> Result<Value, &str> {
        if index >= self.len() {
            return Err("Index out of bounds");
        }
        match self {
            Column::Int(values) => Ok(Value::Int(values.remove(index))),
            Column::Float(values) => Ok(Value::Float(values.remove(index))),
            Column::Bool(values) => Ok(Value::Bool(values.remove(index))),
            Column::Str(values) => Ok(Value::Str(values.remove(index))),
        }
    }

    /// Returns the value at the specified index
    pub fn get(&self, index: usize) -> Result<Value, &str> {
        if index >= self.len() {
            return Err("Index out of bounds");
        }
        match self {
            Column::Int(values) => Ok(Value::Int(values[index])),
            Column::Float(values) => Ok(Value::Float(values[index])),
            Column::Bool(values) => Ok(Value::Bool(values[index])),
            Column::Str(values) => Ok(Value::Str(values[index].clone())),
        }
    }

    /// Sets the value at the specified index
    pub fn set(&mut self, index: usize, value: &Value) -> Result<(), &str> {
        if index >= self.len() {
            return Err("Index out of bounds");
        }
        match self {
            Column::Int(values) => {
                if let Value::Int(v) = value {
                    values[index] = *v;
                } else {
                    return Err("Value type mismatch");
                }
            }
            Column::Float(values) => {
                if let Value::Float(v) = value {
                    values[index] = *v;
                } else {
                    return Err("Value type mismatch");
                }
            }
            Column::Bool(values) => {
                if let Value::Bool(v) = value {
                    values[index] = *v;
                } else {
                    return Err("Value type mismatch");
                }
            }
            Column::Str(values) => {
                if let Value::Str(v) = value {
                    values[index] = v.clone();
                } else {
                    return Err("Value type mismatch");
                }
            }
        }
        Ok(())
    }

    /// Returns the length of the Column
    pub fn len(&self) -> usize {
        match self {
            Column::Int(values) => values.len(),
            Column::Float(values) => values.len(),
            Column::Bool(values) => values.len(),
            Column::Str(values) => values.len(),
        }
    }

    /// Iterate over the values in the Column
    pub fn iter(&self) -> Box<dyn Iterator<Item = Value> + '_> {
        match self {
            Column::Int(values) => {
                Box::new(values.iter().copied().map(Value::Int))
            }
            Column::Float(values) => {
                Box::new(values.iter().copied().map(Value::Float))
            }
            Column::Bool(values) => {
                Box::new(values.iter().copied().map(Value::Bool))
            }
            Column::Str(values) => {
                Box::new(values.iter().cloned().map(Value::Str))
            }
        }
    }

    /// Formats a string representation of the Column
    pub fn format(&self) -> String {
        match self {
            Column::Int(values) => format!("{:?}", values),
            Column::Float(values) => format!("{:?}", values),
            Column::Bool(values) => format!("{:?}", values),
            Column::Str(values) => format!("{:?}", values),
        }
    }

    /// Returns the type of the column as a string
    pub fn get_type(&self) -> &'static str {
        match self {
            Column::Int(_) => "int",
            Column::Float(_) => "float",
            Column::Bool(_) => "bool",
            Column::Str(_) => "str",
        }
    }
}

impl DataTable {
    /// Creates a new DataTable with the given column names and types
    pub fn new(
        column_names: &[&str],
        column_types: &[&str],
    ) -> Result<Self, &'static str> {
        if column_names.len() != column_types.len() {
            return Err("Column names and types must have the same length.");
        }
        let mut columns: Vec<(String, Column)> = Vec::new();
        let mut name_map: HashMap<String, usize> = HashMap::new();
        for (name, column_type) in column_names.iter().zip(column_types.iter())
        {
            let column = Column::new(column_type)?;
            columns.push((name.to_string(), column));
            name_map.insert(name.to_string(), columns.len() - 1);
        }
        Ok(DataTable { columns, name_map })
    }

    /// Creates a new DataTable from arrays of values
    pub fn from_arrays(
        column_names: &[&str],
        column_types: &[&str],
        data: &[&[Value]],
    ) -> Result<Self, &'static str> {
        let mut dt = DataTable::new(column_names, column_types)?;
        for row in data {
            dt.append(row.to_vec())
                .map_err(|_| "Failed to append row")?;
        }
        Ok(dt)
    }

    /// Appends a row to the DataTable
    pub fn append(&mut self, row: Vec<Value>) -> Result<(), &str> {
        if row.len() != self.columns.len() {
            return Err(
                "Row must have the same number of columns as the DataTable.",
            );
        }
        for (column, value) in self.columns.iter_mut().zip(row.iter()) {
            column.1.append(value)?;
        }
        Ok(())
    }

    /// Removes and returns a row from the DataTable at the specified index
    pub fn pop(&mut self, index: usize) -> Result<Vec<Value>, &str> {
        let mut row: Vec<Value> = Vec::new();
        for (_, column) in self.columns.iter_mut() {
            row.push(column.pop(index)?);
        }
        Ok(row)
    }

    /// Returns a reference to the column with the specified name, if it exists
    pub fn get_col(&self, name: &str) -> Result<&Column, &str> {
        self.name_map
            .get(name)
            .map(|i| &self.columns[*i].1)
            .ok_or("Column not found")
    }

    /// Returns a mutable reference to the column with the specified name, if it
    /// exists
    fn get_col_mut(&mut self, name: &str) -> Result<&mut Column, &str> {
        self.name_map
            .get(name)
            .map(|i| &mut self.columns[*i].1)
            .ok_or("Column not found")
    }

    /// Returns a reference to the value at the specified column and row indices
    /// if they exist
    pub fn get(&self, col_name: &str, row_index: usize) -> Result<Value, &str> {
        if let Ok(column) = self.get_col(col_name) {
            column.get(row_index)
        } else {
            Err("Column not found")
        }
    }

    /// Returns a row from the DataTable as a vector of values
    pub fn get_row(&self, row_index: usize) -> Result<Vec<Value>, &str> {
        let mut row: Vec<Value> = Vec::new();
        for (_, column) in self.columns.iter() {
            match column.get(row_index) {
                Ok(value) => row.push(value),
                Err(e) => return Err(e),
            }
        }
        Ok(row)
    }

    /// Iterate over the rows in the DataTable
    pub fn iter_rows(&self) -> impl Iterator<Item = Vec<Value>> + '_ {
        (0..self.len()).map(move |i| self.get_row(i).unwrap())
    }

    /// Sets the value at the specified column and row indices, if they exist
    pub fn set(
        &mut self,
        col_name: &str,
        row_index: usize,
        value: &Value,
    ) -> Result<(), &str> {
        if let Ok(col) = self.get_col_mut(col_name) {
            col.set(row_index, value)
        } else {
            Err("Column not found")
        }
    }

    /// Returns the number of rows in the DataTable
    pub fn len(&self) -> usize {
        if self.columns.is_empty() {
            return 0;
        }
        self.columns[0].1.len()
    }

    /// Returns the column names in the DataTable
    pub fn get_colnames(&self) -> Vec<String> {
        self.columns.iter().map(|(name, _)| name.clone()).collect()
    }

    /// Returns the column types in the DataTable
    pub fn get_coltypes(&self) -> Vec<String> {
        self.columns
            .iter()
            .map(|(_, col)| col.get_type().to_string())
            .collect()
    }

    /// Formats the DataTable as a string for display
    pub fn format(&self) -> String {
        let mut result = String::new();
        // Header
        for (i, (name, _)) in self.columns.iter().enumerate() {
            result.push_str(name);
            if i < self.columns.len() - 1 {
                result.push_str("\t");
            }
        }
        result.push('\n');

        // Rows
        for i in 0..self.len() {
            if let Ok(row) = self.get_row(i) {
                for (j, value) in row.iter().enumerate() {
                    let s = match value {
                        Value::Int(v) => v.to_string(),
                        Value::Float(v) => v.to_string(),
                        Value::Bool(v) => v.to_string(),
                        Value::Str(v) => v.clone(),
                    };
                    result.push_str(&s);
                    if j < row.len() - 1 {
                        result.push_str("\t");
                    }
                }
                result.push('\n');
            }
        }
        result
    }
}
