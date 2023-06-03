#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(
    // We're fast and loose with our castin around here
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss
)]

use nih_plug::prelude::*;
use std::sync::Arc;

use oscillator::{Wavetable, WavetableType};

pub mod oscillator;

struct ToneGenerator<'a> {
    params: Arc<ToneGeneratorParams>,

    oscillator: Wavetable<'a>,
}

#[derive(Params)]
struct ToneGeneratorParams {
    #[id = "gain"]
    pub gain: FloatParam,

    #[id = "frequency"]
    pub frequency: FloatParam,

    #[id = "waveform"]
    pub wavetable: EnumParam<WavetableType>,
}

impl<'a> Default for ToneGenerator<'a> {
    fn default() -> Self {
        Self {
            params: Arc::new(ToneGeneratorParams::default()),
            oscillator: Wavetable::default(),
        }
    }
}

impl Default for ToneGeneratorParams {
    fn default() -> Self {
        Self {
            gain: FloatParam::new(
                "Gain",
                util::db_to_gain(-15.0),
                FloatRange::Skewed {
                    min: util::db_to_gain(-30.0),
                    max: util::db_to_gain(30.0),
                    factor: FloatRange::gain_skew_factor(-30.0, 30.0),
                },
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0))
            .with_unit(" dB")
            .with_value_to_string(formatters::v2s_f32_gain_to_db(2))
            .with_string_to_value(formatters::s2v_f32_gain_to_db()),

            frequency: FloatParam::new(
                "Frequency",
                110.0,
                FloatRange::Skewed {
                    min: 1.0,
                    max: 22000.0,
                    factor: FloatRange::skew_factor(-1.5),
                },
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0))
            .with_value_to_string(formatters::v2s_f32_hz_then_khz_with_note_name(2, true))
            .with_string_to_value(formatters::s2v_f32_hz_then_khz()),

            wavetable: EnumParam::new("Waveform", WavetableType::Sine),
        }
    }
}

impl Plugin for ToneGenerator<'static> {
    const NAME: &'static str = "Tone Generator";
    const VENDOR: &'static str = "marquessv";
    const URL: &'static str = env!("CARGO_PKG_HOMEPAGE");
    const EMAIL: &'static str = "marquessavaldez@gmail.com";

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    // Stereo by default, but can be configured to be mono.
    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[
        AudioIOLayout {
            main_input_channels: None,
            main_output_channels: NonZeroU32::new(2),
            aux_input_ports: &[],
            aux_output_ports: &[],
            names: PortNames::const_default(),
        },
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(1),
            main_output_channels: NonZeroU32::new(1),
            ..AudioIOLayout::const_default()
        },
    ];

    const MIDI_INPUT: MidiConfig = MidiConfig::None;
    const MIDI_OUTPUT: MidiConfig = MidiConfig::None;

    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        nih_dbg!("initializing ToneGenerator");
        self.oscillator.set_sample_rate(buffer_config.sample_rate);
        self.oscillator.set_frequency(self.params.frequency.value());
        self.oscillator
            .set_wavetable(&self.params.wavetable.value());
        nih_dbg!("done initializing ToneGenerator");
        true
    }

    fn reset(&mut self) {
        // Reset buffers and envelopes here. This can be called from the audio thread and may not
        // allocate. You can remove this function if you do not need it.
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        self.oscillator
            .set_wavetable(&self.params.wavetable.value());
        for channel_samples in buffer.iter_samples() {
            let gain = self.params.gain.smoothed.next();
            let frequency = self.params.frequency.smoothed.next();
            self.oscillator.set_frequency(frequency);

            for sample in channel_samples {
                *sample = self.oscillator.get_next_sample() * gain;
            }
        }

        ProcessStatus::Normal
    }
}

impl ClapPlugin for ToneGenerator<'static> {
    const CLAP_ID: &'static str = "com.marquessv.dev-tone-generator";
    const CLAP_DESCRIPTION: Option<&'static str> =
        Some("A stereo tone generator with configurable waveforms and frequency");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;

    const CLAP_FEATURES: &'static [ClapFeature] = &[
        ClapFeature::Utility,
        ClapFeature::Stereo,
        ClapFeature::Instrument,
        ClapFeature::Synthesizer,
    ];
}

impl Vst3Plugin for ToneGenerator<'static> {
    // Must be exactly 16 characters
    const VST3_CLASS_ID: [u8; 16] = *b"MrqssDev_ToneGen";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Fx, Vst3SubCategory::Generator];
}

nih_export_clap!(ToneGenerator<'static>);
nih_export_vst3!(ToneGenerator);
