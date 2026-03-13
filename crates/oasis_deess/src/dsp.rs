use oasis_core::constants::*;
use oasis_core::dsp::dynamics::EnvelopeDetector;
use oasis_core::dsp::filter::Biquad;
use oasis_core::util::denormal::sanitize;
use oasis_core::util::math;
use oasis_core::util::sample_rate::SampleRateContext;

use crate::params::DeEssMode;

pub struct DeEssProcessor {
    sample_rate_ctx: SampleRateContext,

    sidechain_bp: Biquad,
    split_bp: Biquad,

    envelope: EnvelopeDetector,

    current_freq: f32,
    current_bw: f32,
}

impl DeEssProcessor {
    pub fn new() -> Self {
        let ctx = SampleRateContext::new(DEFAULT_SAMPLE_RATE);
        let mut processor = Self {
            sample_rate_ctx: ctx,
            sidechain_bp: Biquad::new(),
            split_bp: Biquad::new(),
            envelope: EnvelopeDetector::new(),
            current_freq: DEESS_FREQ_DEFAULT_HZ,
            current_bw: DEESS_Q_DEFAULT,
        };
        processor.update_filters(DEESS_FREQ_DEFAULT_HZ, DEESS_Q_DEFAULT);
        processor.envelope.set_times(1.0, 50.0, DEFAULT_SAMPLE_RATE);
        processor
    }

    pub fn set_sample_rate(&mut self, ctx: &SampleRateContext) {
        self.sample_rate_ctx = *ctx;
        self.update_filters(self.current_freq, self.current_bw);
        self.envelope.set_times(1.0, 50.0, ctx.sample_rate);
    }

    pub fn reset(&mut self) {
        self.sidechain_bp.reset();
        self.split_bp.reset();
        self.envelope.reset();
    }

    fn update_filters(&mut self, freq: f32, bw: f32) {
        let sr = self.sample_rate_ctx.sample_rate;
        self.sidechain_bp.set_bandpass(freq, bw, sr);
        self.split_bp.set_bandpass(freq, bw, sr);
        self.current_freq = freq;
        self.current_bw = bw;
    }

    #[inline]
    pub fn process_sample(
        &mut self,
        left_in: f32,
        right_in: f32,
        freq: f32,
        bw: f32,
        threshold_db: f32,
        range_db: f32,
        mode: DeEssMode,
        listen: bool,
        mix: f32,
        output_gain_db: f32,
    ) -> (f32, f32) {
        if (freq - self.current_freq).abs() > 0.5
            || (bw - self.current_bw).abs() > 0.01
        {
            self.update_filters(freq, bw);
        }

        let mono = (left_in + right_in) * 0.5;
        let sidechain = self.sidechain_bp.process(mono);
        let level = self.envelope.process_peak(sidechain);

        if listen {
            let out_gain = math::db_to_gain(output_gain_db);
            let sc = sanitize(sidechain * out_gain);
            return (sc, sc);
        }

        let level_db = math::gain_to_db(level);
        let gain_reduction_db = if level_db > threshold_db {
            (level_db - threshold_db).min(range_db)
        } else {
            0.0
        };
        let reduction_gain = math::db_to_gain(-gain_reduction_db);

        let (mut left_out, mut right_out) = match mode {
            DeEssMode::Split => {
                let left_band = self.split_bp.process(left_in);
                let right_band = left_band * (right_in / (left_in + f32::EPSILON));

                let left_reduced = left_in - left_band + left_band * reduction_gain;
                let right_reduced = right_in - right_band + right_band * reduction_gain;
                (left_reduced, right_reduced)
            }
            DeEssMode::Wideband => {
                (left_in * reduction_gain, right_in * reduction_gain)
            }
        };

        left_out = math::lerp(left_in, left_out, mix);
        right_out = math::lerp(right_in, right_out, mix);

        let out_gain = math::db_to_gain(output_gain_db);
        left_out *= out_gain;
        right_out *= out_gain;

        (sanitize(left_out), sanitize(right_out))
    }
}
