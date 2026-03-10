/// Useful model-related traits.

use crate::datatable::{Column, DataTable, Value};

pub trait SupervisedModel {
    fn new() -> Self;
    fn train(&mut self, X: &DataTable, y: &Column);
    fn predict(&self, row: &Vec<Value>) -> Value;
    fn accuracy(&self, X: &DataTable, y: &Column) -> f64;
}
