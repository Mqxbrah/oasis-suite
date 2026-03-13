pub mod formatting;

use std::sync::Arc;
use nih_plug::prelude::*;
use crate::constants::*;

pub fn freq_param(name: &'static str, default_hz: f32) -> FloatParam {
    FloatParam::new(
        name,
        default_hz,
        FloatRange::Skewed {
            min: MIN_FREQ_HZ,
            max: MAX_FREQ_HZ,
            factor: FloatRange::skew_factor(-2.0),
        },
    )
    .with_unit(" Hz")
    .with_smoother(SmoothingStyle::Logarithmic(SMOOTHING_MS_NORMAL))
    .with_value_to_string(formatters::v2s_f32_hz_then_khz(2))
    .with_string_to_value(formatters::s2v_f32_hz_then_khz())
}

pub fn bass_mono_freq_param(name: &'static str, default_hz: f32) -> FloatParam {
    FloatParam::new(
        name,
        default_hz,
        FloatRange::Skewed {
            min: BASS_MONO_MIN_FREQ,
            max: BASS_MONO_MAX_FREQ,
            factor: FloatRange::skew_factor(-2.0),
        },
    )
    .with_unit(" Hz")
    .with_smoother(SmoothingStyle::Logarithmic(SMOOTHING_MS_NORMAL))
    .with_value_to_string(formatters::v2s_f32_hz_then_khz(1))
    .with_string_to_value(formatters::s2v_f32_hz_then_khz())
}

pub fn gain_param(name: &'static str, default_db: f32) -> FloatParam {
    FloatParam::new(
        name,
        default_db,
        FloatRange::Linear {
            min: GAIN_MIN_DB,
            max: GAIN_MAX_DB,
        },
    )
    .with_unit(" dB")
    .with_smoother(SmoothingStyle::Linear(SMOOTHING_MS_FAST))
    .with_value_to_string(formatters::v2s_f32_rounded(1))
    .with_string_to_value(formatters::s2v_f32_percentage())
}

pub fn percent_param(name: &'static str, default: f32) -> FloatParam {
    FloatParam::new(
        name,
        default,
        FloatRange::Linear { min: 0.0, max: 1.0 },
    )
    .with_unit("%")
    .with_smoother(SmoothingStyle::Linear(SMOOTHING_MS_FAST))
    .with_value_to_string(formatting::v2s_percentage())
    .with_string_to_value(formatting::s2v_percentage())
}

pub fn time_ms_param(name: &'static str, default: f32, min: f32, max: f32) -> FloatParam {
    FloatParam::new(
        name,
        default,
        FloatRange::Skewed {
            min,
            max,
            factor: FloatRange::skew_factor(-1.0),
        },
    )
    .with_unit(" ms")
    .with_smoother(SmoothingStyle::Linear(SMOOTHING_MS_FAST))
    .with_value_to_string(Arc::new(|v| format!("{:.1} ms", v)))
    .with_string_to_value(Arc::new(|s| s.trim().trim_end_matches(" ms").parse().ok()))
}
