use std::f32::consts::TAU;

use nih_plug::nih_dbg;
use once_cell::sync::Lazy;

use crate::oscillator::wavetable::{Table, TABLE_SIZE, TABLE_SIZE_MASK};

pub static SIN_TABLE: Lazy<Table> = Lazy::new(|| {
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
});

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use crate::oscillator::wavetable::TABLE_SIZE_MASK;

    use super::{SIN_TABLE, TABLE_SIZE};

    #[test]
    fn test_sin_table() {
        let half = TABLE_SIZE / 2;

        assert_relative_eq!(SIN_TABLE[0], 0.0, epsilon = 0.01);
        assert_relative_eq!(SIN_TABLE[half / 2], 1.0, epsilon = 0.01);
        assert_relative_eq!(SIN_TABLE[half], 0.0, epsilon = 0.01);
        assert_relative_eq!(SIN_TABLE[half + half / 2], -1.0, epsilon = 0.01);
        assert_relative_eq!(SIN_TABLE[TABLE_SIZE_MASK], 0.0, epsilon = 0.01);
    }
}
