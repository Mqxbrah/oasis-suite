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
            ("width", 0.5),        // 1.0 (100%) = normal stereo, range 0-2
            ("mid_gain", 0.5),     // 0.0 dB, range -12..+12
            ("side_gain", 0.5),    // 0.0 dB
            ("haas_delay", 0.0),   // 0.0 ms
            ("haas_channel", 0.0), // Right
            ("bass_mono_on", 0.0), // Off
            ("bass_mono_freq", 0.208), // ~120 Hz (skewed)
            ("mix", 1.0),          // 100%
            ("output_gain", 0.5),  // 0.0 dB
        ],
    },
    FactoryPreset {
        name: "Ultra Wide",
        category: "Creative",
        values: &[
            ("width", 0.76),       // 1.52 (152%)
            ("mid_gain", 0.4375),  // -1.5 dB
            ("side_gain", 0.679),  // 4.3 dB
            ("haas_delay", 0.601), // 13.3 ms (skewed range 0-30)
            ("haas_channel", 0.0), // Right
            ("bass_mono_on", 1.0), // On
            ("bass_mono_freq", 0.069), // ~53.1 Hz (skewed)
            ("mix", 1.0),          // 100%
            ("output_gain", 0.5),  // 0.0 dB
        ],
    },
    FactoryPreset {
        name: "Subtle Spread",
        category: "Mixing",
        values: &[
            ("width", 0.6),        // 1.2 (120%)
            ("mid_gain", 0.5),     // 0.0 dB
            ("side_gain", 0.542),  // 1.0 dB
            ("haas_delay", 0.0),   // 0.0 ms
            ("haas_channel", 0.0), // Right
            ("bass_mono_on", 1.0), // On
            ("bass_mono_freq", 0.116), // ~80 Hz
            ("mix", 1.0),          // 100%
            ("output_gain", 0.5),  // 0.0 dB
        ],
    },
    FactoryPreset {
        name: "Haas Stereo",
        category: "Creative",
        values: &[
            ("width", 0.5),        // 100%
            ("mid_gain", 0.5),     // 0.0 dB
            ("side_gain", 0.5),    // 0.0 dB
            ("haas_delay", 0.47),  // ~10 ms
            ("haas_channel", 0.0), // Right
            ("bass_mono_on", 1.0), // On
            ("bass_mono_freq", 0.116), // ~80 Hz
            ("mix", 0.7),          // 70%
            ("output_gain", 0.5),  // 0.0 dB
        ],
    },
    FactoryPreset {
        name: "Mono Maker",
        category: "Utility",
        values: &[
            ("width", 0.0),        // 0% = full mono
            ("mid_gain", 0.5),     // 0.0 dB
            ("side_gain", 0.5),    // 0.0 dB
            ("haas_delay", 0.0),   // off
            ("haas_channel", 0.0),
            ("bass_mono_on", 0.0),
            ("bass_mono_freq", 0.208),
            ("mix", 1.0),
            ("output_gain", 0.5),
        ],
    },
    FactoryPreset {
        name: "Bass Tightener",
        category: "Mixing",
        values: &[
            ("width", 0.5),        // 100% (unchanged width)
            ("mid_gain", 0.5),     // 0.0 dB
            ("side_gain", 0.5),    // 0.0 dB
            ("haas_delay", 0.0),   // off
            ("haas_channel", 0.0),
            ("bass_mono_on", 1.0), // On
            ("bass_mono_freq", 0.157), // ~100 Hz
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
