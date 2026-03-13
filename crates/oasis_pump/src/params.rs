use nih_plug::prelude::*;
use oasis_core::constants::*;
use oasis_core::params::formatting;
use oasis_ui::ViziaState;
use std::sync::Arc;

#[derive(Enum, PartialEq, Clone, Copy)]
pub enum PumpShape {
    #[name = "Sine"]
    Sine,
    #[name = "Saw"]
    Saw,
    #[name = "Square"]
    Square,
    #[name = "Exponential"]
    Exponential,
}

#[derive(Params)]
pub struct OasisPumpParams {
    #[persist = "editor-state"]
    pub editor_state: Arc<ViziaState>,

    #[id = "depth"]
    pub depth: FloatParam,

    #[id = "rate"]
    pub rate_hz: FloatParam,

    #[id = "smoothing"]
    pub smoothing: FloatParam,

    #[id = "shape"]
    pub shape: EnumParam<PumpShape>,

    #[id = "mix"]
    pub mix: FloatParam,

    #[id = "output_gain"]
    pub output_gain: FloatParam,
}

impl Default for OasisPumpParams {
    fn default() -> Self {
        Self {
            editor_state: oasis_ui::default_editor_size(),

            depth: FloatParam::new(
                "Depth",
                PUMP_DEPTH_DEFAULT,
                FloatRange::Linear {
                    min: PUMP_DEPTH_MIN,
                    max: PUMP_DEPTH_MAX,
                },
            )
            .with_smoother(SmoothingStyle::Linear(SMOOTHING_MS_FAST))
            .with_value_to_string(formatting::v2s_percentage())
            .with_string_to_value(formatting::s2v_percentage()),

            rate_hz: FloatParam::new(
                "Rate",
                PUMP_RATE_HZ_DEFAULT,
                FloatRange::Skewed {
                    min: PUMP_RATE_HZ_MIN,
                    max: PUMP_RATE_HZ_MAX,
                    factor: FloatRange::skew_factor(-1.0),
                },
            )
            .with_smoother(SmoothingStyle::Linear(SMOOTHING_MS_FAST))
            .with_value_to_string(formatting::v2s_hz())
            .with_string_to_value(formatting::s2v_hz()),

            smoothing: FloatParam::new(
                "Smoothing",
                PUMP_SMOOTHING_DEFAULT,
                FloatRange::Linear {
                    min: PUMP_SMOOTHING_MIN,
                    max: PUMP_SMOOTHING_MAX,
                },
            )
            .with_smoother(SmoothingStyle::Linear(SMOOTHING_MS_FAST))
            .with_value_to_string(formatting::v2s_percentage())
            .with_string_to_value(formatting::s2v_percentage()),

            shape: EnumParam::new("Shape", PumpShape::Sine),

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
