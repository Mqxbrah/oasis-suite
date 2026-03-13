use nih_plug::prelude::*;
use oasis_core::constants::*;
use oasis_core::params::formatting;
use oasis_ui::ViziaState;
use std::sync::Arc;

#[derive(Params)]
pub struct OasisPunchParams {
    #[persist = "editor-state"]
    pub editor_state: Arc<ViziaState>,

    #[id = "attack"]
    pub attack: FloatParam,

    #[id = "sustain"]
    pub sustain: FloatParam,

    #[id = "speed"]
    pub speed: EnumParam<Speed>,

    #[id = "mix"]
    pub mix: FloatParam,

    #[id = "output_gain"]
    pub output_gain: FloatParam,
}

#[derive(Enum, PartialEq, Clone, Copy)]
pub enum Speed {
    #[name = "Fast"]
    Fast,
    #[name = "Medium"]
    Medium,
    #[name = "Slow"]
    Slow,
}

impl Default for OasisPunchParams {
    fn default() -> Self {
        Self {
            editor_state: oasis_ui::default_editor_size(),

            attack: FloatParam::new(
                "Attack",
                PUNCH_ATTACK_DEFAULT,
                FloatRange::Linear {
                    min: PUNCH_AMOUNT_MIN,
                    max: PUNCH_AMOUNT_MAX,
                },
            )
            .with_smoother(SmoothingStyle::Linear(SMOOTHING_MS_FAST))
            .with_value_to_string(formatting::v2s_bipolar_percent())
            .with_string_to_value(formatting::s2v_bipolar_percent()),

            sustain: FloatParam::new(
                "Sustain",
                PUNCH_SUSTAIN_DEFAULT,
                FloatRange::Linear {
                    min: PUNCH_AMOUNT_MIN,
                    max: PUNCH_AMOUNT_MAX,
                },
            )
            .with_smoother(SmoothingStyle::Linear(SMOOTHING_MS_FAST))
            .with_value_to_string(formatting::v2s_bipolar_percent())
            .with_string_to_value(formatting::s2v_bipolar_percent()),

            speed: EnumParam::new("Speed", Speed::Fast),

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
