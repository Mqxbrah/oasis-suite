use oasis_core::constants::*;
use oasis_core::dsp::delay::DelayLine;
use oasis_core::dsp::filter::Biquad;
use oasis_core::dsp::waveshaper;
use oasis_core::util::denormal::sanitize;
use oasis_core::util::math;
use oasis_core::util::sample_rate::SampleRateContext;

const MAX_SAMPLE_RATE: f32 = 192_000.0;
const FILTER_Q: f32 = 0.707;

pub struct DelayProcessor {
    sample_rate_ctx: SampleRateContext,

    delay_l: DelayLine,
    delay_r: DelayLine,

    lp_l: Biquad,
    lp_r: Biquad,
    hp_l: Biquad,
    hp_r: Biquad,

    lfo_phase: f32,

    current_lp_freq: f32,
    current_hp_freq: f32,

    feedback_l: f32,
    feedback_r: f32,
}

impl DelayProcessor {
    pub fn new() -> Self {
        let max_delay_samples =
            (DELAY_TIME_MAX_MS * 0.001 * MAX_SAMPLE_RATE) as usize + 2;

        Self {
            sample_rate_ctx: SampleRateContext::new(DEFAULT_SAMPLE_RATE),
            delay_l: DelayLine::new(max_delay_samples),
            delay_r: DelayLine::new(max_delay_samples),
            lp_l: Biquad::new(),
            lp_r: Biquad::new(),
            hp_l: Biquad::new(),
            hp_r: Biquad::new(),
            lfo_phase: 0.0,
            current_lp_freq: DELAY_LP_DEFAULT_HZ,
            current_hp_freq: DELAY_HP_DEFAULT_HZ,
            feedback_l: 0.0,
            feedback_r: 0.0,
        }
    }

    pub fn set_sample_rate(&mut self, ctx: &SampleRateContext) {
        self.sample_rate_ctx = *ctx;

        let max_delay_samples =
            (DELAY_TIME_MAX_MS * 0.001 * ctx.sample_rate) as usize + 2;
        self.delay_l.resize(max_delay_samples);
        self.delay_r.resize(max_delay_samples);

        self.update_lp_filter(self.current_lp_freq);
        self.update_hp_filter(self.current_hp_freq);
    }

    pub fn reset(&mut self) {
        self.delay_l.reset();
        self.delay_r.reset();
        self.lp_l.reset();
        self.lp_r.reset();
        self.hp_l.reset();
        self.hp_r.reset();
        self.lfo_phase = 0.0;
        self.feedback_l = 0.0;
        self.feedback_r = 0.0;
    }

    fn update_lp_filter(&mut self, freq: f32) {
        self.lp_l.set_lowpass(freq, FILTER_Q, self.sample_rate_ctx.sample_rate);
        self.lp_r.set_lowpass(freq, FILTER_Q, self.sample_rate_ctx.sample_rate);
        self.current_lp_freq = freq;
    }

    fn update_hp_filter(&mut self, freq: f32) {
        self.hp_l.set_highpass(freq, FILTER_Q, self.sample_rate_ctx.sample_rate);
        self.hp_r.set_highpass(freq, FILTER_Q, self.sample_rate_ctx.sample_rate);
        self.current_hp_freq = freq;
    }

    #[inline]
    #[allow(clippy::too_many_arguments)]
    pub fn process_sample(
        &mut self,
        left_in: f32,
        right_in: f32,
        delay_l_ms: f32,
        delay_r_ms: f32,
        feedback: f32,
        ping_pong: bool,
        lp_hz: f32,
        hp_hz: f32,
        saturation: f32,
        mod_rate: f32,
        mod_depth: f32,
        mix: f32,
        output_gain_db: f32,
    ) -> (f32, f32) {
        // Update filter coefficients when they change meaningfully
        if (lp_hz - self.current_lp_freq).abs() > 0.5 {
            self.update_lp_filter(lp_hz);
        }
        if (hp_hz - self.current_hp_freq).abs() > 0.5 {
            self.update_hp_filter(hp_hz);
        }

        // LFO for delay modulation
        let lfo_value = (self.lfo_phase * std::f32::consts::TAU).sin();
        let phase_inc = mod_rate / self.sample_rate_ctx.sample_rate;
        self.lfo_phase += phase_inc;
        if self.lfo_phase >= 1.0 {
            self.lfo_phase -= 1.0;
        }

        // Modulate delay times
        let mod_offset = mod_depth * lfo_value * 2.0;
        let effective_delay_l = (delay_l_ms + mod_offset)
            .clamp(DELAY_TIME_MIN_MS, DELAY_TIME_MAX_MS);
        let effective_delay_r = (delay_r_ms + mod_offset)
            .clamp(DELAY_TIME_MIN_MS, DELAY_TIME_MAX_MS);

        // Convert ms to samples
        let delay_l_samples = effective_delay_l * 0.001 * self.sample_rate_ctx.sample_rate;
        let delay_r_samples = effective_delay_r * 0.001 * self.sample_rate_ctx.sample_rate;

        // Read delayed signals
        let wet_l = self.delay_l.read_linear(delay_l_samples);
        let wet_r = self.delay_r.read_linear(delay_r_samples);

        // Build feedback signals through the filter/saturation chain
        let mut fb_l = if ping_pong { wet_r } else { wet_l };
        let mut fb_r = if ping_pong { wet_l } else { wet_r };

        fb_l *= feedback;
        fb_r *= feedback;

        // LP filter in feedback path
        fb_l = self.lp_l.process(fb_l);
        fb_r = self.lp_r.process(fb_r);

        // HP filter in feedback path
        fb_l = self.hp_l.process(fb_l);
        fb_r = self.hp_r.process(fb_r);

        // Saturation in feedback path
        if saturation > 0.001 {
            fb_l = waveshaper::tape_saturate(fb_l, saturation);
            fb_r = waveshaper::tape_saturate(fb_r, saturation);
        }

        fb_l = sanitize(fb_l);
        fb_r = sanitize(fb_r);

        self.feedback_l = fb_l;
        self.feedback_r = fb_r;

        // Write input + feedback into delay lines
        self.delay_l.write(left_in + fb_l);
        self.delay_r.write(right_in + fb_r);

        // Dry/wet mix
        let out_l = math::lerp(left_in, wet_l, mix);
        let out_r = math::lerp(right_in, wet_r, mix);

        // Output gain
        let out_gain = math::db_to_gain(output_gain_db);

        (sanitize(out_l * out_gain), sanitize(out_r * out_gain))
    }
}
