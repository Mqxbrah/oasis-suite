#![allow(dead_code)]
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct FactoryPreset {
    pub name: &'static str,
    pub category: &'static str,
    /// (param_id, normalized_value)
    pub values: &'static [(&'static str, f32)],
}

// Normalized value notes for skewed ranges:
//   delay_l/delay_r: Skewed(-1.0), range 1..2000, factor = 2^(-1) = 0.5
//     normalized = ((val - 1) / 1999)^0.5
//     250ms => ((249)/1999)^0.5 = 0.3528^0.5 ≈ 0.353
//   lp_freq: Skewed(-2.0), range 200..20000, factor = 2^(-2) = 0.25
//     normalized = ((val - 200) / 19800)^0.25
//   hp_freq: Skewed(-2.0), range 20..2000, factor = 2^(-2) = 0.25
//     normalized = ((val - 20) / 1980)^0.25
//   mod_rate: Skewed(-1.0), range 0.05..5.0, factor = 2^(-1) = 0.5
//     normalized = ((val - 0.05) / 4.95)^0.5
//   feedback: Linear, range 0..0.95 => normalized = val / 0.95
//   saturation/mod_depth/mix: Linear 0..1 => normalized = val
//   output_gain: Linear -12..12 => normalized = (val + 12) / 24

pub const FACTORY_PRESETS: &[FactoryPreset] = &[
    // Init: Default starting point
    FactoryPreset {
        name: "Init",
        category: "Default",
        values: &[
            ("delay_l", 0.353),     // 250 ms
            ("delay_r", 0.353),     // 250 ms
            ("feedback", 0.368),    // 0.35 / 0.95
            ("ping_pong", 0.0),     // Off
            ("lp_freq", 0.878),     // 12000 Hz
            ("hp_freq", 0.400),     // 80 Hz
            ("saturation", 0.0),    // 0%
            ("mod_rate", 0.302),    // 0.5 Hz
            ("mod_depth", 0.0),     // 0%
            ("mix", 0.3),           // 30%
            ("output_gain", 0.5),   // 0 dB
        ],
    },
    // Stereo Slapback: Short delay for doubling/widening
    FactoryPreset {
        name: "Stereo Slapback",
        category: "Creative",
        values: &[
            ("delay_l", 0.168),     // ~57 ms
            ("delay_r", 0.194),     // ~76 ms
            ("feedback", 0.105),    // 0.10 / 0.95
            ("ping_pong", 0.0),     // Off
            ("lp_freq", 0.811),     // ~8000 Hz
            ("hp_freq", 0.400),     // ~80 Hz
            ("saturation", 0.0),    // 0%
            ("mod_rate", 0.302),    // 0.5 Hz
            ("mod_depth", 0.0),     // 0%
            ("mix", 0.35),          // 35%
            ("output_gain", 0.5),   // 0 dB
        ],
    },
    // Ping-Pong Eighth: Bouncing eighth-note delays
    FactoryPreset {
        name: "Ping-Pong Eighth",
        category: "Creative",
        values: &[
            ("delay_l", 0.353),     // 250 ms (~8th at 120 BPM)
            ("delay_r", 0.353),     // 250 ms
            ("feedback", 0.421),    // 0.40 / 0.95
            ("ping_pong", 1.0),     // On
            ("lp_freq", 0.811),     // ~8000 Hz
            ("hp_freq", 0.501),     // ~150 Hz
            ("saturation", 0.0),    // 0%
            ("mod_rate", 0.302),    // 0.5 Hz
            ("mod_depth", 0.0),     // 0%
            ("mix", 0.3),           // 30%
            ("output_gain", 0.5),   // 0 dB
        ],
    },
    // Dark Tape Echo: Warm, filtered repeats with tape character
    FactoryPreset {
        name: "Dark Tape Echo",
        category: "Vintage",
        values: &[
            ("delay_l", 0.398),     // ~318 ms
            ("delay_r", 0.398),     // ~318 ms
            ("feedback", 0.474),    // 0.45 / 0.95
            ("ping_pong", 0.0),     // Off
            ("lp_freq", 0.688),     // ~3500 Hz
            ("hp_freq", 0.501),     // ~150 Hz
            ("saturation", 0.35),   // 35%
            ("mod_rate", 0.393),    // 0.82 Hz
            ("mod_depth", 0.15),    // 15%
            ("mix", 0.3),           // 30%
            ("output_gain", 0.5),   // 0 dB
        ],
    },
    // Modulated Space: Chorus-like modulated delay
    FactoryPreset {
        name: "Modulated Space",
        category: "Creative",
        values: &[
            ("delay_l", 0.281),     // ~159 ms
            ("delay_r", 0.316),     // ~200 ms
            ("feedback", 0.368),    // 0.35 / 0.95
            ("ping_pong", 0.0),     // Off
            ("lp_freq", 0.811),     // ~8000 Hz
            ("hp_freq", 0.400),     // ~80 Hz
            ("saturation", 0.1),    // 10%
            ("mod_rate", 0.560),    // ~1.6 Hz
            ("mod_depth", 0.4),     // 40%
            ("mix", 0.35),          // 35%
            ("output_gain", 0.5),   // 0 dB
        ],
    },
    // Ambient Wash: Long, diffuse delays for pads/textures
    FactoryPreset {
        name: "Ambient Wash",
        category: "Ambient",
        values: &[
            ("delay_l", 0.530),     // ~563 ms
            ("delay_r", 0.590),     // ~698 ms
            ("feedback", 0.632),    // 0.60 / 0.95
            ("ping_pong", 1.0),     // On
            ("lp_freq", 0.735),     // ~4500 Hz
            ("hp_freq", 0.501),     // ~150 Hz
            ("saturation", 0.15),   // 15%
            ("mod_rate", 0.226),    // 0.3 Hz
            ("mod_depth", 0.25),    // 25%
            ("mix", 0.5),           // 50%
            ("output_gain", 0.5),   // 0 dB
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
