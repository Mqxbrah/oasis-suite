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
            ("frequency", 0.383),  // 6500 Hz (skewed -2.0, range 2000-16000)
            ("bandwidth", 0.2),    // 2.0 Q (linear, range 0.5-8.0)
            ("threshold", 0.667),  // -20.0 dB (linear, range -60..0)
            ("range", 0.5),        // 12.0 dB (linear, range 0..24)
            ("mode", 0.0),         // Split
            ("listen", 0.0),       // Off
            ("mix", 1.0),          // 100%
            ("output_gain", 0.5),  // 0.0 dB
        ],
    },
    FactoryPreset {
        name: "Vocal De-Ess",
        category: "Vocals",
        values: &[
            ("frequency", 0.430),  // ~7200 Hz
            ("bandwidth", 0.267),  // ~2.5 Q
            ("threshold", 0.583),  // -25.0 dB
            ("range", 0.333),      // 8.0 dB
            ("mode", 0.0),         // Split
            ("listen", 0.0),
            ("mix", 1.0),
            ("output_gain", 0.5),
        ],
    },
    FactoryPreset {
        name: "Harsh Highs",
        category: "Mixing",
        values: &[
            ("frequency", 0.536),  // ~9000 Hz
            ("bandwidth", 0.133),  // ~1.5 Q
            ("threshold", 0.5),    // -30.0 dB
            ("range", 0.417),      // 10.0 dB
            ("mode", 1.0),         // Wideband
            ("listen", 0.0),
            ("mix", 1.0),
            ("output_gain", 0.5),
        ],
    },
    FactoryPreset {
        name: "Gentle Sibilance",
        category: "Vocals",
        values: &[
            ("frequency", 0.383),  // 6500 Hz
            ("bandwidth", 0.333),  // ~3.0 Q
            ("threshold", 0.5),    // -30.0 dB
            ("range", 0.25),       // 6.0 dB
            ("mode", 0.0),         // Split
            ("listen", 0.0),
            ("mix", 1.0),
            ("output_gain", 0.5),
        ],
    },
    FactoryPreset {
        name: "Cymbal Tamer",
        category: "Drums",
        values: &[
            ("frequency", 0.607),  // ~10500 Hz
            ("bandwidth", 0.133),  // ~1.5 Q
            ("threshold", 0.667),  // -20.0 dB
            ("range", 0.375),      // 9.0 dB
            ("mode", 0.0),         // Split
            ("listen", 0.0),
            ("mix", 0.85),
            ("output_gain", 0.5),
        ],
    },
    FactoryPreset {
        name: "Broadcast Voice",
        category: "Vocals",
        values: &[
            ("frequency", 0.323),  // ~5500 Hz
            ("bandwidth", 0.2),    // ~2.0 Q
            ("threshold", 0.75),   // -15.0 dB
            ("range", 0.5),        // 12.0 dB
            ("mode", 0.0),         // Split
            ("listen", 0.0),
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
