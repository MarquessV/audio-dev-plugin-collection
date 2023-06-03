use std::sync::OnceLock;

use nih_plug::nih_dbg;

use crate::oscillator::wavetable::{Table, TABLE_SIZE};

static SQUARE_TABLE: OnceLock<Table> = OnceLock::new();

/// Get a reference to a square wavetable. This initializes the wavetable
/// on first use.
pub fn get_square_table() -> &'static Table {
    SQUARE_TABLE.get_or_init(|| {
        nih_dbg!("initializing SQUARE_TABLE");
        let mut samples: [f32; TABLE_SIZE] = [0.0; TABLE_SIZE];
        (TABLE_SIZE / 2..TABLE_SIZE).for_each(|i| {
            samples[i] = 1.0;
        });
        nih_dbg!("done initializing SQUARE_TABLE");
        Table { table: samples }
    })
}

#[cfg(test)]
mod test {
    use approx::assert_relative_eq;

    use crate::oscillator::wavetable::TABLE_SIZE_MASK;

    use super::{get_square_table, TABLE_SIZE};

    #[test]
    fn test_square_table() {
        let table = get_square_table();

        let half = TABLE_SIZE / 2;

        assert_relative_eq!(table[0], 0.0);
        assert_relative_eq!(table[half - 1], 0.0);
        assert_relative_eq!(table[half], 1.0);
        assert_relative_eq!(table[TABLE_SIZE_MASK], 1.0);
    }
}
