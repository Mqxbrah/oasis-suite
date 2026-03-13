#![allow(dead_code)]
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct FactoryPreset {
    pub name: &'static str,
    pub category: &'static str,
    /// (param_id, normalized_value)
    pub values: &'static [(&'static str, f32)],
}

// Waveform enum indices: Sine=0, Saw=1, Square=2, Triangle=3
// Normalized enum values: Sine=0.0, Saw≈0.333, Square≈0.667, Triangle=1.0
// nih-plug enums: normalized = variant_index / (variant_count - 1)
// 4 variants => Sine=0/3=0.0, Saw=1/3=0.333, Square=2/3=0.667, Triangle=3/3=1.0

const WAVE_SINE: f32 = 0.0;
const WAVE_SAW: f32 = 0.333;
const WAVE_SQUARE: f32 = 0.667;
const WAVE_TRIANGLE: f32 = 1.0;

pub const FACTORY_PRESETS: &[FactoryPreset] = &[
    // Init — clean saw, filter open, usable immediately
    FactoryPreset {
        name: "Init",
        category: "Default",
        values: &[
            ("osc1_wave", WAVE_SAW),
            ("osc1_level", 1.0),
            ("osc1_detune", 0.0),
            ("osc2_wave", WAVE_SAW),
            ("osc2_level", 0.0),
            ("osc2_detune", 0.0),
            ("filter_cutoff", 0.75),    // ~10 kHz (skewed -2.0)
            ("filter_reso", 0.0),
            ("env_attack", 0.033),      // ~5 ms (skewed -1.0, range 0.5–5000)
            ("env_decay", 0.141),       // ~200 ms
            ("env_sustain", 0.7),
            ("env_release", 0.176),     // ~300 ms
            ("master_gain", 0.5),       // 0 dB
        ],
    },
    // Classic Saw Lead — bright, punchy mono lead
    FactoryPreset {
        name: "Classic Saw Lead",
        category: "Lead",
        values: &[
            ("osc1_wave", WAVE_SAW),
            ("osc1_level", 1.0),
            ("osc1_detune", 0.0),
            ("osc2_wave", WAVE_SAW),
            ("osc2_level", 0.7),
            ("osc2_detune", 0.07),      // ~7 cents
            ("filter_cutoff", 0.55),    // ~3 kHz
            ("filter_reso", 0.3),
            ("env_attack", 0.014),      // ~2 ms
            ("env_decay", 0.141),       // ~200 ms
            ("env_sustain", 0.6),
            ("env_release", 0.141),     // ~200 ms
            ("master_gain", 0.5),
        ],
    },
    // Warm Pad — slow attack, high sustain, soft waveforms
    FactoryPreset {
        name: "Warm Pad",
        category: "Pad",
        values: &[
            ("osc1_wave", WAVE_TRIANGLE),
            ("osc1_level", 0.8),
            ("osc1_detune", 0.0),
            ("osc2_wave", WAVE_SAW),
            ("osc2_level", 0.4),
            ("osc2_detune", 0.05),      // ~5 cents
            ("filter_cutoff", 0.4),     // ~1 kHz
            ("filter_reso", 0.15),
            ("env_attack", 0.4),        // ~800 ms
            ("env_decay", 0.3),         // ~500 ms
            ("env_sustain", 0.8),
            ("env_release", 0.5),       // ~1500 ms
            ("master_gain", 0.5),
        ],
    },
    // Pluck Bass — short decay, low sustain for plucked sounds
    FactoryPreset {
        name: "Pluck Bass",
        category: "Bass",
        values: &[
            ("osc1_wave", WAVE_SAW),
            ("osc1_level", 1.0),
            ("osc1_detune", 0.0),
            ("osc2_wave", WAVE_SQUARE),
            ("osc2_level", 0.5),
            ("osc2_detune", 0.0),
            ("filter_cutoff", 0.35),    // ~700 Hz
            ("filter_reso", 0.4),
            ("env_attack", 0.005),      // ~1 ms
            ("env_decay", 0.1),         // ~100 ms
            ("env_sustain", 0.0),
            ("env_release", 0.07),      // ~60 ms
            ("master_gain", 0.5),
        ],
    },
    // Sub Bass — pure sine, tight envelope
    FactoryPreset {
        name: "Sub Bass",
        category: "Bass",
        values: &[
            ("osc1_wave", WAVE_SINE),
            ("osc1_level", 1.0),
            ("osc1_detune", 0.0),
            ("osc2_wave", WAVE_SINE),
            ("osc2_level", 0.0),
            ("osc2_detune", 0.0),
            ("filter_cutoff", 0.25),    // ~300 Hz
            ("filter_reso", 0.0),
            ("env_attack", 0.014),      // ~2 ms
            ("env_decay", 0.07),        // ~60 ms
            ("env_sustain", 0.9),
            ("env_release", 0.07),      // ~60 ms
            ("master_gain", 0.5),
        ],
    },
    // Bright Arp — square wave, fast envelope for arpeggiated sequences
    FactoryPreset {
        name: "Bright Arp",
        category: "Lead",
        values: &[
            ("osc1_wave", WAVE_SQUARE),
            ("osc1_level", 1.0),
            ("osc1_detune", 0.0),
            ("osc2_wave", WAVE_SAW),
            ("osc2_level", 0.3),
            ("osc2_detune", 0.03),      // ~3 cents
            ("filter_cutoff", 0.65),    // ~5 kHz
            ("filter_reso", 0.2),
            ("env_attack", 0.005),      // ~1 ms
            ("env_decay", 0.12),        // ~150 ms
            ("env_sustain", 0.15),
            ("env_release", 0.1),       // ~100 ms
            ("master_gain", 0.5),
        ],
    },
    // Thick Unison — two detuned saws for fat unison character
    FactoryPreset {
        name: "Thick Unison",
        category: "Lead",
        values: &[
            ("osc1_wave", WAVE_SAW),
            ("osc1_level", 1.0),
            ("osc1_detune", 0.0),
            ("osc2_wave", WAVE_SAW),
            ("osc2_level", 1.0),
            ("osc2_detune", 0.15),      // ~15 cents
            ("filter_cutoff", 0.6),     // ~4 kHz
            ("filter_reso", 0.1),
            ("env_attack", 0.014),      // ~2 ms
            ("env_decay", 0.2),         // ~300 ms
            ("env_sustain", 0.7),
            ("env_release", 0.2),       // ~300 ms
            ("master_gain", 0.458),     // -1 dB (slight reduction for 2 full-level oscs)
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
