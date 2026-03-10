use ml361::dataman::train_test_split;
use ml361::datatable::{DataTable, Value};

fn create_sample_datatable(rows: usize) -> DataTable {
    let names = vec!["id", "value"];
    let types = vec!["int", "float"];
    let mut dt = DataTable::new(&names, &types).unwrap();
    for i in 0..rows {
        dt.append(vec![Value::Int(i as isize), Value::Float(i as f64)])
            .unwrap();
    }
    dt
}

#[test]
fn test_train_test_split_sizes() {
    let dt = create_sample_datatable(100);
    let (train, test) = train_test_split(&dt, 0.2, Some(42));

    // For a 0.2 split on 100 rows, we expect 20 in test and 80 in train
    assert_eq!(test.len(), 20);
    assert_eq!(train.len(), 80);
    assert_eq!(train.len() + test.len(), 100);

    // Verify column structure is preserved
    assert_eq!(train.get_colnames(), dt.get_colnames());
    assert_eq!(test.get_colnames(), dt.get_colnames());
    assert_eq!(train.get_coltypes(), dt.get_coltypes());
    assert_eq!(test.get_coltypes(), dt.get_coltypes());
}

#[test]
fn test_train_test_split_determinism() {
    let dt = create_sample_datatable(100);
    let (train1, test1) = train_test_split(&dt, 0.2, Some(123));
    let (train2, test2) = train_test_split(&dt, 0.2, Some(123));

    // If a seed is provided, the splits should be identical
    let ids1_test: Vec<Value> = test1.get_col("id").unwrap().iter().collect();
    let ids2_test: Vec<Value> = test2.get_col("id").unwrap().iter().collect();
    assert_eq!(ids1_test, ids2_test);

    let ids1_train: Vec<Value> = train1.get_col("id").unwrap().iter().collect();
    let ids2_train: Vec<Value> = train2.get_col("id").unwrap().iter().collect();
    assert_eq!(ids1_train, ids2_train);
}

#[test]
fn test_train_test_split_all_test() {
    let dt = create_sample_datatable(10);
    let (train, test) = train_test_split(&dt, 1.0, Some(42));

    assert_eq!(train.len(), 0);
    assert_eq!(test.len(), 10);
}

#[test]
fn test_train_test_split_no_test() {
    let dt = create_sample_datatable(10);
    let (train, test) = train_test_split(&dt, 0.0, Some(42));

    assert_eq!(train.len(), 10);
    assert_eq!(test.len(), 0);
}

#[test]
fn test_train_test_split_empty() {
    let dt = create_sample_datatable(0);
    let (train, test) = train_test_split(&dt, 0.2, Some(42));

    assert_eq!(train.len(), 0);
    assert_eq!(test.len(), 0);
}

#[test]
fn test_train_test_split_content_integrity() {
    let dt = create_sample_datatable(50);
    let (train, test) = train_test_split(&dt, 0.3, Some(7));

    // Combine all IDs from train and test and verify they match the original IDs
    let mut all_ids: Vec<isize> = Vec::new();

    for val in train.get_col("id").unwrap().iter() {
        if let Value::Int(i) = val {
            all_ids.push(i);
        }
    }
    for val in test.get_col("id").unwrap().iter() {
        if let Value::Int(i) = val {
            all_ids.push(i);
        }
    }

    all_ids.sort();
    let original_ids: Vec<isize> = (0..50).collect();
    assert_eq!(all_ids, original_ids);
}
