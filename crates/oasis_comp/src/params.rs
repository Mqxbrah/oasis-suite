use nih_plug::prelude::*;
use oasis_core::constants::*;
use oasis_core::params::formatting;
use oasis_ui::ViziaState;
use std::sync::Arc;

#[derive(Enum, PartialEq, Clone, Copy)]
pub enum DetectionMode {
    #[name = "Peak"]
    Peak,
    #[name = "RMS"]
    Rms,
}

#[derive(Params)]
pub struct OasisCompParams {
    #[persist = "editor-state"]
    pub editor_state: Arc<ViziaState>,

    #[id = "threshold"]
    pub threshold: FloatParam,

    #[id = "ratio"]
    pub ratio: FloatParam,

    #[id = "attack"]
    pub attack_ms: FloatParam,

    #[id = "release"]
    pub release_ms: FloatParam,

    #[id = "knee"]
    pub knee_db: FloatParam,

    #[id = "makeup"]
    pub makeup_db: FloatParam,

    #[id = "mix"]
    pub mix: FloatParam,

    #[id = "detection"]
    pub detection_mode: EnumParam<DetectionMode>,

    #[id = "sc_hp_freq"]
    pub sc_hp_freq: FloatParam,

    #[id = "output_gain"]
    pub output_gain: FloatParam,
}

impl Default for OasisCompParams {
    fn default() -> Self {
        Self {
            editor_state: oasis_ui::default_editor_size(),

            threshold: FloatParam::new(
                "Threshold",
                COMP_THRESHOLD_DEFAULT_DB,
                FloatRange::Linear {
                    min: COMP_THRESHOLD_MIN_DB,
                    max: COMP_THRESHOLD_MAX_DB,
                },
            )
            .with_smoother(SmoothingStyle::Linear(SMOOTHING_MS_FAST))
            .with_value_to_string(formatting::v2s_db())
            .with_string_to_value(formatting::s2v_db()),

            ratio: FloatParam::new(
                "Ratio",
                COMP_RATIO_DEFAULT,
                FloatRange::Skewed {
                    min: COMP_RATIO_MIN,
                    max: COMP_RATIO_MAX,
                    factor: FloatRange::skew_factor(-1.0),
                },
            )
            .with_smoother(SmoothingStyle::Linear(SMOOTHING_MS_FAST))
            .with_value_to_string(formatting::v2s_ratio())
            .with_string_to_value(formatting::s2v_ratio()),

            attack_ms: FloatParam::new(
                "Attack",
                COMP_ATTACK_DEFAULT_MS,
                FloatRange::Skewed {
                    min: COMP_ATTACK_MIN_MS,
                    max: COMP_ATTACK_MAX_MS,
                    factor: FloatRange::skew_factor(-1.0),
                },
            )
            .with_smoother(SmoothingStyle::Linear(SMOOTHING_MS_FAST))
            .with_value_to_string(formatting::v2s_ms())
            .with_string_to_value(formatting::s2v_ms()),

            release_ms: FloatParam::new(
                "Release",
                COMP_RELEASE_DEFAULT_MS,
                FloatRange::Skewed {
                    min: COMP_RELEASE_MIN_MS,
                    max: COMP_RELEASE_MAX_MS,
                    factor: FloatRange::skew_factor(-1.0),
                },
            )
            .with_smoother(SmoothingStyle::Linear(SMOOTHING_MS_FAST))
            .with_value_to_string(formatting::v2s_ms())
            .with_string_to_value(formatting::s2v_ms()),

            knee_db: FloatParam::new(
                "Knee",
                COMP_KNEE_DEFAULT_DB,
                FloatRange::Linear {
                    min: COMP_KNEE_MIN_DB,
                    max: COMP_KNEE_MAX_DB,
                },
            )
            .with_smoother(SmoothingStyle::Linear(SMOOTHING_MS_FAST))
            .with_value_to_string(formatting::v2s_db())
            .with_string_to_value(formatting::s2v_db()),

            makeup_db: FloatParam::new(
                "Makeup",
                COMP_MAKEUP_DEFAULT_DB,
                FloatRange::Linear {
                    min: COMP_MAKEUP_MIN_DB,
                    max: COMP_MAKEUP_MAX_DB,
                },
            )
            .with_smoother(SmoothingStyle::Linear(SMOOTHING_MS_FAST))
            .with_value_to_string(formatting::v2s_db())
            .with_string_to_value(formatting::s2v_db()),

            mix: FloatParam::new(
                "Mix",
                1.0,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            )
            .with_smoother(SmoothingStyle::Linear(SMOOTHING_MS_FAST))
            .with_value_to_string(formatting::v2s_percentage())
            .with_string_to_value(formatting::s2v_percentage()),

            detection_mode: EnumParam::new("Detection", DetectionMode::Peak),

            sc_hp_freq: FloatParam::new(
                "SC HP Freq",
                COMP_SC_HP_DEFAULT_HZ,
                FloatRange::Skewed {
                    min: COMP_SC_HP_MIN_HZ,
                    max: COMP_SC_HP_MAX_HZ,
                    factor: FloatRange::skew_factor(-2.0),
                },
            )
            .with_smoother(SmoothingStyle::Linear(SMOOTHING_MS_FAST))
            .with_value_to_string(formatting::v2s_hz())
            .with_string_to_value(formatting::s2v_hz()),

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
