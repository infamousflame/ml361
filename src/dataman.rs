/// Module for manipulating data.
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::SeedableRng;

use crate::datatable::DataTable;

pub fn train_test_split(
    data: &DataTable,
    test_size: f64,
    seed: Option<u64>,
) -> Result<(DataTable, DataTable), &'static str> {
    let total_rows = data.len();
    let test_count = (total_rows as f64 * test_size).round_ties_even() as usize;

    let mut indices: Vec<usize> = (0..total_rows).collect();

    if let Some(s) = seed {
        let mut rng = StdRng::seed_from_u64(s);
        indices.shuffle(&mut rng);
    } else {
        let mut rng = rand::rng();
        indices.shuffle(&mut rng);
    }

    let (test_indices, train_indices) = indices.split_at(test_count);

    let colnames = data.get_colnames();
    let colnames_str: Vec<&str> =
        colnames.iter().map(|s| s.as_str()).collect();
    let coltypes = data.get_coltypes();
    let coltypes_str: Vec<&str> =
        coltypes.iter().map(|s| s.as_str()).collect();

    let mut train_data = DataTable::new(&colnames_str, &coltypes_str)?;
    let mut test_data = DataTable::new(&colnames_str, &coltypes_str)?;

    for &i in train_indices {
        train_data.append(data.get_row(i)?)?;
    }
    for &i in test_indices {
        test_data.append(data.get_row(i)?)?;
    }

    Ok((train_data.into(), test_data.into()))
}
