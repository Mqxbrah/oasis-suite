#![allow(dead_code)]
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct FactoryPreset {
    pub name: &'static str,
    pub category: &'static str,
    /// (param_id, normalized_value)
    pub values: &'static [(&'static str, f32)],
}

// Normalized value reference:
//   Freq (20-20000 Hz, skew -2.0): norm = ((f-20)/19980)^0.25
//   Gain (-18..18 dB, linear):     norm = (g+18)/36
//   Q    (0.1-18.0, skew -1.0):    norm = ((q-0.1)/17.9)^0.5
//   FilterType (7 variants):       norm = variant_index / 6.0
//     Bell=0.0 LowShelf=0.167 HighShelf=0.333 LowCut=0.5 HighCut=0.667 Notch=0.833 Bandpass=1.0
//   Enabled (bool):                false=0.0 true=1.0
//   OutputGain (-12..12 dB):       norm = (g+12)/24

pub const FACTORY_PRESETS: &[FactoryPreset] = &[
    // ── Init ──
    FactoryPreset {
        name: "Init",
        category: "Default",
        values: &[
            ("b1_freq", 0.150),  // 30 Hz
            ("b1_gain", 0.500),  // 0 dB
            ("b1_q",    0.224),  // Q 1.0
            ("b1_type", 0.500),  // LowCut
            ("b1_on",   1.0),
            ("b2_freq", 0.234),  // 80 Hz
            ("b2_gain", 0.500),
            ("b2_q",    0.224),
            ("b2_type", 0.0),    // Bell
            ("b2_on",   1.0),
            ("b3_freq", 0.328),  // 250 Hz
            ("b3_gain", 0.500),
            ("b3_q",    0.224),
            ("b3_type", 0.0),
            ("b3_on",   1.0),
            ("b4_freq", 0.445),  // 800 Hz
            ("b4_gain", 0.500),
            ("b4_q",    0.224),
            ("b4_type", 0.0),
            ("b4_on",   1.0),
            ("b5_freq", 0.594),  // 2500 Hz
            ("b5_gain", 0.500),
            ("b5_q",    0.224),
            ("b5_type", 0.0),
            ("b5_on",   1.0),
            ("b6_freq", 0.707),  // 5000 Hz
            ("b6_gain", 0.500),
            ("b6_q",    0.224),
            ("b6_type", 0.0),
            ("b6_on",   1.0),
            ("b7_freq", 0.841),  // 10000 Hz
            ("b7_gain", 0.500),
            ("b7_q",    0.224),
            ("b7_type", 0.0),
            ("b7_on",   1.0),
            ("b8_freq", 0.946),  // 16000 Hz
            ("b8_gain", 0.500),
            ("b8_q",    0.224),
            ("b8_type", 0.667),  // HighCut
            ("b8_on",   1.0),
            ("output_gain", 0.5),
        ],
    },
    // ── Vocal Presence ──
    FactoryPreset {
        name: "Vocal Presence",
        category: "Vocals",
        values: &[
            ("b1_freq", 0.234),  // 80 Hz LowCut
            ("b1_gain", 0.500),
            ("b1_q",    0.184),  // Q 0.707
            ("b1_type", 0.500),  // LowCut
            ("b1_on",   1.0),
            ("b2_freq", 0.328),  // 250 Hz
            ("b2_gain", 0.444),  // -2 dB
            ("b2_q",    0.224),
            ("b2_type", 0.0),    // Bell
            ("b2_on",   1.0),
            ("b3_freq", 0.445),  // 800 Hz
            ("b3_gain", 0.500),  // 0 dB
            ("b3_q",    0.224),
            ("b3_type", 0.0),
            ("b3_on",   1.0),
            ("b4_freq", 0.594),  // 2500 Hz
            ("b4_gain", 0.583),  // +3 dB
            ("b4_q",    0.224),
            ("b4_type", 0.0),
            ("b4_on",   1.0),
            ("b5_freq", 0.668),  // 4000 Hz
            ("b5_gain", 0.556),  // +2 dB
            ("b5_q",    0.280),  // Q 1.5
            ("b5_type", 0.0),
            ("b5_on",   1.0),
            ("b6_freq", 0.707),  // 5000 Hz
            ("b6_gain", 0.500),  // 0 dB
            ("b6_q",    0.224),
            ("b6_type", 0.0),
            ("b6_on",   1.0),
            ("b7_freq", 0.841),  // 10000 Hz HighShelf
            ("b7_gain", 0.556),  // +2 dB
            ("b7_q",    0.184),
            ("b7_type", 0.333),  // HighShelf
            ("b7_on",   1.0),
            ("b8_freq", 0.946),  // 16000 Hz
            ("b8_gain", 0.500),
            ("b8_q",    0.184),
            ("b8_type", 0.667),  // HighCut
            ("b8_on",   1.0),
            ("output_gain", 0.5),
        ],
    },
    // ── Bass Scoop ──
    FactoryPreset {
        name: "Bass Scoop",
        category: "Mixing",
        values: &[
            ("b1_freq", 0.178),  // 40 Hz LowShelf +2 dB
            ("b1_gain", 0.556),
            ("b1_q",    0.184),
            ("b1_type", 0.167),  // LowShelf
            ("b1_on",   1.0),
            ("b2_freq", 0.234),  // 80 Hz
            ("b2_gain", 0.500),  // 0 dB
            ("b2_q",    0.224),
            ("b2_type", 0.0),
            ("b2_on",   1.0),
            ("b3_freq", 0.328),  // 250 Hz -4 dB
            ("b3_gain", 0.389),
            ("b3_q",    0.224),
            ("b3_type", 0.0),
            ("b3_on",   1.0),
            ("b4_freq", 0.394),  // 500 Hz -3 dB
            ("b4_gain", 0.417),
            ("b4_q",    0.280),  // Q 1.5
            ("b4_type", 0.0),
            ("b4_on",   1.0),
            ("b5_freq", 0.594),  // 2500 Hz
            ("b5_gain", 0.500),
            ("b5_q",    0.224),
            ("b5_type", 0.0),
            ("b5_on",   1.0),
            ("b6_freq", 0.707),  // 5000 Hz +2 dB
            ("b6_gain", 0.556),
            ("b6_q",    0.224),
            ("b6_type", 0.0),
            ("b6_on",   1.0),
            ("b7_freq", 0.841),  // 10000 Hz
            ("b7_gain", 0.500),
            ("b7_q",    0.224),
            ("b7_type", 0.0),
            ("b7_on",   1.0),
            ("b8_freq", 0.946),  // 16000 Hz HighCut
            ("b8_gain", 0.500),
            ("b8_q",    0.184),
            ("b8_type", 0.667),
            ("b8_on",   1.0),
            ("output_gain", 0.5),
        ],
    },
    // ── High Shelf Air ──
    FactoryPreset {
        name: "High Shelf Air",
        category: "Mixing",
        values: &[
            ("b1_freq", 0.150),  // 30 Hz LowCut
            ("b1_gain", 0.500),
            ("b1_q",    0.224),
            ("b1_type", 0.500),
            ("b1_on",   1.0),
            ("b2_freq", 0.234),
            ("b2_gain", 0.500),
            ("b2_q",    0.224),
            ("b2_type", 0.0),
            ("b2_on",   0.0),    // disabled
            ("b3_freq", 0.328),
            ("b3_gain", 0.500),
            ("b3_q",    0.224),
            ("b3_type", 0.0),
            ("b3_on",   0.0),
            ("b4_freq", 0.445),
            ("b4_gain", 0.500),
            ("b4_q",    0.224),
            ("b4_type", 0.0),
            ("b4_on",   0.0),
            ("b5_freq", 0.594),
            ("b5_gain", 0.500),
            ("b5_q",    0.224),
            ("b5_type", 0.0),
            ("b5_on",   0.0),
            ("b6_freq", 0.707),
            ("b6_gain", 0.500),
            ("b6_q",    0.224),
            ("b6_type", 0.0),
            ("b6_on",   0.0),
            ("b7_freq", 0.795),  // 8000 Hz HighShelf +4 dB
            ("b7_gain", 0.611),
            ("b7_q",    0.184),
            ("b7_type", 0.333),  // HighShelf
            ("b7_on",   1.0),
            ("b8_freq", 0.946),
            ("b8_gain", 0.500),
            ("b8_q",    0.184),
            ("b8_type", 0.667),
            ("b8_on",   0.0),    // disabled
            ("output_gain", 0.5),
        ],
    },
    // ── Low Cut 80Hz ──
    FactoryPreset {
        name: "Low Cut 80Hz",
        category: "Utility",
        values: &[
            ("b1_freq", 0.234),  // 80 Hz LowCut
            ("b1_gain", 0.500),
            ("b1_q",    0.184),  // Q 0.707
            ("b1_type", 0.500),  // LowCut
            ("b1_on",   1.0),
            ("b2_freq", 0.234),
            ("b2_gain", 0.500),
            ("b2_q",    0.224),
            ("b2_type", 0.0),
            ("b2_on",   0.0),
            ("b3_freq", 0.328),
            ("b3_gain", 0.500),
            ("b3_q",    0.224),
            ("b3_type", 0.0),
            ("b3_on",   0.0),
            ("b4_freq", 0.445),
            ("b4_gain", 0.500),
            ("b4_q",    0.224),
            ("b4_type", 0.0),
            ("b4_on",   0.0),
            ("b5_freq", 0.594),
            ("b5_gain", 0.500),
            ("b5_q",    0.224),
            ("b5_type", 0.0),
            ("b5_on",   0.0),
            ("b6_freq", 0.707),
            ("b6_gain", 0.500),
            ("b6_q",    0.224),
            ("b6_type", 0.0),
            ("b6_on",   0.0),
            ("b7_freq", 0.841),
            ("b7_gain", 0.500),
            ("b7_q",    0.224),
            ("b7_type", 0.0),
            ("b7_on",   0.0),
            ("b8_freq", 0.946),
            ("b8_gain", 0.500),
            ("b8_q",    0.224),
            ("b8_type", 0.667),
            ("b8_on",   0.0),
            ("output_gain", 0.5),
        ],
    },
    // ── Mid Dip ──
    FactoryPreset {
        name: "Mid Dip",
        category: "Mixing",
        values: &[
            ("b1_freq", 0.150),
            ("b1_gain", 0.500),
            ("b1_q",    0.224),
            ("b1_type", 0.500),  // LowCut
            ("b1_on",   1.0),
            ("b2_freq", 0.234),
            ("b2_gain", 0.500),
            ("b2_q",    0.224),
            ("b2_type", 0.0),
            ("b2_on",   0.0),
            ("b3_freq", 0.413),  // 600 Hz Bell -4 dB
            ("b3_gain", 0.389),
            ("b3_q",    0.280),  // Q 1.5
            ("b3_type", 0.0),
            ("b3_on",   1.0),
            ("b4_freq", 0.471),  // 1000 Hz Bell -3 dB
            ("b4_gain", 0.417),
            ("b4_q",    0.280),
            ("b4_type", 0.0),
            ("b4_on",   1.0),
            ("b5_freq", 0.594),
            ("b5_gain", 0.500),
            ("b5_q",    0.224),
            ("b5_type", 0.0),
            ("b5_on",   0.0),
            ("b6_freq", 0.707),
            ("b6_gain", 0.500),
            ("b6_q",    0.224),
            ("b6_type", 0.0),
            ("b6_on",   0.0),
            ("b7_freq", 0.841),
            ("b7_gain", 0.500),
            ("b7_q",    0.224),
            ("b7_type", 0.0),
            ("b7_on",   0.0),
            ("b8_freq", 0.946),
            ("b8_gain", 0.500),
            ("b8_q",    0.184),
            ("b8_type", 0.667),
            ("b8_on",   1.0),
            ("output_gain", 0.5),
        ],
    },
    // ── Telephone ──
    FactoryPreset {
        name: "Telephone",
        category: "Creative",
        values: &[
            ("b1_freq", 0.344),  // 300 Hz LowCut
            ("b1_gain", 0.500),
            ("b1_q",    0.326),  // Q 2.0
            ("b1_type", 0.500),  // LowCut
            ("b1_on",   1.0),
            ("b2_freq", 0.234),
            ("b2_gain", 0.500),
            ("b2_q",    0.224),
            ("b2_type", 0.0),
            ("b2_on",   0.0),
            ("b3_freq", 0.328),
            ("b3_gain", 0.500),
            ("b3_q",    0.224),
            ("b3_type", 0.0),
            ("b3_on",   0.0),
            ("b4_freq", 0.445),
            ("b4_gain", 0.500),
            ("b4_q",    0.224),
            ("b4_type", 0.0),
            ("b4_on",   0.0),
            ("b5_freq", 0.594),
            ("b5_gain", 0.500),
            ("b5_q",    0.224),
            ("b5_type", 0.0),
            ("b5_on",   0.0),
            ("b6_freq", 0.707),
            ("b6_gain", 0.500),
            ("b6_q",    0.224),
            ("b6_type", 0.0),
            ("b6_on",   0.0),
            ("b7_freq", 0.841),
            ("b7_gain", 0.500),
            ("b7_q",    0.224),
            ("b7_type", 0.0),
            ("b7_on",   0.0),
            ("b8_freq", 0.621),  // 3000 Hz HighCut
            ("b8_gain", 0.500),
            ("b8_q",    0.326),  // Q 2.0
            ("b8_type", 0.667),  // HighCut
            ("b8_on",   1.0),
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
