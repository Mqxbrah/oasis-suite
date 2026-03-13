#![allow(dead_code)]
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct FactoryPreset {
    pub name: &'static str,
    pub category: &'static str,
    /// (param_id, normalized_value)
    pub values: &'static [(&'static str, f32)],
}

pub const FACTORY_PRESETS: &[FactoryPreset] = &[
    FactoryPreset {
        name: "Init",
        category: "Default",
        values: &[
            ("threshold", 0.7),    // -18 dB (linear -60..0)
            ("ratio", 0.397),      // 4:1 (skewed 1..20)
            ("attack", 0.315),     // 10 ms (skewed 0.1..100)
            ("release", 0.309),    // 100 ms (skewed 5..1000)
            ("knee", 0.25),        // 6 dB (linear 0..24)
            ("makeup", 0.0),       // 0 dB (linear 0..30)
            ("mix", 1.0),          // 100%
            ("detection", 0.0),    // Peak
            ("sc_hp_freq", 0.0),   // 20 Hz
            ("output_gain", 0.5),  // 0 dB
        ],
    },
    FactoryPreset {
        name: "Vocal Glue",
        category: "Vocals",
        values: &[
            ("threshold", 0.6),    // -24 dB
            ("ratio", 0.281),      // 2.5:1
            ("attack", 0.386),     // 15 ms
            ("release", 0.382),    // 150 ms
            ("knee", 0.333),       // 8 dB
            ("makeup", 0.1),       // 3 dB
            ("mix", 1.0),          // 100%
            ("detection", 0.0),    // Peak
            ("sc_hp_freq", 0.595), // ~80 Hz
            ("output_gain", 0.5),  // 0 dB
        ],
    },
    FactoryPreset {
        name: "Drum Punch",
        category: "Drums",
        values: &[
            ("threshold", 0.8),    // -12 dB
            ("ratio", 0.513),      // 6:1
            ("attack", 0.095),     // ~1 ms
            ("release", 0.274),    // ~80 ms
            ("knee", 0.125),       // 3 dB
            ("makeup", 0.133),     // ~4 dB
            ("mix", 1.0),          // 100%
            ("detection", 0.0),    // Peak
            ("sc_hp_freq", 0.0),   // 20 Hz
            ("output_gain", 0.5),  // 0 dB
        ],
    },
    FactoryPreset {
        name: "Bus Glue",
        category: "Master Bus",
        values: &[
            ("threshold", 0.667),  // -20 dB
            ("ratio", 0.229),      // 2:1
            ("attack", 0.446),     // ~20 ms
            ("release", 0.443),    // ~200 ms
            ("knee", 0.5),         // 12 dB
            ("makeup", 0.067),     // ~2 dB
            ("mix", 1.0),          // 100%
            ("detection", 1.0),    // RMS
            ("sc_hp_freq", 0.0),   // 20 Hz
            ("output_gain", 0.5),  // 0 dB
        ],
    },
    FactoryPreset {
        name: "Gentle Squeeze",
        category: "Mixing",
        values: &[
            ("threshold", 0.5),    // -30 dB
            ("ratio", 0.162),      // 1.5:1
            ("attack", 0.499),     // ~25 ms
            ("release", 0.496),    // ~250 ms
            ("knee", 0.5),         // 12 dB
            ("makeup", 0.167),     // ~5 dB
            ("mix", 1.0),          // 100%
            ("detection", 1.0),    // RMS
            ("sc_hp_freq", 0.0),   // 20 Hz
            ("output_gain", 0.5),  // 0 dB
        ],
    },
    FactoryPreset {
        name: "Parallel Smash",
        category: "Creative",
        values: &[
            ("threshold", 0.6),    // -24 dB
            ("ratio", 0.688),      // 10:1
            ("attack", 0.063),     // ~0.5 ms
            ("release", 0.235),    // ~60 ms
            ("knee", 0.0),         // 0 dB (hard knee)
            ("makeup", 0.267),     // ~8 dB
            ("mix", 0.5),          // 50%
            ("detection", 0.0),    // Peak
            ("sc_hp_freq", 0.0),   // 20 Hz
            ("output_gain", 0.5),  // 0 dB
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
