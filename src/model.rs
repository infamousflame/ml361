use crate::datatable::{Column, DataTable, Value};

pub trait SupervisedModel {
    pub fn new() -> Self;
    pub fn train(&mut self, X: &DataTable, y: &Column);
    pub fn predict(&self, row: &Vec<Value>) -> Value;
    pub fn accuracy(&self, X: &DataTable, y: &Column) -> f64;
}
