import pytest
from ml361.dataman import train_test_split
from ml361.datatable import DataTable


def create_sample_datatable(rows: int) -> DataTable:
    names = ["id", "value"]
    types = ["int", "float"]
    dt = DataTable(names, types)
    for i in range(rows):
        dt.append([i, float(i)])
    return dt


def test_train_test_split_sizes():
    dt = create_sample_datatable(100)
    train, test = train_test_split(dt, 0.2, 42)

    # For a 0.2 split on 100 rows, we expect 20 in test and 80 in train
    assert len(test) == 20
    assert len(train) == 80
    assert len(train) + len(test) == 100

    # Verify column structure is preserved
    assert train.get_colnames() == dt.get_colnames()
    assert test.get_colnames() == dt.get_colnames()
    assert train.get_coltypes() == dt.get_coltypes()
    assert test.get_coltypes() == dt.get_coltypes()


def test_train_test_split_determinism():
    dt = create_sample_datatable(100)
    train1, test1 = train_test_split(dt, 0.2, 123)
    train2, test2 = train_test_split(dt, 0.2, 123)

    # If a seed is provided, the splits should be identical
    ids1_test = [test1["id", i] for i in range(len(test1))]
    ids2_test = [test2["id", i] for i in range(len(test2))]
    assert ids1_test == ids2_test

    ids1_train = [train1["id", i] for i in range(len(train1))]
    ids2_train = [train2["id", i] for i in range(len(train2))]
    assert ids1_train == ids2_train


def test_train_test_split_all_test():
    dt = create_sample_datatable(10)
    train, test = train_test_split(dt, 1.0, 42)
    assert len(train) == 0
    assert len(test) == 10


def test_train_test_split_no_test():
    dt = create_sample_datatable(10)
    train, test = train_test_split(dt, 0.0, 42)
    assert len(train) == 10
    assert len(test) == 0


def test_train_test_split_empty():
    dt = create_sample_datatable(0)
    train, test = train_test_split(dt, 0.2, 42)
    assert len(train) == 0
    assert len(test) == 0


def test_train_test_split_content_integrity():
    dt = create_sample_datatable(50)
    train, test = train_test_split(dt, 0.3, 7)

    # Combine all IDs from train and test and verify they match the original IDs
    all_ids = []

    for i in range(len(train)):
        all_ids.append(train["id", i])

    for i in range(len(test)):
        all_ids.append(test["id", i])

    all_ids.sort()
    original_ids = list(range(50))
    assert all_ids == original_ids
