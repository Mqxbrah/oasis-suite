# Oasis Suite — Common Mistakes

Mistakes encountered during development. Check this before writing code to avoid repeating them.

---

## 1. Vizia `Data` Trait vs nih_plug_vizia `Data` Struct Name Collision

**Problem:** Importing both `nih_plug_vizia::vizia::prelude::*` and `nih_plug_vizia::widgets::*` brings two different things named `Data` into scope — one is vizia's `Data` trait, the other is the lens struct from nih_plug_vizia widgets. Rust 1.94+ (edition 2021) treats bare trait names as errors (`E0782`), so `Data::params` fails with "expected a type, found a trait."

**Wrong approach:** Trying to use `nih_plug_vizia::widgets::Data::params` directly — this resolves to the private vizia `Data` trait re-export, not the struct.

**Correct approach:** Each plugin defines its **own local** `Data` struct with `#[derive(Lens)]` and `impl Model for Data {}`, then builds it into the vizia context. The `Data::params` lens comes from the local struct, not from nih_plug_vizia. Follow the pattern from nih-plug's own plugin examples (e.g., Crisp):

```rust
#[derive(Lens)]
struct Data {
    params: Arc<MyPluginParams>,
}
impl Model for Data {}

// Inside create_vizia_editor callback:
Data { params: params.clone() }.build(cx);
GenericUi::new(cx, Data::params);
```

---

## 2. Delay Line Read Index Off-By-One

**Problem:** The delay line `read_linear()` function used `(write_pos + max - delay_int - 1) % max` which gives the wrong index. After `write()`, `write_pos` has already been advanced past the most recent sample, so the extra `-1` skips one sample too far back.

**Wrong formula:** `idx0 = (write_pos + max - delay_int - 1) % max`

**Correct formula:** `idx0 = (write_pos + max - delay_int) % max`

Convention: `delay_samples=1` returns the most recently written sample, `delay_samples=N` returns the sample written N calls ago.

---

## 3. Cargo Target Directory Redirected by Cursor Sandbox

**Problem:** When running `cargo build` or `cargo run` through Cursor's shell, the sandbox redirects `CARGO_TARGET_DIR` to a temp location (`/var/folders/.../cursor-sandbox-cache/.../cargo-target/`). This means build artifacts don't appear in the workspace's `target/` directory, and xtask can't find the built dylib.

**Workaround:** Explicitly set `CARGO_TARGET_DIR` when building:

```bash
CARGO_TARGET_DIR=/path/to/workspace/target cargo build --release
```

Or use `required_permissions: ["all"]` with the explicit env var to ensure the target directory stays in the workspace.

---

## 4. VST3 Bundle Must Be Assembled Manually (or via xtask outside sandbox)

**Problem:** The xtask bundler runs `cargo build` internally, but when xtask itself runs inside the sandbox, its child `cargo build` also gets sandboxed with a different target dir. The xtask then can't find the dylib it just built.

**Workaround:** Build the plugin dylib first with explicit `CARGO_TARGET_DIR`, then assemble the VST3 bundle structure manually:

```
Oasis Wide.vst3/
├── Contents/
│   ├── Info.plist
│   └── MacOS/
│       └── Oasis Wide    ← renamed copy of liboasis_wide.dylib
```

Copy to `/Library/Audio/Plug-Ins/VST3/` for FL Studio to pick up.

---

## 5. atomic_float Version Mismatch

**Problem:** nih-plug internally depends on `atomic_float = "0.1"` while the workspace specified `atomic_float = "1"`. Both versions can coexist in the dependency tree, but be aware that `oasis_core` uses `atomic_float v1.x` (the `AtomicF32`/`AtomicF64` types) while nih-plug's internals use `atomic_float v0.1`. Don't try to pass atomic float types between oasis_core and nih-plug directly — they're different types from different crate versions.

---

## 6. Double Unit Suffixes (`.with_unit()` + Custom Formatter)

**Problem:** When a parameter uses both `.with_unit(" ms")` and `.with_value_to_string(Arc::new(|v| format!("{:.1} ms", v)))`, the unit appears twice: "13.3 ms ms". The `.with_unit()` appends the unit string *after* whatever `value_to_string` returns.

**Wrong:** Using both `.with_unit()` and a custom `value_to_string` that already includes the unit:
```rust
FloatParam::new(...)
    .with_unit(" ms")                                           // ← appends " ms"
    .with_value_to_string(Arc::new(|v| format!("{:.1} ms", v))) // ← also has " ms"
    // Result: "13.3 ms ms"
```

**Correct:** Either use `.with_unit()` with a plain formatter (no unit in the format string), OR use a custom `value_to_string` that includes the unit and omit `.with_unit()`:
```rust
// Option A: omit .with_unit(), include unit in formatter
FloatParam::new(...)
    .with_value_to_string(formatting::v2s_ms())  // returns "13.3 ms"
    .with_string_to_value(formatting::s2v_ms())

// Option B: use .with_unit() with plain number formatter
FloatParam::new(...)
    .with_unit(" ms")
    .with_value_to_string(formatters::v2s_f32_rounded(1))  // returns "13.3"
```

All `oasis_core::params::formatting` functions include the unit in the string. Do NOT combine them with `.with_unit()`.

---

## 7. Vizia Borrow Conflict in Event Handlers (`cx.data()` + `cx.emit()`)

