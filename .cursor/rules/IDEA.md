# Oasis Suite — Plugin Collection

A professional VST3 plugin suite built from scratch. No JUCE, no frameworks — pure VST3 SDK and custom code.

## Technical Constraints

- **VST3 SDK only** — no JUCE, no external frameworks
- **No Xcode dependency** — build system should work from command line
- **Cross-platform** — macOS and Windows support
- **Consistent UI** — shared visual language across all plugins

---

## The Plugins

### Oasis Comp — Compressor

A versatile compressor that handles everything from transparent glue to aggressive pumping.

**Features:**
- Threshold, Ratio, Attack, Release, Knee
- Mix (parallel compression) knob
- Makeup gain (auto and manual)
- Sidechain filter (high-pass to ignore low end)
- Multiple detection modes: Peak, RMS, hybrid
- Multiple knee options: hard, soft, variable
- Gain reduction metering
- Lookahead option for transparent limiting-style compression

**Character Modes:**
- Clean/Transparent
- Punchy (adds subtle harmonics on compression)
- Vintage (modeled nonlinearities)

---

### Oasis Drive — Saturator

Multi-algorithm saturation for warmth, grit, and harmonic enhancement.

**Features:**
- Drive amount with input/output gain staging
- Mix (dry/wet) knob for parallel saturation
- Tone control (pre or post saturation filtering)
- Oversampling (2x, 4x, 8x) to reduce aliasing

**Saturation Algorithms:**
- **Tape** — Soft, warm compression with subtle high-frequency rolloff
- **Tube** — Asymmetric harmonics, even-order warmth
- **Transistor** — Harder clipping, odd harmonics, aggressive edge
- **Digital** — Hard clip, wavefolder, modern sound design textures

**Extras:**
- Multi-band mode (low/mid/high independent saturation)
- Stereo/mid-side processing option

---

### Oasis Punch — Transient Shaper

Precise control over attack and sustain characteristics.

**Features:**
- Attack control (boost or reduce transients)
- Sustain control (boost or reduce body/tail)
- Detection speed (fast for drums, slow for buses)
- Mix knob
- Gain compensation

**Advanced:**
- Per-band transient shaping (low/mid/high)
- Sidechain input for ducking based on external signal
- Visual transient detection display

---

### Oasis Limit — Limiter

True peak brickwall limiting for maximizing loudness without clipping.

**Features:**
- Ceiling (output level)
- Threshold / Input gain
- Release (auto and manual modes)
- True peak detection (inter-sample peak limiting)
- Multiple release algorithms: fast, slow, multi-band, adaptive
- Detailed metering: input, output, gain reduction, LUFS

**Modes:**
- Transparent — minimal coloration
- Aggressive — pumping for electronic/EDM
- Mastering — optimized for final output

---

### Oasis DeEss — De-Esser

Surgical sibilance control for vocals and harsh high frequencies.

**Features:**
- Frequency targeting (adjustable center frequency and bandwidth)
- Threshold and range (how much reduction)
- Detection: wideband or split-band
- Listen mode (solo the sibilance band to dial in)
- Lookahead for transparent reduction
- Gain reduction metering

**Modes:**
- Split-band — only affects the sibilance frequency range
- Wideband — ducks the entire signal when sibilance detected

---

### Oasis Wide — Stereo Widener

Stereo image control and enhancement.

**Features:**
- Width control (mono to ultra-wide)
- Mid/Side balance
- Haas effect (subtle delay for widening)
- Bass mono (collapse low frequencies to center)
- Correlation meter (mono compatibility check)
- Stereo vectorscope visualization

**Advanced:**
- Per-band widening (keep bass mono, widen highs)
- Safe mode (prevents phase issues)

---

### Oasis Delay — Delay

Versatile delay with multiple characters and creative options.

**Features:**
- Delay time (ms and tempo-synced)
- Feedback amount
- Mix (dry/wet)
- Ping-pong stereo mode
- Low-pass and high-pass filters in feedback loop
- Saturation/color in feedback loop
- Modulation (chorus-like movement on delays)
- Ducking (delays duck when input signal present)

**Delay Types:**
- **Digital** — Clean, pristine repeats
- **Tape** — Degrading repeats with wow/flutter, saturation, high-end rolloff
- **Analog** — BBD-style, dark and gritty with pitch artifacts
- **Reverse** — Reversed delay tails
- **Multi-tap** — Multiple delay taps with independent timing

**Extras:**
- Freeze/infinite feedback button
- Tempo sync with dotted/triplet options
- Stereo offset (different L/R delay times)

---

### Oasis Verb — Reverb

