import pytest
from ml361 import Column, DataTable


def test_column_int():
    col = Column("int")
    col.append(100)
    col.append(200)
    col.append(300)
    assert len(col) == 3
    assert col[0] == 100
    assert col[1] == 200
    assert col[2] == 300

    col[1] = 250
    assert col[1] == 250

    val = col.pop(0)
    assert val == 100
    assert len(col) == 2
    assert col[0] == 250


def test_column_float():
    col = Column("float")
    col.append(1.1)
    col.append(2.2)
    assert len(col) == 2
    assert col[0] == 1.1
    assert col[1] == 2.2

    with pytest.raises(ValueError):
        col.append("not a float")


def test_column_str():
    col = Column("str")
    col.append("apple")
    col.append("banana")
    col.append("cherry")
    assert str(col) == '["apple", "banana", "cherry"]'
    assert col[1] == "banana"

    col[2] = "date"
    assert col[2] == "date"


def test_column_bool():
    col = Column("bool")
    col.append(False)
    col.append(True)
    col.append(False)
    assert col[0] is False
    assert col[1] is True
    assert col[2] is False


def test_column_from_list():
    col: Column = Column.from_list("int", [10, 20, 30])
    assert len(col) == 3
    assert col[0] == 10
    assert col[1] == 20
    assert col[2] == 30
    with pytest.raises(IndexError):
        col[3]


def test_column_iteration():
    col = Column("int")
    data = [10, 20, 30]
    for x in data:
        col.append(x)

    iterated = [x for x in col]
    assert iterated == data


def test_column_errors():
    col = Column("int")
    with pytest.raises(ValueError):
        Column("invalid_type")

    with pytest.raises(ValueError):
        col.append(1.5)

    with pytest.raises(IndexError):
        col[0]

    col.append(1)
    with pytest.raises(IndexError):
        col[1]


def test_datatable_basic():
    names = ["username", "age", "is_admin"]
    types = ["str", "int", "bool"]
    dt = DataTable(names, types)

    dt.append(["alice", 30, True])
    dt.append(["bob", 25, False])
    dt.append(["charlie", 35, True])

    assert dt["username", 0] == "alice"
    assert dt["age", 1] == 25
    assert dt["is_admin", 2] is True

    dt["username", 1] = "robert"
    assert dt["username", 1] == "robert"

    row = dt.get_row(2)
    assert row == ["charlie", 35, True]

    popped_row = dt.pop(0)
    assert popped_row == ["alice", 30, True]
    assert dt["username", 0] == "robert"


def test_datatable_float():
    names = ["product", "price"]
    types = ["str", "float"]
    dt = DataTable(names, types)

    dt.append(["laptop", 999.99])
    dt.append(["mouse", 25.50])

    assert dt["price", 0] == 999.99
    assert dt["product", 1] == "mouse"


def test_datatable_from_lists():
    names = ["id", "name"]
    types = ["int", "str"]
    data = [
        [1, "a"],
        [2, "b"],
    ]
    dt = DataTable.from_lists(names, types, data)
    assert len(dt) == 2
    assert dt["id", 0] == 1
    assert dt["name", 1] == "b"


def test_datatable_errors():
    names = ["id"]
    types = ["int"]
    dt = DataTable(names, types)

    # Schema mismatch at creation
    with pytest.raises(ValueError):
        DataTable(["a"], ["int", "float"])

    # Row length mismatch
    with pytest.raises(ValueError):
        dt.append([1, 2])

    # Type mismatch in row
    with pytest.raises(ValueError):
        dt.append(["not an int"])

    # Index out of bounds
    with pytest.raises(IndexError):
        dt["id", 0]

    dt.append([1])
    with pytest.raises(IndexError):
        dt["id", 1]

    # Invalid column name
    with pytest.raises(IndexError):
        dt["nonexistent", 0]
