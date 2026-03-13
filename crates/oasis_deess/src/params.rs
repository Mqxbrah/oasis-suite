use nih_plug::prelude::*;
use oasis_core::constants::*;
use oasis_core::params::formatting;
use oasis_ui::ViziaState;
use std::sync::Arc;

#[derive(Enum, PartialEq, Clone, Copy)]
pub enum DeEssMode {
    #[name = "Split"]
    Split,
    #[name = "Wideband"]
    Wideband,
}

#[derive(Params)]
pub struct OasisDeEssParams {
    #[persist = "editor-state"]
    pub editor_state: Arc<ViziaState>,

    #[id = "frequency"]
    pub frequency: FloatParam,

    #[id = "bandwidth"]
    pub bandwidth: FloatParam,

    #[id = "threshold"]
    pub threshold_db: FloatParam,

    #[id = "range"]
    pub range_db: FloatParam,

    #[id = "mode"]
    pub mode: EnumParam<DeEssMode>,

    #[id = "listen"]
    pub listen: BoolParam,

    #[id = "mix"]
    pub mix: FloatParam,

    #[id = "output_gain"]
    pub output_gain: FloatParam,
}

impl Default for OasisDeEssParams {
    fn default() -> Self {
        Self {
            editor_state: oasis_ui::default_editor_size(),

            frequency: FloatParam::new(
                "Frequency",
                DEESS_FREQ_DEFAULT_HZ,
                FloatRange::Skewed {
                    min: DEESS_FREQ_MIN_HZ,
                    max: DEESS_FREQ_MAX_HZ,
                    factor: FloatRange::skew_factor(-2.0),
                },
            )
            .with_smoother(SmoothingStyle::Logarithmic(SMOOTHING_MS_NORMAL))
            .with_value_to_string(formatting::v2s_hz())
            .with_string_to_value(formatting::s2v_hz()),

            bandwidth: FloatParam::new(
                "Bandwidth",
                DEESS_Q_DEFAULT,
                FloatRange::Linear {
                    min: DEESS_Q_MIN,
                    max: DEESS_Q_MAX,
                },
            )
            .with_smoother(SmoothingStyle::Linear(SMOOTHING_MS_FAST))
            .with_value_to_string(Arc::new(|v| format!("{:.1}", v)))
            .with_string_to_value(Arc::new(|s| s.parse().ok())),

            threshold_db: FloatParam::new(
                "Threshold",
                DEESS_THRESHOLD_DEFAULT_DB,
                FloatRange::Linear {
                    min: DEESS_THRESHOLD_MIN_DB,
                    max: DEESS_THRESHOLD_MAX_DB,
                },
            )
            .with_smoother(SmoothingStyle::Linear(SMOOTHING_MS_FAST))
            .with_value_to_string(formatting::v2s_db())
            .with_string_to_value(formatting::s2v_db()),

            range_db: FloatParam::new(
                "Range",
                DEESS_RANGE_DEFAULT_DB,
                FloatRange::Linear {
                    min: DEESS_RANGE_MIN_DB,
                    max: DEESS_RANGE_MAX_DB,
                },
            )
            .with_smoother(SmoothingStyle::Linear(SMOOTHING_MS_FAST))
            .with_value_to_string(formatting::v2s_db())
            .with_string_to_value(formatting::s2v_db()),

            mode: EnumParam::new("Mode", DeEssMode::Split),

            listen: BoolParam::new("Listen", false),

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
