use nih_plug::prelude::*;
use oasis_core::prelude::*;
use std::sync::Arc;

use crate::dsp::EqProcessor;
use crate::params::OasisEqParams;
use crate::ui;

pub struct OasisEq {
    params: Arc<OasisEqParams>,
    processor: EqProcessor,
    sample_rate_ctx: SampleRateContext,
}

impl Default for OasisEq {
    fn default() -> Self {
        Self {
            params: Arc::new(OasisEqParams::default()),
            processor: EqProcessor::new(),
            sample_rate_ctx: SampleRateContext::new(DEFAULT_SAMPLE_RATE),
        }
    }
}

impl Plugin for OasisEq {
    const NAME: &'static str = "Oasis EQ";
    const VENDOR: &'static str = "Oasis Suite";
    const URL: &'static str = "https://oasis-suite.com";
    const EMAIL: &'static str = "";
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(2),
            main_output_channels: NonZeroU32::new(2),
            ..AudioIOLayout::const_default()
        },
    ];

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
        self.sample_rate_ctx = SampleRateContext::new(buffer_config.sample_rate);
        self.processor.set_sample_rate(self.sample_rate_ctx.sample_rate);
        true
    }

    fn reset(&mut self) {
        self.processor.reset();
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        let _guard = DenormalGuard::new();

        for mut channel_samples in buffer.iter_samples() {
            let num_channels = channel_samples.len();
            if num_channels < 2 {
                continue;
            }

            let mut band_enabled = [false; 8];
            for i in 0..8 {
                let freq = self.params.bands[i].freq.smoothed.next();
                let gain = self.params.bands[i].gain.smoothed.next();
                let q = self.params.bands[i].q.smoothed.next();
                let filter_type = self.params.bands[i].filter_type.value();
                let enabled = self.params.bands[i].enabled.value();
                band_enabled[i] = enabled;

                self.processor.update_band(i, freq, gain, q, filter_type, enabled);
            }

            let left_in = *channel_samples.get_mut(0).unwrap();
            let right_in = *channel_samples.get_mut(1).unwrap();

            let (mut left_out, mut right_out) =
                self.processor.process_sample(left_in, right_in, &band_enabled);

            let output_gain_db = self.params.output_gain.smoothed.next();
            let out_gain = math::db_to_gain(output_gain_db);
            left_out *= out_gain;
            right_out *= out_gain;

            *channel_samples.get_mut(0).unwrap() = left_out;
            *channel_samples.get_mut(1).unwrap() = right_out;
        }

        ProcessStatus::Normal
    }
}

impl ClapPlugin for OasisEq {
    const CLAP_ID: &'static str = "com.oasis-suite.oasis-eq";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("8-band parametric equalizer");
    const CLAP_MANUAL_URL: Option<&'static str> = None;
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[
        ClapFeature::AudioEffect,
        ClapFeature::Equalizer,
        ClapFeature::Stereo,
    ];
}

impl Vst3Plugin for OasisEq {
    const VST3_CLASS_ID: [u8; 16] = *b"OasisEQ___v1.0\0\0";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[
        Vst3SubCategory::Fx,
        Vst3SubCategory::Eq,
    ];
}
