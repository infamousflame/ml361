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
    let coltypes = data.get_coltypes();
    let colnames_str: Vec<&str> = colnames.iter().map(|s| s.as_str()).collect();
    let coltypes_str: Vec<&str> = coltypes.iter().map(|s| s.as_str()).collect();

    let mut train_data = match DataTable::new(&colnames_str, &coltypes_str) {
        Ok(d) => d,
        Err(e) => return Err(e),
    };
    let mut test_data = match DataTable::new(&colnames_str, &coltypes_str) {
        Ok(d) => d,
        Err(e) => return Err(e),
    };

    for &i in train_indices {
        let row = match data.get_row(i) {
            Ok(r) => r,
            Err(e) => return Err(e),
        };
        if let Err(e) = train_data.append(row) {
            return Err(e);
        }
    }
    for &i in test_indices {
        let row = match data.get_row(i) {
            Ok(r) => r,
            Err(e) => return Err(e),
        };
        if let Err(e) = test_data.append(row) {
            return Err(e);
        }
    }

    Ok((train_data.into(), test_data.into()))
}
