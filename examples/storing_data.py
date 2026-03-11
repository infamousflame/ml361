from typing import Any

from ml361.datatable import Column, DataTable

def main():
    column_names: list[str] = ["id", "name", "score", "advanced"]
    column_types: list[str] = ["int", "str", "float", "bool"]
    table_data: list[list[Any]] = [
        [0, "Ada", 1.2, True],
        [1, "Bob", 2.3, False],
        [2, "Charlie", 3.4, True],
    ]
    free_columns: list[Column] = [
        Column.from_list(type_, [table_data[row][column] for row in range(len(table_data))])
        for column, type_ in enumerate(column_types)
    ]
    print('Original columns:')
    for column in free_columns:
        print('\t'.join([str(x) for x in column]))
    print('Appending 3 to 1st column.')
    free_columns[0].append(3)
    for column in free_columns:
        print('\t'.join([str(x) for x in column]))
    print('Removing 2nd element from 2nd column.')
    free_columns[1].pop(1)
    for column in free_columns:
        print('\t'.join([str(x) for x in column]))
    print('Updating 1st score to 4.5.')
    free_columns[2][0] = 4.5
    for column in free_columns:
        print('\t'.join([str(x) for x in column]))
    data_table = DataTable.from_lists(column_names, column_types, table_data)
    print('Original data table:')
    print(data_table)
    print('Appending [3, "Daniel", 5.6, False].')
    data_table.append([3, "Daniel", 5.6, False])
    print(data_table)
    print('Removing 2nd row.')
    data_table.pop(1)
    print(data_table)
    print('Updating 1st score to 4.5.')
    data_table["score", 0] = 4.5
    print(data_table)

if __name__ == "__main__":
    main()
