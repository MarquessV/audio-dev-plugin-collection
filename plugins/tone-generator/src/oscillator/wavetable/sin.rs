use std::f32::consts::TAU;
use std::sync::OnceLock;

use nih_plug::nih_dbg;

use crate::oscillator::wavetable::{Table, TABLE_SIZE, TABLE_SIZE_MASK};

static SIN_TABLE: OnceLock<Table> = OnceLock::new();

/// Get a reference to a sin wavetable. This initializes the wavetable
/// on first use.
pub fn get_sin_table() -> &'static Table {
    SIN_TABLE.get_or_init(|| {
        nih_dbg!("initializing SIN_TABLE");
        let mut samples: [f32; TABLE_SIZE] = [0.0; TABLE_SIZE];
        let step = TAU / (TABLE_SIZE_MASK) as f32;
        let mut angle: f32 = 0.0;
        (0..TABLE_SIZE).for_each(|i| {
            samples[i] = angle.sin();
            angle += step;
        });
        nih_dbg!("done initializing SIN_TABLE");
        Table { table: samples }
    })
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use crate::oscillator::wavetable::TABLE_SIZE_MASK;

    use super::{get_sin_table, TABLE_SIZE};

    #[test]
    fn test_sin_table() {
        let half = TABLE_SIZE / 2;

        let table = get_sin_table();

        assert_relative_eq!(table[0], 0.0, epsilon = 0.01);
        assert_relative_eq!(table[half / 2], 1.0, epsilon = 0.01);
        assert_relative_eq!(table[half], 0.0, epsilon = 0.01);
        assert_relative_eq!(table[half + half / 2], -1.0, epsilon = 0.01);
        assert_relative_eq!(table[TABLE_SIZE_MASK], 0.0, epsilon = 0.01);
    }
}
