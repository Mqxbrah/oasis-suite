#![allow(dead_code)]
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct FactoryPreset {
    pub name: &'static str,
    pub category: &'static str,
    /// (param_id, normalized_value)
    pub values: &'static [(&'static str, f32)],
}

// Normalized value reference:
//   depth:       Linear 0.0..1.0, normalized = value
//   rate:        Skewed 0.25..32.0 Hz (factor=0.5), normalized = ((val-0.25)/31.75).sqrt()
//   smoothing:   Linear 0.0..1.0, normalized = value
//   shape:       Enum 4 variants: Sine=0.0, Saw=0.333, Square=0.667, Exponential=1.0
//   mix:         Linear 0.0..1.0, normalized = value
//   output_gain: Linear -12..12 dB, normalized = (val+12)/24

pub const FACTORY_PRESETS: &[FactoryPreset] = &[
    FactoryPreset {
        name: "Init",
        category: "Default",
        values: &[
            ("depth", 0.5),
            ("rate", 0.344),       // 4.0 Hz
            ("smoothing", 0.3),
            ("shape", 0.0),        // Sine
            ("mix", 1.0),
            ("output_gain", 0.5),  // 0.0 dB
        ],
    },
    FactoryPreset {
        name: "Sidechain Pump",
        category: "Creative",
        values: &[
            ("depth", 0.85),
            ("rate", 0.344),       // 4.0 Hz
            ("smoothing", 0.15),
            ("shape", 1.0),        // Exponential
            ("mix", 1.0),
            ("output_gain", 0.5),
        ],
    },
    FactoryPreset {
        name: "Subtle Pulse",
        category: "Mixing",
        values: &[
            ("depth", 0.25),
            ("rate", 0.235),       // 2.0 Hz
            ("smoothing", 0.6),
            ("shape", 0.0),        // Sine
            ("mix", 0.7),
            ("output_gain", 0.5),
        ],
    },
    FactoryPreset {
        name: "Trance Gate",
        category: "Creative",
        values: &[
            ("depth", 1.0),
            ("rate", 0.494),       // 8.0 Hz
            ("smoothing", 0.0),
            ("shape", 0.667),      // Square
            ("mix", 1.0),
            ("output_gain", 0.5),
        ],
    },
    FactoryPreset {
        name: "Half-Time Swell",
        category: "Creative",
        values: &[
            ("depth", 0.6),
            ("rate", 0.154),       // 1.0 Hz
            ("smoothing", 0.7),
            ("shape", 0.0),        // Sine
            ("mix", 1.0),
            ("output_gain", 0.5),
        ],
    },
    FactoryPreset {
        name: "Choppy Eighth",
        category: "Creative",
        values: &[
            ("depth", 0.75),
            ("rate", 0.494),       // 8.0 Hz
            ("smoothing", 0.1),
            ("shape", 0.333),      // Saw
            ("mix", 1.0),
            ("output_gain", 0.5),
        ],
    },
];

pub static CURRENT_PRESET_INDEX: AtomicUsize = AtomicUsize::new(0);

pub fn current_preset_name() -> &'static str {
    let idx = CURRENT_PRESET_INDEX.load(Ordering::Relaxed);
    if idx < FACTORY_PRESETS.len() {
        FACTORY_PRESETS[idx].name
    } else {
        "Custom"
    }
}

pub fn preset_count() -> usize {
    FACTORY_PRESETS.len()
}

pub fn next_preset() -> usize {
    let count = FACTORY_PRESETS.len();
    CURRENT_PRESET_INDEX
        .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |idx| {
            Some((idx + 1) % count)
        })
        .unwrap_or(0)
}

pub fn prev_preset() -> usize {
    let count = FACTORY_PRESETS.len();
    CURRENT_PRESET_INDEX
        .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |idx| {
            Some(if idx == 0 { count - 1 } else { idx - 1 })
        })
        .unwrap_or(0)
}