Algorithmic reverb with everything from tight rooms to infinite spaces.

**Features:**
- Size/Decay time
- Pre-delay
- Damping (high-frequency decay)
- Width (stereo spread)
- Mix (dry/wet)
- Low-cut and high-cut filters on reverb tail
- Modulation (adds movement, reduces metallic artifacts)

**Reverb Algorithms:**
- **Room** — Small, reflective spaces
- **Hall** — Large concert halls, smooth decay
- **Plate** — Classic studio plate, bright and dense
- **Chamber** — Warm, vintage studio chambers
- **Shimmer** — Pitch-shifted reverb tails for ambient/ethereal sounds
- **Spring** — Lo-fi spring reverb character

**Advanced:**
- Freeze button (infinite sustain)
- Ducking (reverb ducks when dry signal present)
- Early reflections control
- Diffusion amount

---

### Oasis Shift — Pitch Shifter

Real-time pitch shifting with formant control and harmony generation.

**Features:**
- Pitch shift amount (semitones and cents)
- Formant preservation (keep natural vocal character when shifting)
- Mix (dry/wet)
- Latency modes (low latency vs. high quality)

**Modes:**
- **Simple** — Monophonic pitch shift
- **Harmony** — Generate harmonies (intervals: 3rd, 5th, octave, custom)
- **Octaver** — Sub-octave and upper octave generation
- **Detune** — Subtle pitch offset for thickening (ADT effect)

**Advanced:**
- Multiple voices with independent pitch
- MIDI input for harmony control
- Glide/portamento between pitches

---

### Oasis EQ — Parametric Equalizer

The main workhorse EQ — surgical precision with visual feedback.

**Features:**
- 8+ fully parametric bands
- Each band: frequency, gain, Q (bandwidth), filter type
- Filter types per band: Bell, Low Shelf, High Shelf, Low Cut, High Cut, Notch, Bandpass
- Variable slope on cuts (6/12/18/24/48 dB per octave)
- Real-time spectrum analyzer (pre/post/overlay)
- A/B comparison

**Advanced:**
- **Linear Phase mode** — no phase distortion, higher latency
- **Dynamic EQ bands** — threshold-based, compressor-like EQ
- **Mid/Side mode** — independent EQ for mid and side channels
- **Auto-gain** — compensates for perceived loudness changes
- **Match EQ** — capture and apply frequency profile from reference
- **Band solo** — audition individual bands in isolation

**Workflow:**
- Click-and-drag band creation on spectrum
- Keyboard modifiers for fine adjustment
- Spectrum analyzer zoom/scale options
- Preset system for common curves

---

### Oasis Synth — Synthesizer

A powerful wavetable/virtual analog hybrid synth inspired by Spire.

**Architecture:**
- 4 oscillators with multiple synthesis modes
- 2 filters with multiple types and routing options
- 4 envelopes (ADSR with curves)
- 4 LFOs with tempo sync
- Modulation matrix (any source to any destination)
- Built-in effects section

**Oscillator Modes:**
- **Classic** — Saw, Square, Triangle, Sine with PWM and shape
- **Wavetable** — Morph through wavetable positions
- **FM** — 2-op FM per oscillator
- **Noise** — Filtered noise with color control
- **Unison** — Up to 8 voices per oscillator with detune and spread

**Filters:**
- Multiple types: Low Pass, High Pass, Band Pass, Notch, Comb, Formant
- 12/24 dB slopes
- Drive/saturation
- Key tracking
- Serial/parallel routing

**Modulation:**
- Drag-and-drop modulation assignment
- Modulation depth per destination
- Velocity, aftertouch, mod wheel as sources
- Envelope followers
- Step sequencer as mod source

**Effects (built-in):**
- Distortion
- Chorus
- Phaser
- Delay
- Reverb
- EQ

**Extras:**
- Arpeggiator with patterns
- Polyphonic, monophonic, legato modes
- Portamento/glide
- Macro controls (4-8 knobs controlling multiple parameters)
- Preset browser with categories and tags

---

### Oasis Pump — Volume Shaper

Tempo-synced volume shaping for sidechain-style effects without needing a sidechain input.

**Features:**
- Drawable LFO curve (click and drag to shape)
- Tempo sync (1/16, 1/8, 1/4, 1/2, 1 bar, etc.)
- Rate control with dotted/triplet options
- Mix (dry/wet)
- Depth (how much the volume ducks)
- Smoothing (round off sharp edges in the curve)

**Curve Tools:**
- Pencil — Freehand draw
- Line — Straight line segments
- Preset shapes — Sine, saw, square, exponential, custom
- Point editing — Add/remove/drag nodes
- Snap to grid

