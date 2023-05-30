use nih_plug::nih_dbg;
use once_cell::sync::Lazy;

use super::{Table, TABLE_SIZE, TABLE_SIZE_MASK};

pub static TRIANGLE_TABLE: Lazy<Table> = Lazy::new(|| {
    nih_dbg!("initializing TRIANGLE_TABLE");
    // Initialize array to -1.0 so the starting and final value are already set correctly.
    let mut samples: [f32; TABLE_SIZE] = [-1.0; TABLE_SIZE];
    // The samples in the indices [0, TABLE_SIZE/2] should equally span the range [-1.0, 1.0], an
    // increase of 2.0.
    // The step between each sample is then 2.0 * 1 / (TABLE_SIZE / 2) or:
    let step = 4.0 / (TABLE_SIZE_MASK as f32);
    let mut i = 1;
    while i <= TABLE_SIZE / 2 {
        samples[i] = samples[i - 1] + step;
        // Mirror the values to the other half of the array.
        samples[TABLE_SIZE - i] = samples[i];
        i += 1;
    }
    nih_dbg!("done initializing TRIANGLE_TABLE");
    Table { table: samples }
});

#[cfg(test)]
mod test {
    use approx::assert_relative_eq;

    use crate::oscillator::wavetable::TABLE_SIZE_MASK;

    use super::{TABLE_SIZE, TRIANGLE_TABLE};

    #[test]
    #[allow(clippy::float_cmp)] // we want to preciesly check these values.
    fn test_triangle_table() {
        let half = TABLE_SIZE / 2;

        assert_relative_eq!(TRIANGLE_TABLE[0], -1.0, epsilon = 0.01);
        assert_relative_eq!(TRIANGLE_TABLE[half / 2], 0.0, epsilon = 0.01);
        assert_relative_eq!(TRIANGLE_TABLE[half], 1.0, epsilon = 0.01);
        assert_relative_eq!(TRIANGLE_TABLE[half + half / 2], 0.0, epsilon = 0.01);
        assert_relative_eq!(TRIANGLE_TABLE[TABLE_SIZE_MASK], -1.0, epsilon = 0.01);
    }
}
