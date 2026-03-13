use nih_plug::prelude::*;
use oasis_core::prelude::*;
use std::sync::Arc;

use crate::dsp::WideProcessor;
use crate::params::OasisWideParams;
use crate::ui;

pub struct OasisWide {
    params: Arc<OasisWideParams>,
    processor: WideProcessor,
    sample_rate_ctx: SampleRateContext,
}

impl Default for OasisWide {
    fn default() -> Self {
        Self {
            params: Arc::new(OasisWideParams::default()),
            processor: WideProcessor::new(),
            sample_rate_ctx: SampleRateContext::new(DEFAULT_SAMPLE_RATE),
        }
    }
}

impl Plugin for OasisWide {
    const NAME: &'static str = "Oasis Wide";
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
            let width = self.params.width.smoothed.next();
            let mid_gain_db = self.params.mid_gain.smoothed.next();
            let side_gain_db = self.params.side_gain.smoothed.next();
            let haas_delay_ms = self.params.haas_delay_ms.smoothed.next();
            let haas_channel = self.params.haas_channel.value();
            let bass_mono_enabled = self.params.bass_mono_enabled.value();
            let bass_mono_freq = self.params.bass_mono_freq.smoothed.next();
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
                width,
                mid_gain_db,
                side_gain_db,
                haas_delay_ms,
                haas_channel,
                bass_mono_enabled,
                bass_mono_freq,
                mix,
                output_gain_db,
            );

            *channel_samples.get_mut(0).unwrap() = left_out;
            *channel_samples.get_mut(1).unwrap() = right_out;
        }

        ProcessStatus::Normal
    }
}

impl ClapPlugin for OasisWide {
    const CLAP_ID: &'static str = "com.oasis-suite.oasis-wide";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("Stereo widening and image control");
    const CLAP_MANUAL_URL: Option<&'static str> = None;
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[
        ClapFeature::AudioEffect,
        ClapFeature::Stereo,
        ClapFeature::Utility,
    ];
}

impl Vst3Plugin for OasisWide {
    const VST3_CLASS_ID: [u8; 16] = *b"OasisWide__v1.0\0";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[
        Vst3SubCategory::Fx,
        Vst3SubCategory::Spatial,
        Vst3SubCategory::Stereo,
    ];
}
