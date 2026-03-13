# Oasis Suite — Development Workflow

Step-by-step procedures for adding any type of content to the project. Follow these exactly.

**Before any task:** Read `ARCHITECTURE.md` for system details. Read `IDEA.md` for product vision. Read `MISTAKES.md` for known pitfalls. This document tells you the **process** — those documents tell you the **why** and **how**.

---

## Table of Contents

1. [Universal Checklist](#universal-checklist)
2. [Adding a New Plugin](#adding-a-new-plugin)
3. [Adding a New DSP Primitive to oasis_core](#adding-a-new-dsp-primitive-to-oasis_core)
4. [Adding a New Parameter to an Existing Plugin](#adding-a-new-parameter-to-an-existing-plugin)
5. [Adding a New Feature to an Existing Plugin](#adding-a-new-feature-to-an-existing-plugin)
6. [Adding a New UI Widget to oasis_ui](#adding-a-new-ui-widget-to-oasis_ui)
7. [Adding Factory Presets](#adding-factory-presets)
8. [Modifying Existing DSP](#modifying-existing-dsp)
9. [Build & Verification](#build--verification)
10. [File Location Quick Reference](#file-location-quick-reference)

---

## Universal Checklist

**Every task, no exceptions:**

1. **Read before writing.** Open and read every file you intend to modify before making changes.
2. **Check existing code.** Does something similar already exist in `oasis_core` or `oasis_ui`? Reuse it. Don't duplicate.
3. **No magic numbers.** Every value goes into `oasis_core/src/constants.rs` or the plugin's local constants. Never hardcode `20.0` when you mean `MIN_FREQ_HZ`.
4. **Audio thread is sacred.** If your code runs in `process()`: zero allocations, zero blocking, zero panics. See the audio thread contract in `ARCHITECTURE.md`.
5. **Parameter smoothing.** Every parameter that affects audio must use a smoother. No exceptions — unsmoothed params cause clicks.
6. **Denormal protection.** Wrap `process()` with `DenormalGuard::new()`. Apply `sanitize()` after nonlinear operations.
7. **Sample rate independence.** All time-based calculations go through `SampleRateContext`. Recalculate coefficients on sample rate change.
8. **Constants, not literals.** If you type a number, ask: should this be a constant?
9. **Match existing patterns.** Look at how other plugins/modules do the same thing. Follow the same structure.
10. **Build after every change.** Run `cargo build --workspace` before considering a task complete.

---

## Adding a New Plugin

Follow this order exactly. Each step must be complete before the next.

### Step 1: Create the Plugin Crate

Create the folder structure under `crates/`:

```
crates/oasis_<name>/
├── Cargo.toml
└── src/
    ├── lib.rs          # Entry point, module declarations, nih_export_vst3!
    ├── plugin.rs       # Plugin struct, Plugin trait impl
    ├── params.rs       # All parameter definitions
    ├── dsp.rs          # Plugin-specific DSP processor
    ├── ui.rs           # UI layout
    └── presets.rs      # Factory presets
```

### Step 2: Cargo.toml

```toml
[package]
name = "oasis_<name>"
version.workspace = true
authors.workspace = true
edition.workspace = true
license.workspace = true

[lib]
crate-type = ["cdylib"]

[dependencies]
nih_plug = { workspace = true }
nih_plug_vizia = { workspace = true }
oasis_core = { workspace = true }
oasis_ui = { workspace = true }

[features]
default = []
debug-dsp = []
debug-metrics = []
debug-logging = []
debug-all = ["debug-dsp", "debug-metrics", "debug-logging"]
```

Verify the new crate is picked up by the workspace — the root `Cargo.toml` uses `members = ["crates/*", "xtask"]` so it should be automatic.

### Step 3: lib.rs

```rust
use nih_plug::prelude::*;

mod plugin;
mod params;
mod dsp;
mod ui;
mod presets;

pub use plugin::Oasis<Name>;

nih_export_vst3!(Oasis<Name>);
```

### Step 4: params.rs

Define parameters using `oasis_core` helper constructors (`freq_param`, `gain_param`, `percent_param`, `time_ms_param`). Group logically with `#[nested(group = "...")]`.

Key rules:
- Every parameter must have a stable string `#[id = "..."]` — this ID is permanent and used in presets/DAW sessions
- Never change a parameter ID after release — it breaks saved sessions
- Include `editor_state: Arc<ViziaState>` for UI window state
- Set sensible defaults (the "Init" preset feeling — usable immediately, not zeroed out)
- All automated float params must have a smoother

### Step 5: dsp.rs

Create the DSP processor struct. Rules:
- Pre-allocate all buffers in `new()` or a dedicated `initialize()` method
- Implement `set_sample_rate(&mut self, ctx: &SampleRateContext)` to recalculate coefficients
- Implement `reset(&mut self)` to clear filter states/delay lines
- Compose from `oasis_core::dsp` primitives wherever possible
- Choose processing strategy (see ARCHITECTURE.md — per-sample for simple plugins, block-based for heavy ones)

### Step 6: plugin.rs

Wire everything together following the template in ARCHITECTURE.md:
- `Default` impl creates all components
- `initialize()` sets sample rate on DSP processor, reports latency if applicable
- `process()` wraps in `DenormalGuard`, reads smoothed params, runs DSP, updates meters
- `editor()` creates the Vizia UI

### Step 7: ui.rs

Build the UI using `oasis_ui` widgets and theme. Follow these requirements:

**Header bar (mandatory, identical across all plugins):**
- Plugin name (bold, left-aligned)
- Version label
- Preset browser (right-aligned): `◀ Preset Name ▶` with left/right arrow buttons

**Content area layout:**
- Two-column layout with grouped sections
- Each section has a title label (uppercase, muted color) and `ParamSlider`/`ParamButton` rows
- Use `ParamSliderStyle::Centered` for bipolar params (gain), `FromLeft` for unipolar (width, mix)
- Use `ParamButton` for bool and enum params

**Footer:**
- "Oasis Suite" branding text, small and muted

**Dark theme:** All plugins use the shared `oasis_ui::stylesheet()` which sets `#121212` background, `#1a1a1a` section panels, `#e0e0e0` text. Do NOT use `ViziaTheming::Default` — always use `ViziaTheming::Custom`.

**Preset browser implementation:** See ARCHITECTURE.md "Preset Browser UI" section. The key pattern is collecting `ParamPtr` values from the `Data` lens, dropping the borrow, then emitting `RawParamEvent`s.

### Step 8: presets.rs

Add at minimum an "Init" preset with sensible defaults. Add category-based factory presets. All values are normalized (0.0–1.0) for forward compatibility.

**Preset storage:** Use `AtomicUsize` for `CURRENT_PRESET_INDEX`. Provide `next_preset()` and `prev_preset()` functions that atomically cycle the index. The preset browser calls these from UI event handlers.

### Step 9: Build & Verify

```bash
cargo build --workspace          # Must compile clean
cargo test --workspace           # Must pass
cargo xtask bundle oasis_<name>  # Must produce valid .vst3
```

---

## Adding a New DSP Primitive to oasis_core

When you need a building block that doesn't exist yet (new filter type, new envelope follower variant, etc.).

### Step 1: Determine Location

- New file in `crates/oasis_core/src/dsp/` if it's a distinct concept (e.g., `compander.rs`)
- Add to existing file if it's a variant of something that exists (e.g., new filter type goes in `filter.rs`)

### Step 2: Implement the Primitive

Requirements:
- **Trait conformance:** Implement relevant traits (`Filter`, etc.) if applicable
- **Pre-allocated state:** All internal buffers sized at construction, never grown
- **`set_sample_rate()`:** Recalculate all coefficients
- **`reset()`:** Clear all state (filter memories, delay lines)
- **`process()` or `process_block()`:** The core DSP — pure computation, no allocation, no blocking
- **`sanitize()` after nonlinear ops:** Guard against NaN/Inf

### Step 3: Export from mod.rs

Add `pub mod <name>;` in `crates/oasis_core/src/dsp/mod.rs` and re-export important types.

### Step 4: Add Constants

Any magic numbers go in `crates/oasis_core/src/constants.rs`.

### Step 5: Write Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_output() { /* feed known input, verify output */ }
    
    #[test]
    fn test_sample_rate_independence() { /* same musical result at 44.1k and 96k */ }
    
    #[test]
    fn test_no_nan_inf() { /* edge case inputs: silence, DC, denormals */ }
    
    #[test]
    fn test_reset_clears_state() { /* process, reset, verify clean state */ }
}
```

### Step 6: Build

```bash
cargo test -p oasis_core
cargo build --workspace
```

---

## Adding a New Parameter to an Existing Plugin

### Step 1: Define the Parameter

Open `crates/oasis_<name>/src/params.rs`.

- Add the field to the appropriate param group struct
- Use `oasis_core` constructors (`freq_param`, `gain_param`, `percent_param`, `time_ms_param`) or create custom `FloatParam`/`IntParam`/`EnumParam`/`BoolParam`
- Choose a **permanent** string ID: `#[id = "my_param"]` — this can never change
- Set a sensible default value
- Add a smoother if it affects audio (`SmoothingStyle::Linear` or `SmoothingStyle::Logarithmic`)
- Add display formatting (unit string, value-to-string/string-to-value converters)

### Step 2: Wire to DSP

Open `crates/oasis_<name>/src/dsp.rs`.

- Read the parameter in the processing loop via `.value()` or `.smoothed.next()` / `.smoothed.next_block()`
- If the parameter affects filter coefficients or other derived state, add update logic that runs when the value changes (either per-sample or per-block depending on processing strategy)

### Step 3: Wire to UI

Open `crates/oasis_<name>/src/ui.rs`.

- Add a widget (knob, slider, button, etc.) bound to the new parameter
- Place it according to the layout philosophy
- Use consistent sizing from `oasis_ui::theme::dimensions`

### Step 4: Update Factory Presets

Open `crates/oasis_<name>/src/presets.rs`.

- Add the new parameter's normalized value to every existing factory preset
- If omitted, the parameter will use its default when loading old presets (forward compatibility), but factory presets should be explicit

### Step 5: Build & Check

```bash
cargo build -p oasis_<name>
cargo test -p oasis_<name>
```

Verify: Does the new parameter have smoothing? Does it display the right unit? Does the Init preset still sound right?

---

## Adding a New Feature to an Existing Plugin

A "feature" combines DSP + parameters + UI. Examples: adding a sidechain filter to the compressor, adding multi-band mode to the saturator.

### Step 1: Plan the Integration

Before writing code, answer:
- What parameters does this feature need? (names, ranges, defaults, IDs)
- What DSP does it require? Does `oasis_core` already have the primitives?
- How does it interact with the existing signal chain? (before? after? parallel? switchable?)
- Does it add latency? (if yes, must report via `context.set_latency_samples()`)
- Does it need new UI visualization? (new widget, new meter, etc.)

### Step 2: Add DSP Primitives (if needed)

Follow [Adding a New DSP Primitive to oasis_core](#adding-a-new-dsp-primitive-to-oasis_core).

### Step 3: Add Parameters

Follow [Adding a New Parameter to an Existing Plugin](#adding-a-new-parameter-to-an-existing-plugin) for each new parameter.

### Step 4: Integrate into DSP Processor

Open `crates/oasis_<name>/src/dsp.rs`.

- Add the new processing stage in the correct position in the signal chain
- If the feature is togglable (e.g., "multi-band mode"), use an enum parameter to switch between code paths
- Ensure the bypassed/off state produces bit-identical output to before the feature was added
- If using `oasis_core::dsp::crossover` or `oasis_core::dsp::mid_side`, follow the patterns in ARCHITECTURE.md

### Step 5: Add UI

Open `crates/oasis_<name>/src/ui.rs`.

- Add controls for the new parameters
- If the feature is togglable, show/hide related controls based on the toggle state
- Add any new visualization (metering, gain reduction display, etc.)

### Step 6: Update Presets

Add the new parameters to all factory presets. Create new presets that showcase the feature.

### Step 7: Build & Verify

```bash
cargo build --workspace
cargo test --workspace
```

---

## Adding a New UI Widget to oasis_ui

Shared widgets used across multiple plugins.

### Step 1: Create or Extend

- New file in `crates/oasis_ui/src/widgets/` for a new widget type
- Extend an existing file for a variant of an existing widget

### Step 2: Implement the Widget

```rust
pub struct MyWidget {
    // Parameter binding or data source
    // Visual configuration
}

impl MyWidget {
    pub fn new(/* constructor params */) -> Handle<Self> {
        // Build with Vizia builder pattern
    }
}

impl View for MyWidget {
    fn draw(&self, cx: &mut DrawContext, canvas: &mut Canvas) {
        // All drawing is vector graphics — no images
        // Use colors from oasis_ui::theme::colors
        // Use dimensions from oasis_ui::theme::dimensions
        // Use spacing from oasis_ui::theme::spacing
    }
    
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        // Handle mouse interaction:
        // - Click + drag = adjust
        // - Shift + drag = fine adjust
        // - Double-click = reset to default
        // - Right-click = context menu
        // - Mouse wheel = increment/decrement
    }
}
```

### Step 3: Export from mod.rs

Add `pub mod <name>;` in `crates/oasis_ui/src/widgets/mod.rs`.

### Step 4: Build

```bash
cargo build -p oasis_ui
cargo build --workspace
```

---

## Adding Factory Presets

### Step 1: Open the Plugin's presets.rs

Location: `crates/oasis_<name>/src/presets.rs`

### Step 2: Define the Preset

```rust
FactoryPreset {
    name: "Descriptive Name",           // Not "Preset 37" or "Cool Vibe 2"
    category: "Vocals",                  // Vocals, Drums, Bass, Synths, Guitars, Master Bus, Creative
    author: "Oasis Suite",
    params: &[
        ("param_id", ParamValue::Float(0.5)),   // Normalized 0.0–1.0
        ("enum_param", ParamValue::Enum(1)),     // Enum variant index
    ],
},
```

### Step 3: Preset Quality Rules

- Every parameter must be listed explicitly — don't rely on defaults
- Values must be normalized (0.0–1.0 for floats, variant index for enums)
- The preset must be immediately usable, not an extreme demo
- Name must describe what it does or sounds like
- Category must match the use case

### Step 4: Build

```bash
cargo build -p oasis_<name>
```

---

## Modifying Existing DSP

Changing how an existing algorithm sounds or behaves.

### Step 1: Understand Current Behavior

Read the existing DSP code. Understand every line before changing anything.

### Step 2: Make the Change

- If changing coefficients or curves: update the math, keep the same interface
- If changing the signal flow: ensure bypassed/default state still sounds correct
- If optimizing: verify output matches (or intentionally improves upon) the original within tolerance

### Step 3: Preserve Compatibility

- **Never change parameter IDs** — breaks saved sessions
- **Never change parameter ranges** — existing automation data would mean different things
- If you must change a range, add a new parameter and deprecate the old one
- Preset values (normalized 0.0–1.0) must map to the same musical result

### Step 4: Verify Signal Integrity

Run existing tests. If tests don't exist for what you changed, write them:

```bash
cargo test -p oasis_core
cargo test -p oasis_<name>
```

---

## Build & Verification

### After Every Change

```bash
cargo build --workspace              # Must compile clean, zero warnings
```

### Before Considering a Task Complete

```bash
cargo build --workspace              # Clean compilation
cargo test --workspace               # All tests pass
cargo clippy --workspace             # No warnings (if clippy is set up)
```

### For Release Builds

```bash
cargo build --workspace --release    # Optimized build
cargo xtask bundle --release         # Bundle into .vst3 files
```

### What NOT To Do

- Do **not** launch the plugin or test it in a DAW — that's the project owner's job
- Do **not** create additional dev servers or listeners on any port
- Do **not** run interactive or GUI-dependent commands
- Do **not** attempt to play audio or verify sound — verify through code correctness and tests only

---

## File Location Quick Reference

| What You're Adding | Where It Goes |
|---|---|
| DSP primitive (filter, envelope, delay, etc.) | `crates/oasis_core/src/dsp/<name>.rs` |
| Global constant | `crates/oasis_core/src/constants.rs` |
| Parameter helper/constructor | `crates/oasis_core/src/params/` |
| Parameter smoother | `crates/oasis_core/src/params/smoothing.rs` |
| Value display formatter | `crates/oasis_core/src/params/formatting.rs` |
| A/B comparison logic | `crates/oasis_core/src/state/ab.rs` |
| Undo/redo logic | `crates/oasis_core/src/state/history.rs` |
| Preset serialization | `crates/oasis_core/src/state/preset.rs` |
| Utility (math, interpolation, denormal) | `crates/oasis_core/src/util/` |
| Debug tools (logger, metrics, scopes) | `crates/oasis_core/src/debug/` |
| Shared UI widget | `crates/oasis_ui/src/widgets/<name>.rs` |
| Composite UI component (header, preset browser) | `crates/oasis_ui/src/components/<name>.rs` |
| Theme (colors, spacing, typography) | `crates/oasis_ui/src/theme.rs` |
| Embedded fonts | `crates/oasis_ui/src/fonts.rs` |
| Vector drawing primitives | `crates/oasis_ui/src/drawing/` |
| Plugin-specific DSP | `crates/oasis_<name>/src/dsp.rs` |
| Plugin parameters | `crates/oasis_<name>/src/params.rs` |
| Plugin UI layout | `crates/oasis_<name>/src/ui.rs` |
| Plugin factory presets | `crates/oasis_<name>/src/presets.rs` |
| Plugin entry point | `crates/oasis_<name>/src/lib.rs` |
| Plugin struct / Plugin trait | `crates/oasis_<name>/src/plugin.rs` |
| Integration tests | `tests/<test_name>.rs` |
| Build automation | `xtask/src/main.rs` |

---

## Common Mistakes to Avoid

These are baked into the process above, but stated explicitly:

1. **Forgetting parameter smoothing** — Every float parameter that touches audio needs a smoother. Unsmoothed = clicks/pops on automation.
2. **Allocating on the audio thread** — No `Vec::push`, no `String`, no `Box::new`, no `format!` inside `process()`. Pre-allocate everything in `initialize()`.
3. **Hardcoded numbers** — `20.0` should be `MIN_FREQ_HZ`. `300.0` should be `METER_DECAY_MS`. Always use constants.
4. **Changing parameter IDs** — Once a parameter ID string exists, it's permanent. Changing it breaks every saved DAW session and preset.
5. **Skipping `set_sample_rate()`** — Every DSP component that uses time-based calculations must recalculate on sample rate change.
6. **Missing denormal protection** — Wrap `process()` with `DenormalGuard`. Apply `sanitize()` after saturation, feedback loops, and other nonlinear operations.
7. **Inconsistent UI** — Use `oasis_ui` theme values. Same knob sizes, same colors, same interaction patterns across all plugins.
8. **Not re-reporting latency** — If a parameter change affects latency (lookahead toggle, oversampling factor), set the `latency_changed` flag so the audio thread re-reports.
