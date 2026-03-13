// ── Global ──
pub const MIN_FREQ_HZ: f32 = 20.0;
pub const MAX_FREQ_HZ: f32 = 20_000.0;
pub const DEFAULT_SAMPLE_RATE: f32 = 44_100.0;
pub const SMOOTHING_MS_FAST: f32 = 20.0;
pub const SMOOTHING_MS_NORMAL: f32 = 50.0;
pub const GAIN_MIN_DB: f32 = -12.0;
pub const GAIN_MAX_DB: f32 = 12.0;

// ── Metering ──
pub const METER_DECAY_MS: f32 = 300.0;
pub const METER_ATTACK_MS: f32 = 5.0;
pub const UI_REFRESH_HZ: f32 = 120.0;

// ── Oasis Wide ──
pub const BASS_MONO_MIN_FREQ: f32 = 20.0;
pub const BASS_MONO_MAX_FREQ: f32 = 500.0;
pub const BASS_MONO_DEFAULT_FREQ: f32 = 120.0;
pub const HAAS_MAX_DELAY_MS: f32 = 200.0;
pub const WIDTH_MIN: f32 = 0.0;
pub const WIDTH_MAX: f32 = 2.0;
pub const WIDTH_DEFAULT: f32 = 1.0;

// ── Oasis Comp ──
pub const COMP_THRESHOLD_MIN_DB: f32 = -60.0;
pub const COMP_THRESHOLD_MAX_DB: f32 = 0.0;
pub const COMP_THRESHOLD_DEFAULT_DB: f32 = -18.0;
pub const COMP_RATIO_MIN: f32 = 1.0;
pub const COMP_RATIO_MAX: f32 = 20.0;
pub const COMP_RATIO_DEFAULT: f32 = 4.0;
pub const COMP_ATTACK_MIN_MS: f32 = 0.1;
pub const COMP_ATTACK_MAX_MS: f32 = 100.0;
pub const COMP_ATTACK_DEFAULT_MS: f32 = 10.0;
pub const COMP_RELEASE_MIN_MS: f32 = 5.0;
pub const COMP_RELEASE_MAX_MS: f32 = 1000.0;
pub const COMP_RELEASE_DEFAULT_MS: f32 = 100.0;
pub const COMP_KNEE_MIN_DB: f32 = 0.0;
pub const COMP_KNEE_MAX_DB: f32 = 24.0;
pub const COMP_KNEE_DEFAULT_DB: f32 = 6.0;
pub const COMP_MAKEUP_MIN_DB: f32 = 0.0;
pub const COMP_MAKEUP_MAX_DB: f32 = 30.0;
pub const COMP_MAKEUP_DEFAULT_DB: f32 = 0.0;
pub const COMP_SC_HP_MIN_HZ: f32 = 20.0;
pub const COMP_SC_HP_MAX_HZ: f32 = 500.0;
pub const COMP_SC_HP_DEFAULT_HZ: f32 = 20.0;

// ── Oasis Drive ──
pub const DRIVE_AMOUNT_MIN: f32 = 0.0;
pub const DRIVE_AMOUNT_MAX: f32 = 1.0;
pub const DRIVE_AMOUNT_DEFAULT: f32 = 0.25;
pub const DRIVE_TONE_MIN_HZ: f32 = 200.0;
pub const DRIVE_TONE_MAX_HZ: f32 = 16000.0;
pub const DRIVE_TONE_DEFAULT_HZ: f32 = 8000.0;
pub const DRIVE_INPUT_GAIN_MIN_DB: f32 = -24.0;
pub const DRIVE_INPUT_GAIN_MAX_DB: f32 = 24.0;

// ── Oasis Punch ──
pub const PUNCH_AMOUNT_MIN: f32 = -1.0;
pub const PUNCH_AMOUNT_MAX: f32 = 1.0;
pub const PUNCH_ATTACK_DEFAULT: f32 = 0.0;
pub const PUNCH_SUSTAIN_DEFAULT: f32 = 0.0;
pub const PUNCH_FAST_ATTACK_MS: f32 = 0.5;
pub const PUNCH_FAST_RELEASE_MS: f32 = 15.0;
pub const PUNCH_SLOW_ATTACK_MS: f32 = 20.0;
pub const PUNCH_SLOW_RELEASE_MS: f32 = 200.0;

