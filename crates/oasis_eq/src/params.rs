use nih_plug::prelude::*;
use oasis_core::constants::*;
use oasis_core::params::formatting;
use oasis_ui::ViziaState;
use std::sync::Arc;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Enum)]
pub enum FilterType {
    #[name = "Bell"]
    Bell,
    #[name = "Low Shelf"]
    LowShelf,
    #[name = "High Shelf"]
    HighShelf,
    #[name = "Low Cut"]
    LowCut,
    #[name = "High Cut"]
    HighCut,
    #[name = "Notch"]
    Notch,
    #[name = "Bandpass"]
    Bandpass,
}

#[derive(Params)]
pub struct BandParams {
    #[id = "freq"]
    pub freq: FloatParam,

    #[id = "gain"]
    pub gain: FloatParam,

    #[id = "q"]
    pub q: FloatParam,

    #[id = "filter_type"]
    pub filter_type: EnumParam<FilterType>,

    #[id = "enabled"]
    pub enabled: BoolParam,
}

impl BandParams {
    pub fn new(index: usize) -> Self {
        let default_freq = EQ_BAND_DEFAULTS_HZ[index];
        let default_type = match index {
            0 => FilterType::LowCut,
            7 => FilterType::HighCut,
            _ => FilterType::Bell,
        };

        Self {
            freq: FloatParam::new(
                "Freq",
                default_freq,
                FloatRange::Skewed {
                    min: MIN_FREQ_HZ,
                    max: MAX_FREQ_HZ,
                    factor: FloatRange::skew_factor(-2.0),
                },
            )
            .with_smoother(SmoothingStyle::Logarithmic(SMOOTHING_MS_NORMAL))
            .with_value_to_string(formatting::v2s_hz())
            .with_string_to_value(formatting::s2v_hz()),

            gain: FloatParam::new(
                "Gain",
                0.0,
                FloatRange::Linear {
                    min: EQ_GAIN_MIN_DB,
                    max: EQ_GAIN_MAX_DB,
                },
            )
            .with_smoother(SmoothingStyle::Linear(SMOOTHING_MS_FAST))
            .with_value_to_string(formatting::v2s_db())
            .with_string_to_value(formatting::s2v_db()),

            q: FloatParam::new(
                "Q",
                EQ_Q_DEFAULT,
                FloatRange::Skewed {
                    min: EQ_Q_MIN,
                    max: EQ_Q_MAX,
                    factor: FloatRange::skew_factor(-1.0),
                },
            )
            .with_smoother(SmoothingStyle::Linear(SMOOTHING_MS_FAST))
            .with_value_to_string(Arc::new(|v| format!("{:.2}", v)))
            .with_string_to_value(Arc::new(|s| s.trim().parse::<f32>().ok())),

            filter_type: EnumParam::new("Type", default_type),

            enabled: BoolParam::new("Enabled", true),
        }
    }
}

#[derive(Params)]
pub struct OasisEqParams {
    #[persist = "editor-state"]
    pub editor_state: Arc<ViziaState>,

    #[nested(array, group = "Band")]
    pub bands: [BandParams; 8],

    #[id = "output_gain"]
    pub output_gain: FloatParam,
}

impl Default for OasisEqParams {
    fn default() -> Self {
        Self {
            editor_state: oasis_ui::default_editor_size(),

            bands: [
                BandParams::new(0),
                BandParams::new(1),
                BandParams::new(2),
                BandParams::new(3),
                BandParams::new(4),
                BandParams::new(5),
                BandParams::new(6),
                BandParams::new(7),
            ],

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
