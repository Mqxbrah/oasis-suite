use nih_plug::prelude::*;
use oasis_core::constants::*;
use oasis_core::params::formatting;
use oasis_ui::ViziaState;
use std::sync::Arc;

#[derive(Params)]
pub struct OasisLimitParams {
    #[persist = "editor-state"]
    pub editor_state: Arc<ViziaState>,

    #[id = "ceiling"]
    pub ceiling_db: FloatParam,

    #[id = "input_gain"]
    pub input_gain_db: FloatParam,

    #[id = "release"]
    pub release_ms: FloatParam,

    #[id = "mode"]
    pub mode: EnumParam<LimiterMode>,

    #[id = "mix"]
    pub mix: FloatParam,

    #[id = "output_gain"]
    pub output_gain: FloatParam,
}

#[derive(Enum, PartialEq, Clone, Copy, Debug)]
pub enum LimiterMode {
    #[name = "Transparent"]
    Transparent,
    #[name = "Aggressive"]
    Aggressive,
    #[name = "Mastering"]
    Mastering,
}

impl Default for OasisLimitParams {
    fn default() -> Self {
        Self {
            editor_state: oasis_ui::default_editor_size(),

            ceiling_db: FloatParam::new(
                "Ceiling",
                LIMIT_CEILING_DEFAULT_DB,
                FloatRange::Linear {
                    min: LIMIT_CEILING_MIN_DB,
                    max: LIMIT_CEILING_MAX_DB,
                },
            )
            .with_smoother(SmoothingStyle::Linear(SMOOTHING_MS_FAST))
            .with_value_to_string(formatting::v2s_db())
            .with_string_to_value(formatting::s2v_db()),

            input_gain_db: FloatParam::new(
                "Input Gain",
                LIMIT_INPUT_GAIN_DEFAULT_DB,
                FloatRange::Linear {
                    min: LIMIT_INPUT_GAIN_MIN_DB,
                    max: LIMIT_INPUT_GAIN_MAX_DB,
                },
            )
            .with_smoother(SmoothingStyle::Linear(SMOOTHING_MS_FAST))
            .with_value_to_string(formatting::v2s_db())
            .with_string_to_value(formatting::s2v_db()),

            release_ms: FloatParam::new(
                "Release",
                LIMIT_RELEASE_DEFAULT_MS,
                FloatRange::Skewed {
                    min: LIMIT_RELEASE_MIN_MS,
                    max: LIMIT_RELEASE_MAX_MS,
                    factor: FloatRange::skew_factor(-1.0),
                },
            )
            .with_smoother(SmoothingStyle::Linear(SMOOTHING_MS_FAST))
            .with_value_to_string(formatting::v2s_ms())
            .with_string_to_value(formatting::s2v_ms()),

            mode: EnumParam::new("Mode", LimiterMode::Transparent),

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
