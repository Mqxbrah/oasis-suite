use oasis_core::constants::DEFAULT_SAMPLE_RATE;
use oasis_core::util::denormal::sanitize;

use crate::params::Waveform;

#[derive(Debug, Clone, Copy, PartialEq)]
enum EnvStage {
    Idle,
    Attack,
    Decay,
    Sustain,
    Release,
}

pub struct SynthEngine {
    sample_rate: f32,

    osc1_phase: f32,
    osc2_phase: f32,

    env_stage: EnvStage,
    env_value: f32,
    env_attack_coeff: f32,
    env_decay_coeff: f32,
    env_release_coeff: f32,
    env_sustain_level: f32,

    filter_state: f32,

    current_note: Option<u8>,
    current_freq: f32,

    gate: bool,
}

impl SynthEngine {
    pub fn new() -> Self {
        Self {
            sample_rate: DEFAULT_SAMPLE_RATE,
            osc1_phase: 0.0,
            osc2_phase: 0.0,
            env_stage: EnvStage::Idle,
            env_value: 0.0,
            env_attack_coeff: 0.0,
            env_decay_coeff: 0.0,
            env_release_coeff: 0.0,
            env_sustain_level: 0.7,
            filter_state: 0.0,
            current_note: None,
            current_freq: 440.0,
            gate: false,
        }
    }

    pub fn set_sample_rate(&mut self, rate: f32) {
        self.sample_rate = rate;
    }

    pub fn reset(&mut self) {
        self.osc1_phase = 0.0;
        self.osc2_phase = 0.0;
        self.env_stage = EnvStage::Idle;
        self.env_value = 0.0;
        self.filter_state = 0.0;
        self.current_note = None;
        self.gate = false;
    }

    pub fn note_on(&mut self, note: u8, _velocity: f32) {
        self.current_note = Some(note);
        self.current_freq = midi_note_to_freq(note);
        self.gate = true;
        self.env_stage = EnvStage::Attack;
    }

    pub fn note_off(&mut self, note: u8) {
        if self.current_note == Some(note) {
            self.gate = false;
            self.env_stage = EnvStage::Release;
        }
    }

    pub fn set_envelope(&mut self, attack_ms: f32, decay_ms: f32, sustain: f32, release_ms: f32) {
        self.env_attack_coeff = time_ms_to_coeff(attack_ms, self.sample_rate);
        self.env_decay_coeff = time_ms_to_coeff(decay_ms, self.sample_rate);
        self.env_release_coeff = time_ms_to_coeff(release_ms, self.sample_rate);
        self.env_sustain_level = sustain;
    }

    #[allow(clippy::too_many_arguments)]
    pub fn process_sample(
        &mut self,
        osc1_wave: Waveform,
        osc1_level: f32,
        osc1_detune: f32,
        osc2_wave: Waveform,
        osc2_level: f32,
        osc2_detune: f32,
        filter_cutoff: f32,
        _filter_reso: f32,
        master_gain_db: f32,
    ) -> (f32, f32) {
        if self.env_stage == EnvStage::Idle {
            return (0.0, 0.0);
        }

        // Oscillator frequencies with detuning (cents)
        let osc1_freq = self.current_freq * cents_to_ratio(osc1_detune);
        let osc2_freq = self.current_freq * cents_to_ratio(osc2_detune);

        // Phase increments
        let osc1_inc = osc1_freq / self.sample_rate;
        let osc2_inc = osc2_freq / self.sample_rate;

        // Generate oscillator samples
        let osc1_out = generate_waveform(osc1_wave, self.osc1_phase);
        let osc2_out = generate_waveform(osc2_wave, self.osc2_phase);

        // Advance phases
        self.osc1_phase += osc1_inc;
        if self.osc1_phase >= 1.0 {
            self.osc1_phase -= 1.0;
        }
        self.osc2_phase += osc2_inc;
        if self.osc2_phase >= 1.0 {
            self.osc2_phase -= 1.0;
        }

        // Mix oscillators
        let mixed = osc1_out * osc1_level + osc2_out * osc2_level;

        // One-pole lowpass filter
        let cutoff_coeff =
            (std::f32::consts::TAU * filter_cutoff / self.sample_rate).min(1.0);
        self.filter_state += cutoff_coeff * (mixed - self.filter_state);
        let filtered = sanitize(self.filter_state);

        // ADSR envelope
        self.advance_envelope();
        let enveloped = filtered * self.env_value;

        // Master gain
        let gain = db_to_linear(master_gain_db);
        let out = sanitize(enveloped * gain);

        (out, out)
    }

    fn advance_envelope(&mut self) {
        match self.env_stage {
            EnvStage::Attack => {
                self.env_value += (1.0 - self.env_value) * (1.0 - self.env_attack_coeff);
                if self.env_value >= 0.999 {
                    self.env_value = 1.0;
                    self.env_stage = EnvStage::Decay;
                }
            }
            EnvStage::Decay => {
                self.env_value +=
                    (self.env_sustain_level - self.env_value) * (1.0 - self.env_decay_coeff);
                if (self.env_value - self.env_sustain_level).abs() < 0.001 {
                    self.env_value = self.env_sustain_level;
                    self.env_stage = EnvStage::Sustain;
                }
            }
            EnvStage::Sustain => {
                self.env_value = self.env_sustain_level;
            }
            EnvStage::Release => {
                self.env_value *= self.env_release_coeff;
                if self.env_value < 0.0001 {
                    self.env_value = 0.0;
                    self.env_stage = EnvStage::Idle;
                    self.current_note = None;
                }
            }
            EnvStage::Idle => {}
        }
    }
}

#[inline]
fn generate_waveform(waveform: Waveform, phase: f32) -> f32 {
    match waveform {
        Waveform::Sine => (phase * std::f32::consts::TAU).sin(),
        Waveform::Saw => 2.0 * phase - 1.0,
        Waveform::Square => {
            if phase < 0.5 {
                1.0
            } else {
                -1.0
            }
        }
        Waveform::Triangle => 4.0 * (phase - (phase + 0.5).floor()).abs() - 1.0,
    }
}

#[inline]
fn midi_note_to_freq(note: u8) -> f32 {
    440.0 * 2.0f32.powf((note as f32 - 69.0) / 12.0)
}

#[inline]
fn cents_to_ratio(cents: f32) -> f32 {
    if cents == 0.0 {
        1.0
    } else {
        2.0f32.powf(cents / 1200.0)
    }
}

#[inline]
fn time_ms_to_coeff(time_ms: f32, sample_rate: f32) -> f32 {
    if time_ms <= 0.0 {
        return 0.0;
    }
    (-1.0 / (time_ms * 0.001 * sample_rate)).exp()
}

#[inline]
fn db_to_linear(db: f32) -> f32 {
    10.0f32.powf(db / 20.0)
}