// ── Oasis Limit ──
pub const LIMIT_CEILING_MIN_DB: f32 = -12.0;
pub const LIMIT_CEILING_MAX_DB: f32 = 0.0;
pub const LIMIT_CEILING_DEFAULT_DB: f32 = -0.3;
pub const LIMIT_INPUT_GAIN_MIN_DB: f32 = 0.0;
pub const LIMIT_INPUT_GAIN_MAX_DB: f32 = 24.0;
pub const LIMIT_INPUT_GAIN_DEFAULT_DB: f32 = 0.0;
pub const LIMIT_RELEASE_MIN_MS: f32 = 1.0;
pub const LIMIT_RELEASE_MAX_MS: f32 = 500.0;
pub const LIMIT_RELEASE_DEFAULT_MS: f32 = 50.0;
pub const LIMIT_LOOKAHEAD_MS: f32 = 5.0;

// ── Oasis DeEss ──
pub const DEESS_FREQ_MIN_HZ: f32 = 2000.0;
pub const DEESS_FREQ_MAX_HZ: f32 = 16000.0;
pub const DEESS_FREQ_DEFAULT_HZ: f32 = 6500.0;
pub const DEESS_Q_MIN: f32 = 0.5;
pub const DEESS_Q_MAX: f32 = 8.0;
pub const DEESS_Q_DEFAULT: f32 = 2.0;
pub const DEESS_THRESHOLD_MIN_DB: f32 = -60.0;
pub const DEESS_THRESHOLD_MAX_DB: f32 = 0.0;
pub const DEESS_THRESHOLD_DEFAULT_DB: f32 = -20.0;
pub const DEESS_RANGE_MIN_DB: f32 = 0.0;
pub const DEESS_RANGE_MAX_DB: f32 = 24.0;
pub const DEESS_RANGE_DEFAULT_DB: f32 = 12.0;

// ── Oasis Delay ──
pub const DELAY_TIME_MIN_MS: f32 = 1.0;
pub const DELAY_TIME_MAX_MS: f32 = 2000.0;
pub const DELAY_TIME_DEFAULT_MS: f32 = 250.0;
pub const DELAY_FEEDBACK_MIN: f32 = 0.0;
pub const DELAY_FEEDBACK_MAX: f32 = 0.95;
pub const DELAY_FEEDBACK_DEFAULT: f32 = 0.35;
pub const DELAY_LP_DEFAULT_HZ: f32 = 12000.0;
pub const DELAY_HP_DEFAULT_HZ: f32 = 80.0;
pub const DELAY_MOD_RATE_MIN_HZ: f32 = 0.05;
pub const DELAY_MOD_RATE_MAX_HZ: f32 = 5.0;
pub const DELAY_MOD_RATE_DEFAULT_HZ: f32 = 0.5;
pub const DELAY_MOD_DEPTH_DEFAULT: f32 = 0.0;
pub const DELAY_SATURATION_DEFAULT: f32 = 0.0;
pub const DELAY_DUCKING_DEFAULT: f32 = 0.0;

// ── Oasis Verb ──
pub const VERB_SIZE_MIN: f32 = 0.0;
pub const VERB_SIZE_MAX: f32 = 1.0;
pub const VERB_SIZE_DEFAULT: f32 = 0.5;
pub const VERB_PREDELAY_MIN_MS: f32 = 0.0;
pub const VERB_PREDELAY_MAX_MS: f32 = 200.0;
pub const VERB_PREDELAY_DEFAULT_MS: f32 = 15.0;
pub const VERB_DAMPING_DEFAULT: f32 = 0.5;
pub const VERB_WIDTH_DEFAULT: f32 = 1.0;
pub const VERB_DIFFUSION_DEFAULT: f32 = 0.7;
pub const VERB_LOWCUT_DEFAULT_HZ: f32 = 80.0;
pub const VERB_HIGHCUT_DEFAULT_HZ: f32 = 12000.0;
pub const VERB_MOD_DEFAULT: f32 = 0.2;
pub const VERB_FDN_DELAY_LENGTHS: [usize; 8] = [1557, 1617, 1491, 1422, 1277, 1356, 1188, 1116];
pub const VERB_ALLPASS_LENGTHS: [usize; 4] = [556, 441, 341, 225];

