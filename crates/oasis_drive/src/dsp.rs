use oasis_core::dsp::filter::Biquad;
use oasis_core::dsp::waveshaper;
use oasis_core::util::denormal::sanitize;
use oasis_core::util::math;
use oasis_core::util::sample_rate::SampleRateContext;
use oasis_core::constants::*;

use crate::params::Algorithm;

pub struct DriveProcessor {
    sample_rate_ctx: SampleRateContext,
    tone_filter_l: Biquad,
    tone_filter_r: Biquad,
    current_tone_hz: f32,
}

impl DriveProcessor {
    pub fn new() -> Self {
        Self {
            sample_rate_ctx: SampleRateContext::new(DEFAULT_SAMPLE_RATE),
            tone_filter_l: Biquad::new(),
            tone_filter_r: Biquad::new(),
            current_tone_hz: DRIVE_TONE_DEFAULT_HZ,
        }
    }

    pub fn set_sample_rate(&mut self, ctx: &SampleRateContext) {
        self.sample_rate_ctx = *ctx;
        self.update_tone_filter(self.current_tone_hz);
    }

    pub fn reset(&mut self) {
        self.tone_filter_l.reset();
        self.tone_filter_r.reset();
    }

    fn update_tone_filter(&mut self, freq_hz: f32) {
        let q = 0.707;
        self.tone_filter_l
            .set_lowpass(freq_hz, q, self.sample_rate_ctx.sample_rate);
        self.tone_filter_r
            .set_lowpass(freq_hz, q, self.sample_rate_ctx.sample_rate);
        self.current_tone_hz = freq_hz;
    }

    #[inline]
    pub fn process_sample(
        &mut self,
        left_in: f32,
        right_in: f32,
        drive: f32,
        input_gain_db: f32,
        tone_hz: f32,
        algorithm: Algorithm,
        mix: f32,
        output_gain_db: f32,
    ) -> (f32, f32) {
        let in_gain = math::db_to_gain(input_gain_db);
        let mut left = left_in * in_gain;
        let mut right = right_in * in_gain;

        left = match algorithm {
            Algorithm::Tape => waveshaper::tape_saturate(left, drive),
            Algorithm::Tube => waveshaper::tube_saturate(left, drive),
            Algorithm::Transistor => waveshaper::transistor_clip(left, drive),
            Algorithm::Digital => waveshaper::digital_fold(left, drive),
        };
        right = match algorithm {
            Algorithm::Tape => waveshaper::tape_saturate(right, drive),
            Algorithm::Tube => waveshaper::tube_saturate(right, drive),
            Algorithm::Transistor => waveshaper::transistor_clip(right, drive),
            Algorithm::Digital => waveshaper::digital_fold(right, drive),
        };

        if (tone_hz - self.current_tone_hz).abs() > 0.5 {
            self.update_tone_filter(tone_hz);
        }
        left = sanitize(self.tone_filter_l.process(left));
        right = sanitize(self.tone_filter_r.process(right));

        left = math::lerp(left_in, left, mix);
        right = math::lerp(right_in, right, mix);

        let out_gain = math::db_to_gain(output_gain_db);
        left *= out_gain;
        right *= out_gain;

        (sanitize(left), sanitize(right))
    }
}
