use oasis_core::constants::*;
use oasis_core::dsp::delay::DelayLine;
use oasis_core::dsp::filter::Biquad;
use oasis_core::util::denormal::sanitize;
use oasis_core::util::math;
use oasis_core::util::sample_rate::SampleRateContext;

use crate::params::ReverbAlgorithm;

const FDN_SIZE: usize = 8;
const ALLPASS_COUNT: usize = 4;
const HOUSEHOLDER_COEFF: f32 = 2.0 / FDN_SIZE as f32;
const LFO_MAX_DEPTH_SAMPLES: f32 = 16.0;

struct AllpassSection {
    delay: DelayLine,
    length: usize,
}

impl AllpassSection {
    fn new(length: usize) -> Self {
        Self {
            delay: DelayLine::new(length + 2),
            length,
        }
    }

    #[inline]
    fn process(&mut self, input: f32, coeff: f32) -> f32 {
        let delayed = self.delay.read_linear(self.length as f32);
        let feedback = input + delayed * coeff;
        self.delay.write(feedback);
        delayed - feedback * coeff
    }

    fn reset(&mut self) {
        self.delay.reset();
    }
}

pub struct VerbProcessor {
    sample_rate: f32,

    predelay_line: DelayLine,

    allpasses: [AllpassSection; ALLPASS_COUNT],
    fdn_delays: [DelayLine; FDN_SIZE],
    fdn_lengths: [usize; FDN_SIZE],
    damping_state: [f32; FDN_SIZE],

    lp_filter_l: Biquad,
    lp_filter_r: Biquad,
    hp_filter_l: Biquad,
    hp_filter_r: Biquad,

    lfo_phase: f32,
    current_low_cut: f32,
    current_high_cut: f32,
}

impl VerbProcessor {
    pub fn new() -> Self {
        let max_predelay = (VERB_PREDELAY_MAX_MS * 0.001 * DEFAULT_SAMPLE_RATE) as usize + 2;

        let scale = DEFAULT_SAMPLE_RATE / DEFAULT_SAMPLE_RATE;
        let scaled_fdn: Vec<usize> = VERB_FDN_DELAY_LENGTHS
            .iter()
            .map(|&l| ((l as f32 * scale) as usize).max(1))
            .collect();
        let scaled_ap: Vec<usize> = VERB_ALLPASS_LENGTHS
            .iter()
            .map(|&l| ((l as f32 * scale) as usize).max(1))
            .collect();

        let fdn_delays = [
            DelayLine::new(scaled_fdn[0] + 32),
            DelayLine::new(scaled_fdn[1] + 32),
            DelayLine::new(scaled_fdn[2] + 32),
            DelayLine::new(scaled_fdn[3] + 32),
            DelayLine::new(scaled_fdn[4] + 32),
            DelayLine::new(scaled_fdn[5] + 32),
            DelayLine::new(scaled_fdn[6] + 32),
            DelayLine::new(scaled_fdn[7] + 32),
        ];

        let allpasses = [
            AllpassSection::new(scaled_ap[0]),
            AllpassSection::new(scaled_ap[1]),
            AllpassSection::new(scaled_ap[2]),
            AllpassSection::new(scaled_ap[3]),
        ];

        let mut proc = Self {
            sample_rate: DEFAULT_SAMPLE_RATE,
            predelay_line: DelayLine::new(max_predelay),
            allpasses,
            fdn_delays,
            fdn_lengths: [
                scaled_fdn[0],
                scaled_fdn[1],
                scaled_fdn[2],
                scaled_fdn[3],
                scaled_fdn[4],
                scaled_fdn[5],
                scaled_fdn[6],
                scaled_fdn[7],
            ],
            damping_state: [0.0; FDN_SIZE],
            lp_filter_l: Biquad::new(),
            lp_filter_r: Biquad::new(),
            hp_filter_l: Biquad::new(),
            hp_filter_r: Biquad::new(),
            lfo_phase: 0.0,
            current_low_cut: VERB_LOWCUT_DEFAULT_HZ,
            current_high_cut: VERB_HIGHCUT_DEFAULT_HZ,
        };

        proc.update_output_filters(VERB_LOWCUT_DEFAULT_HZ, VERB_HIGHCUT_DEFAULT_HZ);
        proc
    }

