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
            ("drive", 0.25),       // 25% drive
            ("input_gain", 0.5),   // 0.0 dB, range -24..+24
            ("tone", 0.755),       // ~8000 Hz (skewed -2.0)
            ("algorithm", 0.0),    // Tape
            ("mix", 1.0),          // 100%
            ("output_gain", 0.5),  // 0.0 dB, range -12..+12
        ],
    },
    FactoryPreset {
        name: "Warm Tape",
        category: "Mixing",
        values: &[
            ("drive", 0.4),        // 40% drive
            ("input_gain", 0.5),   // 0.0 dB
            ("tone", 0.565),       // ~4000 Hz (skewed)
            ("algorithm", 0.0),    // Tape
            ("mix", 0.8),          // 80%
            ("output_gain", 0.5),  // 0.0 dB
        ],
    },
    FactoryPreset {
        name: "Tube Warmth",
        category: "Mixing",
        values: &[
            ("drive", 0.3),        // 30% drive
            ("input_gain", 0.5),   // 0.0 dB
            ("tone", 0.658),       // ~6000 Hz (skewed)
            ("algorithm", 0.333),  // Tube (enum index 1 of 4)
            ("mix", 0.7),          // 70%
            ("output_gain", 0.5),  // 0.0 dB
        ],
    },
    FactoryPreset {
        name: "Transistor Edge",
        category: "Creative",
        values: &[
            ("drive", 0.6),        // 60% drive
            ("input_gain", 0.563), // +3.0 dB
            ("tone", 0.755),       // ~8000 Hz
            ("algorithm", 0.667),  // Transistor (enum index 2 of 4)
            ("mix", 1.0),          // 100%
            ("output_gain", 0.458), // -1.0 dB
        ],
    },
    FactoryPreset {
        name: "Digital Crush",
        category: "Creative",
        values: &[
            ("drive", 0.8),        // 80% drive
            ("input_gain", 0.625), // +6.0 dB
            ("tone", 0.888),       // ~12000 Hz (skewed)
            ("algorithm", 1.0),    // Digital (enum index 3 of 4)
            ("mix", 0.85),         // 85%
            ("output_gain", 0.417), // -2.0 dB
        ],
    },
    FactoryPreset {
        name: "Subtle Grit",
        category: "Mixing",
        values: &[
            ("drive", 0.15),       // 15% drive
            ("input_gain", 0.5),   // 0.0 dB
            ("tone", 0.658),       // ~6000 Hz
            ("algorithm", 0.0),    // Tape
            ("mix", 0.5),          // 50%
            ("output_gain", 0.5),  // 0.0 dB
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