**Extras:**
- MIDI trigger mode (curve resets on MIDI note)
- Multiple curve lanes (volume + filter cutoff)
- Phase offset for stereo (L/R or mid/side)
- Envelope follower input (duck based on external signal)

---

## Naming Convention

All plugins follow the pattern: **Oasis [Name]**

| Plugin | Full Name |
|--------|-----------|
| Compressor | Oasis Comp |
| Saturator | Oasis Drive |
| Transient Shaper | Oasis Punch |
| Limiter | Oasis Limit |
| De-Esser | Oasis DeEss |
| Stereo Widener | Oasis Wide |
| Delay | Oasis Delay |
| Reverb | Oasis Verb |
| Pitch Shifter | Oasis Shift |
| Parametric EQ | Oasis EQ |
| Synthesizer | Oasis Synth |
| Volume Shaper | Oasis Pump |

---

## Suite-Wide UX Standards

These apply to **all plugins** in the suite.

### Layout Philosophy

Every plugin follows the same spatial layout:
- **Left** — Input section (input gain, metering)
- **Center** — Main controls (the core of the plugin)
- **Right** — Output section (output gain, metering)
- **Metering always visible** — Never hidden, always showing activity

### Parameter Interaction

- **Double-click** — Reset parameter to default value
- **Click + drag** — Adjust parameter
- **Shift + drag** — Fine adjustment (slower movement)
- **Mouse wheel** — Increment/decrement value

**Right-click context menu:**
- Edit Value — Opens text input for manual entry
- Copy — Copies parameter value
- Paste — Pastes copied value
- Reset to Default
- (Additional options per plugin as needed)

### Undo/Redo

- **Ctrl/Cmd + Z** — Undo last parameter change
- **Ctrl/Cmd + Shift + Z** — Redo
- Full parameter history (not just one level)
- Undo history survives preset changes

### A/B Comparison

- **A/B button** — Toggle between two states
- **Copy A→B / B→A** — Copy current state to the other slot

### Bypass

- Global bypass button on every plugin
- Bypass should be latency-compensated (no click/pop)

### Preset System

**Factory Presets:**
- Ship with high-quality presets organized by use case
- Categories: Vocals, Drums, Bass, Synths, Guitars, Master Bus, Creative, etc.
- All factory presets baked into the .vst3 binary (no external files)

**Init Preset:**
- Good default starting point (not just zeroed out)
- Sensible values that sound reasonable immediately

**User Presets:**
- Saved as `.oasis` files (custom format containing parameter data + plugin identifier)
- User chooses save location
- Loading a preset from the wrong plugin type shows a clear error message (not crash)
- Copy/paste preset state between instances

### Visual

- **Resizable UI** — 50% to 200% scaling for any monitor setup
- **Version number** — Always visible in UI (starts at 1.0)
- **Consistent color scheme** — Same visual language across all plugins

### Metering

- Input and output level meters on every plugin
- Gain reduction / activity indicators where relevant

---

## Distribution

**Model:** DRM-free, no license keys, no activation required.

- Sold via Shopify as digital downloads
- Anyone with the files can use them — no phoning home
- Free updates for the version purchased

**File Format:**
- **No installer** — Just the raw `.vst3` file(s)
- Everything baked into the binary: code, factory presets, graphics, all data
- No external dependencies, no additional data files required
- User manually places .vst3 in their plugin folder:
  - macOS: `/Library/Audio/Plug-Ins/VST3/`
  - Windows: `C:\Program Files\Common Files\VST3\`
- Download is a simple ZIP containing the .vst3 file(s)

**User Presets:**
- Saved as `.oasis` files
- Contains: plugin identifier, version, parameter values
- Cross-platform compatible (same file works on macOS and Windows)
- Loading wrong plugin type → error message, not crash

## Versioning

- All plugins start at **version 1.0**
- Version number displayed in every plugin UI
- Version changes are decided solely by the project owner
- Version embedded in .oasis preset files for compatibility checking

---

## Development Priority

1. **Oasis EQ** — Core utility, most used
2. **Oasis Comp** — Essential dynamics
3. **Oasis Verb** — Time-based essential
4. **Oasis Delay** — Time-based essential
5. **Oasis Drive** — Tone shaping
6. **Oasis Limit** — Mastering essential
7. **Oasis Punch** — Dynamics
8. **Oasis DeEss** — Vocal processing
9. **Oasis Wide** — Stereo utility
10. **Oasis Shift** — Creative effect
11. **Oasis Pump** — Volume shaper, simple scope
12. **Oasis Synth** — Largest scope, last
