use nih_plug::prelude::*;
use oasis_core::prelude::*;
use std::sync::Arc;

use crate::dsp::VerbProcessor;
use crate::params::OasisVerbParams;
use crate::ui;

pub struct OasisVerb {
    params: Arc<OasisVerbParams>,
    processor: VerbProcessor,
    sample_rate_ctx: SampleRateContext,
}

impl Default for OasisVerb {
    fn default() -> Self {
        Self {
            params: Arc::new(OasisVerbParams::default()),
            processor: VerbProcessor::new(),
            sample_rate_ctx: SampleRateContext::new(DEFAULT_SAMPLE_RATE),
        }
    }
}

impl Plugin for OasisVerb {
    const NAME: &'static str = "Oasis Verb";
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
        self.processor.set_sample_rate(&self.sample_rate_ctx);
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
            let size = self.params.size.smoothed.next();
            let predelay_ms = self.params.predelay_ms.smoothed.next();
            let damping = self.params.damping.smoothed.next();
            let width = self.params.width.smoothed.next();
            let diffusion = self.params.diffusion.smoothed.next();
            let low_cut = self.params.low_cut_hz.smoothed.next();
            let high_cut = self.params.high_cut_hz.smoothed.next();
            let modulation = self.params.modulation.smoothed.next();
            let algorithm = self.params.algorithm.value();
            let mix = self.params.mix.smoothed.next();
            let output_gain_db = self.params.output_gain.smoothed.next();

            let num_channels = channel_samples.len();
            if num_channels < 2 {
                continue;
            }

            let left_in = *channel_samples.get_mut(0).unwrap();
            let right_in = *channel_samples.get_mut(1).unwrap();

            let (left_out, right_out) = self.processor.process_sample(
                left_in,
                right_in,
                size,
                predelay_ms,
                damping,
                width,
                diffusion,
                low_cut,
                high_cut,
                modulation,
                algorithm,
                mix,
                output_gain_db,
            );

            *channel_samples.get_mut(0).unwrap() = left_out;
            *channel_samples.get_mut(1).unwrap() = right_out;
        }

        ProcessStatus::Normal
    }
}

impl ClapPlugin for OasisVerb {
    const CLAP_ID: &'static str = "com.oasis-suite.oasis-verb";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("Algorithmic reverb");
    const CLAP_MANUAL_URL: Option<&'static str> = None;
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[
        ClapFeature::AudioEffect,
        ClapFeature::Reverb,
    ];
}

impl Vst3Plugin for OasisVerb {
    const VST3_CLASS_ID: [u8; 16] = *b"OasisVerb_v1.0\0\0";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[
        Vst3SubCategory::Fx,
        Vst3SubCategory::Reverb,
    ];
}
