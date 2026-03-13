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
9. [Multi-Band and Crossover Architecture](#multi-band-and-crossover-architecture)
10. [Mid/Side Processing](#midside-processing)
11. [Latency Reporting](#latency-reporting)
12. [Error Handling Strategy](#error-handling-strategy)
13. [UI Framework](#ui-framework)
14. [Plugin Template](#plugin-template)
15. [Debug Infrastructure](#debug-infrastructure)
16. [Data Flow](#data-flow)
17. [Preset System](#preset-system)
18. [Testing Strategy](#testing-strategy)

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
- Pre-allocated Vec/Box (allocated once during init, never resized on audio thread)
- Structs containing only safe types

// FORBIDDEN on audio thread (allocations/blocking)
- Vec::push, Vec::extend, Vec::resize (can reallocate)
- String::new, format!, to_string (allocates)
- Box::new() (allocates)
- Mutex::lock() (blocks)
- std::io::* (system calls)
- println!, log::*, eprintln! (I/O)
- HashMap/BTreeMap operations (can allocate)
```

**Clarification:** A `Vec<f32>` used as a pre-allocated buffer (created during `initialize()`, indexed but never grown on the audio thread) is perfectly safe. The danger is any operation that could trigger reallocation.

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

### How nih-plug State Works

nih-plug serializes parameter state via serde. Each `Params` struct can be snapshot to/from JSON using `nih_plug::params::serialize` and `nih_plug::params::deserialize`. A/B and Undo/Redo are built on top of this.

### A/B Comparison

Two complete parameter state snapshots stored as serialized JSON, instantly switchable:

```rust
pub struct ABState {
    current: ABSlot,
    slot_a: Option<serde_json::Value>,  // Serialized param state
    slot_b: Option<serde_json::Value>,
}

#[derive(Clone, Copy, PartialEq)]
pub enum ABSlot { A, B }

impl ABState {
    pub fn new() -> Self {
        Self { current: ABSlot::A, slot_a: None, slot_b: None }
    }

    /// Snapshot current params into the active slot, load the other slot
    pub fn toggle(&mut self, setter: &ParamSetter, params: &impl Params) {
        let snapshot = nih_plug::params::serialize::serialize_json(params);
        match self.current {
            ABSlot::A => {
                self.slot_a = Some(snapshot);
                if let Some(ref state) = self.slot_b {
                    nih_plug::params::serialize::deserialize_json(params, state.clone(), setter);
                }
                self.current = ABSlot::B;
            }
            ABSlot::B => {
                self.slot_b = Some(snapshot);
                if let Some(ref state) = self.slot_a {
                    nih_plug::params::serialize::deserialize_json(params, state.clone(), setter);
                }
                self.current = ABSlot::A;
            }
        }
    }

    pub fn copy_to_other(&mut self, params: &impl Params) {
        let snapshot = nih_plug::params::serialize::serialize_json(params);
        match self.current {
            ABSlot::A => self.slot_b = Some(snapshot),
            ABSlot::B => self.slot_a = Some(snapshot),
        }
    }
}
```

### Undo/Redo

Delta-based history using a `VecDeque` with a max capacity (not a ring buffer, since we need random access and truncation):

```rust
use std::collections::VecDeque;

const MAX_UNDO_STEPS: usize = 128;

pub struct UndoHistory {
    changes: VecDeque<ParamChange>,
    position: usize,
}

pub struct ParamChange {
    param_hash: u32,          // nih-plug param hash (stable identifier)
    old_normalized: f32,      // Normalized 0..1 value before change
    new_normalized: f32,      // Normalized 0..1 value after change
}

impl UndoHistory {
    pub fn new() -> Self {
        Self { changes: VecDeque::with_capacity(MAX_UNDO_STEPS), position: 0 }
    }

    pub fn record(&mut self, param_hash: u32, old_normalized: f32, new_normalized: f32) {
        // Discard any redo-able future beyond current position
        self.changes.truncate(self.position);
        self.changes.push_back(ParamChange {
            param_hash,
            old_normalized,
            new_normalized,
        });
        // Evict oldest if over capacity
        if self.changes.len() > MAX_UNDO_STEPS {
            self.changes.pop_front();
        }
        self.position = self.changes.len();
    }

    /// Undo: look up param by hash via ParamSetter, apply old_normalized
    pub fn undo(&mut self, setter: &ParamSetter, param_lookup: &dyn Fn(u32) -> Option<&dyn Param>) -> bool {
        if self.position == 0 { return false; }
        self.position -= 1;
        let change = &self.changes[self.position];
        if let Some(param) = param_lookup(change.param_hash) {
            setter.set_parameter_normalized(param, change.old_normalized);
        }
        true
    }

    /// Redo: apply new_normalized
    pub fn redo(&mut self, setter: &ParamSetter, param_lookup: &dyn Fn(u32) -> Option<&dyn Param>) -> bool {
        if self.position >= self.changes.len() { return false; }
        let change = &self.changes[self.position];
        if let Some(param) = param_lookup(change.param_hash) {
            setter.set_parameter_normalized(param, change.new_normalized);
        }
        self.position += 1;
        true
    }
}
```

**Integration:** The UI layer hooks into parameter change callbacks. When the user adjusts a knob, the widget records the before/after normalized values into `UndoHistory`. A/B snapshots are taken when the user clicks the A/B toggle — this is a UI-thread-only operation and never touches the audio thread.

---

## DSP Library

All DSP primitives live in `oasis_core/src/dsp/` (see [Project Structure](#project-structure) for the full file listing). Each module is a focused, composable building block.

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

A single wavetable per waveform would alias at higher frequencies. Professional playback requires **multi-resolution wavetables** — one table per octave range, each with harmonics rolled off at that range's Nyquist.

```rust
const TABLE_SIZE: usize = 2048;
const NUM_OCTAVE_TABLES: usize = 11;  // Covers MIDI range ~16Hz to ~16kHz

/// One waveform stored at multiple resolutions for alias-free playback
pub struct WavetableSet {
    /// tables[0] = full harmonics (low frequencies)
    /// tables[10] = fundamental only (highest frequencies)
    tables: [[f32; TABLE_SIZE]; NUM_OCTAVE_TABLES],
}

pub struct Oscillator {
    phase: f32,
    phase_increment: f32,
    current_table_index: usize,
}

impl Oscillator {
    pub fn set_frequency(&mut self, freq_hz: f32, sample_rate: f32) {
        self.phase_increment = freq_hz / sample_rate;
        // Select the table whose highest harmonic stays below Nyquist
        let max_harmonic = (sample_rate * 0.5 / freq_hz) as usize;
        self.current_table_index = octave_index_from_harmonic_count(max_harmonic);
    }

    pub fn next_sample(&mut self, wavetable: &WavetableSet) -> f32 {
        let table = &wavetable.tables[self.current_table_index];
        let idx = self.phase * TABLE_SIZE as f32;
        let sample = interpolate_cubic(table, idx);

        self.phase += self.phase_increment;
        if self.phase >= 1.0 { self.phase -= 1.0; }
        sample
    }
}

// Wavetable sets generated at init time (too large for const eval)
// Built once in initialize(), stored in plugin struct, never reallocated
pub fn build_saw_tables() -> WavetableSet { ... }
pub fn build_square_tables() -> WavetableSet { ... }
pub fn build_tri_tables() -> WavetableSet { ... }

// Sine doesn't need multi-resolution (only fundamental)
pub static SINE_TABLE: [f32; TABLE_SIZE] = generate_sine();
```

**Crossfading between tables:** For smooth transitions when frequency changes octave boundaries, interpolate between adjacent tables using the fractional octave position. This prevents audible "popping" during portamento or pitch modulation.

---

## Multi-Band and Crossover Architecture

Several plugins offer multi-band modes (Drive, Punch, Limit, Wide). Rather than each plugin implementing its own crossover, `oasis_core` provides a shared Linkwitz-Riley crossover system.

```rust
// oasis_core/src/dsp/crossover.rs

/// Linkwitz-Riley crossover (phase-coherent sum)
/// 4th-order (LR4) = two cascaded 2nd-order Butterworth filters
pub struct CrossoverBand {
    low_pass: [Biquad; 2],   // Two cascaded for LR4
    high_pass: [Biquad; 2],
}

/// 3-band crossover for low/mid/high splits
pub struct ThreeBandCrossover {
    low_mid: CrossoverBand,   // Split at low-mid frequency
    mid_high: CrossoverBand,  // Split at mid-high frequency
}

pub struct BandBuffers {
    pub low: [f32; MAX_BLOCK_SIZE],
    pub mid: [f32; MAX_BLOCK_SIZE],
    pub high: [f32; MAX_BLOCK_SIZE],
}

impl ThreeBandCrossover {
    pub fn new(low_mid_hz: f32, mid_high_hz: f32, sample_rate: f32) -> Self { ... }

    /// Split a mono signal into three bands (pre-allocated output buffers)
    pub fn split(&mut self, input: &[f32], output: &mut BandBuffers) { ... }

    /// Recombine bands (sums back into output slice)
    pub fn recombine(bands: &BandBuffers, output: &mut [f32]) {
        for i in 0..output.len() {
            output[i] = bands.low[i] + bands.mid[i] + bands.high[i];
        }
    }

    pub fn set_frequencies(&mut self, low_mid_hz: f32, mid_high_hz: f32, sample_rate: f32) { ... }
}
```

**Usage in plugins:** Plugins that support multi-band processing hold a `ThreeBandCrossover` and per-band DSP instances. The crossover splits → each band is processed independently → bands are recombined. The Linkwitz-Riley topology guarantees flat summing when bands are unprocessed.

---

## Mid/Side Processing

Multiple plugins (Drive, Wide, EQ) support mid/side processing. Shared encode/decode utilities live in `oasis_core`:

```rust
// oasis_core/src/dsp/mid_side.rs

/// Encode stereo (L, R) to mid/side (M, S)
#[inline]
pub fn encode(left: f32, right: f32) -> (f32, f32) {
    let mid = (left + right) * 0.5;
    let side = (left - right) * 0.5;
    (mid, side)
}

/// Decode mid/side (M, S) back to stereo (L, R)
#[inline]
pub fn decode(mid: f32, side: f32) -> (f32, f32) {
    let left = mid + side;
    let right = mid - side;
    (left, right)
}

/// Block-based M/S encode for efficiency
pub fn encode_block(left: &[f32], right: &[f32], mid: &mut [f32], side: &mut [f32]) {
    for i in 0..left.len() {
        mid[i] = (left[i] + right[i]) * 0.5;
        side[i] = (left[i] - right[i]) * 0.5;
    }
}

pub fn decode_block(mid: &[f32], side: &[f32], left: &mut [f32], right: &mut [f32]) {
    for i in 0..mid.len() {
        left[i] = mid[i] + side[i];
        right[i] = mid[i] - side[i];
    }
}
```

---

## Latency Reporting

Plugins that introduce latency (lookahead, linear phase, oversampling) must report it so the DAW can compensate. nih-plug handles this via `ProcessContext::set_latency_samples()`.

### Which Plugins Report Latency

| Plugin | Source of Latency | Typical Samples |
|--------|-------------------|-----------------|
| Oasis EQ | Linear phase mode (FFT-based) | FFT size / 2 |
| Oasis Comp | Lookahead | User-set (e.g. 0–5ms → 0–220 samples @ 44.1kHz) |
| Oasis Limit | True peak / lookahead | ~64–256 samples |
| Oasis DeEss | Lookahead | Similar to Comp |
| Any plugin | Oversampling | Filter group delay × oversample factor |

### Implementation Pattern

```rust
// In the Plugin impl
fn initialize(&mut self, ..., context: &mut impl InitContext<Self>) -> bool {
    // Report latency during init
    let latency = self.processor.compute_latency_samples(&self.sample_rate_ctx);
    context.set_latency_samples(latency);
    true
}

fn process(&mut self, ..., context: &mut impl ProcessContext<Self>) -> ProcessStatus {
    // Re-report if latency-affecting parameters changed (e.g. lookahead toggled)
    if self.latency_changed.swap(false, Ordering::Relaxed) {
        let latency = self.processor.compute_latency_samples(&self.sample_rate_ctx);
        context.set_latency_samples(latency);
    }
    // ... process audio ...
}
```

**Rule:** When a latency-affecting parameter changes (lookahead on/off, linear phase toggle, oversample factor), set a flag that the audio thread checks to re-report latency. The flag is an `AtomicBool` — no allocation, no blocking.

---

## Error Handling Strategy

### Audio Thread: No Panics, No Errors — Defensive Defaults

The audio thread cannot fail. Every edge case must produce a safe output.

```rust
// NaN/Inf protection at DSP stage boundaries
#[inline]
pub fn sanitize(x: f32) -> f32 {
    if x.is_finite() { x } else { 0.0 }
}

// Apply after any nonlinear operation that could produce NaN/Inf
let output = sanitize(waveshaper_output);
```

| Edge Case | Response |
|-----------|----------|
| NaN/Inf from DSP | Replace with 0.0, log in debug builds |
| Sample rate = 0 | Default to 44100.0 |
| Buffer size = 0 | Return immediately (ProcessStatus::Normal) |
| Parameter out of range | Clamp to valid range |
| Division by zero risk | Guard with epsilon or pre-check |

### UI Thread: Graceful Recovery

```rust
// Preset loading with clear error reporting
match OasisPreset::load(path) {
    Ok(preset) => apply_preset(preset, params, setter),
    Err(PresetError::WrongPlugin { expected, got }) => {
        show_error_toast(&format!("This preset is for {}, not {}", got, expected));
    }
    Err(PresetError::CorruptFile) => {
        show_error_toast("Preset file is corrupt or unreadable");
    }
    Err(PresetError::VersionTooNew { file_ver, plugin_ver }) => {
        show_error_toast(&format!("Preset requires v{}, you have v{}", file_ver, plugin_ver));
    }
}
```

### Debug-Only Error Tracking

```rust
#[cfg(feature = "debug-dsp")]
pub struct ErrorTracker {
    nan_count: AtomicU64,
    inf_count: AtomicU64,
    denormal_count: AtomicU64,
    clamp_count: AtomicU64,
}
```

---

## UI Framework

### Visual Theme: Monochrome Minimalist

Clean, spacious, sophisticated. Dark backgrounds, white-on-dark typography, one accent color.

```rust
// oasis_ui/src/theme.rs

pub mod colors {
    use vizia::vg::Color;
    
    // Base palette (monochrome)
    pub const BG_DARK: Color = Color::rgb(18, 18, 18);        // #121212 — deepest background
    pub const BG_MID: Color = Color::rgb(28, 28, 30);         // #1c1c1e — panels, sections
    pub const BG_LIGHT: Color = Color::rgb(44, 44, 46);       // #2c2c2e — raised elements
    pub const BG_ELEVATED: Color = Color::rgb(58, 58, 60);    // #3a3a3c — dropdowns, tooltips
    
    pub const FG_PRIMARY: Color = Color::rgb(255, 255, 255);  // #ffffff
    pub const FG_SECONDARY: Color = Color::rgb(152, 152, 157);// #98989d
    pub const FG_TERTIARY: Color = Color::rgb(99, 99, 102);   // #636366
    pub const FG_DISABLED: Color = Color::rgb(72, 72, 74);    // #48484a
    
    // Accent (subtle blue)
    pub const ACCENT: Color = Color::rgb(10, 132, 255);       // #0a84ff
    pub const ACCENT_DIM: Color = Color::rgba(10, 132, 255, 128);
    
    // Meters
    pub const METER_GREEN: Color = Color::rgb(52, 199, 89);   // #34c759
    pub const METER_YELLOW: Color = Color::rgb(255, 214, 10); // #ffd60a
    pub const METER_RED: Color = Color::rgb(255, 69, 58);     // #ff453a
    
    // Interactive states
    pub const HOVER: Color = Color::rgba(255, 255, 255, 20);
    pub const ACTIVE: Color = Color::rgba(255, 255, 255, 40);
    pub const FOCUS_RING: Color = Color::rgba(10, 132, 255, 180);
}

pub mod spacing {
    pub const UNIT: f32 = 4.0;
    pub const XS: f32 = 4.0;
    pub const SM: f32 = 8.0;
    pub const MD: f32 = 16.0;
    pub const LG: f32 = 24.0;
    pub const XL: f32 = 32.0;
    pub const XXL: f32 = 48.0;
}

pub mod typography {
    pub const FONT_FAMILY: &str = "Inter";
    pub const FONT_FAMILY_MEDIUM: &str = "Inter Medium";
    pub const FONT_FAMILY_SEMIBOLD: &str = "Inter SemiBold";
    pub const FONT_SIZE_XS: f32 = 10.0;
    pub const FONT_SIZE_SM: f32 = 12.0;
    pub const FONT_SIZE_MD: f32 = 14.0;
    pub const FONT_SIZE_LG: f32 = 18.0;
    pub const FONT_SIZE_XL: f32 = 24.0;
}

pub mod animation {
    pub const TRANSITION_FAST: f32 = 0.1;
    pub const TRANSITION_NORMAL: f32 = 0.2;
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

Inter is open-source (SIL Open Font License), excellent for UI at all sizes, and legally redistributable.

```rust
// oasis_ui/src/fonts.rs

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
        let _guard = DenormalGuard::new();

        // --- See "Processing Strategies" below for which to use ---
        
        for mut channel_samples in buffer.iter_samples() {
            let params = self.processor.update_params(&self.params);
            
            let (left, right) = channel_samples.split_at_mut(1);
            let l = left[0];
            let r = right[0];
            
            let (out_l, out_r) = self.processor.process_stereo(l, r, &params);
            left[0] = out_l;
            right[0] = out_r;
            
            self.meter_data.update(out_l, out_r);
        }
        
        self.spectrum_data.update_from_buffer(buffer);
        
        ProcessStatus::Normal
    }
}
```

### Processing Strategies

Two approaches, chosen per-plugin based on DSP complexity:

#### Per-Sample Processing (simple plugins)

Used when DSP is lightweight and every sample needs freshly smoothed parameters (e.g. Comp, Punch, DeEss, Wide, Drive).

```rust
for mut channel_samples in buffer.iter_samples() {
    let gain = self.params.gain.smoothed.next();
    // ... process one stereo pair ...
}
```

**Pros:** Smoothest automation, simplest code.
**Cons:** Function call overhead per sample, can't vectorize.

#### Block-Based Processing (heavy plugins)

Used when DSP is expensive or benefits from vectorization (e.g. EQ with 8+ filter bands, Synth, Verb, Delay with modulation). Parameters are updated once per block rather than per sample.

```rust
const BLOCK_SIZE: usize = 64;

for (_, block) in buffer.iter_blocks(BLOCK_SIZE) {
    // Advance smoothers by block size, get current values
    let gain = self.params.gain.smoothed.next_block_exact(BLOCK_SIZE);
    
    // Process entire block at once (can use SIMD, vectorizes better)
    for i in 0..block.samples() {
        let l = block.get_mut(0)[i];
        let r = block.get_mut(1)[i];
        // ... process using gain[i] for per-sample smoothed value ...
    }
}
```

**Pros:** Cache-friendly, can vectorize inner loops, lower overhead.
**Cons:** Slightly more complex structure.

#### Guidance

| Plugin | Strategy | Reason |
|--------|----------|--------|
| Comp, Punch, DeEss | Per-sample | Envelope detection needs per-sample accuracy |
| Drive | Per-sample | Waveshaping with per-sample param smoothing |
| EQ | Block-based | 8+ filter passes benefit from block processing |
| Verb, Delay | Block-based | Heavy DSP, FDN/diffusion benefits from blocks |
| Synth | Block-based | Multiple oscillators + filters, must vectorize |
| Limit | Per-sample | True peak detection needs per-sample accuracy |
| Wide, Pump | Per-sample | Lightweight DSP, per-sample is fine |

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
