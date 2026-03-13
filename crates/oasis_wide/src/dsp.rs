use oasis_core::dsp::delay::DelayLine;
use oasis_core::dsp::filter::Biquad;
use oasis_core::dsp::mid_side;
use oasis_core::util::denormal::sanitize;
use oasis_core::util::math;
use oasis_core::util::sample_rate::SampleRateContext;
use oasis_core::constants::*;

use crate::params::HaasChannel;

pub struct WideProcessor {
    sample_rate_ctx: SampleRateContext,

    haas_delay_l: DelayLine,
    haas_delay_r: DelayLine,

    /// Highpass on the side signal to collapse bass to mono
    bass_mono_hp_l: Biquad,
    bass_mono_hp_r: Biquad,

    current_bass_mono_freq: f32,
}

impl WideProcessor {
    pub fn new() -> Self {
        let ctx = SampleRateContext::new(DEFAULT_SAMPLE_RATE);
        let max_haas_samples = (HAAS_MAX_DELAY_MS * 0.001 * DEFAULT_SAMPLE_RATE) as usize + 2;

        Self {
            sample_rate_ctx: ctx,
            haas_delay_l: DelayLine::new(max_haas_samples),
            haas_delay_r: DelayLine::new(max_haas_samples),
            bass_mono_hp_l: Biquad::new(),
            bass_mono_hp_r: Biquad::new(),
            current_bass_mono_freq: BASS_MONO_DEFAULT_FREQ,
        }
    }

    pub fn set_sample_rate(&mut self, ctx: &SampleRateContext) {
        self.sample_rate_ctx = *ctx;

        let max_haas_samples = (HAAS_MAX_DELAY_MS * 0.001 * ctx.sample_rate) as usize + 2;
        self.haas_delay_l.resize(max_haas_samples);
        self.haas_delay_r.resize(max_haas_samples);

        self.update_bass_mono_filter(self.current_bass_mono_freq);
    }

    pub fn reset(&mut self) {
        self.haas_delay_l.reset();
        self.haas_delay_r.reset();
        self.bass_mono_hp_l.reset();
        self.bass_mono_hp_r.reset();
    }

    fn update_bass_mono_filter(&mut self, freq: f32) {
        let q = 0.707;
        self.bass_mono_hp_l.set_highpass(freq, q, self.sample_rate_ctx.sample_rate);
        self.bass_mono_hp_r.set_highpass(freq, q, self.sample_rate_ctx.sample_rate);
        self.current_bass_mono_freq = freq;
    }

    /// Process a single stereo pair.
    ///
    /// Returns (left_out, right_out).
    #[inline]
    pub fn process_sample(
        &mut self,
        left_in: f32,
        right_in: f32,
        width: f32,
        mid_gain_db: f32,
        side_gain_db: f32,
        haas_delay_ms: f32,
        haas_channel: HaasChannel,
        bass_mono_enabled: bool,
        bass_mono_freq: f32,
        mix: f32,
        output_gain_db: f32,
    ) -> (f32, f32) {
        // M/S encode
        let (mid, side) = mid_side::encode(left_in, right_in);

        // Apply gains
        let mid_gain = math::db_to_gain(mid_gain_db);
        let side_gain = math::db_to_gain(side_gain_db);

        let mid_processed = mid * mid_gain;
        let side_processed = side * side_gain * width;

        // M/S decode
        let (mut left, mut right) = mid_side::decode(mid_processed, side_processed);

        // Bass mono: highpass the side component to collapse bass to center.
        // We re-encode to M/S, highpass the side, then decode.
        if bass_mono_enabled {
            if (bass_mono_freq - self.current_bass_mono_freq).abs() > 0.5 {
                self.update_bass_mono_filter(bass_mono_freq);
            }

            let (bm_mid, bm_side) = mid_side::encode(left, right);
            // Highpass on the side signal removes low-frequency stereo content
            let bm_side_filtered = self.bass_mono_hp_l.process(bm_side);
            let (bm_l, bm_r) = mid_side::decode(bm_mid, sanitize(bm_side_filtered));
            left = bm_l;
            right = bm_r;
        }

        // Haas delay
        let delay_samples = haas_delay_ms * 0.001 * self.sample_rate_ctx.sample_rate;
        if delay_samples > 0.5 {
            match haas_channel {
                HaasChannel::Right => {
                    self.haas_delay_r.write(right);
                    right = self.haas_delay_r.read_linear(delay_samples);
                    self.haas_delay_l.write(left);
                    let _ = self.haas_delay_l.read_linear(0.0);
                }
                HaasChannel::Left => {
                    self.haas_delay_l.write(left);
                    left = self.haas_delay_l.read_linear(delay_samples);
                    self.haas_delay_r.write(right);
                    let _ = self.haas_delay_r.read_linear(0.0);
                }
            }
        } else {
            self.haas_delay_l.write(left);
            self.haas_delay_r.write(right);
        }

        // Dry/wet mix
        left = math::lerp(left_in, left, mix);
        right = math::lerp(right_in, right, mix);

        // Output gain
        let out_gain = math::db_to_gain(output_gain_db);
        left *= out_gain;
        right *= out_gain;

        (sanitize(left), sanitize(right))
    }
}
