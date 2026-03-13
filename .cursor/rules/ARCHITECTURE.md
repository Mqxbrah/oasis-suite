# Oasis Suite — Architecture

This document defines the complete architecture for the Oasis Suite VST3/CLAP plugin collection. Every system is designed for **minimal complexity, maximum capability** — only as complex as necessary, but robust enough to handle anything we throw at it.

---

## Table of Contents

1. [Design Philosophy](#design-philosophy)
2. [Project Structure](#project-structure)
3. [Build System](#build-system)
4. [Core Architecture](#core-architecture)
5. [Audio Thread Contract](#audio-thread-contract)
6. [Parameter System](#parameter-system)
7. [State Management](#state-management)
8. [DSP Library](#dsp-library)
9. [UI Framework](#ui-framework)
10. [Plugin Template](#plugin-template)
11. [Debug Infrastructure](#debug-infrastructure)
12. [Data Flow](#data-flow)
13. [Preset System](#preset-system)
14. [Testing Strategy](#testing-strategy)

---

## Design Philosophy

### The Three Rules

1. **Audio thread is sacred** — Zero allocations, zero blocking, zero exceptions
2. **Single source of truth** — One place for each piece of data, everything else references it
3. **Composition over complexity** — Small, focused systems that combine cleanly

### Minimalist Maximalist Principle

Every system asks: *What's the simplest implementation that handles all our use cases?*

- If a simple solution works, use it
- If complexity is needed, isolate it in one place
- Never spread complexity across multiple systems

---

## Project Structure

```
oasis-suite/
├── Makefile                        # Entry point: make, make test, make release
├── Cargo.toml                      # Workspace configuration
├── rust-toolchain.toml             # Pinned Rust version
│
├── crates/
│   ├── oasis_core/                 # Shared DSP + infrastructure
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs              # Public API
│   │       ├── dsp/                # DSP primitives
│   │       │   ├── mod.rs
│   │       │   ├── filter.rs       # Biquad, SVF, etc.
│   │       │   ├── dynamics.rs     # Envelope detection, gain computation
│   │       │   ├── delay.rs        # Delay lines with interpolation
│   │       │   ├── oscillator.rs   # Wavetable, classic waveforms
│   │       │   ├── envelope.rs     # ADSR, envelope follower
│   │       │   ├── lfo.rs          # LFO with shapes and sync
│   │       │   ├── waveshaper.rs   # Saturation curves
│   │       │   ├── reverb.rs       # FDN, diffusion networks
│   │       │   ├── pitch.rs        # Pitch shifting, formants
│   │       │   ├── fft.rs          # Real FFT for analysis
│   │       │   ├── oversampler.rs  # Polyphase up/downsampling
│   │       │   └── simd.rs         # SIMD abstractions
│   │       ├── params/             # Parameter system extensions
│   │       │   ├── mod.rs
│   │       │   ├── smoothing.rs    # Parameter smoothers
│   │       │   └── formatting.rs   # Value display formatting
│   │       ├── state/              # State management
│   │       │   ├── mod.rs
│   │       │   ├── ab.rs           # A/B comparison
│   │       │   ├── history.rs      # Undo/redo
│   │       │   └── preset.rs       # Preset serialization
│   │       ├── util/               # Utilities
│   │       │   ├── mod.rs
│   │       │   ├── denormal.rs     # Denormal protection
│   │       │   ├── interpolation.rs
│   │       │   ├── math.rs         # Common math functions
│   │       │   └── sample_rate.rs  # Sample rate context
│   │       ├── debug/              # Debug infrastructure
│   │       │   ├── mod.rs
│   │       │   ├── logger.rs       # Lock-free logging
│   │       │   └── metrics.rs      # Performance metrics
│   │       └── constants.rs        # Global constants
│   │
│   ├── oasis_ui/                   # Shared UI framework
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── theme.rs            # Colors, spacing, typography
│   │       ├── fonts.rs            # Embedded font data
│   │       ├── widgets/            # Custom Vizia widgets
│   │       │   ├── mod.rs
│   │       │   ├── knob.rs
│   │       │   ├── slider.rs
│   │       │   ├── button.rs
│   │       │   ├── meter.rs
│   │       │   ├── spectrum.rs
│   │       │   ├── waveform.rs
│   │       │   ├── vectorscope.rs
│   │       │   ├── curve_editor.rs
│   │       │   └── xy_pad.rs
│   │       ├── components/         # Composite UI components
│   │       │   ├── mod.rs
│   │       │   ├── header.rs       # Plugin header bar
│   │       │   ├── preset_browser.rs
│   │       │   └── ab_panel.rs
│   │       └── drawing/            # Vector graphics primitives
│   │           ├── mod.rs
│   │           └── primitives.rs
│   │
│   ├── oasis_eq/                   # EQ plugin
│   ├── oasis_comp/                 # Compressor plugin
│   ├── oasis_verb/                 # Reverb plugin
│   ├── oasis_delay/                # Delay plugin
│   ├── oasis_drive/                # Saturator plugin
│   ├── oasis_limit/                # Limiter plugin
│   ├── oasis_punch/                # Transient shaper plugin
│   ├── oasis_deess/                # De-esser plugin
│   ├── oasis_wide/                 # Stereo widener plugin
│   ├── oasis_shift/                # Pitch shifter plugin
│   ├── oasis_pump/                 # Volume shaper plugin
│   └── oasis_synth/                # Synthesizer plugin
│
├── xtask/                          # Build automation
│   ├── Cargo.toml
│   └── src/
│       └── main.rs                 # Bundle, test, release commands
│
└── tests/                          # Integration tests
    ├── audio_thread_safety.rs
    ├── preset_compatibility.rs
    └── parameter_automation.rs
```

### Why This Structure

- **Two shared crates** (`oasis_core`, `oasis_ui`) contain everything reusable
- **Each plugin is a thin shell** that composes shared systems
- **xtask handles complexity** of bundling and testing
- **Flat plugin structure** — no nested hierarchies, easy to navigate

---

## Build System

### Makefile (Entry Point)

```makefile
.PHONY: all build release test clean bundle

# Default: build all plugins (debug)
all: build

# Build all plugins (debug)
build:
	cargo build --workspace

# Build all plugins (release, optimized)
release:
	cargo build --workspace --release

# Bundle all plugins into .vst3 and .clap
bundle:
	cargo xtask bundle --release

# Run all tests
test:
	cargo test --workspace
	cargo xtask test-audio-thread

# Clean build artifacts
clean:
	cargo clean

# Build and bundle a specific plugin
%:
	cargo xtask bundle $@ --release
```

### Usage

```bash
make              # Build all (debug)
make release      # Build all (release)
make test         # Run test suite
make bundle       # Bundle all plugins
make oasis_eq     # Bundle specific plugin
```

### Cargo Workspace (Cargo.toml)

```toml
[workspace]
resolver = "2"
members = ["crates/*", "xtask"]

[workspace.package]
version = "1.0.0"
authors = ["Oasis Suite"]
edition = "2021"
license = "Proprietary"

[workspace.dependencies]
# Core framework
nih_plug = { git = "https://github.com/robbert-vdh/nih-plug.git" }
nih_plug_vizia = { git = "https://github.com/robbert-vdh/nih-plug.git" }

# Internal crates
oasis_core = { path = "crates/oasis_core" }
oasis_ui = { path = "crates/oasis_ui" }

# DSP
realfft = "3.3"              # FFT
rustfft = "6.2"              # Complex FFT if needed

# Utilities
atomic_float = "0.1"         # Atomic f32/f64
crossbeam-utils = "0.8"      # Lock-free primitives
parking_lot = "0.12"         # Fast mutexes (UI thread only)

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Build
anyhow = "1.0"               # Error handling in xtask

[profile.release]
lto = "thin"
opt-level = 3
strip = true

[profile.release-debug]
inherits = "release"
debug = true
strip = false
```

---

## Core Architecture

### The Plugin Lifecycle

```
┌─────────────────────────────────────────────────────────────────┐
│                        PLUGIN INSTANCE                          │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│   ┌─────────────┐         ┌─────────────────────────────────┐  │
│   │   PARAMS    │◄───────►│         STATE MANAGER           │  │
│   │  (Arc<T>)   │         │  (A/B, Undo/Redo, Presets)      │  │
│   └──────┬──────┘         └─────────────────────────────────┘  │
│          │                                                      │
│          │ Arc (shared ownership)                               │
│          │                                                      │
│   ┌──────▼──────┐         ┌─────────────────────────────────┐  │
│   │   AUDIO     │────────►│      LOCK-FREE CHANNEL          │  │
│   │  PROCESSOR  │ atomics │   (Meter data, waveforms)       │  │
│   └─────────────┘         └──────────────┬──────────────────┘  │
│                                          │                      │
│   ┌─────────────┐                        ▼                      │
│   │     UI      │◄───────────────────────┘                      │
│   │   (Vizia)   │                                               │
│   └─────────────┘                                               │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

### Key Architectural Decisions

#### 1. Parameters Are The Source Of Truth

All parameter values live in the `Params` struct (wrapped in `Arc`). Both audio thread and UI thread read from the same source. nih-plug handles thread-safe access via atomics internally.

```rust
// Parameters are defined once, shared everywhere
#[derive(Params)]
struct MyPluginParams {
    #[id = "gain"]
    gain: FloatParam,  // Internally uses atomics
}

// Audio thread reads via .value() or .smoothed.next()
// UI thread reads via .value()
// UI thread writes via .set_plain_value()
```

#### 2. One-Way Data Flow For Visualization

Audio thread → UI thread communication is strictly one-way via atomics or ring buffers. The UI never sends commands that affect audio processing (parameters are the exception, handled by nih-plug).

```rust
// Audio thread writes
self.meter_data.peak_l.store(peak, Ordering::Relaxed);

// UI thread reads (polling at frame rate)
let peak = self.meter_data.peak_l.load(Ordering::Relaxed);
```

#### 3. DSP Is Stateless Where Possible

DSP primitives are pure functions or minimal-state structs. State that must exist (filter memories, delay buffers) is pre-allocated and never grows.

```rust
// Good: Pre-allocated, fixed-size state
struct BiquadFilter {
    coeffs: BiquadCoeffs,
    z1: f32,
    z2: f32,
}

// The filter processes samples without allocation
fn process(&mut self, input: f32) -> f32 {
    // Pure computation using existing state
}
```

#### 4. Configuration Via Constants, Not Runtime

All magic numbers are constants. Sample-rate-dependent values are computed once at initialization and on sample rate change.

```rust
// In oasis_core/src/constants.rs
pub const MIN_FREQ_HZ: f32 = 20.0;
pub const MAX_FREQ_HZ: f32 = 20000.0;
pub const METER_DECAY_MS: f32 = 300.0;
pub const UI_REFRESH_HZ: f32 = 120.0;

// Sample-rate-dependent values computed once
struct RuntimeConfig {
    sample_rate: f32,
    meter_decay_coeff: f32,  // Computed from METER_DECAY_MS
}
```

---

## Audio Thread Contract

### The Golden Rules

These rules are **inviolable**. Every piece of code that touches the audio thread must follow them.

| Rule | Why | How We Enforce |
|------|-----|----------------|
| **No allocations** | Allocator may lock, causing audio glitches | All buffers pre-allocated; no `Vec::push`, `String`, `Box::new` |
| **No blocking** | Any wait = audio dropout | No `Mutex::lock`, no I/O, no system calls |
| **No unbounded loops** | Must complete in bounded time | All loops have fixed iteration counts |
| **No panics** | Panic = crash = bad UX | Defensive coding; handle all edge cases |
| **Denormal protection** | Denormals cause CPU spikes | Flush denormals at DSP entry points |

### Audio Thread Safe Types

```rust
// SAFE: Can be used on audio thread
- f32, f64, i32, etc. (primitives)
- AtomicF32, AtomicBool (atomics)
- &[f32], &mut [f32] (borrowed slices)
- Fixed-size arrays [f32; N]
- Structs containing only safe types

// UNSAFE: Never use on audio thread
- Vec (can reallocate)
- String (allocates)
- Box::new() (allocates)
- Mutex::lock() (blocks)
- std::io::* (system calls)
- println!, log::* (I/O)
```

### Sample Rate Independence

All time-based calculations use a `SampleRateContext` that's updated on rate changes:

```rust
pub struct SampleRateContext {
    pub sample_rate: f32,
    pub sample_rate_recip: f32,      // 1.0 / sample_rate (avoid division)
    pub nyquist: f32,                // sample_rate / 2.0
}

impl SampleRateContext {
    // Convert milliseconds to samples
    pub fn ms_to_samples(&self, ms: f32) -> f32 {
        ms * 0.001 * self.sample_rate
    }
    
    // Convert Hz to normalized frequency (0..1 = 0..nyquist)
    pub fn hz_to_normalized(&self, hz: f32) -> f32 {
        hz * self.sample_rate_recip
    }
}
```

Every DSP component receives this context and recalculates coefficients when sample rate changes:

```rust
impl BiquadFilter {
    pub fn set_sample_rate(&mut self, ctx: &SampleRateContext) {
        self.recalculate_coefficients(ctx);
    }
}
```

---

## Parameter System

### Design Goals

1. **Consistent across all plugins** — Same parameter types, same behaviors
2. **Automatic smoothing** — No clicks on automation
3. **Built-in automation support** — nih-plug handles DAW integration
4. **State snapshot capable** — For A/B and undo/redo

### Parameter Types (Wrapped nih-plug)

```rust
// Frequency parameter (20Hz - 20kHz, logarithmic)
pub fn freq_param(name: &'static str, default_hz: f32) -> FloatParam {
    FloatParam::new(
        name,
        default_hz,
        FloatRange::Skewed {
            min: MIN_FREQ_HZ,
            max: MAX_FREQ_HZ,
            factor: FloatRange::skew_factor(-2.0),  // Log scale
        },
    )
    .with_unit(" Hz")
    .with_smoother(SmoothingStyle::Logarithmic(50.0))
    .with_value_to_string(formatters::v2s_f32_hz_then_khz(2))
    .with_string_to_value(formatters::s2v_f32_hz_then_khz())
}

// Gain parameter (-inf to +12dB)
pub fn gain_param(name: &'static str, default_db: f32) -> FloatParam {
    FloatParam::new(
        name,
        util::db_to_gain(default_db),
        FloatRange::Skewed {
            min: util::db_to_gain(-80.0),
            max: util::db_to_gain(12.0),
            factor: FloatRange::gain_skew_factor(-80.0, 12.0),
        },
    )
    .with_unit(" dB")
    .with_smoother(SmoothingStyle::Logarithmic(50.0))
    .with_value_to_string(formatters::v2s_f32_gain_to_db(2))
    .with_string_to_value(formatters::s2v_f32_gain_to_db())
}

// Percentage parameter (0% - 100%)
pub fn percent_param(name: &'static str, default: f32) -> FloatParam {
    FloatParam::new(name, default, FloatRange::Linear { min: 0.0, max: 1.0 })
        .with_unit(" %")
        .with_smoother(SmoothingStyle::Linear(20.0))
        .with_value_to_string(formatters::v2s_f32_percentage(1))
        .with_string_to_value(formatters::s2v_f32_percentage())
}

// Time parameter in milliseconds
pub fn time_ms_param(name: &'static str, default: f32, min: f32, max: f32) -> FloatParam {
    FloatParam::new(
        name,
        default,
        FloatRange::Skewed {
            min,
            max,
            factor: FloatRange::skew_factor(-1.0),
        },
    )
    .with_unit(" ms")
    .with_smoother(SmoothingStyle::Linear(20.0))
    .with_value_to_string(Arc::new(|v| format!("{:.1} ms", v)))
    .with_string_to_value(Arc::new(|s| s.trim().trim_end_matches(" ms").parse().ok()))
}
```

### Parameter Grouping

```rust
#[derive(Params)]
pub struct CompressorParams {
    // Grouped visually and logically
    #[nested(group = "input")]
    pub input: InputParams,
    
    #[nested(group = "dynamics")]
    pub dynamics: DynamicsParams,
    
    #[nested(group = "output")]
    pub output: OutputParams,
}

#[derive(Params)]
pub struct DynamicsParams {
    #[id = "threshold"]
    pub threshold: FloatParam,
    
    #[id = "ratio"]
    pub ratio: FloatParam,
    
    #[id = "attack"]
    pub attack: FloatParam,
    
    #[id = "release"]
    pub release: FloatParam,
}
```

---

## State Management

### A/B Comparison

Two complete parameter state snapshots, instantly switchable:

```rust
pub struct ABState<P: Params> {
    current: ABSlot,           // Which slot is active
    slot_a: ParamSnapshot,     // Serialized parameter state
    slot_b: ParamSnapshot,
}

#[derive(Clone, Copy, PartialEq)]
pub enum ABSlot { A, B }

impl<P: Params> ABState<P> {
    // Toggle between A and B
    pub fn toggle(&mut self, params: &P) {
        self.save_current(params);
        self.current = match self.current {
            ABSlot::A => ABSlot::B,
            ABSlot::B => ABSlot::A,
        };
        self.load_current(params);
    }
    
    // Copy current slot to the other
    pub fn copy_to_other(&mut self, params: &P) {
        self.save_current(params);
        match self.current {
            ABSlot::A => self.slot_b = self.slot_a.clone(),
            ABSlot::B => self.slot_a = self.slot_b.clone(),
        }
    }
}
```

### Undo/Redo

Ring buffer of parameter changes with efficient state reconstruction:

```rust
pub struct UndoHistory {
    // Store deltas, not full snapshots (memory efficient)
    changes: RingBuffer<ParamChange>,
    position: usize,  // Current position in history
}

pub struct ParamChange {
    param_id: &'static str,
    old_value: f32,
    new_value: f32,
    timestamp: u64,
}

impl UndoHistory {
    pub fn record(&mut self, param_id: &'static str, old: f32, new: f32) {
        // Truncate any redo history
        self.changes.truncate(self.position);
        self.changes.push(ParamChange {
            param_id,
            old_value: old,
            new_value: new,
            timestamp: now(),
        });
        self.position = self.changes.len();
    }
    
    pub fn undo(&mut self, params: &impl Params) -> bool {
        if self.position == 0 { return false; }
        self.position -= 1;
        let change = &self.changes[self.position];
        // Apply old value
        params.set_value(change.param_id, change.old_value);
        true
    }
    
    pub fn redo(&mut self, params: &impl Params) -> bool {
        if self.position >= self.changes.len() { return false; }
        let change = &self.changes[self.position];
        params.set_value(change.param_id, change.new_value);
        self.position += 1;
        true
    }
}
```

---

## DSP Library

### Module Organization

```
oasis_core/src/dsp/
├── mod.rs              # Re-exports all public types
├── filter.rs           # Filters (biquad, SVF, etc.)
├── dynamics.rs         # Dynamics processing
├── delay.rs            # Delay lines
├── oscillator.rs       # Wavetable, classic waveforms
├── envelope.rs         # ADSR, envelope follower
├── lfo.rs              # LFO with shapes
├── waveshaper.rs       # Saturation curves
├── reverb.rs           # FDN, diffusion
├── pitch.rs            # Pitch shifting
├── fft.rs              # Spectrum analysis
├── oversampler.rs      # Up/downsampling
└── simd.rs             # SIMD abstractions
```

### Filter System

```rust
// Unified filter interface
pub trait Filter {
    fn process(&mut self, input: f32) -> f32;
    fn process_block(&mut self, buffer: &mut [f32]);
    fn reset(&mut self);
    fn set_sample_rate(&mut self, sample_rate: f32);
}

// Biquad filter (most common)
pub struct Biquad {
    coeffs: BiquadCoeffs,
    z1: f32,
    z2: f32,
}

pub struct BiquadCoeffs {
    pub b0: f32, pub b1: f32, pub b2: f32,
    pub a1: f32, pub a2: f32,
}

impl Biquad {
    pub fn lowpass(freq: f32, q: f32, sample_rate: f32) -> Self { ... }
    pub fn highpass(freq: f32, q: f32, sample_rate: f32) -> Self { ... }
    pub fn bandpass(freq: f32, q: f32, sample_rate: f32) -> Self { ... }
    pub fn notch(freq: f32, q: f32, sample_rate: f32) -> Self { ... }
    pub fn peak(freq: f32, q: f32, gain_db: f32, sample_rate: f32) -> Self { ... }
    pub fn low_shelf(freq: f32, q: f32, gain_db: f32, sample_rate: f32) -> Self { ... }
    pub fn high_shelf(freq: f32, q: f32, gain_db: f32, sample_rate: f32) -> Self { ... }
}

// State Variable Filter (for modulation, good at high frequencies)
pub struct SVF {
    g: f32,    // Frequency coefficient
    k: f32,    // Damping
    ic1eq: f32,
    ic2eq: f32,
}
```

### Dynamics System

```rust
// Envelope detection modes
pub enum DetectionMode {
    Peak,
    Rms { window_ms: f32 },
    TruePeak,  // With oversampling
}

pub struct EnvelopeDetector {
    mode: DetectionMode,
    attack_coeff: f32,
    release_coeff: f32,
    envelope: f32,
    rms_buffer: Option<RingBuffer<f32>>,  // Pre-allocated for RMS mode
}

impl EnvelopeDetector {
    pub fn process(&mut self, input: f32) -> f32 {
        let level = match self.mode {
            DetectionMode::Peak => input.abs(),
            DetectionMode::Rms { .. } => self.calculate_rms(input),
            DetectionMode::TruePeak => self.calculate_true_peak(input),
        };
        
        let coeff = if level > self.envelope {
            self.attack_coeff
        } else {
            self.release_coeff
        };
        
        self.envelope = self.envelope + coeff * (level - self.envelope);
        self.envelope
    }
}

// Gain computer (determines gain reduction)
pub struct GainComputer {
    threshold_db: f32,
    ratio: f32,         // 1.0 = no compression, inf = limiting
    knee_db: f32,       // Soft knee width
}

impl GainComputer {
    pub fn compute_gain_db(&self, input_db: f32) -> f32 {
        if input_db < self.threshold_db - self.knee_db / 2.0 {
            // Below knee: no compression
            0.0
        } else if input_db > self.threshold_db + self.knee_db / 2.0 {
            // Above knee: full compression
            (self.threshold_db - input_db) * (1.0 - 1.0 / self.ratio)
        } else {
            // In knee: smooth transition
            self.soft_knee_gain(input_db)
        }
    }
}
```

### Delay Line

```rust
pub struct DelayLine {
    buffer: Vec<f32>,      // Pre-allocated, fixed size
    write_pos: usize,
    max_delay_samples: usize,
}

impl DelayLine {
    pub fn new(max_delay_samples: usize) -> Self {
        Self {
            buffer: vec![0.0; max_delay_samples],
            write_pos: 0,
            max_delay_samples,
        }
    }
    
    pub fn write(&mut self, sample: f32) {
        self.buffer[self.write_pos] = sample;
        self.write_pos = (self.write_pos + 1) % self.max_delay_samples;
    }
    
    // Read with fractional delay (for modulated delays)
    pub fn read_linear(&self, delay_samples: f32) -> f32 {
        let delay_int = delay_samples as usize;
        let frac = delay_samples - delay_int as f32;
        
        let idx0 = (self.write_pos + self.max_delay_samples - delay_int - 1) % self.max_delay_samples;
        let idx1 = (idx0 + self.max_delay_samples - 1) % self.max_delay_samples;
        
        self.buffer[idx0] * (1.0 - frac) + self.buffer[idx1] * frac
    }
    
    // Higher quality cubic interpolation
    pub fn read_cubic(&self, delay_samples: f32) -> f32 {
        // Hermite interpolation for smoother modulated delays
        ...
    }
}
```

### Oscillator System (For Synth)

```rust
// Band-limited waveforms via wavetable
pub struct Oscillator {
    phase: f32,                  // 0.0 to 1.0
    phase_increment: f32,        // Set by frequency
    wavetable: &'static [f32],   // Baked wavetable
}

// Wavetables generated at compile time
pub static SINE_TABLE: [f32; 2048] = generate_sine();
pub static SAW_TABLE: [f32; 2048] = generate_saw_bandlimited();
pub static SQUARE_TABLE: [f32; 2048] = generate_square_bandlimited();
pub static TRI_TABLE: [f32; 2048] = generate_tri_bandlimited();

impl Oscillator {
    pub fn set_frequency(&mut self, freq_hz: f32, sample_rate: f32) {
        self.phase_increment = freq_hz / sample_rate;
    }
    
    pub fn next_sample(&mut self) -> f32 {
        let idx = self.phase * self.wavetable.len() as f32;
        let sample = self.read_interpolated(idx);
        
        self.phase += self.phase_increment;
        if self.phase >= 1.0 {
            self.phase -= 1.0;
        }
        
        sample
    }
}
```

---

## UI Framework

### Visual Theme: Monochrome Minimalist

Inspired by Apple's design language — clean, spacious, sophisticated.

```rust
// oasis_ui/src/theme.rs

pub mod colors {
    use vizia::vg::Color;
    
    // Base palette (monochrome)
    pub const BG_DARK: Color = Color::rgb(18, 18, 18);       // #121212
    pub const BG_MID: Color = Color::rgb(28, 28, 30);        // #1c1c1e
    pub const BG_LIGHT: Color = Color::rgb(44, 44, 46);      // #2c2c2e
    
    pub const FG_PRIMARY: Color = Color::rgb(255, 255, 255); // #ffffff
    pub const FG_SECONDARY: Color = Color::rgb(152, 152, 157); // #98989d
    pub const FG_TERTIARY: Color = Color::rgb(99, 99, 102);  // #636366
    
    // Accent (subtle)
    pub const ACCENT: Color = Color::rgb(10, 132, 255);      // #0a84ff (Apple blue)
    pub const ACCENT_DIM: Color = Color::rgba(10, 132, 255, 128);
    
    // Meters
    pub const METER_GREEN: Color = Color::rgb(52, 199, 89);  // #34c759
    pub const METER_YELLOW: Color = Color::rgb(255, 214, 10); // #ffd60a
    pub const METER_RED: Color = Color::rgb(255, 69, 58);    // #ff453a
    
    // Interactive states
    pub const HOVER: Color = Color::rgba(255, 255, 255, 20);
    pub const ACTIVE: Color = Color::rgba(255, 255, 255, 40);
}

pub mod spacing {
    pub const UNIT: f32 = 4.0;       // Base unit (4px)
    pub const XS: f32 = 4.0;         // 1 unit
    pub const SM: f32 = 8.0;         // 2 units
    pub const MD: f32 = 16.0;        // 4 units
    pub const LG: f32 = 24.0;        // 6 units
    pub const XL: f32 = 32.0;        // 8 units
    pub const XXL: f32 = 48.0;       // 12 units
}

pub mod typography {
    pub const FONT_FAMILY: &str = "SF Pro Display";  // Embedded
    pub const FONT_SIZE_XS: f32 = 10.0;
    pub const FONT_SIZE_SM: f32 = 12.0;
    pub const FONT_SIZE_MD: f32 = 14.0;
    pub const FONT_SIZE_LG: f32 = 18.0;
    pub const FONT_SIZE_XL: f32 = 24.0;
    pub const FONT_WEIGHT_REGULAR: u32 = 400;
    pub const FONT_WEIGHT_MEDIUM: u32 = 500;
    pub const FONT_WEIGHT_SEMIBOLD: u32 = 600;
}

pub mod animation {
    pub const TRANSITION_FAST: f32 = 0.1;   // 100ms
    pub const TRANSITION_NORMAL: f32 = 0.2; // 200ms
    pub const METER_ATTACK_MS: f32 = 5.0;
    pub const METER_RELEASE_MS: f32 = 300.0;
}

pub mod dimensions {
    pub const KNOB_SIZE_SM: f32 = 40.0;
    pub const KNOB_SIZE_MD: f32 = 56.0;
    pub const KNOB_SIZE_LG: f32 = 72.0;
    pub const METER_WIDTH: f32 = 12.0;
    pub const HEADER_HEIGHT: f32 = 40.0;
}
```

### Widget System

All widgets are custom Vizia views that draw with vector graphics:

```rust
// Knob widget
pub struct Knob {
    // Binding to parameter
    param: ParamBinding,
    // Visual style
    size: f32,
    arc_width: f32,
    show_value: bool,
}

impl View for Knob {
    fn draw(&self, cx: &mut DrawContext, canvas: &mut Canvas) {
        let bounds = cx.bounds();
        let center = (bounds.center_x(), bounds.center_y());
        let radius = bounds.width().min(bounds.height()) / 2.0 - self.arc_width / 2.0;
        
        // Background arc
        let mut path = Path::new();
        path.arc(center.0, center.1, radius, START_ANGLE, END_ANGLE, Solidity::Hole);
        canvas.stroke_path(&path, &paint_bg);
        
        // Value arc
        let value_angle = lerp(START_ANGLE, END_ANGLE, self.param.normalized_value());
        path.reset();
        path.arc(center.0, center.1, radius, START_ANGLE, value_angle, Solidity::Hole);
        canvas.stroke_path(&path, &paint_value);
        
        // Center indicator dot
        let dot_pos = polar_to_cartesian(center, radius * 0.6, value_angle);
        canvas.fill_circle(dot_pos.0, dot_pos.1, 3.0, &paint_dot);
    }
}
```

### Font Embedding

```rust
// oasis_ui/src/fonts.rs

// Font data embedded at compile time
pub static SF_PRO_REGULAR: &[u8] = include_bytes!("../assets/fonts/SFProDisplay-Regular.otf");
pub static SF_PRO_MEDIUM: &[u8] = include_bytes!("../assets/fonts/SFProDisplay-Medium.otf");
pub static SF_PRO_SEMIBOLD: &[u8] = include_bytes!("../assets/fonts/SFProDisplay-Semibold.otf");

// Alternative: Inter (open source, excellent for UI)
pub static INTER_REGULAR: &[u8] = include_bytes!("../assets/fonts/Inter-Regular.otf");
pub static INTER_MEDIUM: &[u8] = include_bytes!("../assets/fonts/Inter-Medium.otf");
pub static INTER_SEMIBOLD: &[u8] = include_bytes!("../assets/fonts/Inter-SemiBold.otf");

pub fn register_fonts(cx: &mut Context) {
    cx.add_font_mem("Inter", INTER_REGULAR);
    cx.add_font_mem("Inter Medium", INTER_MEDIUM);
    cx.add_font_mem("Inter SemiBold", INTER_SEMIBOLD);
}
```

---

## Plugin Template

Every plugin follows the same structure:

```
oasis_eq/
├── Cargo.toml
└── src/
    ├── lib.rs          # Plugin entry point, exports
    ├── plugin.rs       # Plugin struct, Plugin trait impl
    ├── params.rs       # Parameter definitions
    ├── dsp.rs          # Plugin-specific DSP
    ├── ui.rs           # UI layout
    └── presets.rs      # Factory presets (const arrays)
```

### Plugin Entry Point (lib.rs)

```rust
use nih_plug::prelude::*;

mod plugin;
mod params;
mod dsp;
mod ui;
mod presets;

pub use plugin::OasisEq;

nih_export_clap!(OasisEq);
nih_export_vst3!(OasisEq);
```

### Plugin Struct (plugin.rs)

```rust
use nih_plug::prelude::*;
use nih_plug_vizia::ViziaState;
use oasis_core::prelude::*;
use std::sync::Arc;

use crate::params::OasisEqParams;
use crate::dsp::EqProcessor;
use crate::ui;

pub struct OasisEq {
    params: Arc<OasisEqParams>,
    processor: EqProcessor,
    sample_rate_ctx: SampleRateContext,
    
    // UI communication (audio → UI only)
    meter_data: Arc<MeterData>,
    spectrum_data: Arc<SpectrumData>,
}

impl Default for OasisEq {
    fn default() -> Self {
        Self {
            params: Arc::new(OasisEqParams::default()),
            processor: EqProcessor::new(),
            sample_rate_ctx: SampleRateContext::new(44100.0),
            meter_data: Arc::new(MeterData::default()),
            spectrum_data: Arc::new(SpectrumData::default()),
        }
    }
}

impl Plugin for OasisEq {
    const NAME: &'static str = "Oasis EQ";
    const VENDOR: &'static str = "Oasis Suite";
    const URL: &'static str = "https://oasis-suite.com";
    const EMAIL: &'static str = "";
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    
    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(2),
            main_output_channels: NonZeroU32::new(2),
            ..AudioIOLayout::const_default()
        },
    ];
    
    type SysExMessage = ();
    type BackgroundTask = ();
    
    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }
    
    fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        ui::create_editor(
            self.params.clone(),
            self.meter_data.clone(),
            self.spectrum_data.clone(),
            self.params.editor_state.clone(),
        )
    }
    
    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        self.sample_rate_ctx = SampleRateContext::new(buffer_config.sample_rate);
        self.processor.set_sample_rate(&self.sample_rate_ctx);
        true
    }
    
    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        // Denormal protection
        let _guard = DenormalGuard::new();
        
        for mut channel_samples in buffer.iter_samples() {
            // Get smoothed parameter values
            let params = self.processor.update_params(&self.params);
            
            // Process stereo pair
            let (left, right) = channel_samples.split_at_mut(1);
            let l = left[0];
            let r = right[0];
            
            let (out_l, out_r) = self.processor.process_stereo(l, r, &params);
            left[0] = out_l;
            right[0] = out_r;
            
            // Update meters (atomic, lock-free)
            self.meter_data.update(out_l, out_r);
        }
        
        // Update spectrum (less frequently, still lock-free)
        self.spectrum_data.update_from_buffer(buffer);
        
        ProcessStatus::Normal
    }
}
```

---

## Debug Infrastructure

### Debug Feature Flags

```toml
# In Cargo.toml for each plugin
[features]
default = []
debug-dsp = []        # DSP debugging overlays
debug-metrics = []    # Performance metrics
debug-logging = []    # Lock-free logging
debug-all = ["debug-dsp", "debug-metrics", "debug-logging"]
```

### Lock-Free Logger

Logging on the audio thread via a ring buffer (written off-thread):

```rust
pub struct AudioLogger {
    buffer: RingBuffer<LogEntry>,
    enabled: AtomicBool,
}

pub struct LogEntry {
    timestamp: u64,
    level: LogLevel,
    message: ArrayString<128>,  // Fixed size, no allocation
}

impl AudioLogger {
    // Safe to call on audio thread
    pub fn log(&self, level: LogLevel, msg: &str) {
        if !self.enabled.load(Ordering::Relaxed) { return; }
        
        let entry = LogEntry {
            timestamp: now_cycles(),
            level,
            message: ArrayString::from(msg).unwrap_or_default(),
        };
        
        // Non-blocking push (drops if full)
        let _ = self.buffer.try_push(entry);
    }
}

// Background thread drains the buffer to disk/console
fn log_drain_thread(logger: Arc<AudioLogger>) {
    loop {
        while let Some(entry) = logger.buffer.try_pop() {
            eprintln!("[{:?}] {}", entry.level, entry.message);
        }
        std::thread::sleep(Duration::from_millis(100));
    }
}
```

### Performance Metrics

```rust
pub struct DspMetrics {
    // Atomic, safe for audio thread
    process_cycles: AtomicU64,
    sample_count: AtomicU64,
    peak_cycles: AtomicU64,
}

impl DspMetrics {
    pub fn measure<F, R>(&self, f: F) -> R 
    where F: FnOnce() -> R 
    {
        let start = rdtsc();
        let result = f();
        let elapsed = rdtsc() - start;
        
        self.process_cycles.fetch_add(elapsed, Ordering::Relaxed);
        self.sample_count.fetch_add(1, Ordering::Relaxed);
        self.peak_cycles.fetch_max(elapsed, Ordering::Relaxed);
        
        result
    }
    
    pub fn average_cycles(&self) -> f64 {
        let cycles = self.process_cycles.load(Ordering::Relaxed);
        let samples = self.sample_count.load(Ordering::Relaxed);
        if samples == 0 { 0.0 } else { cycles as f64 / samples as f64 }
    }
}
```

### Visual Debug Overlays

Conditional compilation for debug views:

```rust
#[cfg(feature = "debug-dsp")]
fn draw_debug_overlay(&self, cx: &mut DrawContext, canvas: &mut Canvas) {
    // Show filter frequency response
    // Show envelope detector state
    // Show parameter smoothing curves
}
```

---

## Data Flow

### Audio Thread → UI Thread

```
┌─────────────────┐
│   AUDIO THREAD  │
│                 │
│  ┌───────────┐  │
│  │ Process() │  │
│  └─────┬─────┘  │
│        │        │
│        ▼        │
│  ┌───────────┐  │      ┌───────────────────┐
│  │  Atomics  │──┼─────►│     UI THREAD     │
│  │ (meters)  │  │      │                   │
│  └───────────┘  │      │  ┌─────────────┐  │
│        │        │      │  │ Read atomic │  │
│        ▼        │      │  │ every frame │  │
│  ┌───────────┐  │      │  └─────────────┘  │
│  │   Ring    │──┼─────►│         │         │
│  │  Buffer   │  │      │         ▼         │
│  │(spectrum) │  │      │  ┌─────────────┐  │
│  └───────────┘  │      │  │   Render    │  │
│                 │      │  └─────────────┘  │
└─────────────────┘      └───────────────────┘
```

### Parameter Updates

```
┌─────────────────┐
│    UI THREAD    │
│                 │
│  User drags     │
│  knob           │
│        │        │
│        ▼        │
│  set_plain_     │
│  value()        │
│        │        │
└────────┼────────┘
         │
         │  (nih-plug handles
         │   thread-safe update)
         │
         ▼
┌─────────────────┐
│   AUDIO THREAD  │
│                 │
│  .smoothed      │
│  .next()        │◄─── Smooth transition
│        │        │     over N samples
│        ▼        │
│  DSP uses       │
│  smoothed value │
│                 │
└─────────────────┘
```

---

## Preset System

### Factory Presets (Baked In)

```rust
// presets.rs

use oasis_core::preset::FactoryPreset;

pub const FACTORY_PRESETS: &[FactoryPreset] = &[
    FactoryPreset {
        name: "Init",
        category: "Default",
        author: "Oasis Suite",
        params: &[
            ("gain", 0.0),
            ("threshold", -20.0),
            ("ratio", 4.0),
            ("attack", 10.0),
            ("release", 100.0),
        ],
    },
    FactoryPreset {
        name: "Vocal Glue",
        category: "Vocals",
        author: "Oasis Suite",
        params: &[
            ("gain", 0.0),
            ("threshold", -18.0),
            ("ratio", 3.0),
            ("attack", 15.0),
            ("release", 150.0),
        ],
    },
    // ... more presets
];
```

### User Presets (.oasis format)

```rust
// Simple JSON-based format
#[derive(Serialize, Deserialize)]
pub struct OasisPreset {
    pub magic: u32,              // "OASI" = 0x4941534F
    pub version: u32,            // Format version
    pub plugin_id: String,       // "oasis_comp", "oasis_eq", etc.
    pub plugin_version: String,  // "1.0.0"
    pub name: String,
    pub author: String,
    pub params: HashMap<String, f32>,
}

impl OasisPreset {
    pub fn save(&self, path: &Path) -> Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }
    
    pub fn load(path: &Path) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let preset: Self = serde_json::from_str(&json)?;
        
        // Validate plugin type
        if preset.magic != 0x4941534F {
            return Err("Invalid preset file".into());
        }
        
        Ok(preset)
    }
}
```

---

## Testing Strategy

### Test Categories

| Category | Location | Runs On |
|----------|----------|---------|
| Unit tests | `#[cfg(test)]` in each module | `cargo test` |
| Integration tests | `tests/` directory | `cargo test` |
| Audio thread safety | `xtask test-audio-thread` | `make test` |
| Preset compatibility | `tests/preset_compatibility.rs` | `cargo test` |

### Audio Thread Safety Tests

```rust
// tests/audio_thread_safety.rs

#[test]
fn test_no_allocations_in_process() {
    // Use a custom allocator that panics on audio thread
    let plugin = OasisComp::default();
    let mut buffer = create_test_buffer(512);
    
    // This should complete without triggering allocator
    enter_audio_thread_context();
    plugin.process(&mut buffer, ...);
    exit_audio_thread_context();
}

#[test]
fn test_sample_rate_change() {
    let mut plugin = OasisComp::default();
    
    // Initialize at 44.1kHz
    plugin.initialize(44100.0);
    let output_44k = plugin.process_test_signal();
    
    // Change to 96kHz
    plugin.initialize(96000.0);
    let output_96k = plugin.process_test_signal();
    
    // Verify frequency response is consistent
    assert_frequency_response_matches(output_44k, output_96k);
}
```

### Automation Stress Test

```rust
#[test]
fn test_rapid_parameter_automation() {
    let plugin = OasisComp::default();
    let mut buffer = create_test_buffer(64);  // Small buffer
    
    for _ in 0..10000 {
        // Randomly change parameters
        plugin.params().threshold.set_plain_value(random_range(-60.0, 0.0));
        plugin.params().ratio.set_plain_value(random_range(1.0, 20.0));
        
        // Process (should not click/pop)
        plugin.process(&mut buffer, ...);
    }
    
    // Verify no discontinuities in output
    assert_no_discontinuities(&buffer);
}
```

---

## Summary: What Makes This Architecture Elegant

### Minimal Moving Parts

1. **Two shared crates** — All reusable code lives in `oasis_core` and `oasis_ui`
2. **One parameter system** — nih-plug params with thin extensions
3. **One communication pattern** — Atomics for scalars, ring buffers for arrays
4. **One state model** — Parameters are the source of truth

### Maximum Capability

1. **Any DSP algorithm** — Primitives compose into anything
2. **Any UI layout** — Vizia widgets are flexible
3. **Any sample rate** — Everything scales correctly
4. **Any automation rate** — Smoothing handles it

### Zero Surprises

1. **Audio thread rules are absolute** — No exceptions, no "just this once"
2. **Data flows one way** — Audio → UI, never the reverse
3. **State is explicit** — A/B, undo, presets all use the same snapshot mechanism
4. **Debug tools are built in** — Not bolted on later

---

*This architecture is complete. Build order: `oasis_core` → `oasis_ui` → individual plugins starting with `oasis_eq`.*
