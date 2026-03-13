#![allow(dead_code)]
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct FactoryPreset {
    pub name: &'static str,
    pub category: &'static str,
    /// (param_id, normalized_value)
    pub values: &'static [(&'static str, f32)],
}

// Normalization reference (all linear ranges):
//   semitones: -24..24  → norm = (val + 24) / 48
//   cents:     -100..100 → norm = (val + 100) / 200
//   detune:    0..50    → norm = val / 50
//   mode:      Simple=0.0, Detune=1.0
//   mix:       0..1     → norm = val
//   output_gain: -12..12 → norm = (val + 12) / 24

pub const FACTORY_PRESETS: &[FactoryPreset] = &[
    FactoryPreset {
        name: "Init",
        category: "Default",
        values: &[
            ("semitones", 0.5),      // 0 st
            ("cents", 0.5),          // 0 ct
            ("detune", 0.0),         // 0 ct
            ("mode", 0.0),           // Simple
            ("mix", 1.0),            // 100%
            ("output_gain", 0.5),    // 0.0 dB
        ],
    },
    FactoryPreset {
        name: "Octave Down",
        category: "Creative",
        values: &[
            ("semitones", 0.25),     // -12 st
            ("cents", 0.5),          // 0 ct
            ("detune", 0.0),         // 0 ct
            ("mode", 0.0),           // Simple
            ("mix", 1.0),            // 100%
            ("output_gain", 0.5),    // 0.0 dB
        ],
    },
    FactoryPreset {
        name: "Fifth Up",
        category: "Creative",
        values: &[
            ("semitones", 0.6458),   // +7 st
            ("cents", 0.5),          // 0 ct
            ("detune", 0.0),         // 0 ct
            ("mode", 0.0),           // Simple
            ("mix", 0.5),            // 50%
            ("output_gain", 0.5),    // 0.0 dB
        ],
    },
    FactoryPreset {
        name: "Subtle Detune",
        category: "Mixing",
        values: &[
            ("semitones", 0.5),      // 0 st
            ("cents", 0.5),          // 0 ct
            ("detune", 0.16),        // 8 ct
            ("mode", 1.0),           // Detune
            ("mix", 1.0),            // 100%
            ("output_gain", 0.5),    // 0.0 dB
        ],
    },
    FactoryPreset {
        name: "ADT Thicken",
        category: "Mixing",
        values: &[
            ("semitones", 0.5),      // 0 st
            ("cents", 0.5),          // 0 ct
            ("detune", 0.3),         // 15 ct
            ("mode", 1.0),           // Detune
            ("mix", 0.5),            // 50%
            ("output_gain", 0.5),    // 0.0 dB
        ],
    },
    FactoryPreset {
        name: "Harmony Third",
        category: "Creative",
        values: &[
            ("semitones", 0.5833),   // +4 st
            ("cents", 0.5),          // 0 ct
            ("detune", 0.0),         // 0 ct
            ("mode", 0.0),           // Simple
            ("mix", 0.5),            // 50%
            ("output_gain", 0.5),    // 0.0 dB
        ],
    },
    FactoryPreset {
        name: "Chipmunk",
        category: "Creative",
        values: &[
            ("semitones", 0.75),     // +12 st
            ("cents", 0.5),          // 0 ct
            ("detune", 0.0),         // 0 ct
            ("mode", 0.0),           // Simple
            ("mix", 1.0),            // 100%
            ("output_gain", 0.5),    // 0.0 dB
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
