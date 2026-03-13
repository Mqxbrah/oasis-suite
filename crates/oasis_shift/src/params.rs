use nih_plug::prelude::*;
use oasis_core::constants::*;
use oasis_core::params::formatting;
use oasis_ui::ViziaState;
use std::sync::Arc;

#[derive(Enum, PartialEq, Clone, Copy)]
pub enum ShiftMode {
    #[name = "Simple"]
    Simple,
    #[name = "Detune"]
    Detune,
}

#[derive(Params)]
pub struct OasisShiftParams {
    #[persist = "editor-state"]
    pub editor_state: Arc<ViziaState>,

    #[id = "semitones"]
    pub semitones: FloatParam,

    #[id = "cents"]
    pub cents: FloatParam,

    #[id = "detune"]
    pub detune: FloatParam,

    #[id = "mode"]
    pub mode: EnumParam<ShiftMode>,

    #[id = "mix"]
    pub mix: FloatParam,

    #[id = "output_gain"]
    pub output_gain: FloatParam,
}

impl Default for OasisShiftParams {
    fn default() -> Self {
        Self {
            editor_state: oasis_ui::default_editor_size(),

            semitones: FloatParam::new(
                "Semitones",
                SHIFT_SEMITONES_DEFAULT,
                FloatRange::Linear {
                    min: SHIFT_SEMITONES_MIN,
                    max: SHIFT_SEMITONES_MAX,
                },
            )
            .with_smoother(SmoothingStyle::Linear(SMOOTHING_MS_FAST))
            .with_step_size(1.0)
            .with_value_to_string(formatting::v2s_semitones())
            .with_string_to_value(formatting::s2v_semitones()),

            cents: FloatParam::new(
                "Cents",
                SHIFT_CENTS_DEFAULT,
                FloatRange::Linear {
                    min: SHIFT_CENTS_MIN,
                    max: SHIFT_CENTS_MAX,
                },
            )
            .with_smoother(SmoothingStyle::Linear(SMOOTHING_MS_FAST))
            .with_value_to_string(formatting::v2s_cents())
            .with_string_to_value(formatting::s2v_cents()),

            detune: FloatParam::new(
                "Detune",
                SHIFT_DETUNE_DEFAULT,
                FloatRange::Linear {
                    min: SHIFT_DETUNE_MIN,
                    max: SHIFT_DETUNE_MAX,
                },
            )
            .with_smoother(SmoothingStyle::Linear(SMOOTHING_MS_FAST))
            .with_value_to_string(formatting::v2s_cents())
            .with_string_to_value(formatting::s2v_cents()),

            mode: EnumParam::new("Mode", ShiftMode::Simple),

            mix: FloatParam::new(
                "Mix",
                1.0,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            )
            .with_smoother(SmoothingStyle::Linear(SMOOTHING_MS_FAST))
            .with_value_to_string(formatting::v2s_percentage())
            .with_string_to_value(formatting::s2v_percentage()),

            output_gain: FloatParam::new(
                "Output Gain",
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