    pub fn set_sample_rate(&mut self, ctx: &SampleRateContext) {
        self.sample_rate = ctx.sample_rate;

        let max_predelay = (VERB_PREDELAY_MAX_MS * 0.001 * ctx.sample_rate) as usize + 2;
        self.predelay_line.resize(max_predelay);

        let scale = ctx.sample_rate / DEFAULT_SAMPLE_RATE;
        for i in 0..FDN_SIZE {
            let new_len = ((VERB_FDN_DELAY_LENGTHS[i] as f32 * scale) as usize).max(1);
            self.fdn_lengths[i] = new_len;
            self.fdn_delays[i].resize(new_len + 32);
        }
        for i in 0..ALLPASS_COUNT {
            let new_len = ((VERB_ALLPASS_LENGTHS[i] as f32 * scale) as usize).max(1);
            self.allpasses[i].length = new_len;
            self.allpasses[i].delay.resize(new_len + 2);
        }

        self.update_output_filters(self.current_low_cut, self.current_high_cut);
    }

    pub fn reset(&mut self) {
        self.predelay_line.reset();
        for ap in &mut self.allpasses {
            ap.reset();
        }
        for dl in &mut self.fdn_delays {
            dl.reset();
        }
        self.damping_state = [0.0; FDN_SIZE];
        self.lp_filter_l.reset();
        self.lp_filter_r.reset();
        self.hp_filter_l.reset();
        self.hp_filter_r.reset();
        self.lfo_phase = 0.0;
    }

    fn update_output_filters(&mut self, low_cut: f32, high_cut: f32) {
        let q = 0.707;
        self.hp_filter_l.set_highpass(low_cut, q, self.sample_rate);
        self.hp_filter_r.set_highpass(low_cut, q, self.sample_rate);
        self.lp_filter_l.set_lowpass(high_cut, q, self.sample_rate);
        self.lp_filter_r.set_lowpass(high_cut, q, self.sample_rate);
        self.current_low_cut = low_cut;
        self.current_high_cut = high_cut;
    }

    fn feedback_for_algorithm(&self, size: f32, algorithm: ReverbAlgorithm) -> f32 {
        match algorithm {
            ReverbAlgorithm::Room => math::lerp(0.6, 0.8, size),
            ReverbAlgorithm::Hall => math::lerp(0.8, 0.95, size),
            ReverbAlgorithm::Plate => math::lerp(0.7, 0.9, size),
            ReverbAlgorithm::Chamber => math::lerp(0.75, 0.9, size),
            ReverbAlgorithm::Shimmer => math::lerp(0.8, 0.95, size),
            ReverbAlgorithm::Spring => math::lerp(0.5, 0.7, size),
        }
    }

    fn mod_depth_for_algorithm(&self, modulation: f32, algorithm: ReverbAlgorithm) -> f32 {
        let base = modulation * LFO_MAX_DEPTH_SAMPLES;
        match algorithm {
            ReverbAlgorithm::Spring => base * 1.5,
            _ => base,
        }
    }

