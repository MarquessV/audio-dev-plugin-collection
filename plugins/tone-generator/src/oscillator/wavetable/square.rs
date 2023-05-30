use nih_plug::nih_dbg;
use once_cell::sync::Lazy;

use crate::oscillator::wavetable::{Table, TABLE_SIZE};

pub static SQUARE_TABLE: Lazy<Table> = Lazy::new(|| {
    nih_dbg!("initializing SQUARE_TABLE");
    let mut samples: [f32; TABLE_SIZE] = [0.0; TABLE_SIZE];
    (TABLE_SIZE / 2..TABLE_SIZE).for_each(|i| {
        samples[i] = 1.0;
    });
    nih_dbg!("done initializing SQUARE_TABLE");
    Table { table: samples }
});

#[cfg(test)]
mod test {
    use approx::assert_relative_eq;

    use crate::oscillator::wavetable::TABLE_SIZE_MASK;

    use super::{SQUARE_TABLE, TABLE_SIZE};

    #[test]
    fn test_square_table() {
        let half = TABLE_SIZE / 2;

        assert_relative_eq!(SQUARE_TABLE[0], 0.0);
        assert_relative_eq!(SQUARE_TABLE[half - 1], 0.0);
        assert_relative_eq!(SQUARE_TABLE[half], 1.0);
        assert_relative_eq!(SQUARE_TABLE[TABLE_SIZE_MASK], 1.0);
    }
}
