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
            ("attack", 0.5),       // 0.0 (center of -1..1)
            ("sustain", 0.5),      // 0.0 (center of -1..1)
            ("speed", 0.0),        // Fast
            ("mix", 1.0),          // 100%
            ("output_gain", 0.5),  // 0.0 dB
        ],
    },
    FactoryPreset {
        name: "Drum Snap",
        category: "Drums",
        values: &[
            ("attack", 0.85),      // strong attack boost
            ("sustain", 0.5),      // neutral sustain
            ("speed", 0.0),        // Fast
            ("mix", 1.0),          // 100%
            ("output_gain", 0.5),  // 0.0 dB
        ],
    },
    FactoryPreset {
        name: "Thicken Sustain",
        category: "Mixing",
        values: &[
            ("attack", 0.5),       // neutral attack
            ("sustain", 0.8),      // boost sustain
            ("speed", 0.5),        // Medium
            ("mix", 1.0),          // 100%
            ("output_gain", 0.5),  // 0.0 dB
        ],
    },
    FactoryPreset {
        name: "Tame Transients",
        category: "Mixing",
        values: &[
            ("attack", 0.2),       // cut attack
            ("sustain", 0.5),      // neutral sustain
            ("speed", 0.0),        // Fast
            ("mix", 1.0),          // 100%
            ("output_gain", 0.5),  // 0.0 dB
        ],
    },
    FactoryPreset {
        name: "Percussive Attack",
        category: "Drums",
        values: &[
            ("attack", 0.95),      // heavy attack boost
            ("sustain", 0.2),      // cut sustain
            ("speed", 0.0),        // Fast
            ("mix", 0.8),          // 80%
            ("output_gain", 0.46), // slight gain reduction to compensate
        ],
    },
    FactoryPreset {
        name: "Gentle Body",
        category: "Mixing",
        values: &[
            ("attack", 0.4),       // slight attack cut
            ("sustain", 0.65),     // slight sustain boost
            ("speed", 1.0),        // Slow
            ("mix", 1.0),          // 100%
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