    #[inline]
    pub fn process_sample(
        &mut self,
        left_in: f32,
        right_in: f32,
        size: f32,
        predelay_ms: f32,
        damping: f32,
        width: f32,
        diffusion: f32,
        low_cut: f32,
        high_cut: f32,
        modulation: f32,
        algorithm: ReverbAlgorithm,
        mix: f32,
        output_gain_db: f32,
    ) -> (f32, f32) {
        // Update output filters when values change meaningfully
        if (low_cut - self.current_low_cut).abs() > 0.5
            || (high_cut - self.current_high_cut).abs() > 0.5
        {
            self.update_output_filters(low_cut, high_cut);
        }

        let mono_in = (left_in + right_in) * 0.5;

        // Pre-delay
        let predelay_samples = predelay_ms * 0.001 * self.sample_rate;
        self.predelay_line.write(mono_in);
        let predelayed = if predelay_samples > 0.5 {
            self.predelay_line.read_linear(predelay_samples)
        } else {
            mono_in
        };

        // Allpass diffusion network
        let ap_coeff = diffusion * 0.6;
        let mut diffused = predelayed;
        for ap in &mut self.allpasses {
            diffused = ap.process(diffused, ap_coeff);
        }

        // LFO for modulation
        let lfo_rate = 0.5; // Hz
        self.lfo_phase += lfo_rate / self.sample_rate;
        if self.lfo_phase >= 1.0 {
            self.lfo_phase -= 1.0;
        }
        let mod_depth = self.mod_depth_for_algorithm(modulation, algorithm);

        let feedback = self.feedback_for_algorithm(size, algorithm);
        let damp_coeff = damping.clamp(0.0, 0.99);

        // Read from FDN delay lines
        let mut fdn_out = [0.0f32; FDN_SIZE];
        for i in 0..FDN_SIZE {
            let base_delay = self.fdn_lengths[i] as f32;
            let lfo_offset = if mod_depth > 0.01 {
                let phase_offset = i as f32 / FDN_SIZE as f32;
                let lfo_i = ((self.lfo_phase + phase_offset).fract() * std::f32::consts::TAU).sin();
                lfo_i * mod_depth
            } else {
                0.0
            };
            let read_delay = (base_delay + lfo_offset).max(1.0);
            fdn_out[i] = self.fdn_delays[i].read_linear(read_delay);
        }

        // Householder mixing matrix: out[i] = in[i] - (2/N) * sum(in)
        let sum: f32 = fdn_out.iter().sum();
        let mut mixed = [0.0f32; FDN_SIZE];
        for i in 0..FDN_SIZE {
            mixed[i] = fdn_out[i] - HOUSEHOLDER_COEFF * sum;
        }

        // Apply damping LPF and feedback, write back
        for i in 0..FDN_SIZE {
            let damped = self.damping_state[i] + damp_coeff * (mixed[i] - self.damping_state[i]);
            self.damping_state[i] = damped;

            let input_to_delay = diffused / (FDN_SIZE as f32).sqrt() + damped * feedback;
            self.fdn_delays[i].write(sanitize(input_to_delay));
        }

        // Sum outputs: first 4 = left, last 4 = right (with width crossfeed)
        let mut wet_l = 0.0f32;
        let mut wet_r = 0.0f32;
        for i in 0..4 {
            wet_l += fdn_out[i];
        }
        for i in 4..FDN_SIZE {
            wet_r += fdn_out[i];
        }
        wet_l *= 0.25;
        wet_r *= 0.25;

        // Width crossfeed: width=1 = full stereo, width=0 = mono
        let mono_wet = (wet_l + wet_r) * 0.5;
        wet_l = math::lerp(mono_wet, wet_l, width);
        wet_r = math::lerp(mono_wet, wet_r, width);

        // Output filters
        wet_l = self.hp_filter_l.process(wet_l);
        wet_l = self.lp_filter_l.process(wet_l);
        wet_r = self.hp_filter_r.process(wet_r);
        wet_r = self.lp_filter_r.process(wet_r);

        // Dry/wet mix
        let out_l = math::lerp(left_in, wet_l, mix);
        let out_r = math::lerp(right_in, wet_r, mix);

        // Output gain
        let out_gain = math::db_to_gain(output_gain_db);

        (sanitize(out_l * out_gain), sanitize(out_r * out_gain))
    }
}
