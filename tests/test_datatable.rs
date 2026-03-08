use ml361::datatable::{Column, DataTable, Value};

#[test]
fn test_column_new() {
    assert!(Column::new("int").is_ok());
    assert!(Column::new("float").is_ok());
    assert!(Column::new("bool").is_ok());
    assert!(Column::new("str").is_ok());
    assert!(Column::new("invalid").is_err());
}

#[test]
fn test_column_from_array() {
    let col = Column::from_array("int", &[Value::Int(10), Value::Int(20)]);
    assert!(col.is_ok());
    let col = col.unwrap();
    assert_eq!(col.len(), 2);
    assert_eq!(col.get(0).unwrap(), Value::Int(10));
    assert_eq!(col.get(1).unwrap(), Value::Int(20));
    assert!(col.get(2).is_err());
}

#[test]
fn test_column_append_and_get() {
    let mut col = Column::new("int").unwrap();
    col.append(&Value::Int(10)).unwrap();
    col.append(&Value::Int(20)).unwrap();
    assert_eq!(col.len(), 2);
    assert_eq!(col.get(0).unwrap(), Value::Int(10));
    assert_eq!(col.get(1).unwrap(), Value::Int(20));
    assert!(col.get(2).is_err());
}

#[test]
fn test_column_type_mismatch() {
    let mut col = Column::new("int").unwrap();
    assert!(col.append(&Value::Float(1.0)).is_err());
    assert!(col.append(&Value::Bool(true)).is_err());
    assert!(col.append(&Value::Str("hi".to_string())).is_err());

    let mut col_bool = Column::new("bool").unwrap();
    assert!(col_bool.append(&Value::Int(1)).is_err());

    let mut col_float = Column::new("float").unwrap();
    assert!(col_float.append(&Value::Int(1)).is_err());

    let mut col_str = Column::new("str").unwrap();
    assert!(col_str.append(&Value::Int(1)).is_err());
}

#[test]
fn test_column_pop() {
    let mut col = Column::new("str").unwrap();
    col.append(&Value::Str("first".to_string())).unwrap();
    col.append(&Value::Str("second".to_string())).unwrap();

    assert_eq!(col.pop(0).unwrap(), Value::Str("first".to_string()));
    assert_eq!(col.len(), 1);
    assert_eq!(col.pop(0).unwrap(), Value::Str("second".to_string()));
    assert_eq!(col.len(), 0);
    assert!(col.pop(0).is_err());
}

#[test]
fn test_column_set() {
    let mut col = Column::new("bool").unwrap();
    col.append(&Value::Bool(true)).unwrap();
    col.set(0, &Value::Bool(false)).unwrap();
    assert_eq!(col.get(0).unwrap(), Value::Bool(false));
    assert!(col.set(0, &Value::Int(1)).is_err());
    assert!(col.set(1, &Value::Bool(true)).is_err());

    let mut col_int = Column::new("int").unwrap();
    col_int.append(&Value::Int(1)).unwrap();
    assert!(col_int.set(0, &Value::Float(1.0)).is_err());

    let mut col_float = Column::new("float").unwrap();
    col_float.append(&Value::Float(1.0)).unwrap();
    assert!(col_float.set(0, &Value::Int(1)).is_err());

    let mut col_str = Column::new("str").unwrap();
    col_str.append(&Value::Str("a".to_string())).unwrap();
    assert!(col_str.set(0, &Value::Int(1)).is_err());
}

#[test]
fn test_column_iter() {
    let mut col = Column::new("int").unwrap();
    col.append(&Value::Int(1)).unwrap();
    col.append(&Value::Int(2)).unwrap();
    col.append(&Value::Int(3)).unwrap();

    let values: Vec<Value> = col.iter().collect();
    assert_eq!(values, vec![Value::Int(1), Value::Int(2), Value::Int(3)]);

    let mut col_str = Column::new("str").unwrap();
    col_str.append(&Value::Str("a".to_string())).unwrap();
    let values_str: Vec<Value> = col_str.iter().collect();
    assert_eq!(values_str, vec![Value::Str("a".to_string())]);
}

#[test]
fn test_column_format() {
    let mut col = Column::new("int").unwrap();
    col.append(&Value::Int(1)).unwrap();
    col.append(&Value::Int(2)).unwrap();
    assert_eq!(col.format(), "[1, 2]");

    let mut col_str = Column::new("str").unwrap();
    col_str.append(&Value::Str("a".to_string())).unwrap();
    assert_eq!(col_str.format(), "[\"a\"]");
}

#[test]
fn test_datatable_new() {
    let names = vec!["id", "name", "active"];
    let types = vec!["int", "str", "bool"];
    let dt = DataTable::new(&names, &types).unwrap();
    assert_eq!(dt.len(), 0);

    let types_mismatch = vec!["int", "str"];
    assert!(DataTable::new(&names, &types_mismatch).is_err());
}

