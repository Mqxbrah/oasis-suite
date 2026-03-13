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
            ("ceiling", 0.975),      // -0.3 dB, range -12..0
            ("input_gain", 0.0),     // 0.0 dB, range 0..24
            ("release", 0.293),      // 50 ms (skewed -1.0, range 1..500)
            ("mode", 0.0),           // Transparent
            ("mix", 1.0),            // 100%
            ("output_gain", 0.5),    // 0.0 dB
        ],
    },
    FactoryPreset {
        name: "Mastering Transparent",
        category: "Mastering",
        values: &[
            ("ceiling", 0.975),      // -0.3 dB
            ("input_gain", 0.0),     // 0.0 dB
            ("release", 0.293),      // 50 ms
            ("mode", 0.5),           // Mastering (enum index 2 / 3 variants = normalized ~0.5)
            ("mix", 1.0),            // 100%
            ("output_gain", 0.5),    // 0.0 dB
        ],
    },
    FactoryPreset {
        name: "Loud Master",
        category: "Mastering",
        values: &[
            ("ceiling", 0.992),      // -0.1 dB
            ("input_gain", 0.25),    // 6.0 dB
            ("release", 0.293),      // 50 ms
            ("mode", 0.5),           // Mastering
            ("mix", 1.0),            // 100%
            ("output_gain", 0.5),    // 0.0 dB
        ],
    },
    FactoryPreset {
        name: "EDM Pump",
        category: "Creative",
        values: &[
            ("ceiling", 0.917),      // -1.0 dB
            ("input_gain", 0.333),   // 8.0 dB
            ("release", 0.126),      // 10 ms (fast release, skewed)
            ("mode", 0.25),          // Aggressive (enum index 1)
            ("mix", 1.0),            // 100%
            ("output_gain", 0.5),    // 0.0 dB
        ],
    },
    FactoryPreset {
        name: "Gentle Ceiling",
        category: "Mixing",
        values: &[
            ("ceiling", 0.917),      // -1.0 dB
            ("input_gain", 0.083),   // 2.0 dB
            ("release", 0.413),      // 100 ms
            ("mode", 0.0),           // Transparent
            ("mix", 0.8),            // 80%
            ("output_gain", 0.5),    // 0.0 dB
        ],
    },
    FactoryPreset {
        name: "Broadcast",
        category: "Mastering",
        values: &[
            ("ceiling", 0.917),      // -1.0 dB
            ("input_gain", 0.167),   // 4.0 dB
            ("release", 0.413),      // 100 ms
            ("mode", 0.5),           // Mastering
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
