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
            ("size", 0.5),          // 50%
            ("predelay", 0.226),    // ~15 ms (skewed -1.0, range 0-200)
            ("damping", 0.5),       // 50%
            ("width", 1.0),         // 100%
            ("diffusion", 0.7),     // 70%
            ("low_cut", 0.035),     // ~80 Hz (skewed -2.0, range 20-2000)
            ("high_cut", 0.739),    // ~12 kHz (skewed -2.0, range 1000-20000)
            ("modulation", 0.2),    // 20%
            ("algorithm", 0.0),     // Room (index 0, normalized = 0/5 ≈ 0.0)
            ("mix", 0.3),           // 30%
            ("output_gain", 0.5),   // 0.0 dB
        ],
    },
    FactoryPreset {
        name: "Small Room",
        category: "Rooms",
        values: &[
            ("size", 0.25),         // 25%
            ("predelay", 0.141),    // ~5 ms
            ("damping", 0.6),       // 60%
            ("width", 0.8),         // 80%
            ("diffusion", 0.8),     // 80%
            ("low_cut", 0.053),     // ~120 Hz
            ("high_cut", 0.608),    // ~8 kHz
            ("modulation", 0.1),    // 10%
            ("algorithm", 0.0),     // Room
            ("mix", 0.25),          // 25%
            ("output_gain", 0.5),   // 0.0 dB
        ],
    },
    FactoryPreset {
        name: "Large Hall",
        category: "Halls",
        values: &[
            ("size", 0.85),         // 85%
            ("predelay", 0.349),    // ~35 ms
            ("damping", 0.4),       // 40%
            ("width", 1.0),         // 100%
            ("diffusion", 0.65),    // 65%
            ("low_cut", 0.035),     // ~80 Hz
            ("high_cut", 0.739),    // ~12 kHz
            ("modulation", 0.25),   // 25%
            ("algorithm", 0.2),     // Hall (index 1, normalized = 1/5 = 0.2)
            ("mix", 0.35),          // 35%
            ("output_gain", 0.5),   // 0.0 dB
        ],
    },
    FactoryPreset {
        name: "Bright Plate",
        category: "Plates",
        values: &[
            ("size", 0.6),          // 60%
            ("predelay", 0.071),    // ~1 ms
            ("damping", 0.25),      // 25%
            ("width", 1.0),         // 100%
            ("diffusion", 0.9),     // 90%
            ("low_cut", 0.078),     // ~180 Hz
            ("high_cut", 0.874),    // ~16 kHz
            ("modulation", 0.15),   // 15%
            ("algorithm", 0.4),     // Plate (index 2, normalized = 2/5 = 0.4)
            ("mix", 0.3),           // 30%
            ("output_gain", 0.5),   // 0.0 dB
        ],
    },
    FactoryPreset {
        name: "Warm Chamber",
        category: "Chambers",
        values: &[
            ("size", 0.55),         // 55%
            ("predelay", 0.274),    // ~20 ms
            ("damping", 0.65),      // 65%
            ("width", 0.9),         // 90%
            ("diffusion", 0.75),    // 75%
            ("low_cut", 0.053),     // ~120 Hz
            ("high_cut", 0.529),    // ~6 kHz
            ("modulation", 0.2),    // 20%
            ("algorithm", 0.6),     // Chamber (index 3, normalized = 3/5 = 0.6)
            ("mix", 0.3),           // 30%
            ("output_gain", 0.5),   // 0.0 dB
        ],
    },
    FactoryPreset {
        name: "Shimmer Pad",
        category: "Creative",
        values: &[
            ("size", 0.9),          // 90%
            ("predelay", 0.349),    // ~35 ms
            ("damping", 0.3),       // 30%
            ("width", 1.0),         // 100%
            ("diffusion", 0.85),    // 85%
            ("low_cut", 0.078),     // ~180 Hz
            ("high_cut", 0.874),    // ~16 kHz
            ("modulation", 0.4),    // 40%
            ("algorithm", 0.8),     // Shimmer (index 4, normalized = 4/5 = 0.8)
            ("mix", 0.45),          // 45%
            ("output_gain", 0.5),   // 0.0 dB
        ],
    },
    FactoryPreset {
        name: "Spring Bounce",
        category: "Creative",
        values: &[
            ("size", 0.4),          // 40%
            ("predelay", 0.071),    // ~1 ms
            ("damping", 0.55),      // 55%
            ("width", 0.7),         // 70%
            ("diffusion", 0.5),     // 50%
            ("low_cut", 0.053),     // ~120 Hz
            ("high_cut", 0.608),    // ~8 kHz
            ("modulation", 0.35),   // 35%
            ("algorithm", 1.0),     // Spring (index 5, normalized = 5/5 = 1.0)
            ("mix", 0.3),           // 30%
            ("output_gain", 0.5),   // 0.0 dB
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
