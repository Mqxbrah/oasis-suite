use nih_plug::prelude::*;
use oasis_core::constants::*;
use oasis_core::params::formatting;
use oasis_ui::ViziaState;
use std::sync::Arc;

#[derive(Enum, PartialEq, Clone, Copy)]
pub enum ReverbAlgorithm {
    #[name = "Room"]
    Room,
    #[name = "Hall"]
    Hall,
    #[name = "Plate"]
    Plate,
    #[name = "Chamber"]
    Chamber,
    #[name = "Shimmer"]
    Shimmer,
    #[name = "Spring"]
    Spring,
}

#[derive(Params)]
pub struct OasisVerbParams {
    #[persist = "editor-state"]
    pub editor_state: Arc<ViziaState>,

    #[id = "size"]
    pub size: FloatParam,

    #[id = "predelay"]
    pub predelay_ms: FloatParam,

    #[id = "damping"]
    pub damping: FloatParam,

    #[id = "width"]
    pub width: FloatParam,

    #[id = "diffusion"]
    pub diffusion: FloatParam,

    #[id = "low_cut"]
    pub low_cut_hz: FloatParam,

    #[id = "high_cut"]
    pub high_cut_hz: FloatParam,

    #[id = "modulation"]
    pub modulation: FloatParam,

    #[id = "algorithm"]
    pub algorithm: EnumParam<ReverbAlgorithm>,

    #[id = "mix"]
    pub mix: FloatParam,

    #[id = "output_gain"]
    pub output_gain: FloatParam,
}

impl Default for OasisVerbParams {
    fn default() -> Self {
        Self {
            editor_state: oasis_ui::default_editor_size(),

            size: FloatParam::new(
                "Size",
                VERB_SIZE_DEFAULT,
                FloatRange::Linear {
                    min: VERB_SIZE_MIN,
                    max: VERB_SIZE_MAX,
                },
            )
            .with_smoother(SmoothingStyle::Linear(SMOOTHING_MS_FAST))
            .with_value_to_string(formatting::v2s_percentage())
            .with_string_to_value(formatting::s2v_percentage()),

            predelay_ms: FloatParam::new(
                "Pre-Delay",
                VERB_PREDELAY_DEFAULT_MS,
                FloatRange::Skewed {
                    min: VERB_PREDELAY_MIN_MS,
                    max: VERB_PREDELAY_MAX_MS,
                    factor: FloatRange::skew_factor(-1.0),
                },
            )
            .with_smoother(SmoothingStyle::Linear(SMOOTHING_MS_FAST))
            .with_value_to_string(formatting::v2s_ms())
            .with_string_to_value(formatting::s2v_ms()),

            damping: FloatParam::new(
                "Damping",
                VERB_DAMPING_DEFAULT,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            )
            .with_smoother(SmoothingStyle::Linear(SMOOTHING_MS_FAST))
            .with_value_to_string(formatting::v2s_percentage())
            .with_string_to_value(formatting::s2v_percentage()),

            width: FloatParam::new(
                "Width",
                VERB_WIDTH_DEFAULT,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            )
            .with_smoother(SmoothingStyle::Linear(SMOOTHING_MS_FAST))
            .with_value_to_string(formatting::v2s_percentage())
            .with_string_to_value(formatting::s2v_percentage()),

            diffusion: FloatParam::new(
                "Diffusion",
                VERB_DIFFUSION_DEFAULT,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            )
            .with_smoother(SmoothingStyle::Linear(SMOOTHING_MS_FAST))
            .with_value_to_string(formatting::v2s_percentage())
            .with_string_to_value(formatting::s2v_percentage()),

            low_cut_hz: FloatParam::new(
                "Low Cut",
                VERB_LOWCUT_DEFAULT_HZ,
                FloatRange::Skewed {
                    min: MIN_FREQ_HZ,
                    max: 2000.0,
                    factor: FloatRange::skew_factor(-2.0),
                },
            )
            .with_smoother(SmoothingStyle::Logarithmic(SMOOTHING_MS_NORMAL))
            .with_value_to_string(formatting::v2s_hz())
            .with_string_to_value(formatting::s2v_hz()),

            high_cut_hz: FloatParam::new(
                "High Cut",
                VERB_HIGHCUT_DEFAULT_HZ,
                FloatRange::Skewed {
                    min: 1000.0,
                    max: MAX_FREQ_HZ,
                    factor: FloatRange::skew_factor(-2.0),
                },
            )
            .with_smoother(SmoothingStyle::Logarithmic(SMOOTHING_MS_NORMAL))
            .with_value_to_string(formatting::v2s_hz())
            .with_string_to_value(formatting::s2v_hz()),

            modulation: FloatParam::new(
                "Modulation",
                VERB_MOD_DEFAULT,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            )
            .with_smoother(SmoothingStyle::Linear(SMOOTHING_MS_FAST))
            .with_value_to_string(formatting::v2s_percentage())
            .with_string_to_value(formatting::s2v_percentage()),

            algorithm: EnumParam::new("Algorithm", ReverbAlgorithm::Hall),

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
