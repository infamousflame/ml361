/// Decision Tree implementation
use crate::datatable::{Column, DataTable, Value};
use crate::model::SupervisedModel;

pub enum DecisionMetric {
    Gini,
    InformationGain,
}

struct DTreeNode {
    feature_name: String,
    threshold: Value,
    left: Option<Box<DTreeNode>>,
    right: Option<Box<DTreeNode>>,
}

struct DecisionTree {
    root: Option<Box<DTreeNode>>,
    decision_metric: DecisionMetric,
    max_depth: Option<usize>,
}

impl DTreeNode {
    pub fn new(feature_name: String, threshold: Value) -> Self {
        Self {
            feature_name,
            threshold,
            left: None,
            right: None,
        }
    }
}

impl DecisionTree {
    pub fn new(
        decision_metric: DecisionMetric,
        max_depth: Option<usize>,
    ) -> Self {
        Self {
            root: None,
            decision_metric,
            max_depth,
        }
    }
}
