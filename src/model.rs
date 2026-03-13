/// Useful model-related traits.
use crate::datatable::{Column, DataTable, Value};

pub trait SupervisedModel {
    fn fit(&mut self, X: &DataTable, y: &Column) -> Result<(), &'static str>;
    fn predict(&self, row: &Vec<Value>) -> Result<Value, &'static str>;
    fn accuracy(&self, X: &DataTable, y: &Column) -> Result<f64, &'static str>;
}
