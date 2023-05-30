use nih_plug::nih_dbg;
use once_cell::sync::Lazy;

use crate::oscillator::wavetable::{Table, TABLE_SIZE};

pub static SAW_TABLE: Lazy<Table> = Lazy::new(|| {
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
});

#[cfg(test)]
mod test {
    use approx::assert_relative_eq;

    use crate::oscillator::wavetable::TABLE_SIZE_MASK;

    use super::{SAW_TABLE, TABLE_SIZE};

    #[test]
    fn test_saw_table() {
        let half = TABLE_SIZE / 2;

        assert_relative_eq!(SAW_TABLE[0], -1.0, epsilon = 0.01);
        assert_relative_eq!(SAW_TABLE[half - 1], 1.0, epsilon = 0.01);
        assert_relative_eq!(SAW_TABLE[half], -1.0, epsilon = 0.01);
        assert_relative_eq!(SAW_TABLE[TABLE_SIZE_MASK], 1.0, epsilon = 0.01);
    }
}
