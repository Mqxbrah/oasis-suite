use oasis_core::constants::*;
use oasis_core::dsp::delay::DelayLine;
use oasis_core::dsp::dynamics::EnvelopeDetector;
use oasis_core::util::denormal::sanitize;
use oasis_core::util::math;
use oasis_core::util::sample_rate::SampleRateContext;

use crate::params::LimiterMode;

pub struct LimitProcessor {
    sample_rate_ctx: SampleRateContext,

    lookahead_l: DelayLine,
    lookahead_r: DelayLine,

    envelope: EnvelopeDetector,

    lookahead_samples: f32,
}

impl LimitProcessor {
    pub fn new() -> Self {
        let max_lookahead = (LIMIT_LOOKAHEAD_MS * 0.001 * DEFAULT_SAMPLE_RATE) as usize + 2;

        Self {
            sample_rate_ctx: SampleRateContext::new(DEFAULT_SAMPLE_RATE),
            lookahead_l: DelayLine::new(max_lookahead),
            lookahead_r: DelayLine::new(max_lookahead),
            envelope: EnvelopeDetector::new(),
            lookahead_samples: LIMIT_LOOKAHEAD_MS * 0.001 * DEFAULT_SAMPLE_RATE,
        }
    }

    pub fn set_sample_rate(&mut self, ctx: &SampleRateContext) {
        self.sample_rate_ctx = *ctx;

        let max_lookahead = (LIMIT_LOOKAHEAD_MS * 0.001 * ctx.sample_rate) as usize + 2;
        self.lookahead_l.resize(max_lookahead);
        self.lookahead_r.resize(max_lookahead);

        self.lookahead_samples = LIMIT_LOOKAHEAD_MS * 0.001 * ctx.sample_rate;

        self.envelope.set_times(0.0, LIMIT_RELEASE_DEFAULT_MS, ctx.sample_rate);
    }

    pub fn reset(&mut self) {
        self.lookahead_l.reset();
        self.lookahead_r.reset();
        self.envelope.reset();
    }

    pub fn lookahead_samples(&self) -> u32 {
        self.lookahead_samples as u32
    }

    #[inline]
    pub fn process_sample(
        &mut self,
        left_in: f32,
        right_in: f32,
        ceiling_db: f32,
        input_gain_db: f32,
        release_ms: f32,
        mode: LimiterMode,
        mix: f32,
        output_gain_db: f32,
    ) -> (f32, f32) {
        let input_gain = math::db_to_gain(input_gain_db);
        let left_gained = left_in * input_gain;
        let right_gained = right_in * input_gain;

        self.lookahead_l.write(left_gained);
        self.lookahead_r.write(right_gained);

        let left_delayed = self.lookahead_l.read_linear(self.lookahead_samples);
        let right_delayed = self.lookahead_r.read_linear(self.lookahead_samples);

        let peak = left_gained.abs().max(right_gained.abs());

        let effective_release = match mode {
            LimiterMode::Transparent => release_ms,
            LimiterMode::Aggressive => release_ms * 0.5,
            LimiterMode::Mastering => release_ms * 2.0,
        };

        self.envelope.set_times(0.0, effective_release, self.sample_rate_ctx.sample_rate);
        let envelope_level = self.envelope.process_peak(peak);

        let ceiling_gain = math::db_to_gain(ceiling_db);
        let gain_reduction = if envelope_level > ceiling_gain {
            ceiling_gain / envelope_level
        } else {
            1.0
        };

        let left_limited = left_delayed * gain_reduction;
        let right_limited = right_delayed * gain_reduction;

        let left_mixed = math::lerp(left_delayed, left_limited, mix);
        let right_mixed = math::lerp(right_delayed, right_limited, mix);

        let out_gain = math::db_to_gain(output_gain_db);
        let left_out = left_mixed * out_gain;
        let right_out = right_mixed * out_gain;

        (sanitize(left_out), sanitize(right_out))
    }
}
