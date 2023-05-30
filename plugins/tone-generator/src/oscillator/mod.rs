use nih_plug::prelude::*;

use wavetable::{
    initialize_tables, SIN_TABLE, SQUARE_TABLE, TABLE_SIZE, TABLE_SIZE_MASK, TRIANGLE_TABLE,
};

use self::wavetable::{Table, SAW_TABLE};

pub mod wavetable;

#[derive(Default, Debug, Enum, PartialEq, Eq)]
pub enum WavetableType {
    #[default]
    Sine,
    Triangle,
    Saw,
    Square,
}

/// A simple wavetable oscillator. The backing tables are single periods of the corresponding
/// waveforms, which is good enough for a recognizable sound, but subject to artifacts,
/// especially at higher frequencies.
#[derive(Debug)]
pub struct Wavetable<'a> {
    // The wavetable to use.
    table: &'a Table,
    // The frequency of the oscillator.
    frequency: f32,
    // The current sample rate of the oscillator.
    sample_rate: f32,
    // The amount to increment the index by for each sample, calculated as
    // the frequency times the size of the table and divided by the sample rate.
    delta: f32,
    // A continuous index allows us to more precisely interpolate the value between
    // two samples.
    continuous_index: f32,
}

impl<'a> Default for Wavetable<'a> {
    fn default() -> Self {
        initialize_tables();
        Self {
            table: &SIN_TABLE,
            frequency: 440.0,
            sample_rate: 44100.0,
            delta: 0.0,
            continuous_index: 0.0,
        }
    }
}

impl Wavetable<'_> {
    /// Set the wavetable to oscillate over.
    pub fn set_wavetable(&mut self, kind: &WavetableType) {
        self.table = match kind {
            WavetableType::Sine => &SIN_TABLE,
            WavetableType::Triangle => &TRIANGLE_TABLE,
            WavetableType::Saw => &SAW_TABLE,
            WavetableType::Square => &SQUARE_TABLE,
        }
    }

    /// Set the sample rate of the oscillator.
    pub fn set_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
        self.update_delta();
    }

    /// Set the frequency of the oscillator.
    pub fn set_frequency(&mut self, frequency: f32) {
        self.frequency = frequency;
        self.update_delta();
    }

    /// Recaculate the delta value based on the current frequency and sample rate.
    fn update_delta(&mut self) {
        self.delta = self.frequency * TABLE_SIZE as f32 / self.sample_rate;
    }

    pub fn get_next_sample(&mut self) -> f32 {
        let index = self.continuous_index as usize;

        let next_index = if index == TABLE_SIZE_MASK {
            0
        } else {
            index + 1
        };

        let diff = self.continuous_index - index as f32;

        let sample = self.table[index];
        let next_sample = self.table[next_index];

        self.continuous_index += self.delta;
        if self.continuous_index > TABLE_SIZE_MASK as f32 {
            self.continuous_index -= TABLE_SIZE as f32;
        }

        diff.mul_add(sample - next_sample, sample)
    }
}