#[test]
fn test_datatable_from_arrays() {
    let names: Vec<&str> = vec!["id", "name"];
    let types: Vec<&str> = vec!["int", "str"];
    let row1 = [Value::Int(1), Value::Str("a".to_string())];
    let row2 = [Value::Int(2), Value::Str("b".to_string())];
    let data: Vec<&[Value]> = vec![&row1, &row2];
    let dt = DataTable::from_arrays(&names, &types, &data).unwrap();
    assert_eq!(dt.len(), 2);
    assert_eq!(dt.get("id", 0).unwrap(), Value::Int(1));
    assert_eq!(dt.get("name", 1).unwrap(), Value::Str("b".to_string()));
}

#[test]
fn test_datatable_append_and_get() {
    let names = vec!["id", "score"];
    let types = vec!["int", "float"];
    let mut dt = DataTable::new(&names, &types).unwrap();

    let row1 = vec![Value::Int(1), Value::Float(95.5)];
    dt.append(row1).unwrap();

    let row2 = vec![Value::Int(2), Value::Float(88.0)];
    dt.append(row2).unwrap();

    assert_eq!(dt.len(), 2);
    assert_eq!(dt.get("id", 0).unwrap(), Value::Int(1));
    assert_eq!(dt.get("score", 1).unwrap(), Value::Float(88.0));
    assert!(dt.get("invalid", 0).is_err());
    assert!(dt.get("id", 2).is_err());

    // Mismatched row length
    assert!(dt.append(vec![Value::Int(3)]).is_err());
}

#[test]
fn test_datatable_pop() {
    let names = vec!["id", "name"];
    let types = vec!["int", "str"];
    let mut dt = DataTable::new(&names, &types).unwrap();

    dt.append(vec![Value::Int(10), Value::Str("A".to_string())])
        .unwrap();
    dt.append(vec![Value::Int(20), Value::Str("B".to_string())])
        .unwrap();

    let row = dt.pop(0).unwrap();
    assert_eq!(row[0], Value::Int(10));
    assert_eq!(row[1], Value::Str("A".to_string()));
    assert_eq!(dt.len(), 1);

    assert!(dt.pop(5).is_err());
}

#[test]
fn test_datatable_set() {
    let names = vec!["name"];
    let types = vec!["str"];
    let mut dt = DataTable::new(&names, &types).unwrap();

    dt.append(vec![Value::Str("Alice".to_string())]).unwrap();
    dt.set("name", 0, &Value::Str("Bob".to_string())).unwrap();
    assert_eq!(dt.get("name", 0).unwrap(), Value::Str("Bob".to_string()));
    assert!(dt.set("invalid", 0, &Value::Str("X".to_string())).is_err());
}

#[test]
fn test_datatable_get_col() {
    let names = vec!["id"];
    let types = vec!["int"];
    let mut dt = DataTable::new(&names, &types).unwrap();
    dt.append(vec![Value::Int(1)]).unwrap();

    let col = dt.get_col("id").unwrap();
    assert_eq!(col.len(), 1);
    assert_eq!(col.get(0).unwrap(), Value::Int(1));
    assert!(dt.get_col("invalid").is_err());
}

#[test]
fn test_datatable_get_row() {
    let names = vec!["id", "name"];
    let types = vec!["int", "str"];
    let mut dt = DataTable::new(&names, &types).unwrap();
    dt.append(vec![Value::Int(1), Value::Str("Alice".to_string())])
        .unwrap();

    let row = dt.get_row(0).unwrap();
    assert_eq!(row, vec![Value::Int(1), Value::Str("Alice".to_string())]);
    assert!(dt.get_row(1).is_err());
}

#[test]
fn test_datatable_iter_rows() {
    let names = vec!["id"];
    let types = vec!["int"];
    let mut dt = DataTable::new(&names, &types).unwrap();
    dt.append(vec![Value::Int(1)]).unwrap();
    dt.append(vec![Value::Int(2)]).unwrap();

    let rows: Vec<Vec<Value>> = dt.iter_rows().collect();
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0], vec![Value::Int(1)]);
    assert_eq!(rows[1], vec![Value::Int(2)]);
}

#[test]
fn test_datatable_format() {
    let names = vec!["id", "active"];
    let types = vec!["int", "bool"];
    let mut dt = DataTable::new(&names, &types).unwrap();
    dt.append(vec![Value::Int(1), Value::Bool(true)]).unwrap();

    let formatted = dt.format();
    assert!(formatted.contains("id\tactive\n"));
    assert!(formatted.contains("1\ttrue\n"));
}
