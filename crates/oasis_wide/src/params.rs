use nih_plug::prelude::*;
use oasis_core::constants::*;
use oasis_core::params::formatting;
use oasis_ui::ViziaState;
use std::sync::Arc;

#[derive(Params)]
pub struct OasisWideParams {
    #[persist = "editor-state"]
    pub editor_state: Arc<ViziaState>,

    #[id = "width"]
    pub width: FloatParam,

    #[id = "mid_gain"]
    pub mid_gain: FloatParam,

    #[id = "side_gain"]
    pub side_gain: FloatParam,

    #[id = "haas_delay"]
    pub haas_delay_ms: FloatParam,

    #[id = "haas_channel"]
    pub haas_channel: EnumParam<HaasChannel>,

    #[id = "bass_mono_on"]
    pub bass_mono_enabled: BoolParam,

    #[id = "bass_mono_freq"]
    pub bass_mono_freq: FloatParam,

    #[id = "mix"]
    pub mix: FloatParam,

    #[id = "output_gain"]
    pub output_gain: FloatParam,
}

#[derive(Enum, PartialEq, Clone, Copy)]
pub enum HaasChannel {
    #[name = "Right"]
    Right,
    #[name = "Left"]
    Left,
}

impl Default for OasisWideParams {
    fn default() -> Self {
        Self {
            editor_state: oasis_ui::default_editor_size(),

            width: FloatParam::new(
                "Width",
                WIDTH_DEFAULT,
                FloatRange::Linear {
                    min: WIDTH_MIN,
                    max: WIDTH_MAX,
                },
            )
            .with_smoother(SmoothingStyle::Linear(SMOOTHING_MS_FAST))
            .with_value_to_string(formatting::v2s_width())
            .with_string_to_value(formatting::s2v_width()),

            mid_gain: FloatParam::new(
                "Mid Gain",
                0.0,
                FloatRange::Linear {
                    min: GAIN_MIN_DB,
                    max: GAIN_MAX_DB,
                },
            )
            .with_smoother(SmoothingStyle::Linear(SMOOTHING_MS_FAST))
            .with_value_to_string(formatting::v2s_db())
            .with_string_to_value(formatting::s2v_db()),

            side_gain: FloatParam::new(
                "Side Gain",
                0.0,
                FloatRange::Linear {
                    min: GAIN_MIN_DB,
                    max: GAIN_MAX_DB,
                },
            )
            .with_smoother(SmoothingStyle::Linear(SMOOTHING_MS_FAST))
            .with_value_to_string(formatting::v2s_db())
            .with_string_to_value(formatting::s2v_db()),

            haas_delay_ms: FloatParam::new(
                "Haas Delay",
                0.0,
                FloatRange::Skewed {
                    min: 0.0,
                    max: HAAS_MAX_DELAY_MS,
                    factor: FloatRange::skew_factor(-1.0),
                },
            )
            .with_smoother(SmoothingStyle::Linear(SMOOTHING_MS_FAST))
            .with_value_to_string(formatting::v2s_ms())
            .with_string_to_value(formatting::s2v_ms()),

            haas_channel: EnumParam::new("Haas Channel", HaasChannel::Right),

            bass_mono_enabled: BoolParam::new("Bass Mono", false),

            bass_mono_freq: FloatParam::new(
                "Bass Mono Freq",
                BASS_MONO_DEFAULT_FREQ,
                FloatRange::Skewed {
                    min: BASS_MONO_MIN_FREQ,
                    max: BASS_MONO_MAX_FREQ,
                    factor: FloatRange::skew_factor(-2.0),
                },
            )
            .with_smoother(SmoothingStyle::Logarithmic(SMOOTHING_MS_NORMAL))
            .with_value_to_string(formatting::v2s_hz())
            .with_string_to_value(formatting::s2v_hz()),

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
