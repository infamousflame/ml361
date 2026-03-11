/// Useful model-related traits.
use crate::datatable::{Column, DataTable, Value};

pub trait SupervisedModel {
    fn fit(&mut self, X: &DataTable, y: &Column) -> Result<(), &'static str>;
    fn predict(&self, row: &Vec<Value>) -> Result<Value, &'static str>;
    fn accuracy(&self, X: &DataTable, y: &Column) -> Result<f64, &'static str>;
    fn sensitivity(&self, X: &DataTable, y: &Column) -> Result<f64, &'static str>;
    fn specificity(&self, X: &DataTable, y: &Column) -> Result<f64, &'static str>;
    fn precision(&self, X: &DataTable, y: &Column) -> Result<f64, &'static str>;
    fn recall(&self, X: &DataTable, y: &Column) -> Result<f64, &'static str>;
}
