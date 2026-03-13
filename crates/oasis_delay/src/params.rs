use nih_plug::prelude::*;
use oasis_core::constants::*;
use oasis_core::params::formatting;
use oasis_ui::ViziaState;
use std::sync::Arc;

#[derive(Params)]
pub struct OasisDelayParams {
    #[persist = "editor-state"]
    pub editor_state: Arc<ViziaState>,

    #[id = "delay_l"]
    pub delay_left_ms: FloatParam,

    #[id = "delay_r"]
    pub delay_right_ms: FloatParam,

    #[id = "feedback"]
    pub feedback: FloatParam,

    #[id = "ping_pong"]
    pub ping_pong: BoolParam,

    #[id = "lp_freq"]
    pub lp_freq: FloatParam,

    #[id = "hp_freq"]
    pub hp_freq: FloatParam,

    #[id = "saturation"]
    pub saturation: FloatParam,

    #[id = "mod_rate"]
    pub mod_rate: FloatParam,

    #[id = "mod_depth"]
    pub mod_depth: FloatParam,

    #[id = "mix"]
    pub mix: FloatParam,

    #[id = "output_gain"]
    pub output_gain: FloatParam,
}

impl Default for OasisDelayParams {
    fn default() -> Self {
        Self {
            editor_state: oasis_ui::default_editor_size(),

            delay_left_ms: FloatParam::new(
                "Delay L",
                DELAY_TIME_DEFAULT_MS,
                FloatRange::Skewed {
                    min: DELAY_TIME_MIN_MS,
                    max: DELAY_TIME_MAX_MS,
                    factor: FloatRange::skew_factor(-1.0),
                },
            )
            .with_smoother(SmoothingStyle::Linear(SMOOTHING_MS_FAST))
            .with_value_to_string(formatting::v2s_ms())
            .with_string_to_value(formatting::s2v_ms()),

            delay_right_ms: FloatParam::new(
                "Delay R",
                DELAY_TIME_DEFAULT_MS,
                FloatRange::Skewed {
                    min: DELAY_TIME_MIN_MS,
                    max: DELAY_TIME_MAX_MS,
                    factor: FloatRange::skew_factor(-1.0),
                },
            )
            .with_smoother(SmoothingStyle::Linear(SMOOTHING_MS_FAST))
            .with_value_to_string(formatting::v2s_ms())
            .with_string_to_value(formatting::s2v_ms()),

            feedback: FloatParam::new(
                "Feedback",
                DELAY_FEEDBACK_DEFAULT,
                FloatRange::Linear {
                    min: DELAY_FEEDBACK_MIN,
                    max: DELAY_FEEDBACK_MAX,
                },
            )
            .with_smoother(SmoothingStyle::Linear(SMOOTHING_MS_FAST))
            .with_value_to_string(formatting::v2s_percentage())
            .with_string_to_value(formatting::s2v_percentage()),

            ping_pong: BoolParam::new("Ping-Pong", false),

            lp_freq: FloatParam::new(
                "LP Freq",
                DELAY_LP_DEFAULT_HZ,
                FloatRange::Skewed {
                    min: 200.0,
                    max: MAX_FREQ_HZ,
                    factor: FloatRange::skew_factor(-2.0),
                },
            )
            .with_smoother(SmoothingStyle::Logarithmic(SMOOTHING_MS_NORMAL))
            .with_value_to_string(formatting::v2s_hz())
            .with_string_to_value(formatting::s2v_hz()),

            hp_freq: FloatParam::new(
                "HP Freq",
                DELAY_HP_DEFAULT_HZ,
                FloatRange::Skewed {
                    min: MIN_FREQ_HZ,
                    max: 2000.0,
                    factor: FloatRange::skew_factor(-2.0),
                },
            )
            .with_smoother(SmoothingStyle::Logarithmic(SMOOTHING_MS_NORMAL))
            .with_value_to_string(formatting::v2s_hz())
            .with_string_to_value(formatting::s2v_hz()),

            saturation: FloatParam::new(
                "Saturation",
                DELAY_SATURATION_DEFAULT,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            )
            .with_smoother(SmoothingStyle::Linear(SMOOTHING_MS_FAST))
            .with_value_to_string(formatting::v2s_percentage())
            .with_string_to_value(formatting::s2v_percentage()),

            mod_rate: FloatParam::new(
                "Mod Rate",
                DELAY_MOD_RATE_DEFAULT_HZ,
                FloatRange::Skewed {
                    min: DELAY_MOD_RATE_MIN_HZ,
                    max: DELAY_MOD_RATE_MAX_HZ,
                    factor: FloatRange::skew_factor(-1.0),
                },
            )
            .with_smoother(SmoothingStyle::Linear(SMOOTHING_MS_FAST))
            .with_value_to_string(formatting::v2s_hz())
            .with_string_to_value(formatting::s2v_hz()),

            mod_depth: FloatParam::new(
                "Mod Depth",
                DELAY_MOD_DEPTH_DEFAULT,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            )
            .with_smoother(SmoothingStyle::Linear(SMOOTHING_MS_FAST))
            .with_value_to_string(formatting::v2s_percentage())
            .with_string_to_value(formatting::s2v_percentage()),

            mix: FloatParam::new(
                "Mix",
                0.3,
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
