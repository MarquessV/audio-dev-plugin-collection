use std::sync::OnceLock;

use nih_plug::nih_dbg;
use rand::Rng;

use crate::oscillator::wavetable::{Table, TABLE_SIZE};

static NOISE_TABLE: OnceLock<Table> = OnceLock::new();

/// Get a reference to a noise wavetable. This initializes the wavetable
/// on first use.
pub fn get_noise_table() -> &'static Table {
    NOISE_TABLE.get_or_init(|| {
        nih_dbg!("initializing NOISE_TABLE");
        let mut samples: [f32; TABLE_SIZE] = [0.0; TABLE_SIZE];
        let mut rng = rand::thread_rng();
        for sample in &mut samples {
            *sample = rng.gen_range(-1.0..=1.0);
        }
        nih_dbg!("done initializing NOISE_TABLE");
        Table { table: samples }
    })
}
