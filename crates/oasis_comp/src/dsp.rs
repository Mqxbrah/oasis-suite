use oasis_core::dsp::dynamics::{EnvelopeDetector, GainComputer};
use oasis_core::dsp::filter::Biquad;
use oasis_core::util::denormal::sanitize;
use oasis_core::util::math::{db_to_gain, gain_to_db, lerp};
use oasis_core::util::sample_rate::SampleRateContext;
use oasis_core::constants::*;

use crate::params::DetectionMode;

pub struct CompProcessor {
    sample_rate_ctx: SampleRateContext,
    envelope_l: EnvelopeDetector,
    envelope_r: EnvelopeDetector,
    gain_computer: GainComputer,
    sc_hp_l: Biquad,
    sc_hp_r: Biquad,
    current_sc_hp_freq: f32,
}

impl CompProcessor {
    pub fn new() -> Self {
        Self {
            sample_rate_ctx: SampleRateContext::new(DEFAULT_SAMPLE_RATE),
            envelope_l: EnvelopeDetector::new(),
            envelope_r: EnvelopeDetector::new(),
            gain_computer: GainComputer::new(
                COMP_THRESHOLD_DEFAULT_DB,
                COMP_RATIO_DEFAULT,
                COMP_KNEE_DEFAULT_DB,
            ),
            sc_hp_l: Biquad::new(),
            sc_hp_r: Biquad::new(),
            current_sc_hp_freq: COMP_SC_HP_DEFAULT_HZ,
        }
    }

    pub fn set_sample_rate(&mut self, ctx: &SampleRateContext) {
        self.sample_rate_ctx = *ctx;
        self.envelope_l.set_times(
            COMP_ATTACK_DEFAULT_MS,
            COMP_RELEASE_DEFAULT_MS,
            ctx.sample_rate,
        );
        self.envelope_r.set_times(
            COMP_ATTACK_DEFAULT_MS,
            COMP_RELEASE_DEFAULT_MS,
            ctx.sample_rate,
        );
        self.update_sc_hp_filter(self.current_sc_hp_freq);
    }

    pub fn reset(&mut self) {
        self.envelope_l.reset();
        self.envelope_r.reset();
        self.sc_hp_l.reset();
        self.sc_hp_r.reset();
    }

    fn update_sc_hp_filter(&mut self, freq: f32) {
        let q = 0.707;
        self.sc_hp_l
            .set_highpass(freq, q, self.sample_rate_ctx.sample_rate);
        self.sc_hp_r
            .set_highpass(freq, q, self.sample_rate_ctx.sample_rate);
        self.current_sc_hp_freq = freq;
    }

    #[inline]
    pub fn process_sample(
        &mut self,
        left_in: f32,
        right_in: f32,
        threshold_db: f32,
        ratio: f32,
        attack_ms: f32,
        release_ms: f32,
        knee_db: f32,
        makeup_db: f32,
        mix: f32,
        detection_mode: DetectionMode,
        sc_hp_freq: f32,
        output_gain_db: f32,
    ) -> (f32, f32) {
        // Update envelope detector times
        self.envelope_l.set_times(
            attack_ms,
            release_ms,
            self.sample_rate_ctx.sample_rate,
        );
        self.envelope_r.set_times(
            attack_ms,
            release_ms,
            self.sample_rate_ctx.sample_rate,
        );

        // Update sidechain HP filter if frequency changed
        if (sc_hp_freq - self.current_sc_hp_freq).abs() > 0.5 {
            self.update_sc_hp_filter(sc_hp_freq);
        }

        // Update gain computer
        self.gain_computer.threshold_db = threshold_db;
        self.gain_computer.ratio = ratio;
        self.gain_computer.knee_db = knee_db;

        // Sidechain signal: HP filter before detection
        let sc_l = self.sc_hp_l.process(left_in);
        let sc_r = self.sc_hp_r.process(right_in);

        // Combine to mono for sidechain detection
        let sc_mono = (sc_l + sc_r) * 0.5;

        // Envelope detection (use left detector for the mono sidechain)
        let envelope = match detection_mode {
            DetectionMode::Peak => self.envelope_l.process_peak(sc_mono),
            DetectionMode::Rms => self.envelope_l.process_rms(sc_mono),
        };

        // Convert envelope to dB and compute gain reduction
        let envelope_db = gain_to_db(envelope);
        let gain_reduction_db = self.gain_computer.compute_gain_db(envelope_db);

        // Apply gain reduction + makeup
        let total_gain = db_to_gain(gain_reduction_db + makeup_db);

        let left_compressed = left_in * total_gain;
        let right_compressed = right_in * total_gain;

        // Dry/wet mix
        let left_mixed = lerp(left_in, left_compressed, mix);
        let right_mixed = lerp(right_in, right_compressed, mix);

        // Output gain
        let out_gain = db_to_gain(output_gain_db);
        let left_out = left_mixed * out_gain;
        let right_out = right_mixed * out_gain;

        (sanitize(left_out), sanitize(right_out))
    }
}
