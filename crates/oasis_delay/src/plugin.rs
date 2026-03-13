use nih_plug::prelude::*;
use oasis_core::prelude::*;
use std::sync::Arc;

use crate::dsp::DelayProcessor;
use crate::params::OasisDelayParams;
use crate::ui;

pub struct OasisDelay {
    params: Arc<OasisDelayParams>,
    processor: DelayProcessor,
    sample_rate_ctx: SampleRateContext,
}

impl Default for OasisDelay {
    fn default() -> Self {
        Self {
            params: Arc::new(OasisDelayParams::default()),
            processor: DelayProcessor::new(),
            sample_rate_ctx: SampleRateContext::new(DEFAULT_SAMPLE_RATE),
        }
    }
}

impl Plugin for OasisDelay {
    const NAME: &'static str = "Oasis Delay";
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
            let delay_l_ms = self.params.delay_left_ms.smoothed.next();
            let delay_r_ms = self.params.delay_right_ms.smoothed.next();
            let feedback = self.params.feedback.smoothed.next();
            let ping_pong = self.params.ping_pong.value();
            let lp_freq = self.params.lp_freq.smoothed.next();
            let hp_freq = self.params.hp_freq.smoothed.next();
            let saturation = self.params.saturation.smoothed.next();
            let mod_rate = self.params.mod_rate.smoothed.next();
            let mod_depth = self.params.mod_depth.smoothed.next();
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
                delay_l_ms,
                delay_r_ms,
                feedback,
                ping_pong,
                lp_freq,
                hp_freq,
                saturation,
                mod_rate,
                mod_depth,
                mix,
                output_gain_db,
            );

            *channel_samples.get_mut(0).unwrap() = left_out;
            *channel_samples.get_mut(1).unwrap() = right_out;
        }

        ProcessStatus::Normal
    }
}

impl ClapPlugin for OasisDelay {
    const CLAP_ID: &'static str = "com.oasis-suite.oasis-delay";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("Versatile stereo delay");
    const CLAP_MANUAL_URL: Option<&'static str> = None;
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[
        ClapFeature::AudioEffect,
        ClapFeature::Delay,
    ];
}

impl Vst3Plugin for OasisDelay {
    const VST3_CLASS_ID: [u8; 16] = *b"OasisDelayv1.0\0\0";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[
        Vst3SubCategory::Fx,
        Vst3SubCategory::Delay,
    ];
}
