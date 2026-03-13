use oasis_core::constants::*;
use oasis_core::dsp::dynamics::EnvelopeDetector;
use oasis_core::util::denormal::sanitize;
use oasis_core::util::math;
use oasis_core::util::sample_rate::SampleRateContext;

use crate::params::Speed;

pub struct PunchProcessor {
    sample_rate: f32,
    fast_env: EnvelopeDetector,
    slow_env: EnvelopeDetector,
    current_speed: Speed,
}

impl PunchProcessor {
    pub fn new() -> Self {
        let mut proc = Self {
            sample_rate: DEFAULT_SAMPLE_RATE,
            fast_env: EnvelopeDetector::new(),
            slow_env: EnvelopeDetector::new(),
            current_speed: Speed::Fast,
        };
        proc.set_speed(Speed::Fast, DEFAULT_SAMPLE_RATE);
        proc
    }

    pub fn set_sample_rate(&mut self, ctx: &SampleRateContext) {
        self.sample_rate = ctx.sample_rate;
        self.set_speed(self.current_speed, ctx.sample_rate);
    }

    pub fn reset(&mut self) {
        self.fast_env.reset();
        self.slow_env.reset();
    }

    pub fn set_speed(&mut self, speed: Speed, sample_rate: f32) {
        self.current_speed = speed;

        let (fast_attack, fast_release) = match speed {
            Speed::Fast => (PUNCH_FAST_ATTACK_MS, PUNCH_FAST_RELEASE_MS),
            Speed::Medium => {
                let attack = (PUNCH_FAST_ATTACK_MS + PUNCH_SLOW_ATTACK_MS) * 0.5;
                let release = (PUNCH_FAST_RELEASE_MS + PUNCH_SLOW_RELEASE_MS) * 0.5;
                (attack, release)
            }
            Speed::Slow => (PUNCH_SLOW_ATTACK_MS, PUNCH_SLOW_RELEASE_MS),
        };

        self.fast_env.set_times(fast_attack, fast_release, sample_rate);
        self.slow_env.set_times(fast_attack * 10.0, fast_release * 10.0, sample_rate);
    }

    #[inline]
    pub fn process_sample(
        &mut self,
        left: f32,
        right: f32,
        attack_amt: f32,
        sustain_amt: f32,
        mix: f32,
        output_gain_db: f32,
    ) -> (f32, f32) {
        let mono = (left + right) * 0.5;

        let fast_level = self.fast_env.process_peak(mono);
        let slow_level = self.slow_env.process_peak(mono);

        let transient = fast_level - slow_level;

        let gain = 1.0
            + attack_amt * (transient / (slow_level + 0.001)).max(0.0)
            + sustain_amt * slow_level;

        let mut left_out = left * gain;
        let mut right_out = right * gain;

        left_out = math::lerp(left, left_out, mix);
        right_out = math::lerp(right, right_out, mix);

        let out_gain = math::db_to_gain(output_gain_db);
        left_out *= out_gain;
        right_out *= out_gain;

        (sanitize(left_out), sanitize(right_out))
    }
}
