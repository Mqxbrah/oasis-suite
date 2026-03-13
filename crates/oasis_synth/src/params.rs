use nih_plug::prelude::*;
use oasis_core::constants::*;
use oasis_core::params::formatting;
use oasis_ui::ViziaState;
use std::sync::Arc;

#[derive(Enum, PartialEq, Clone, Copy, Debug)]
pub enum Waveform {
    #[name = "Sine"]
    Sine,
    #[name = "Saw"]
    Saw,
    #[name = "Square"]
    Square,
    #[name = "Triangle"]
    Triangle,
}

#[derive(Params)]
pub struct OasisSynthParams {
    #[persist = "editor-state"]
    pub editor_state: Arc<ViziaState>,

    // ── Oscillator 1 ──
    #[id = "osc1_wave"]
    pub osc1_waveform: EnumParam<Waveform>,

    #[id = "osc1_level"]
    pub osc1_level: FloatParam,

    #[id = "osc1_detune"]
    pub osc1_detune: FloatParam,

    // ── Oscillator 2 ──
    #[id = "osc2_wave"]
    pub osc2_waveform: EnumParam<Waveform>,

    #[id = "osc2_level"]
    pub osc2_level: FloatParam,

    #[id = "osc2_detune"]
    pub osc2_detune: FloatParam,

    // ── Filter ──
    #[id = "filter_cutoff"]
    pub filter_cutoff: FloatParam,

    #[id = "filter_reso"]
    pub filter_resonance: FloatParam,

    // ── Envelope (ADSR) ──
    #[id = "env_attack"]
    pub env_attack: FloatParam,

    #[id = "env_decay"]
    pub env_decay: FloatParam,

    #[id = "env_sustain"]
    pub env_sustain: FloatParam,

    #[id = "env_release"]
    pub env_release: FloatParam,

    // ── Master ──
    #[id = "master_gain"]
    pub master_gain: FloatParam,
}

impl Default for OasisSynthParams {
    fn default() -> Self {
        Self {
            editor_state: oasis_ui::default_editor_size(),

            osc1_waveform: EnumParam::new("Osc1 Wave", Waveform::Saw),

            osc1_level: FloatParam::new(
                "Osc1 Level",
                1.0,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            )
            .with_smoother(SmoothingStyle::Linear(SMOOTHING_MS_FAST))
            .with_value_to_string(formatting::v2s_percentage())
            .with_string_to_value(formatting::s2v_percentage()),

            osc1_detune: FloatParam::new(
                "Osc1 Detune",
                0.0,
                FloatRange::Linear {
                    min: 0.0,
                    max: SYNTH_DETUNE_MAX_CENTS,
                },
            )
            .with_smoother(SmoothingStyle::Linear(SMOOTHING_MS_FAST))
            .with_value_to_string(formatting::v2s_cents())
            .with_string_to_value(formatting::s2v_cents()),

            osc2_waveform: EnumParam::new("Osc2 Wave", Waveform::Saw),

            osc2_level: FloatParam::new(
                "Osc2 Level",
                0.0,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            )
            .with_smoother(SmoothingStyle::Linear(SMOOTHING_MS_FAST))
            .with_value_to_string(formatting::v2s_percentage())
            .with_string_to_value(formatting::s2v_percentage()),

            osc2_detune: FloatParam::new(
                "Osc2 Detune",
                0.0,
                FloatRange::Linear {
                    min: 0.0,
                    max: SYNTH_DETUNE_MAX_CENTS,
                },
            )
            .with_smoother(SmoothingStyle::Linear(SMOOTHING_MS_FAST))
            .with_value_to_string(formatting::v2s_cents())
            .with_string_to_value(formatting::s2v_cents()),

            filter_cutoff: FloatParam::new(
                "Filter Cutoff",
                SYNTH_FILTER_CUTOFF_DEFAULT_HZ,
                FloatRange::Skewed {
                    min: SYNTH_FILTER_CUTOFF_MIN_HZ,
                    max: SYNTH_FILTER_CUTOFF_MAX_HZ,
                    factor: FloatRange::skew_factor(-2.0),
                },
            )
            .with_smoother(SmoothingStyle::Logarithmic(SMOOTHING_MS_NORMAL))
            .with_value_to_string(formatting::v2s_hz())
            .with_string_to_value(formatting::s2v_hz()),

            filter_resonance: FloatParam::new(
                "Filter Resonance",
                SYNTH_FILTER_RESO_DEFAULT,
                FloatRange::Linear {
                    min: SYNTH_FILTER_RESO_MIN,
                    max: SYNTH_FILTER_RESO_MAX,
                },
            )
            .with_smoother(SmoothingStyle::Linear(SMOOTHING_MS_FAST))
            .with_value_to_string(formatting::v2s_percentage())
            .with_string_to_value(formatting::s2v_percentage()),

            env_attack: FloatParam::new(
                "Attack",
                SYNTH_ATTACK_DEFAULT_MS,
                FloatRange::Skewed {
                    min: SYNTH_ENV_MIN_MS,
                    max: SYNTH_ENV_MAX_MS,
                    factor: FloatRange::skew_factor(-1.0),
                },
            )
            .with_smoother(SmoothingStyle::Linear(SMOOTHING_MS_FAST))
            .with_value_to_string(formatting::v2s_ms())
            .with_string_to_value(formatting::s2v_ms()),

            env_decay: FloatParam::new(
                "Decay",
                SYNTH_DECAY_DEFAULT_MS,
                FloatRange::Skewed {
                    min: SYNTH_ENV_MIN_MS,
                    max: SYNTH_ENV_MAX_MS,
                    factor: FloatRange::skew_factor(-1.0),
                },
            )
            .with_smoother(SmoothingStyle::Linear(SMOOTHING_MS_FAST))
            .with_value_to_string(formatting::v2s_ms())
            .with_string_to_value(formatting::s2v_ms()),

            env_sustain: FloatParam::new(
                "Sustain",
                SYNTH_SUSTAIN_DEFAULT,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            )
            .with_smoother(SmoothingStyle::Linear(SMOOTHING_MS_FAST))
            .with_value_to_string(formatting::v2s_percentage())
            .with_string_to_value(formatting::s2v_percentage()),

            env_release: FloatParam::new(
                "Release",
                SYNTH_RELEASE_DEFAULT_MS,
                FloatRange::Skewed {
                    min: SYNTH_ENV_MIN_MS,
                    max: SYNTH_ENV_MAX_MS,
                    factor: FloatRange::skew_factor(-1.0),
                },
            )
            .with_smoother(SmoothingStyle::Linear(SMOOTHING_MS_FAST))
            .with_value_to_string(formatting::v2s_ms())
            .with_string_to_value(formatting::s2v_ms()),

            master_gain: FloatParam::new(
                "Master Gain",
                0.0,
                FloatRange::Linear {
                    min: GAIN_MIN_DB,
                    max: GAIN_MAX_DB,
                },
            )
            .with_smoother(SmoothingStyle::Linear(SMOOTHING_MS_FAST))
            .with_value_to_string(formatting::v2s_db())
            .with_string_to_value(formatting::s2v_db()),
        }
    }
}
