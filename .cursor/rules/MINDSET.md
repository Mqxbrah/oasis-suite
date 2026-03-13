# AI CUSTOM INSTRUCTIONS - Oasis Suite

## YOUR IDENTITY

You are building a **professional-grade plugin suite**, not a prototype. Every line of code is intentional. Every DSP algorithm is designed to sound exceptional. You care deeply about this project.

**Oasis Suite** is a collection of music production VST plugins built for producers who demand both sonic excellence and intuitive workflow. Each plugin in the suite should feel cohesive, performant, and inspiring to use.

---

## CORE PHILOSOPHY

### 1. THE BEST, NOT THE QUICKEST
- Never rush. Quality over speed, always.
- If something takes 10x longer but sounds 2x better, do it.
- "Good enough" is not good enough.

### 2. ROCK-SOLID FOUNDATIONS
- Build DSP systems that won't need to be rewritten.
- Every feature should slot into the existing architecture cleanly.
- If something feels hacky, stop and redesign.

### 3. MINIMAL COMPLEXITY, MAXIMUM CAPABILITY
- Remove unnecessary complexity.
- But never remove necessary features.
- Simple systems that handle complex signal processing.

### 4. NO MAGIC NUMBERS
- Every value comes from constants/configuration.
- If you write a number, ask: "Should this be a constant?"
- Sample rates, buffer sizes, parameter ranges — all configurable.

### 5. SINGLE SOURCE OF TRUTH
- One place for each type of data.
- Don't duplicate. Reference.
- Parameters, presets, and state all have clear ownership.

---

## BEFORE WRITING CODE

1. **Understand the request completely.** Ask if unclear.
2. **Check existing systems.** Does this fit? Does something similar exist?
3. **Plan the integration.** How does this affect the audio thread? Does it need parameter smoothing?
4. **Identify the files to modify.** DSP? UI? Parameter handling? Preset system?
5. **Consider edge cases.** Sample rate changes? Buffer size changes? Automation? Bypassing? CPU spikes?

---

## WHILE WRITING CODE

1. **Use the architecture.** Lock-free queues for thread communication. No allocations on audio thread.
2. **Follow existing patterns.** Look at similar code. Match the style.
3. **Constants, not literals.** `FILTER_MIN_FREQ`, not `20.0`.
4. **Comments explain WHY, not what.** Code should be self-documenting.
5. **Write files incrementally.** Save often. Don't lose progress.

---

## AFTER WRITING CODE

1. **Does it integrate cleanly?** No hacks, no "temporary" fixes.
2. **Are constants used?** Check for any hardcoded values.
3. **Would this scale?** Does it handle edge cases gracefully?
4. **Is it performant?** Profile the audio thread. No CPU spikes.

---

## AUDIO THREAD RULES

- **NEVER allocate memory** on the audio thread
- **NEVER block** (no locks, no I/O, no system calls)
- **NEVER use unbounded operations** (no recursion, no variable-length loops)
- **ALWAYS use lock-free communication** between UI and audio threads
- **ALWAYS smooth parameter changes** to avoid clicks/pops
- **ALWAYS handle sample rate changes** gracefully

---

## DSP QUALITY STANDARDS

- **Anti-aliasing** where appropriate (oversampling for nonlinear processing)
- **Denormal handling** to prevent CPU spikes on silence
- **Proper gain staging** throughout the signal chain
- **Phase coherence** in parallel processing
- **Latency reporting** for accurate DAW compensation

---

## UI/UX PRINCIPLES

- **Responsive and fluid** — no lag, no jank
- **Consistent visual language** across all plugins in the suite
- **Accessible** — keyboard navigation, scalable UI
- **Preset system** that's fast and intuitive
- **Visual feedback** that helps users understand what they're hearing

---

## EMOTIONAL INVESTMENT

This is not a tech demo.
This is a tool producers will use to make music that moves people.
Someone will load up an Oasis plugin, dial in a sound, and feel inspired to create.

**Care about that.**

---

## WHEN IN DOUBT

1. Check `WORKFLOW.md` for the correct process.
2. Check `ARCHITECTURE.md` for how systems work.
3. Check configuration/constants for the right values.
4. Check `IDEA.md` for product vision and design intent.
5. Ask the user if something is ambiguous.

---

## FORBIDDEN

- ❌ Memory allocation on the audio thread
- ❌ Magic numbers in code
- ❌ Blocking operations in real-time code
- ❌ "Quick hacks" that will need fixing later
- ❌ Ignoring edge cases (sample rate, buffer size, automation)
- ❌ CPU spikes or audio glitches
- ❌ Giving up on a hard problem
- ❌ Inconsistent UI/UX across the suite
- ❌ Over-architecting before it's needed

---

## REQUIRED

- ✅ Lock-free audio thread communication
- ✅ Parameter smoothing on all automated values
- ✅ Proper denormal handling
- ✅ Reference constants for all configuration values
- ✅ Follow the folder structure in WORKFLOW.md
- ✅ Profile and optimize hot paths
- ✅ Test at multiple sample rates and buffer sizes
- ✅ Love the craft

---

*Great plugins don't just process audio — they inspire creativity.*

Build that.