// ── Oasis Shift ──
pub const SHIFT_SEMITONES_MIN: f32 = -24.0;
pub const SHIFT_SEMITONES_MAX: f32 = 24.0;
pub const SHIFT_SEMITONES_DEFAULT: f32 = 0.0;
pub const SHIFT_CENTS_MIN: f32 = -100.0;
pub const SHIFT_CENTS_MAX: f32 = 100.0;
pub const SHIFT_CENTS_DEFAULT: f32 = 0.0;
pub const SHIFT_DETUNE_MIN: f32 = 0.0;
pub const SHIFT_DETUNE_MAX: f32 = 50.0;
pub const SHIFT_DETUNE_DEFAULT: f32 = 0.0;
pub const SHIFT_GRAIN_SIZE: usize = 2048;
pub const SHIFT_OVERLAP: usize = 4;

// ── Oasis EQ ──
pub const EQ_BAND_COUNT: usize = 8;
pub const EQ_GAIN_MIN_DB: f32 = -18.0;
pub const EQ_GAIN_MAX_DB: f32 = 18.0;
pub const EQ_Q_MIN: f32 = 0.1;
pub const EQ_Q_MAX: f32 = 18.0;
pub const EQ_Q_DEFAULT: f32 = 1.0;
pub const EQ_BAND_DEFAULTS_HZ: [f32; 8] = [30.0, 80.0, 250.0, 800.0, 2500.0, 5000.0, 10000.0, 16000.0];

// ── Oasis Pump ──
pub const PUMP_DEPTH_MIN: f32 = 0.0;
pub const PUMP_DEPTH_MAX: f32 = 1.0;
pub const PUMP_DEPTH_DEFAULT: f32 = 0.5;
pub const PUMP_SMOOTHING_MIN: f32 = 0.0;
pub const PUMP_SMOOTHING_MAX: f32 = 1.0;
pub const PUMP_SMOOTHING_DEFAULT: f32 = 0.3;
pub const PUMP_RATE_HZ_DEFAULT: f32 = 4.0;
pub const PUMP_RATE_HZ_MIN: f32 = 0.25;
pub const PUMP_RATE_HZ_MAX: f32 = 32.0;

// ── Oasis Synth ──
pub const SYNTH_OSC_COUNT: usize = 2;
pub const SYNTH_MAX_VOICES: usize = 8;
pub const SYNTH_FILTER_CUTOFF_MIN_HZ: f32 = 20.0;
pub const SYNTH_FILTER_CUTOFF_MAX_HZ: f32 = 20000.0;
pub const SYNTH_FILTER_CUTOFF_DEFAULT_HZ: f32 = 10000.0;
pub const SYNTH_FILTER_RESO_MIN: f32 = 0.0;
pub const SYNTH_FILTER_RESO_MAX: f32 = 1.0;
pub const SYNTH_FILTER_RESO_DEFAULT: f32 = 0.0;
pub const SYNTH_ENV_MIN_MS: f32 = 0.5;
pub const SYNTH_ENV_MAX_MS: f32 = 5000.0;
pub const SYNTH_ATTACK_DEFAULT_MS: f32 = 5.0;
pub const SYNTH_DECAY_DEFAULT_MS: f32 = 200.0;
pub const SYNTH_SUSTAIN_DEFAULT: f32 = 0.7;
pub const SYNTH_RELEASE_DEFAULT_MS: f32 = 300.0;
pub const SYNTH_LFO_RATE_MIN_HZ: f32 = 0.05;
pub const SYNTH_LFO_RATE_MAX_HZ: f32 = 20.0;
pub const SYNTH_LFO_RATE_DEFAULT_HZ: f32 = 2.0;
pub const SYNTH_DETUNE_MAX_CENTS: f32 = 100.0;
pub const SYNTH_GLIDE_MAX_MS: f32 = 1000.0;
