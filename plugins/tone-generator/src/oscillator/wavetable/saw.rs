use std::sync::OnceLock;

use nih_plug::nih_dbg;

use crate::oscillator::wavetable::{Table, TABLE_SIZE};

static SAW_TABLE: OnceLock<Table> = OnceLock::new();

/// Get a reference to a saw wavetable. This initializes the wavetable
/// on first use.
pub fn get_saw_table() -> &'static Table {
    SAW_TABLE.get_or_init(|| {
        nih_dbg!("initializing SAW_TABLE");
        let mut samples: [f32; TABLE_SIZE] = [-1.0; TABLE_SIZE];
        let half = TABLE_SIZE / 2;
        let step = 4.0 / TABLE_SIZE as f32;
        (1..half).for_each(|i| {
            samples[i] = samples[i - 1] + step;
            samples[i + half] = samples[i];
        });
        nih_dbg!("done initializing SAW_TABLE");
        Table { table: samples }
    })
}

#[cfg(test)]
mod test {
    use approx::assert_relative_eq;

    use crate::oscillator::wavetable::TABLE_SIZE_MASK;

    use super::{get_saw_table, TABLE_SIZE};

    #[test]
    fn test_saw_table() {
        let table = get_saw_table();

        let half = TABLE_SIZE / 2;

        assert_relative_eq!(table[0], -1.0, epsilon = 0.01);
        assert_relative_eq!(table[half - 1], 1.0, epsilon = 0.01);
        assert_relative_eq!(table[half], -1.0, epsilon = 0.01);
        assert_relative_eq!(table[TABLE_SIZE_MASK], 1.0, epsilon = 0.01);
    }
}
