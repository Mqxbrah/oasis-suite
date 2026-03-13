use oasis_core::constants::*;
use oasis_core::util::denormal::sanitize;
use oasis_core::util::math;
use oasis_core::util::sample_rate::SampleRateContext;

use crate::params::PumpShape;

pub struct PumpProcessor {
    sample_rate: f32,
    phase: f32,
    smoothed_lfo: f32,
}

impl PumpProcessor {
    pub fn new() -> Self {
        Self {
            sample_rate: DEFAULT_SAMPLE_RATE,
            phase: 0.0,
            smoothed_lfo: 1.0,
        }
    }

    pub fn set_sample_rate(&mut self, ctx: &SampleRateContext) {
        self.sample_rate = ctx.sample_rate;
    }

    pub fn reset(&mut self) {
        self.phase = 0.0;
        self.smoothed_lfo = 1.0;
    }

    #[inline]
    pub fn process_sample(
        &mut self,
        left: f32,
        right: f32,
        depth: f32,
        rate_hz: f32,
        smoothing_amt: f32,
        shape: PumpShape,
        mix: f32,
        output_gain_db: f32,
    ) -> (f32, f32) {
        self.phase += rate_hz / self.sample_rate;
        if self.phase >= 1.0 {
            self.phase -= 1.0;
        }

        let raw_lfo = match shape {
            PumpShape::Sine => {
                0.5 + 0.5 * (self.phase * std::f32::consts::TAU).cos()
            }
            PumpShape::Saw => {
                1.0 - self.phase
            }
            PumpShape::Square => {
                if self.phase < 0.5 { 1.0 } else { 0.0 }
            }
            PumpShape::Exponential => {
                (-self.phase * 4.0).exp()
            }
        };

        let coeff = (-std::f32::consts::TAU * (1.0 + smoothing_amt * 20.0) / self.sample_rate).exp();
        self.smoothed_lfo = coeff * self.smoothed_lfo + (1.0 - coeff) * raw_lfo;

        let gain = 1.0 - depth * (1.0 - self.smoothed_lfo);

        let wet_left = left * gain;
        let wet_right = right * gain;

        let out_left = math::lerp(left, wet_left, mix);
        let out_right = math::lerp(right, wet_right, mix);

        let out_gain = math::db_to_gain(output_gain_db);

        (sanitize(out_left * out_gain), sanitize(out_right * out_gain))
    }
}