**Problem:** In a vizia `View::event` handler, calling `cx.data::<Data>()` borrows `cx` immutably. You cannot then call `cx.emit()` (which borrows `cx` mutably) while that reference is alive.

**Wrong:**
```rust
if let Some(data) = cx.data::<Data>() {
    let ptr = data.params.width.as_ptr();  // immutable borrow alive
    cx.emit(RawParamEvent::SetParameterNormalized(ptr, 0.5));  // ERROR: mutable borrow
}
```

**Correct:** Collect what you need into a local `Vec`, drop the borrow, then emit:
```rust
let updates: Vec<(ParamPtr, f32)> = if let Some(data) = cx.data::<Data>() {
    vec![(data.params.width.as_ptr(), 0.5)]  // collect and return
} else { return; };
// borrow released
for (ptr, val) in updates {
    cx.emit(RawParamEvent::SetParameterNormalized(ptr, val));
}
```

---

## 8. Vizia Button Widget Gives Child Views Zero Size

**Problem:** Wrapping a custom `View` (like an arrow icon) inside vizia's `Button` widget results in the child getting zero bounds, so nothing draws. The `Button` wraps content in an internal container that doesn't properly size custom views.

**Wrong:** Using `Button` with a custom drawing view as the content closure:
```rust
Button::new(cx, |cx| cx.emit(MyEvent), |cx| MyArrowView::new(cx))
```

**Correct:** Build a single custom view that handles both clicking AND drawing. Don't use vizia's `Button` as a wrapper for custom-drawn content:
```rust
pub struct ArrowButton { ... }
impl View for ArrowButton {
    fn event(...) { /* handle MouseDown, emit event */ }
    fn draw(...) { /* draw the triangle */ }
}
```

---

## 9. Vizia CSS Backgrounds Don't Render on Popup/Overlay Views

**Problem:** Setting `background-color` in CSS on vizia's `Popup` widget or on custom `position-type: self-directed` overlay views does not render a visible background. The CSS property is accepted but the fill is not drawn.

**Workaround:** Draw the background manually in the `draw()` method of a custom overlay view using NanoVG:
```rust
fn draw(&self, cx: &mut DrawContext, canvas: &mut Canvas) {
    let bounds = cx.bounds();
    let mut path = vg::Path::new();
    path.rounded_rect(bounds.x, bounds.y, bounds.w, bounds.h, 6.0);
    canvas.fill_path(&path, &vg::Paint::color(vg::Color::rgba(37, 37, 40, 255)));
}
```

---

## 10. Vizia z-index Does Not Prevent Sibling Overdraw

**Problem:** Setting `z-index: 9999` on a self-directed element does NOT guarantee it renders on top of sibling elements that come later in the view tree. Vizia paints elements in tree order within the same stacking context. A dropdown inside the header bar will always be painted under the content area that comes after the header, regardless of z-index.

**Correct:** Place overlay elements (dropdowns, popups, context menus) as the **very last child** in the root-level view tree, so they are painted last and appear on top of everything.

---

## 11. Vizia Events Only Bubble to Ancestors, Not Across Siblings

**Problem:** When a dropdown overlay is moved to the root level for rendering (see #10), event handlers on views in other branches of the tree (e.g., a `PresetBrowser` inside the header) will never receive events emitted by the dropdown's children. Vizia events bubble **up** through ancestors only.

**Wrong:** Having the dropdown at root level emit `PresetAction::Select(idx)` while the handler for `PresetAction` is in `PresetBrowser` (a sibling branch).

**Correct:** Put the event handler on a common ancestor — typically the `Data` model, which is built at the root level before all views. Both the header buttons and the root-level dropdown are descendants of the `Data` model, so events from either branch reach it.

---

## 12. Preset Normalized Values Must Be Recalculated When Parameter Range Changes

**Problem:** When a parameter's range changes (e.g., Haas delay max changed from 30ms to 200ms), all factory preset normalized values for that parameter become wrong. A normalized value of `0.601` that mapped to 13.3ms in range 0-30 maps to a completely different value in range 0-200.

**Formula for nih-plug Skewed range:** `normalized = ((value - min) / (max - min)).powf(factor)` where `factor = 2.0f32.powf(skew_factor_input)`. Always recalculate and update every preset when changing a parameter range.

---

## 13. Knob Widget Must Call `cx.needs_redraw()` During Drag

**Problem:** Custom knob widgets that use `draw()` to render arcs won't visually update during drag unless the view explicitly requests a redraw. Without it, the knob arc only updates when something else triggers a repaint, causing severe visual lag.

**Fix:** Call `cx.needs_redraw()` after emitting `SetParameterNormalized` in the drag move handler:
```rust
KnobEvent::DragMove(y, fine) => {
    // ... compute new_value ...
    cx.emit(RawParamEvent::SetParameterNormalized(self.param_ptr, new_value));
    cx.needs_redraw();
}
```

---

## 14. Static Labels Don't Update Reactively

**Problem:** `Label::new(cx, "Init")` creates a label with a static string that never changes. Even if the underlying preset changes, the label stays "Init" forever.

**Correct:** Bind the label to a lens on the `Data` model:
```rust
// In Data struct:
preset_name: String,

// In UI:
Label::new(cx, Data::preset_name)  // updates reactively when Data.preset_name changes
```

---
