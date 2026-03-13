use nih_plug::prelude::*;
use oasis_core::prelude::*;
use std::sync::Arc;

use crate::dsp::SynthEngine;
use crate::params::OasisSynthParams;
use crate::ui;

pub struct OasisSynth {
    params: Arc<OasisSynthParams>,
    engine: SynthEngine,
}

impl Default for OasisSynth {
    fn default() -> Self {
        Self {
            params: Arc::new(OasisSynthParams::default()),
            engine: SynthEngine::new(),
        }
    }
}

impl Plugin for OasisSynth {
    const NAME: &'static str = "Oasis Synth";
    const VENDOR: &'static str = "Oasis Suite";
    const URL: &'static str = "https://oasis-suite.com";
    const EMAIL: &'static str = "";
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
        main_input_channels: None,
        main_output_channels: NonZeroU32::new(2),
        ..AudioIOLayout::const_default()
    }];

    const MIDI_INPUT: MidiConfig = MidiConfig::Basic;

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        ui::create_editor(self.params.clone())
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        self.engine.set_sample_rate(buffer_config.sample_rate);
        true
    }

    fn reset(&mut self) {
        self.engine.reset();
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        let _guard = DenormalGuard::new();

        self.engine.set_envelope(
            self.params.env_attack.smoothed.next(),
            self.params.env_decay.smoothed.next(),
            self.params.env_sustain.smoothed.next(),
            self.params.env_release.smoothed.next(),
        );

        let mut next_event = context.next_event();

        for (sample_idx, mut channel_samples) in buffer.iter_samples().enumerate() {
            while let Some(event) = next_event {
                if event.timing() > sample_idx as u32 {
                    break;
                }
                match event {
                    NoteEvent::NoteOn { note, velocity, .. } => {
                        self.engine.note_on(note, velocity);
                    }
                    NoteEvent::NoteOff { note, .. } => {
                        self.engine.note_off(note);
                    }
                    _ => {}
                }
                next_event = context.next_event();
            }

            let (l, r) = self.engine.process_sample(
                self.params.osc1_waveform.value(),
                self.params.osc1_level.smoothed.next(),
                self.params.osc1_detune.smoothed.next(),
                self.params.osc2_waveform.value(),
                self.params.osc2_level.smoothed.next(),
                self.params.osc2_detune.smoothed.next(),
                self.params.filter_cutoff.smoothed.next(),
                self.params.filter_resonance.smoothed.next(),
                self.params.master_gain.smoothed.next(),
            );

            if channel_samples.len() >= 2 {
                *channel_samples.get_mut(0).unwrap() = l;
                *channel_samples.get_mut(1).unwrap() = r;
            }
        }

        ProcessStatus::KeepAlive
    }
}

impl ClapPlugin for OasisSynth {
    const CLAP_ID: &'static str = "com.oasis-suite.oasis-synth";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("Monophonic synthesizer");
    const CLAP_MANUAL_URL: Option<&'static str> = None;
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[
        ClapFeature::Instrument,
        ClapFeature::Synthesizer,
    ];
}

impl Vst3Plugin for OasisSynth {
    const VST3_CLASS_ID: [u8; 16] = *b"OasisSynth_v10\0\0";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[
        Vst3SubCategory::Instrument,
        Vst3SubCategory::Synth,
    ];
}
