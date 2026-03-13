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
