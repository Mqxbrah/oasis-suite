use nih_plug::prelude::*;
use oasis_core::constants::*;
use oasis_core::params::formatting;
use oasis_ui::ViziaState;
use std::sync::Arc;

#[derive(Params)]
pub struct OasisDriveParams {
    #[persist = "editor-state"]
    pub editor_state: Arc<ViziaState>,

    #[id = "drive"]
    pub drive: FloatParam,

    #[id = "input_gain"]
    pub input_gain: FloatParam,

    #[id = "tone"]
    pub tone: FloatParam,

    #[id = "algorithm"]
    pub algorithm: EnumParam<Algorithm>,

    #[id = "mix"]
    pub mix: FloatParam,

    #[id = "output_gain"]
    pub output_gain: FloatParam,
}

#[derive(Enum, PartialEq, Clone, Copy)]
pub enum Algorithm {
    #[name = "Tape"]
    Tape,
    #[name = "Tube"]
    Tube,
    #[name = "Transistor"]
    Transistor,
    #[name = "Digital"]
    Digital,
}

impl Default for OasisDriveParams {
    fn default() -> Self {
        Self {
            editor_state: oasis_ui::default_editor_size(),

            drive: FloatParam::new(
                "Drive",
                DRIVE_AMOUNT_DEFAULT,
                FloatRange::Linear {
                    min: DRIVE_AMOUNT_MIN,
                    max: DRIVE_AMOUNT_MAX,
                },
            )
            .with_smoother(SmoothingStyle::Linear(SMOOTHING_MS_FAST))
            .with_value_to_string(formatting::v2s_percentage())
            .with_string_to_value(formatting::s2v_percentage()),

            input_gain: FloatParam::new(
                "Input Gain",
                0.0,
                FloatRange::Linear {
                    min: DRIVE_INPUT_GAIN_MIN_DB,
                    max: DRIVE_INPUT_GAIN_MAX_DB,
                },
            )
            .with_smoother(SmoothingStyle::Linear(SMOOTHING_MS_FAST))
            .with_value_to_string(formatting::v2s_db())
            .with_string_to_value(formatting::s2v_db()),

            tone: FloatParam::new(
                "Tone",
                DRIVE_TONE_DEFAULT_HZ,
                FloatRange::Skewed {
                    min: DRIVE_TONE_MIN_HZ,
                    max: DRIVE_TONE_MAX_HZ,
                    factor: FloatRange::skew_factor(-2.0),
                },
            )
            .with_smoother(SmoothingStyle::Linear(SMOOTHING_MS_FAST))
            .with_value_to_string(formatting::v2s_hz())
            .with_string_to_value(formatting::s2v_hz()),

            algorithm: EnumParam::new("Algorithm", Algorithm::Tape),

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
