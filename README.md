# Clack (htype)

Clack is a highly realistic human typing simulator based on the original idea from [htype](https://github.com/lord-of-the-strings/htype) by Aadity Setu. It acts as a standard UNIX pipe, reading text from `stdin` and emitting it to `stdout` with human-like delays, stochastic typing errors, corrections, and fatigue modeling.

This project is a Rust port and expansion of the original concept, designed with an architecture that cleanly separates the simulation engine (`clack-core`) from the CLI driver (`clack-cli`).

## Installation

### From Source using Cargo
If you have Rust installed, you can build and install Clack directly from source:
```bash
cargo install --path clack-cli
```
This will install the `clack` binary to your `~/.cargo/bin` directory.

## Quick Start

Simulate typing a simple string at the default speed of 60 WPM:
```bash
echo "Hello, world!" | clack
```

Simulate typing at a faster speed (120 WPM):
```bash
echo "This is a speed test." | clack --wpm 120
```

Read from a file and pipe it to another command (or directly to a terminal):
```bash
cat my_script.sh | clack --wpm 80 | bash
```

## CLI Options

| Flag | Description | Default |
|---|---|---|
| `--wpm <FLOAT>` | Target average words per minute | `60.0` |
| `--jitter <FLOAT>` | IKI jitter coefficient (0.0 = none, 1.0 = extreme) | `0.15` |
| `--error-rate <FLOAT>` | Base probability of generating an error per character | `0.04` |
| `--correction-rate <FLOAT>`| Fraction of generated errors that get corrected | `0.85` |
| `--no-errors` | Disable all error generation | `false` |
| `--seed <INT>` | RNG seed for deterministic output | `None` |
| `--session-length <INT>` | Expected total character count for fatigue modeling | `500` |
| `--no-fatigue` | Disable warmup and fatigue session modelling | `false` |
| `--max-pause <INT>` | Maximum any single pause may be in milliseconds | `5000` |
| `--thinking-pause-prob <FLOAT>` | Probability of a stochastic thinking pause | `0.015` |
| `--state-output` | Emit behavioral state transitions to stderr | `false` |

---

# Project Documentation & Architecture

The following sections contain the complete specifications, architecture, decisions, and roadmaps that define how Clack works under the hood.


---

# PROJECT.md

# PROJECT.md — What Clack Is

---

## Overview

Clack is a Unix pipe tool that simulates realistic human typing behavior. It reads text from stdin and writes it to stdout character by character, with timing delays, errors, and corrections that mimic how a real person types.

It is informed by keystroke-dynamics research — not by guesswork. Every default value, timing distribution, and behavioral model is grounded in published empirical studies (see spec §2 for full citations).

Clack is implemented as a **reusable Rust library** (`clack-core`) with a **thin CLI driver** (`clack-cli`). A graphical user interface is planned for a future release, using the same core library.

---

## The Problem It Solves

Existing typing simulators produce uniform or trivially randomized delays between characters. The result looks robotic — a human can immediately tell the output is fake.

Real human typing has:
- **Momentum** — typing speed gradually shifts, it does not jump randomly
- **Behavioral states** — focused, flowing, thinking, distracted, fatigued
- **Word-level cognitive effects** — common words are typed faster; long or complex words are typed slower
- **Error patterns** — adjacent-key substitutions, insertions, omissions, transpositions, and doublings, each with empirically observed frequencies
- **Correction behavior** — immediate backspace-and-retype, delayed correction after typing several more characters, or leaving the typo uncorrected
- **Session dynamics** — warmup at the start, fatigue toward the end, occasional mental lapses

Clack models all of these. The result is stdout that looks like a person typed it.

---

## Target Users

| User | Use case |
|---|---|
| **Developers building terminal demos or screencasts** | Pipe script content through `clack` to produce realistic typing for recordings |
| **Developers testing UI components that react to streamed input** | Generate character-by-character input with human-like timing to test live preview, autocomplete, or syntax highlighting |
| **Researchers needing synthetic human typing data** | Generate keystroke timing datasets with controlled parameters (WPM, error rate, fatigue) for analysis or model training |
| **Security researchers studying timing-based detection systems** | Produce typing patterns that match human statistical profiles to test behavioral biometric systems |
| **Anyone who needs stdout that looks like a person typed it** | General-purpose human typing simulation for any pipeline |

---

## Usage

```bash
# Basic usage — type a file at 60 WPM (default)
cat essay.txt | clack

# Fast typing with errors
echo "The quick brown fox" | clack --wpm 100 --error-rate 0.06

# Deterministic output for testing
echo "Hello world" | clack --seed 42

# No errors, no fatigue
cat script.sh | clack --no-errors --no-fatigue

# Slow, deliberate typing
cat letter.txt | clack --wpm 30 --jitter 0.08

# Debug: show state transitions
echo "A long text..." | clack --state-output 2>states.log
```

### CLI Options

| Flag | Type | Default | Description |
|---|---|---|---|
| `--wpm` | float | `60.0` | Target average words per minute |
| `--jitter` | float | `0.15` | IKI jitter coefficient (0.0 = uniform, 1.0 = extreme) |
| `--error-rate` | float | `0.04` | Base error probability per character |
| `--correction-rate` | float | `0.85` | Fraction of errors that get corrected |
| `--no-errors` | flag | off | Disable all error generation |
| `--seed` | int | random | RNG seed for fully deterministic output |
| `--session-length` | int | `500` | Expected total character count for warmup/fatigue curves |
| `--no-fatigue` | flag | off | Disable warmup and fatigue modeling |
| `--max-pause` | int | `5000` | Maximum single pause in milliseconds |
| `--thinking-pause-prob` | float | `0.015` | Probability of a thinking pause between words |
| `--state-output` | flag | off | Emit state transitions to stderr |
| `--version` | flag | — | Print version and exit |

---

## Platform Roadmap

### MVP (Current Target) — v1.0.0
- CLI tool: Unix stdin → stdout streaming
- Single statically-linked binary
- All timing, state, error, correction, language, and session models implemented
- Deterministic mode via `--seed`
- QWERTY keyboard layout

### Post-MVP CLI — v2.x
- Code-aware typing (bracket depth parser, identifier slowdown)
- Trigraph timing tables
- Panic events (prefix repetition, stuck-key bursts)
- `--script` mode (predefined state sequences)
- Profile recording and playback
- Digraph timing table from user-supplied file
- Additional keyboard layouts (Dvorak, Colemak)

### Future — GUI
- Real-time visualization of behavioral state
- Typing speed graph
- Error overlay showing where errors occurred and how they were corrected
- Uses the same `clack-core` library — no simulation code duplication
- GUI framework TBD at end of Phase 3

---

## Explicit Non-Goals for the MVP

These items are **not** in the MVP and must not be partially implemented: [VERIFIED — spec §1.2]

1. **Code-aware typing** — bracket depth parser, identifier slowdown. Moved to v2.1 due to parser complexity and scope risk.
2. **User profile recording and playback (`--profile`)** — no recording infrastructure exists yet.
3. **Trigraph timing tables** — adds table complexity for marginal gain over digraph-level timing.
4. **Panic events** — prefix repetition (`gitgit`) and stuck-key bursts. Rare by definition; absence does not reduce overall believability.
5. **External state observation** — stderr state stream via named pipe. `--state-output` (basic stderr format) IS in scope.
6. **`--script` mode** — predefined state sequences. Adds a new input format and parser; not necessary for core simulation.
7. **Digraph timing table from user-supplied file** — future enhancement, not necessary for MVP believability.

---

## Attribution

Clack is the spiritual successor to [htype](https://github.com/lord-of-the-strings/htype) by lord-of-the-strings. The original project demonstrated the value of human-like typing simulation via Unix pipes. Clack takes the concept to a research-grounded implementation with a behavioral state machine, empirically derived timing distributions, and a reusable library architecture.

---

## License

MIT


---

# ARCHITECTURE.md

# ARCHITECTURE.md — Project Structure and Technical Architecture

> This document defines the complete repository layout, module breakdown,
> public API surface, data flow, and build system for the Clack MVP.
> Every file listed will exist at the end of Phase 2. No orphan files.
> No mystery directories.

---

## Repository Layout

```
clack/
├── Cargo.toml                  # Workspace manifest: members = ["clack-core", "clack-cli"]
├── README.md                   # Project overview, installation, usage examples
├── LICENSE                     # MIT license
├── .gitignore                  # Rust/Cargo ignore patterns
│
├── AGENT.md                    # Instructions for AI agents working on this codebase
├── PROJECT.md                  # What Clack is, who it's for, roadmap summary
├── ARCHITECTURE.md             # This file
├── DECISIONS.md                # Architectural decision log
├── ROADMAP.md                  # Six-phase development roadmap
├── TECH_DEBT.md                # Known limitations and future risks
├── CONTRIBUTING.md             # How to contribute: build, test, submit changes
│
├── spec/
│   └── htype-2-mvp-spec.md    # Reference copy of the MVP specification (read-only)
│
├── clack-core/                 # Library crate — all simulation logic, zero I/O
│   ├── Cargo.toml              # Dependencies: rand, rand_chacha, rand_distr
│   └── src/
│       ├── lib.rs              # Public API: ClackEngine, ClackConfig, ClackEvent, re-exports
│       ├── constants.rs        # All §15 named constants as `pub const` values
│       ├── rng.rs              # ClackRng: wrapper around ChaCha8Rng with helper methods
│       ├── timing.rs           # §5: Base IKI, log-normal sampling, momentum, burst mode
│       ├── state.rs            # §6: BehavioralState enum, transition matrix, dwell times
│       ├── pause.rs            # §7: Word/sentence/thinking/line pauses, stacking rules
│       ├── keyboard.rs         # §8: QWERTY layout table, distance, hand/finger, shift penalty
│       ├── error.rs            # §9: Error types, selection, substitution/insertion/etc.
│       ├── correction.rs       # §10: Immediate/delayed/uncorrected correction, backspace strategies
│       ├── language.rs         # §11: Common word list, difficult word detection
│       ├── session.rs          # §12: Session progress, warmup curve, fatigue curve, lapses
│       └── tokenizer.rs        # Word/sentence boundary detection, word extraction
│
├── clack-cli/                  # Binary crate — thin CLI driver, I/O only
│   ├── Cargo.toml              # Dependencies: clack-core, clap (with derive)
│   └── src/
│       └── main.rs             # Arg parsing, stdin→engine→sleep→stdout loop
│
└── clack-cli/tests/            # End-to-end CLI integration tests
    ├── deterministic.rs        # Fixed-seed byte-identical output verification
    ├── edge_cases.rs           # §14 defined behaviors tested via CLI
    └── snapshots/              # Expected output files for deterministic tests
        └── (generated during Phase 3)
```

### File Purposes (One-Line Each)

| File | Purpose |
|---|---|
| `Cargo.toml` (root) | Workspace manifest defining `clack-core` and `clack-cli` as members |
| `clack-core/Cargo.toml` | Library crate manifest; depends on `rand`, `rand_chacha`, `rand_distr` |
| `clack-cli/Cargo.toml` | Binary crate manifest; depends on `clack-core` (path) and `clap` |
| `clack-core/src/lib.rs` | Public API surface: `ClackEngine`, `ClackConfig`, `ClackEvent`, `BehavioralState` |
| `clack-core/src/constants.rs` | Every constant from spec §15 as a named `pub const` |
| `clack-core/src/rng.rs` | Seeded PRNG wrapper: `ClackRng` over `ChaCha8Rng` with sampling helpers |
| `clack-core/src/timing.rs` | IKI computation: WPM→IKI, log-normal jitter, momentum, hard floor, burst mode |
| `clack-core/src/state.rs` | Five behavioral states, transition probability matrix, dwell time enforcement |
| `clack-core/src/pause.rs` | Pause computation: word, sentence, thinking, line-start; stacking rules |
| `clack-core/src/keyboard.rs` | QWERTY coordinate table, Euclidean distance, hand assignment, shift penalty |
| `clack-core/src/error.rs` | Error decision, type selection, generation for all 5 error types |
| `clack-core/src/correction.rs` | Correction mode selection, immediate/delayed/uncorrected execution, backspace output |
| `clack-core/src/language.rs` | 60-word common word list, difficult word heuristics (length, clusters, consonants) |
| `clack-core/src/session.rs` | Session progress `p`, warmup multiplier, fatigue multiplier, lapse events |
| `clack-core/src/tokenizer.rs` | Splits input into characters with word/sentence boundary annotations |
| `clack-cli/src/main.rs` | CLI entry point: parse args, read stdin, call engine, sleep, write stdout |
| `spec/htype-2-mvp-spec.md` | Verbatim copy of the MVP specification for reference |

---

## Module Breakdown: Spec Section → Source Module

| Spec Section | Module | Primary Responsibilities |
|---|---|---|
| §4 (Architecture) | `lib.rs` | Engine lifecycle, event pipeline orchestration |
| §5 (Timing) | `timing.rs` | `compute_base_iki()`, `apply_momentum()`, `sample_log_normal()`, burst state |
| §6 (State Machine) | `state.rs` | `BehavioralState` enum, `try_transition()`, dwell counter, multiplier lookup |
| §7 (Pauses) | `pause.rs` | `compute_pause()`, stacking logic, max-pause clamp |
| §8 (Keyboard) | `keyboard.rs` | `key_position()`, `distance()`, `hand_modifier()`, `shift_penalty()` |
| §9 (Errors) | `error.rs` | `should_generate_error()`, `select_error_type()`, `generate_*()` for each type |
| §10 (Corrections) | `correction.rs` | `select_correction_mode()`, `emit_immediate()`, `emit_delayed()`, backspace strategy |
| §11 (Language) | `language.rs` + `tokenizer.rs` | `is_common_word()`, `is_difficult_word()`, `Tokenizer` struct |
| §12 (Session) | `session.rs` | `session_progress()`, `warmup_multiplier()`, `fatigue_multiplier()`, `check_lapse()` |
| §13 (Output) | `correction.rs` | Backspace encoding `[0x08, 0x20, 0x08]`, erase-space flagging |
| §15 (Constants) | `constants.rs` | All named constants from the spec's configuration reference |

---

## The Library API Surface

### Core Types

```rust
// ── Configuration ──────────────────────────────────────────

/// Configuration for a Clack simulation session.
/// All fields map directly to CLI flags (spec §3.2).
#[non_exhaustive]
pub struct ClackConfig {
    pub wpm: f64,                    // Default: 60.0 — target words per minute
    pub jitter: f64,                 // Default: 0.15 — IKI jitter coefficient (0.0–1.0)
    pub error_rate: f64,             // Default: 0.04 — base error probability per character
    pub correction_rate: f64,        // Default: 0.85 — fraction of errors that get corrected
    pub no_errors: bool,             // Default: false — disable all error generation
    pub seed: Option<u64>,           // Default: None — RNG seed for deterministic output
    pub session_length: usize,       // Default: 500 — expected total character count
    pub no_fatigue: bool,            // Default: false — disable warmup/fatigue modeling
    pub max_pause_ms: u64,           // Default: 5000 — maximum single pause in ms
    pub thinking_pause_prob: f64,    // Default: 0.015 — stochastic thinking pause probability
    pub state_output: bool,          // Default: false — emit state transitions
}

// ── Output Events ──────────────────────────────────────────

/// A single output event from the simulation engine.
/// The CLI driver sleeps for `delay_ms`, then writes `bytes` to stdout.
#[non_exhaustive]
pub struct ClackEvent {
    pub delay_ms: u64,                              // Sleep duration before emitting bytes
    pub bytes: Vec<u8>,                              // Raw bytes to emit (chars, backspace seqs)
    pub state_transition: Option<StateTransition>,   // Present only on state changes
}

/// Reported when the behavioral state machine transitions.
pub struct StateTransition {
    pub prev_state: BehavioralState,
    pub new_state: BehavioralState,
    pub word_count: usize,
}

/// The five behavioral states (spec §6.1).
#[non_exhaustive]
pub enum BehavioralState {
    Focused,
    Flow,
    Thinking,
    Distracted,
    Fatigued,
}
```

### Engine Interface

```rust
/// The simulation engine. Owns all internal state including the RNG.
/// Zero I/O — accepts bytes in, produces events out.
pub struct ClackEngine {
    // ── Private fields ──
    config: ClackConfig,
    rng: ClackRng,                   // Seeded ChaCha8Rng wrapper
    state_machine: StateMachine,     // Current behavioral state + dwell counter
    timing: TimingState,             // Momentum, burst state, previous IKI
    session: SessionState,           // Progress, warmup/fatigue multipliers
    tokenizer: Tokenizer,            // Word boundary detection, current word buffer
    correction_queue: CorrectionQueue, // Pending delayed corrections
    input_buffer: VecDeque<u8>,      // Buffered input for lookahead
    output_queue: VecDeque<ClackEvent>, // Ready events waiting to be pulled
    chars_emitted: usize,            // Total characters emitted (for session progress)
    prev_char: Option<u8>,           // Previous character (for keyboard model)
    at_line_start: bool,             // Track line-start hesitation
    finished: bool,                  // EOF signaled
}

impl ClackEngine {
    /// Create a new engine with the given configuration.
    /// Returns Err if config values are out of valid range.
    pub fn new(config: ClackConfig) -> Result<Self, ConfigError>;

    /// Feed input bytes into the engine.
    /// The engine buffers them internally and processes them into events.
    /// Call `next_event()` after feeding to retrieve output events.
    pub fn feed(&mut self, input: &[u8]);

    /// Retrieve the next output event, if one is ready.
    /// Returns None when all buffered input has been processed into events
    /// and pulled out. Feed more input or call finish() to produce more events.
    pub fn next_event(&mut self) -> Option<ClackEvent>;

    /// Signal end-of-input. Flushes any pending delayed corrections.
    /// After calling finish(), continue calling next_event() until it returns None
    /// to drain all remaining events.
    pub fn finish(&mut self);
}

/// Configuration validation error.
pub enum ConfigError {
    InvalidWpm(f64),           // ≤ 0
    InvalidJitter(f64),        // < 0 or > 1
    InvalidErrorRate(f64),     // < 0 or > 1
    InvalidCorrectionRate(f64),// < 0 or > 1
    InvalidMaxPause(u64),      // 0
}
```

---

## The CLI Driver

### Pseudocode

The CLI driver (`clack-cli/src/main.rs`) contains **zero simulation logic**.
Its entire job is I/O plumbing:

```
fn main():
    args = parse_cli_args()              // clap derive

    if args.version:
        print version string
        exit(0)

    config = ClackConfig from args       // map CLI flags to config fields
    engine = ClackEngine::new(config)?   // exit(1) on ConfigError

    stdin = io::stdin().lock()
    stdout = io::stdout().lock()
    stderr = io::stderr().lock()
    buf = [0u8; 4096]

    loop:
        n = stdin.read(&mut buf)         // blocks until data or EOF
        if n == 0:                       // EOF
            engine.finish()
            drain_events(engine, stdout, stderr)
            exit(0)
        if n == Err:
            exit(2)

        engine.feed(&buf[..n])
        drain_events(engine, stdout, stderr)

fn drain_events(engine, stdout, stderr):
    while let Some(event) = engine.next_event():
        if event.delay_ms > 0:
            thread::sleep(Duration::from_millis(event.delay_ms))
        stdout.write_all(&event.bytes)?
        stdout.flush()?
        if let Some(transition) = event.state_transition:
            if config.state_output:
                writeln!(stderr, "STATE:{} PREV:{} WORD:{}",
                    transition.new_state,
                    transition.prev_state,
                    transition.word_count)?
```

### What the Driver Does NOT Do

- ❌ Compute IKI values
- ❌ Sample from distributions
- ❌ Track behavioral state
- ❌ Generate or correct errors
- ❌ Detect word boundaries
- ❌ Apply keyboard distance modifiers
- ❌ Track session progress or fatigue
- ❌ Make any random decisions

---

## Data Flow Diagram

```
                    ┌─────────────────────────────────────────────────────────┐
                    │                    clack-core                           │
                    │                                                         │
stdin ──→ [CLI] ──→ │  feed(bytes)                                           │
           │        │     │                                                   │
           │        │     ▼                                                   │
           │        │  ┌──────────┐    ┌─────────────┐                       │
           │        │  │ Tokenizer │──→│ Language     │  common/difficult    │
           │        │  │ §11.3     │   │ §11.1, §11.2│  word detection       │
           │        │  └────┬─────┘    └──────┬──────┘                       │
           │        │       │                  │ word_multiplier              │
           │        │       ▼                  ▼                              │
           │        │  ┌──────────────────────────┐                          │
           │        │  │ State Machine  §6        │                          │
           │        │  │ FOCUSED/FLOW/THINKING/   │                          │
           │        │  │ DISTRACTED/FATIGUED      │──→ state multipliers     │
           │        │  └────────────┬─────────────┘                          │
           │        │               │                                         │
           │        │               ▼                                         │
           │        │  ┌──────────────────────────┐                          │
           │        │  │ Timing Engine  §5        │                          │
           │        │  │ log-normal IKI + momentum │                          │
           │        │  │ + keyboard modifier §8    │──→ IKI_final            │
           │        │  │ + burst mode §5.6         │                          │
           │        │  └────────────┬─────────────┘                          │
           │        │               │                                         │
           │        │               ▼                                         │
           │        │  ┌──────────────────────────┐                          │
           │        │  │ Pause System  §7         │                          │
           │        │  │ word/sentence/thinking/   │──→ pause_ms             │
           │        │  │ line-start + stacking     │                          │
           │        │  └────────────┬─────────────┘                          │
           │        │               │                                         │
           │        │               ▼                                         │
           │        │  ┌──────────────────────────┐                          │
           │        │  │ Session Model  §12       │                          │
           │        │  │ warmup + fatigue curves   │──→ session multipliers  │
           │        │  │ + lapse events            │                          │
           │        │  └────────────┬─────────────┘                          │
           │        │               │                                         │
           │        │               ▼                                         │
           │        │  ┌──────────────────────────┐                          │
           │        │  │ Error Engine  §9         │                          │
           │        │  │ substitution/insertion/   │──→ error chars           │
           │        │  │ omission/transposition/   │                          │
           │        │  │ doubling                  │                          │
           │        │  └────────────┬─────────────┘                          │
           │        │               │                                         │
           │        │               ▼                                         │
           │        │  ┌──────────────────────────┐                          │
           │        │  │ Correction Engine  §10   │                          │
           │        │  │ immediate/delayed/        │──→ backspace + retype   │
           │        │  │ uncorrected + strategies  │    sequences             │
           │        │  └────────────┬─────────────┘                          │
           │        │               │                                         │
           │        │               ▼                                         │
           │        │         ClackEvent { delay_ms, bytes, state_transition }│
           │        └──────────────────────────────────────┬──────────────────┘
           │                                               │
           │  ◄── next_event() ────────────────────────────┘
           │
           ▼
    sleep(delay_ms)
    stdout.write(bytes)
    stderr.write(state_transition)  // if --state-output
```

---

## Internal Processing Pipeline (Per Character)

When the engine processes a character `c` from the input buffer:

```
1. Tokenizer classifies `c`:
   - Is it a word boundary (space, tab, newline)?
   - Is it a sentence boundary (. ? ! : ;)?
   - Is it the start of a new line?
   - What word does it belong to (for language model)?

2. If at word boundary → evaluate state transition (§6.3)
   - Check minimum dwell time
   - Roll transition probability matrix
   - Block FATIGUED if session_progress < 0.60
   - Block FLOW if fatigue_multiplier > 1.20
   - Update state, emit StateTransition if changed

3. If at word boundary → evaluate burst trigger (§5.6)
   - 8% chance if in FOCUSED or FLOW state
   - Set burst_remaining_chars if triggered

4. Compute base IKI:
   a. IKI_target = 60000 / (wpm × 5)
   b. IKI_raw = log_normal_sample(mu, sigma)  where sigma = jitter × 0.4
   c. IKI_raw = max(IKI_raw, 60)              // hard floor
   d. IKI_smoothed = momentum blend with IKI_prev
   e. IKI_keyboard = IKI_smoothed × keyboard_modifier(prev_char, c)
   f. IKI_state = IKI_keyboard × state.speed_multiplier
      (or × burst_speed_multiplier if burst active)
   g. IKI_session = IKI_state × warmup_multiplier × fatigue_speed_multiplier
   h. word_modifier from language model (common: 0.80, difficult: 1.25, normal: 1.0)
   i. IKI_final = IKI_session × word_modifier

5. Compute pause (§7):
   - Apply pause stacking rules
   - Add pause to total_delay
   - Clamp total_delay to max_pause_ms

6. Add shift penalty if character requires Shift (§8.6)

7. Error decision (§9.1):
   - Compute effective_error_prob with all multipliers
   - Clamp to 0.35 max (§9.8)
   - If error triggered → select type, generate error chars
   - If error generated → select correction mode (§10.1)

8. Produce ClackEvent(s):
   - Primary event: delay_ms = total_delay, bytes = [output char(s)]
   - If immediate correction: additional events for backspace + retype
   - If delayed correction: queue for future execution
   - If insertion: two events (extra char + correct char)

9. Update engine state:
   - IKI_prev = IKI_final (for momentum)
   - prev_char = c (for keyboard model)
   - chars_emitted += 1 (for session progress)
   - burst_remaining_chars -= 1 (if burst active)
   - at_line_start = (c == '\n')
```

---

## Build System

### Cargo Workspace Configuration

**Root `Cargo.toml`:**
```toml
[workspace]
members = ["clack-core", "clack-cli"]
resolver = "2"
```

**`clack-core/Cargo.toml`:**
```toml
[package]
name = "clack-core"
version = "1.0.0"
edition = "2021"
description = "Human typing simulation engine"
license = "MIT"

[dependencies]
rand = "0.8"
rand_chacha = "0.3"
rand_distr = "0.4"
```

**`clack-cli/Cargo.toml`:**
```toml
[package]
name = "clack-cli"
version = "1.0.0"
edition = "2021"
description = "CLI driver for the Clack human typing simulator"
license = "MIT"

[[bin]]
name = "clack"
path = "src/main.rs"

[dependencies]
clack-core = { path = "../clack-core" }
clap = { version = "4", features = ["derive"] }

[dev-dependencies]
assert_cmd = "2"
```

### Build Commands

| Task | Command |
|---|---|
| Build everything (debug) | `cargo build` |
| Build release binary | `cargo build --release` |
| Run tests (all crates) | `cargo test` |
| Run only library tests | `cargo test -p clack-core` |
| Run only CLI tests | `cargo test -p clack-cli` |
| Run the CLI | `echo "hello world" \| cargo run --release -p clack-cli` |
| Check without building | `cargo check` |
| Lint | `cargo clippy -- -D warnings` |
| Format | `cargo fmt --check` |
| Build release binary location | `target/release/clack` |

### CI Pipeline (GitHub Actions)

```yaml
# .github/workflows/ci.yml
on: [push, pull_request]
jobs:
  build-and-test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: clippy, rustfmt
      - run: cargo fmt --check
      - run: cargo clippy -- -D warnings
      - run: cargo test
      - run: cargo build --release
```

---

## Module Dependency Graph

```
clack-cli/src/main.rs
    │
    ├── depends on: clack-core (ClackEngine, ClackConfig, ClackEvent)
    └── depends on: clap (argument parsing)

clack-core/src/lib.rs
    │
    ├── constants.rs    (no internal deps)
    ├── rng.rs          (depends on: rand, rand_chacha)
    ├── keyboard.rs     (depends on: constants, rng)
    ├── timing.rs       (depends on: constants, rng, keyboard, rand_distr)
    ├── state.rs        (depends on: constants, rng)
    ├── pause.rs        (depends on: constants, rng, rand_distr)
    ├── language.rs     (depends on: constants)
    ├── tokenizer.rs    (depends on: language)
    ├── session.rs      (depends on: constants)
    ├── error.rs        (depends on: constants, rng, keyboard)
    ├── correction.rs   (depends on: constants, rng, error)
    └── lib.rs          (orchestrates all of the above)
```

No circular dependencies. The dependency graph is a DAG rooted at `lib.rs`.


---

# MVP-SPEC.md

# HType 2.0 — MVP Implementation Specification

> **Purpose of this document**
> This is a complete, unambiguous implementation specification for the HType 2.0 MVP.
> Every value, formula, table, algorithm, edge case, and interface contract is defined here.
> No implementer — human or AI agent — should need to guess, infer, or make assumptions.
> Where values originate from research, the source is cited inline.

---

## Table of Contents

1. [Scope and Non-Scope](#1-scope-and-non-scope)
2. [Empirical Foundation](#2-empirical-foundation)
3. [CLI Interface](#3-cli-interface)
4. [Core Architecture Overview](#4-core-architecture-overview)
5. [Timing Model](#5-timing-model)
6. [Behavioral State Machine](#6-behavioral-state-machine)
7. [Pause System](#7-pause-system)
8. [Keyboard Layout Model](#8-keyboard-layout-model)
9. [Error System](#9-error-system)
10. [Correction System](#10-correction-system)
11. [Language Awareness](#11-language-awareness)
12. [Session Model (Warmup and Fatigue)](#12-session-model-warmup-and-fatigue)
13. [Output Protocol](#13-output-protocol)
14. [Edge Cases and Defined Behaviors](#14-edge-cases-and-defined-behaviors)
15. [Configuration Reference](#15-configuration-reference)
16. [What Changed from the Original Proposal](#16-what-changed-from-the-original-proposal)

---

## 1. Scope and Non-Scope

### 1.1 In MVP Scope

- Momentum-based inter-key timing with empirically grounded distribution
- Behavioral state machine (five states)
- Word, sentence, and stochastic thinking pauses
- Line-start hesitation
- Error generation: substitution, transposition, omission, insertion, doubling
- Delayed and uncorrected error modes
- Keyboard distance and hand-alternation timing modifiers
- Shift-key penalty
- Common-word acceleration
- Difficult-word slowdown
- Session warmup and fatigue curves
- Unix stdin→stdout streaming
- Deterministic mode via fixed seed (for testing)

### 1.2 Explicitly Out of MVP Scope

The following items are **not** implemented in the MVP and must not be partially stubbed or silently approximated:

- Code-aware typing (bracket depth parser, identifier slowdown) — moved to v2.1
- User profile recording and playback (`--profile`)
- Trigraph timing tables
- Panic events (prefix repetition, stuck-key bursts)
- External state observation (stderr state stream, named pipe)
- `--script` mode (predefined state sequences)
- Digraph timing table from a user-supplied file

These items are removed from the MVP because:

- Code-awareness requires a mini-parser; scope risk is high (noted in original proposal review)
- Profile recording has no runtime payoff until recording infra exists
- Trigraph tables add complexity without meaningful realism gain over digraph-level timing
- Panic events are rare by definition and their absence does not reduce overall believability

---

## 2. Empirical Foundation

All numeric defaults in this specification derive from published research. This section
records the sources so defaults can be traced and updated.

### 2.1 Inter-Key Interval (IKI) Distribution

**Source:** Dhakal et al., "Observations on Typing from 136 Million Keystrokes", CHI 2018.
URL: https://userinterfaces.aalto.fi/136Mkeystrokes/

| Metric | Value |
|---|---|
| Population mean IKI | 238.7 ms |
| Population SD | 111.6 ms |
| IKI skewness | 1.98 |
| IKI kurtosis | 7.1 |
| Hard floor (physical minimum) | 60 ms |
| Fast typist mean IKI (~100 WPM) | ~120 ms |
| Slow typist mean IKI (~20 WPM) | ~480 ms |

The IKI distribution is **right-skewed**, not Gaussian.
The implementation uses a **log-normal** distribution as the sampling model.

**Source for log-normal choice:** "On the shape of timings distributions in free-text
keystroke dynamics profiles", PMC8606350. The paper establishes that human inter-key
flight times best fit a log-normal (or ex-Gaussian) rather than a normal distribution,
consistent with Barabási's queuing model of human decision-making.

### 2.2 Keypress Hold (Dwell) Time

**Source:** Dhakal et al. (same dataset); secondary confirmation from multiple
digraph studies.

| Metric | Value |
|---|---|
| Mean hold/dwell time | 76.9 ms |
| SD | 22.2 ms |
| Skewness | ~0.4 (approximately normal) |

Hold time is **not** emitted in the output stream (HType outputs characters, not keydown/up
events) but the hold time value is used internally to compute the **next-key delay**:

```
next_key_delay = IKI - hold_time_of_previous_key
```

In practice, HType uses the IKI value directly as the sleep between character emissions
and does not model hold time separately in the MVP. This note exists so future versions
can refine the model.

### 2.3 WPM to IKI Conversion

Standard definition: 1 word = 5 characters (including space).

```
IKI_base_ms = (60_000 ms/min) / (WPM × 5 chars/word)
```

Examples:

| WPM | IKI_base_ms |
|---|---|
| 20 | 600.0 ms |
| 40 | 300.0 ms |
| 60 | 200.0 ms |
| 80 | 150.0 ms |
| 100 | 120.0 ms |
| 120 | 100.0 ms |

### 2.4 Error Rates and Type Distribution

**Sources:**
- Dhakal et al.: uncorrected error rate 1.0%–3.2% of keystrokes in normal typing
- Clarkson (2005) mini-QWERTY study; multiple replication studies including CHI 2025
  "Simulating Errors in Touchscreen Typing" (arXiv 2502.03560)

| Error Type | Share of All Errors |
|---|---|
| Substitution (adjacent key) | ~39% |
| Insertion (extra character) | ~33% |
| Omission (missing character) | ~21% |
| Transposition (swap two adjacent) | ~5% |
| Doubling (letter doubled) | ~2% |

**Total raw error probability (before correction):** 4%–8% of characters in normal
fast typing. Most errors are corrected immediately; the **uncorrected** rate is 1%–3%.

**Transposition note:** One study found ~1 transposition per 1,800 typed characters,
with ~76% occurring across hands (source: ScienceDirect transposition error overview).

### 2.5 Hand Alternation Effect

**Source:** Dhakal et al. IKI bigram analysis; CHI 2018 Fig. 4.

Hand-alternating bigrams (e.g., `e` left hand → `r` right hand) are typed faster
than same-hand bigrams by a consistent margin. The effect is larger for fast typists.

Approximate multipliers relative to same-hand same-finger baseline:

| Finger movement type | IKI multiplier |
|---|---|
| Same finger repetition | 1.30× |
| Same hand, adjacent finger | 1.00× (baseline) |
| Alternate hands | 0.82× |

### 2.6 Fatigue Effects

**Sources:**
- "Age Modulates the Effects of Mental Fatigue on Typewriting", Frontiers in Psychology
  2018 (PMC6049040)
- "Dynamics in typewriting performance reflect mental fatigue during real-life office work",
  PLOS ONE 2020 (PMC7537853)

Key findings:
- IKI increases with time-on-task (ToT) in all age groups
- Error rate increases with ToT in young typists
- Backspace usage increases with ToT (delayed error correction)
- Short-term lapses ("mental blocks") produce extremely long IKIs clustered together

For simulation purposes: fatigue is modeled as a gradual IKI multiplier increase
combined with a small error probability increase. See Section 12.

---

## 3. CLI Interface

### 3.1 Invocation

```
htype [OPTIONS]
```

HType reads from **stdin** and writes to **stdout**.
All diagnostic and state output goes to **stderr** only.

### 3.2 Options

| Flag | Type | Default | Description |
|---|---|---|---|
| `--wpm` | float | `60.0` | Target average words per minute |
| `--jitter` | float | `0.15` | IKI jitter coefficient (0.0 = none, 1.0 = extreme) |
| `--error-rate` | float | `0.04` | Base probability of generating an error per character (0.0–1.0) |
| `--correction-rate` | float | `0.85` | Fraction of generated errors that get corrected |
| `--no-errors` | flag | off | Disable all error generation |
| `--seed` | int | (random) | RNG seed for deterministic output. When set, behavior is fully reproducible |
| `--session-length` | int | `500` | Expected total character count; used to compute warmup/fatigue curves |
| `--no-fatigue` | flag | off | Disable warmup and fatigue session modelling |
| `--max-pause` | int | `5000` | Maximum any single pause may be in milliseconds (clamp for pipe stall safety) |
| `--thinking-pause-prob` | float | `0.015` | Probability of a stochastic thinking pause between any two words |
| `--state-output` | flag | off | Emit behavioral state transitions to stderr in machine-readable format |
| `--version` | flag | — | Print version string and exit |

### 3.3 Stdin Handling

- HType reads stdin line-by-line (or character-by-character when buffering permits)
- Stdin must be text (UTF-8). Non-UTF-8 bytes are passed through unchanged with no delay
- If stdin is empty, HType exits cleanly with code 0, emitting nothing
- HType does **not** require stdin to be seekable (pipes are supported)

### 3.4 Exit Codes

| Code | Meaning |
|---|---|
| 0 | Success |
| 1 | Invalid option or option value out of range |
| 2 | stdin read error |

---

## 4. Core Architecture Overview

```
stdin
  │
  ▼
[Reader]
  │  character stream
  ▼
[Tokenizer]  — splits into: regular chars, word boundaries, sentence boundaries,
  │             newlines, whitespace
  ▼
[State Machine]  — current state: FOCUSED / FLOW / THINKING / DISTRACTED / FATIGUED
  │                modifies: speed multiplier, pause probability, error probability
  ▼
[Timing Engine]  — computes per-character delay using:
  │                 base IKI, jitter, momentum, keyboard model, word/sentence pauses
  ▼
[Error Engine]  — stochastically injects errors before emission
  │
  ▼
[Correction Engine]  — decides: immediate correct, delayed correct, uncorrected
  │
  ▼
[Output]  — sleep(delay), write(char) to stdout
```

All components share a **single RNG instance** seeded by `--seed` (or random seed at
startup). The RNG state is threaded through every random decision so that a fixed seed
produces byte-identical output across runs.

---

## 5. Timing Model

### 5.1 Base IKI Computation

Step 1: Compute the target IKI in ms from WPM:

```
IKI_target = 60_000 / (wpm × 5)
```

Step 2: Apply the log-normal jitter.

The IKI jitter is modeled as a log-normal sample. The `--jitter` coefficient `j`
controls the sigma parameter of the underlying normal distribution:

```
sigma = j × 0.4       // at j=1.0, sigma=0.4 which gives ~40% CV
mu    = ln(IKI_target) - (sigma² / 2)   // ensures E[sample] = IKI_target

IKI_raw = exp(normal_sample(mu, sigma))
```

This preserves the mean at `IKI_target` while producing right-skewed samples consistent
with empirical IKI distributions (skewness ≈ 1.98 at default jitter).

Step 3: Apply the hard floor:

```
IKI_raw = max(IKI_raw, 60)   // 60 ms physical minimum from Dhakal et al.
```

### 5.2 Momentum Smoothing

The momentum system prevents discontinuous timing jumps. Each new raw IKI is blended
with the previous delivered IKI:

```
momentum_factor = 0.35     // fixed in MVP; configurable in v2.x
IKI_smoothed = (momentum_factor × IKI_prev) + ((1 - momentum_factor) × IKI_raw)
```

On the very first character, `IKI_prev = IKI_target` (no prior history).

### 5.3 Keyboard Modifier

After smoothing, apply the keyboard model modifier for the current bigram. See
Section 8 for the full modifier table.

```
IKI_keyboard = IKI_smoothed × keyboard_modifier(prev_char, curr_char)
```

### 5.4 State Multiplier

Apply the behavioral state speed multiplier (Section 6):

```
IKI_final_pre_pause = IKI_keyboard × state.speed_multiplier
```

### 5.5 Pause Injection

Pauses are additive delays applied **after** the character's IKI but **before** the
next character is fetched. A pause is not jittered separately; it is a clean sleep.

Pause selection logic is evaluated in this order:

1. Sentence boundary pause (highest priority; overrides word pause if both apply)
2. Word boundary pause
3. Line-start hesitation
4. Stochastic thinking pause (only eligible at word boundaries)

At most one pause type fires per position. See Section 7 for all pause values.

Final delivery delay for a character:

```
total_delay = IKI_final_pre_pause + pause_duration
total_delay = min(total_delay, max_pause_ms)   // clamp per --max-pause
```

### 5.6 Burst Mode

Burst mode temporarily reduces IKI to simulate flow-state bursts of fast typing.

A burst is triggered stochastically at word boundaries only:

```
burst_trigger_probability = 0.08   // 8% chance at each word boundary
burst_duration_chars: uniform_int(8, 25)
burst_speed_multiplier = 0.65      // 35% faster than current base
```

While a burst is active:
- The burst multiplier replaces (not stacks with) the state speed multiplier
- The burst ends after `burst_duration_chars` characters have been emitted
- Bursts cannot trigger while another burst is active
- Bursts cannot trigger in state THINKING, DISTRACTED, or FATIGUED

After a burst ends, a short recovery pause is added at the next word boundary:

```
post_burst_pause = uniform(200, 600) ms
```

---

## 6. Behavioral State Machine

### 6.1 States

| State | Description |
|---|---|
| `FOCUSED` | Normal attentive typing. Default starting state. |
| `FLOW` | Accelerated, rhythmic, highly consistent. Fewer errors. |
| `THINKING` | Slowed, with frequent pauses. Increased pre-word hesitation. |
| `DISTRACTED` | Irregular timing, higher error rate, some long gaps. |
| `FATIGUED` | Slow, more errors, delayed corrections. Cannot enter FLOW. |

### 6.2 State Parameters

| State | Speed multiplier | Error rate multiplier | Thinking pause prob multiplier | Correction rate multiplier |
|---|---|---|---|---|
| FOCUSED | 1.00 | 1.00 | 1.00 | 1.00 |
| FLOW | 0.78 | 0.50 | 0.25 | 1.10 |
| THINKING | 1.55 | 1.30 | 3.50 | 0.90 |
| DISTRACTED | 1.40 | 1.80 | 2.00 | 0.75 |
| FATIGUED | 1.35 | 1.60 | 1.50 | 0.70 |

Speed multiplier < 1 = faster (lower IKI).
All multipliers are applied on top of the base values, not each other.

### 6.3 Transition Rules

State transitions are evaluated at **word boundaries only** (at each space character).

The transition probability table (read: probability of transitioning FROM row TO column
at any given word boundary):

|  | → FOCUSED | → FLOW | → THINKING | → DISTRACTED | → FATIGUED |
|---|---|---|---|---|---|
| **FOCUSED** | 0.85 | 0.08 | 0.04 | 0.03 | 0.00 |
| **FLOW** | 0.12 | 0.80 | 0.05 | 0.02 | 0.01 |
| **THINKING** | 0.30 | 0.02 | 0.55 | 0.10 | 0.03 |
| **DISTRACTED** | 0.35 | 0.01 | 0.15 | 0.45 | 0.04 |
| **FATIGUED** | 0.20 | 0.00 | 0.20 | 0.15 | 0.45 |

Notes:
- Transition to FATIGUED is blocked until `session_progress > 0.6` (60% of expected
  session length). Before this threshold, FATIGUED is unreachable.
- Transition to FLOW is blocked when the session fatigue multiplier exceeds 1.20.
- All rows sum to 1.00.
- If a transition roll is blocked by the above rules, re-roll using the same row
  but with the blocked column's probability redistributed proportionally to FOCUSED.

### 6.4 Minimum Dwell Time Per State

A state must persist for at least this many word boundaries before another transition
is eligible:

| State | Minimum words in state |
|---|---|
| FOCUSED | 3 |
| FLOW | 5 |
| THINKING | 2 |
| DISTRACTED | 2 |
| FATIGUED | 8 |

### 6.5 State Output Format

When `--state-output` is set, emit to stderr on every state transition:

```
STATE:<new_state> PREV:<old_state> WORD:<word_count>
```

Example:
```
STATE:FLOW PREV:FOCUSED WORD:17
```

---

## 7. Pause System

### 7.1 Word Boundary Pause

Applied after emitting a space character that follows a word.

```
duration = log_normal_sample(mu=ln(80), sigma=0.3)   // mean ~80 ms
duration = clamp(duration, 30, 300)
```

### 7.2 Sentence Boundary Pause

Applied after emitting one of: `.` `?` `!` `:` `;`

These characters represent cognitive boundaries (end of thought or clause).

```
duration = log_normal_sample(mu=ln(600), sigma=0.4)  // mean ~600 ms
duration = clamp(duration, 200, 2000)
```

Note: A sentence pause overrides a word pause if both apply. They do not stack.

### 7.3 Stochastic Thinking Pause

Applied stochastically at word boundaries (space characters), independent of
sentence boundaries.

```
effective_prob = thinking_pause_prob × state.thinking_pause_prob_multiplier

if random() < effective_prob:
    duration = uniform(300, 2000) ms
```

The thinking pause **stacks** with the word boundary pause (applied as a second additive
delay after the word pause). This represents a genuine cognitive interruption on top of
the normal inter-word gap.

### 7.4 Line-Start Hesitation

Applied before emitting the **first character of a new line** (i.e., after processing
a `\n` and before the next non-newline character).

```
duration = log_normal_sample(mu=ln(200), sigma=0.45)  // mean ~200 ms
duration = clamp(duration, 80, 800)
```

This hesitation models the brief re-orientation before beginning a new line of thought.

### 7.5 Pause Stacking Rules

```
total_pause = 0

if at sentence boundary:
    total_pause += sentence_boundary_pause()
elif at word boundary:
    total_pause += word_boundary_pause()

if at line start:
    total_pause += line_start_hesitation()

if at word boundary and not at sentence boundary:
    if random() < effective_thinking_pause_prob:
        total_pause += thinking_pause()

total_pause = min(total_pause, max_pause_ms)
```

---

## 8. Keyboard Layout Model

### 8.1 Layout Definition

The MVP supports **QWERTY only**. The layout is defined as a coordinate grid where
each key has a (row, col) position. Row 0 is the top row (number row).

```
Row 0:  `  1  2  3  4  5  6  7  8  9  0  -  =
         0  1  2  3  4  5  6  7  8  9  10 11 12

Row 1:     q  w  e  r  t  y  u  i  o  p  [  ]  \
           1  2  3  4  5  6  7  8  9  10 11 12 13

Row 2:     a  s  d  f  g  h  j  k  l  ;  '
           1  2  3  4  5  6  7  8  9  10 11

Row 3:     z  x  c  v  b  n  m  ,  .  /
           1  2  3  4  5  6  7  8  9  10
```

Column positions use 0.5-unit offsets for staggered rows:

```
Row 0 offset: 0.0
Row 1 offset: 0.5
Row 2 offset: 0.75
Row 3 offset: 1.0
```

Key coordinate:
```
x = col + row_offset
y = row × 1.0
```

### 8.2 Euclidean Distance

```
distance(key_a, key_b) = sqrt((x_a - x_b)² + (y_a - y_b)²)
```

### 8.3 Distance-to-Modifier Mapping

Distance is converted to a timing multiplier. Values derived from Fitts's Law
applied to keystroke timing (see arXiv 1810.07665 for the Fitts model in typing):

| Distance range (key units) | IKI multiplier |
|---|---|
| 0.0 (same key repeat) | 1.30 |
| 0.0–1.0 (adjacent) | 1.00 (baseline) |
| 1.0–2.0 | 1.08 |
| 2.0–3.5 | 1.16 |
| 3.5–5.0 | 1.24 |
| > 5.0 | 1.35 |

### 8.4 Hand Assignment

```
Left hand keys:  q w e r t a s d f g z x c v b  (and their shifted variants)
Right hand keys: y u i o p h j k l n m  (and their shifted variants)
Space bar: alternates — modeled as whichever hand is NOT the current dominant hand
```

### 8.5 Hand Alternation Modifier

Applied on top of the distance modifier:

```
if prev_key.hand != curr_key.hand:
    hand_modifier = 0.82
elif prev_key.hand == curr_key.hand:
    hand_modifier = 1.00
```

Same-finger repetition modifier (applied only when the exact same physical key is typed
twice in a row, or when two keys share the same finger assignment):

```
if same_finger(prev_key, curr_key):
    finger_modifier = 1.30
```

When both hand_modifier and finger_modifier would apply, use `finger_modifier`
(same finger is the dominant cost).

### 8.6 Shift Key Penalty

When a character requires the Shift key (uppercase letters, `!@#$%^&*()_+{}|:"<>?`):

```
shift_penalty_ms = log_normal_sample(mu=ln(45), sigma=0.3)   // mean ~45 ms
shift_penalty_ms = clamp(shift_penalty_ms, 20, 120)
```

Add `shift_penalty_ms` to `total_delay` for that character.

### 8.7 Unknown Character Fallback

If a character is not in the layout table (emoji, non-ASCII, control characters):
- Distance modifier: 1.0 (baseline, no change)
- Hand assignment: `unknown` (no hand alternation bonus or penalty)
- Shift penalty: not applied

---

## 9. Error System

### 9.1 Error Decision

For each character `c` that would be emitted:

```
effective_error_prob = error_rate × state.error_rate_multiplier × session_error_multiplier

if random() < effective_error_prob:
    generate_error(c)
else:
    emit(c)
```

### 9.2 Error Type Selection

Error type is selected using the empirical distribution from Section 2.4:

| Error Type | Cumulative probability |
|---|---|
| Substitution | 0.39 |
| Insertion | 0.72 |
| Omission | 0.93 |
| Transposition | 0.98 |
| Doubling | 1.00 |

Implementation: draw `r = random()`; select first type where cumulative ≥ r.

### 9.3 Substitution Error

Replace the intended character with an adjacent key character.

**Adjacent key selection:**

1. Look up `c` in the layout table
2. Find all keys with `distance(c, candidate) ≤ 1.5`
3. Exclude `c` itself
4. Select uniformly at random from candidates

If `c` has no adjacent keys in the layout (rare for symbols), skip and emit `c` normally.

The substituted character preserves the case of the original (if `c` is uppercase
and the adjacent key maps to a letter, emit the uppercase version of that letter).

### 9.4 Insertion Error

Emit an extra character before or after the intended character.

```
position = random choice: BEFORE or AFTER (equal probability)
extra_char = random adjacent key of c (same selection as substitution)
```

If `position == BEFORE`:
  emit extra_char, then emit c
If `position == AFTER`:
  emit c, then emit extra_char

The extra character is emitted with a delay drawn from the normal IKI model.
The insertion does **not** trigger a recursive error check.

### 9.5 Omission Error

Simply do not emit `c`. No output for this character.

Omission errors are only generated for non-space, non-newline characters.
If `c` is a space or newline, re-roll a different error type.

### 9.6 Transposition Error

Swap the current character with the next character in the stream.

Implementation:
1. Look ahead one character: `next_c`
2. If `next_c` is a space or newline: downgrade to substitution error instead
   (transpositions across word boundaries are not generated)
3. Otherwise: emit `next_c` first, then emit `c`
   (the lookahead character is consumed; the "next" slot after that resumes normally)

The timing between the two swapped characters uses normal IKI (no special delay).

### 9.7 Doubling Error

Emit `c` twice.

```
emit c
sleep(IKI_for_repeated_key)   // uses same-key distance modifier
emit c
```

Doubling errors are only generated for alphabetic characters. If `c` is a symbol,
digit, or space, re-roll a different error type.

### 9.8 Error Rate Bounds

The effective error probability is clamped regardless of multipliers:

```
effective_error_prob = min(effective_error_prob, 0.35)
```

No more than 35% of characters may be errors. This prevents extreme configurations
from producing unreadable output.

---

## 10. Correction System

### 10.1 Correction Decision

Immediately after generating an error, decide how it will be handled:

```
effective_correction_rate = correction_rate × state.correction_rate_multiplier

r = random()

if r < effective_correction_rate × 0.70:
    mode = IMMEDIATE
elif r < effective_correction_rate:
    mode = DELAYED
else:
    mode = UNCORRECTED
```

This produces: ~70% of corrected errors are immediate; ~30% are delayed.

### 10.2 Immediate Correction

After emitting the error character(s), immediately emit backspace(s) and retype:

```
// Substitution: 1 wrong char emitted
sleep(correction_pause)
emit BACKSPACE
sleep(correction_pause)
emit correct_char

// Insertion: 1 extra char emitted before/after correct char
// Before: wrong_char, correct_char already emitted
sleep(correction_pause)
emit BACKSPACE × 2
sleep(correction_pause)
emit correct_char

// Transposition: two chars emitted in wrong order
// Must backspace 2, retype both
sleep(correction_pause)
emit BACKSPACE × 2
sleep(correction_pause)
emit correct_char
emit lookahead_char

// Doubling: one extra char emitted
sleep(correction_pause)
emit BACKSPACE
```

Where:

```
correction_pause = log_normal_sample(mu=ln(120), sigma=0.35)   // mean ~120 ms
correction_pause = clamp(correction_pause, 60, 350)
```

Backspace is emitted as a literal ASCII backspace character (0x08) followed by a
space character (0x20) followed by another backspace (0x08). This is the standard
terminal overwrite sequence for erasing a character visually:

```
BACKSPACE sequence: \x08 \x20 \x08
```

**Important:** The space in the sequence must not trigger a word-boundary pause.
Flag the space as `erase_space` and skip all pause logic for it.

### 10.3 Delayed Correction

The error is emitted and continues to be visible in the output for a number of
characters before correction occurs.

```
delay_chars = uniform_int(3, 12)   // continue typing this many chars before correcting
```

After `delay_chars` characters of normal typing:
1. Pause briefly (the typist "notices" the error):
   ```
   notice_pause = uniform(200, 600) ms
   ```
2. Emit backspaces to return to the error position.
   The number of backspaces equals `delay_chars + error_length`:
   ```
   // error_length: 1 for substitution/omission/doubling; 2 for transposition/insertion
   backspace_count = delay_chars + error_length
   emit BACKSPACE × backspace_count
   ```
3. Re-type the `delay_chars` characters correctly (they were typed correctly originally,
   so the content is in the lookahead buffer).
4. No additional pause after retyping; normal typing resumes.

**Lookahead buffer:** The engine must buffer the `delay_chars` characters that are
typed after the error but before correction. These are stored in memory; they are
emitted to stdout as normal, then after the delay, the correction sequence rewinds
and re-emits them correctly. Since HType is a stream processor, the actual output
at this point is:

```
[error_char(s)][normal_chars × delay_chars][backspaces × (delay_chars + error_len)][correct_chars × (delay_chars + 1)]
```

This faithfully represents what appears on a real terminal.

### 10.4 Uncorrected Errors

The error is emitted and no correction follows. Nothing further happens. The output
contains the typo permanently.

### 10.5 Correction Strategy Variation

Two correction backspace strategies are modeled:

**Strategy A — Character-by-character** (probability 0.65):
Backspaces are emitted one at a time with a per-backspace delay:
```
per_backspace_delay = uniform(60, 130) ms
```

**Strategy B — Held backspace** (probability 0.35):
All backspaces are emitted with minimal delay between them (simulating held key):
```
per_backspace_delay = uniform(15, 35) ms
```

Strategy is selected once per correction event, not per session.

---

## 11. Language Awareness

### 11.1 Common Word Acceleration

Words in the common-word list receive a speed boost because they are typed as
"motor chunks" rather than letter-by-letter.

**Mechanism:** When the tokenizer identifies that the upcoming word (the characters
up to the next space) matches a common word, a `chunk_multiplier` is applied to
all IKIs within that word:

```
chunk_multiplier = 0.80   // 20% faster for common words
```

**Common word list (MVP — 60 words):**

```
the, be, to, of, and, a, in, that, have, it,
for, not, on, with, he, as, you, do, at, this,
but, his, by, from, they, we, say, her, she, or,
an, will, my, one, all, would, there, their, what,
so, up, out, if, about, who, get, which, go, me,
when, make, can, like, time, no, just, him, know, take
```

Matching is **case-insensitive**. Punctuation attached to the word (e.g., `the,`) is
stripped before lookup. The punctuation character itself does not receive the boost.

### 11.2 Difficult Word Slowdown

Long or structurally complex words trigger a slowdown because they require
deliberate letter-by-letter attention.

**Trigger conditions (any one is sufficient):**

1. Word length ≥ 9 characters
2. Word contains a double-letter cluster that is not a common pattern
   (double-letter common patterns: `ss`, `tt`, `ll`, `nn`, `ee`, `oo` — these are
   familiar and do not trigger slowdown)
3. Word contains 3 or more consecutive consonants (excluding `th`, `sh`, `ch`, `wh`,
   `ph`, `gh` which are digraphs treated as single units)

**Mechanism:**
```
slowdown_multiplier = 1.25   // 25% slower for difficult words
```

Applied to all IKIs within the word. Does not stack with common-word acceleration.
If a word matches both conditions (theoretically impossible but defensively handled),
apply no modifier (1.0×).

### 11.3 Word Detection

The tokenizer defines a "word" as a maximal sequence of characters that are not
spaces, tabs, or newlines. Punctuation is part of the word token for timing purposes
but stripped for dictionary lookup.

---

## 12. Session Model (Warmup and Fatigue)

### 12.1 Session Progress

Session progress `p` is a float in [0.0, 1.0]:

```
p = chars_emitted / session_length
```

Where `session_length` = value of `--session-length`.

If `chars_emitted` exceeds `session_length`, clamp `p` at 1.0.

### 12.2 Warmup Curve

Active during `p ∈ [0.0, 0.10]` (first 10% of session).

```
warmup_multiplier = 1.0 + 0.30 × (1.0 - p / 0.10)
```

At `p = 0.0`: multiplier = 1.30 (30% slower)
At `p = 0.10`: multiplier = 1.00 (back to baseline)

### 12.3 Fatigue Curve

Active during `p ∈ [0.60, 1.0]` (final 40% of session).

```
fatigue_progress = (p - 0.60) / 0.40         // 0.0 at p=0.60, 1.0 at p=1.0
fatigue_speed_multiplier = 1.0 + 0.25 × fatigue_progress   // up to 25% slower
fatigue_error_multiplier = 1.0 + 0.50 × fatigue_progress   // up to 50% more errors
```

### 12.4 Middle Zone

During `p ∈ [0.10, 0.60]` (the middle 50%):
- warmup_multiplier = 1.0
- fatigue_speed_multiplier = 1.0
- fatigue_error_multiplier = 1.0

### 12.5 Applying Session Multipliers

The session multipliers are applied **after** all other timing calculations and
**after** the state machine multiplier. They are cumulative:

```
IKI_final = IKI_final_pre_pause × warmup_multiplier × fatigue_speed_multiplier
```

For error rate:
```
session_error_multiplier = fatigue_error_multiplier   // warmup does not affect errors
```

### 12.6 Fatigue Lapse Events

Research shows fatigue produces sudden "mental block" lapses with very long IKIs
clustered together. This is modeled as:

```
// Only eligible when p > 0.70 and state == FATIGUED
lapse_probability_per_word = 0.03    // 3% at each word boundary

if random() < lapse_probability_per_word:
    lapse_duration = uniform(1500, 4000) ms
    lapse_duration = min(lapse_duration, max_pause_ms)
    // Inject as a thinking pause at the next word boundary
```

---

## 13. Output Protocol

### 13.1 Character Emission

Each character is emitted to stdout immediately after its sleep delay.
HType does **not** buffer output. Use `fflush` (or equivalent) after each character.

### 13.2 Backspace Encoding

Backspace erases are encoded as the three-byte sequence: `\x08 \x20 \x08`
(backspace, space, backspace). This is compatible with ANSI terminals and most
terminal emulators.

**Not used:** ANSI escape sequences (e.g., `\x1b[1D`) are not used in the MVP because
HType must work in dumb terminals and raw pipes where ANSI is not guaranteed.

### 13.3 Newline Handling

Newlines in stdin are passed through as-is (LF on Unix). No CRLF conversion.
The line-start hesitation is triggered by a newline on stdin, not on the
newline character itself — the hesitation occurs before the next non-whitespace
character following the newline.

### 13.4 EOF Behavior

When stdin reaches EOF:

1. Complete any in-progress delayed correction (emit remaining backspaces and retyped chars)
2. Emit nothing else
3. Flush stdout
4. Exit with code 0

If a delayed correction is in progress and EOF occurs **mid-correction** (e.g., only
3 of 7 backspaces have been emitted): complete the full correction sequence, then exit.
Do **not** leave the output in a partially corrected state.

### 13.5 Pipe Stall Behavior

If stdin blocks (upstream producer is slow), HType waits.
No special behavior is triggered; the typist is simply paused.
When data arrives, HType resumes. This is invisible to the output.

The `--max-pause` clamp applies only to internally generated pauses, not to
the latency caused by stdin blocking.

---

## 14. Edge Cases and Defined Behaviors

| Scenario | Defined behavior |
|---|---|
| Input is a single character | Apply IKI + any applicable pause; emit; exit |
| Input is only whitespace | Pass through with word-boundary pauses; no errors generated on whitespace |
| `--wpm 0` or negative | Reject at startup; exit code 1 |
| `--error-rate 0.0` | Equivalent to `--no-errors`; no errors generated |
| `--correction-rate 0.0` | All generated errors are uncorrected |
| `--correction-rate 1.0` | All generated errors are corrected (split 70% immediate / 30% delayed) |
| Error on the very first character | Generate error normally; correction works normally |
| Error on the very last character before EOF | Immediate correction: complete before EOF. Delayed correction with not enough remaining chars: convert to immediate correction |
| Transposition lookahead hits EOF | Downgrade to substitution error |
| Transposition lookahead hits a newline | Downgrade to substitution error |
| Transposition lookahead hits a space | Downgrade to substitution error |
| Common word is also long (≥ 9 chars) | Common-word acceleration takes priority; no slowdown |
| `--no-fatigue` flag | warmup_multiplier = 1.0, fatigue multipliers = 1.0 for entire session; lapse events disabled |
| Non-ASCII UTF-8 character | Emitted with baseline IKI and no keyboard model; no error generated on it |
| Control characters other than `\n` and `\t` | Passed through with baseline IKI; no errors |
| Tab character | Treated as a word boundary; word-boundary pause may apply; no line-start hesitation |
| Two consecutive sentence-boundary characters (e.g., `...`) | Only the first triggers a sentence pause; subsequent ones get only word-boundary pause |
| Empty word (double space) | Passes through; second space gets word-boundary pause |
| Burst triggers and then state transitions to THINKING | Burst ends immediately on state transition to THINKING/DISTRACTED/FATIGUED |
| `--seed` provided | All random draws use the seeded RNG. Including state transitions, pauses, errors, and corrections. Output is byte-identical across runs with the same seed and same input |

---

## 15. Configuration Reference

Complete set of tunable internal constants. These are **not** CLI flags in the MVP
but are named constants in the source code that can be changed at compile/configure
time.

```
// Timing
MOMENTUM_FACTOR             = 0.35
IKI_HARD_FLOOR_MS           = 60
JITTER_SIGMA_SCALE          = 0.40    // sigma = jitter × JITTER_SIGMA_SCALE

// Pauses
WORD_PAUSE_MU_MS            = 80
WORD_PAUSE_SIGMA            = 0.30
WORD_PAUSE_MIN_MS           = 30
WORD_PAUSE_MAX_MS           = 300

SENTENCE_PAUSE_MU_MS        = 600
SENTENCE_PAUSE_SIGMA        = 0.40
SENTENCE_PAUSE_MIN_MS       = 200
SENTENCE_PAUSE_MAX_MS       = 2000

LINE_START_MU_MS            = 200
LINE_START_SIGMA            = 0.45
LINE_START_MIN_MS           = 80
LINE_START_MAX_MS           = 800

// Bursts
BURST_TRIGGER_PROB          = 0.08
BURST_DURATION_MIN_CHARS    = 8
BURST_DURATION_MAX_CHARS    = 25
BURST_SPEED_MULTIPLIER      = 0.65
POST_BURST_PAUSE_MIN_MS     = 200
POST_BURST_PAUSE_MAX_MS     = 600

// Keyboard
SHIFT_PENALTY_MU_MS         = 45
SHIFT_PENALTY_SIGMA         = 0.30
SHIFT_PENALTY_MIN_MS        = 20
SHIFT_PENALTY_MAX_MS        = 120

// Corrections
CORRECTION_PAUSE_MU_MS      = 120
CORRECTION_PAUSE_SIGMA      = 0.35
CORRECTION_PAUSE_MIN_MS     = 60
CORRECTION_PAUSE_MAX_MS     = 350

DELAYED_CORRECTION_MIN_CHARS = 3
DELAYED_CORRECTION_MAX_CHARS = 12
NOTICE_PAUSE_MIN_MS         = 200
NOTICE_PAUSE_MAX_MS         = 600

CHAR_BY_CHAR_BACKSPACE_PROB = 0.65
CHAR_BY_CHAR_MIN_MS         = 60
CHAR_BY_CHAR_MAX_MS         = 130
HELD_BACKSPACE_MIN_MS       = 15
HELD_BACKSPACE_MAX_MS       = 35

IMMEDIATE_CORRECTION_SHARE  = 0.70   // of corrected errors

// Language
COMMON_WORD_MULTIPLIER      = 0.80
DIFFICULT_WORD_MULTIPLIER   = 1.25
DIFFICULT_WORD_MIN_LENGTH   = 9

// Session
WARMUP_FRACTION             = 0.10   // first 10% of session
FATIGUE_START_FRACTION      = 0.60   // fatigue begins at 60%
WARMUP_SPEED_PENALTY        = 0.30   // +30% slower at start
FATIGUE_MAX_SPEED_PENALTY   = 0.25   // up to +25% slower at end
FATIGUE_MAX_ERROR_PENALTY   = 0.50   // up to +50% more errors at end
FATIGUE_LAPSE_START         = 0.70
FATIGUE_LAPSE_PROB          = 0.03
FATIGUE_LAPSE_MIN_MS        = 1500
FATIGUE_LAPSE_MAX_MS        = 4000

// Error rate cap
ERROR_RATE_MAX_EFFECTIVE    = 0.35
```

---

## 16. What Changed from the Original Proposal

### Added

**From new research findings:**

- **Empirical IKI values** — All default timing values are now anchored to the
  Dhakal et al. 136-million-keystroke dataset. The original proposal had no specific
  millisecond values.
- **Log-normal distribution** — The proposal mentioned log-normal as a possibility;
  this spec mandates it with the correct parameterization (mean-preserving log-normal).
- **Hard IKI floor of 60 ms** — Empirically derived from the same dataset; prevents
  physically impossible outputs.
- **Error type distribution table** — The original proposal listed error types without
  frequencies. This spec provides empirical proportions from multiple studies.
- **Transposition rate and cross-hand note** — Research shows ~1 transposition per
  1,800 chars and 76% are cross-hand; reflected in the type distribution table.
- **Fatigue lapse events** — Derived from the Frontiers in Psychology (2018) fatigue
  study showing clustered "mental block" IKI spikes. Not in the original proposal.

**From the architecture review (suggestions from original proposal critique):**

- **Correction strategy variation** — Character-by-character vs. held backspace
  (probability 0.65/0.35). This was raised in the review as very distinctive per-person.
- **Look-ahead hesitation** — Renamed "pre-word hesitation" and merged into the
  line-start hesitation and sentence-boundary pause system. Distinct pre-word pause is
  captured by the stochastic thinking pause which fires before the next word, not after
  the previous one.
- **State output flag** (`--state-output`) — From the "externally observable state"
  suggestion.
- **Capital letter context note** — Shift penalty is applied uniformly in the MVP;
  distinguishing sentence-initial capitals from mid-word capitals (e.g., `GitHub`) is
  noted as a v2.1 refinement.
- **EOF mid-correction defined behavior** — Section 14 defines this explicitly.
- **Pipe stall behavior** — Section 13.5 defines this explicitly.
- **Transposition at EOF / newline / space** — All downgrade behaviors defined in
  Section 14.

**New items not in either the original or the review:**

- **Minimum state dwell times** — Prevents flickering state transitions; improves
  rhythm realism.
- **Burst end on state transition** — Logically correct; burst cannot continue into
  a THINKING state.
- **`--seed` determinism guarantee** — Explicit byte-identical reproducibility for
  testing.
- **Common-word list** — The original mentioned the concept; this spec provides the
  actual 60-word list.
- **Difficult-word detection rules** — Three specific trigger conditions defined;
  the original said only "long or uncommon words."
- **Backspace encoding spec** — `\x08 \x20 \x08` sequence; explicit choice of no ANSI
  escape sequences for pipe/dumb-terminal compatibility.
- **Erase-space guard** — The space in the backspace sequence must not trigger a
  word-boundary pause; guarded explicitly.
- **Error rate cap (35%)** — Safety clamp preventing unreadable output at extreme settings.

### Removed

- **Code-aware typing** — Removed from MVP scope. The original included it as an MVP
  item. Moved to v2.1 due to parser complexity and scope risk (explicitly flagged in
  the review as a known risk).
- **Trigraph timing** — Removed. Adds significant table complexity for marginal gain
  over digraph-level timing.
- **Panic events** (`gitgit`, `pproject` style) — Removed. By definition rare; their
  absence does not meaningfully reduce realism. Can be added in a future update.
- **`--script` mode** — Removed. Useful but adds a new input format and parser; not
  necessary for the core simulation.
- **Profile recording** — Removed. No infra to record or replay; deferred to post-MVP.
- **Digraph timing table from file** — Removed. Noted as a future enhancement; not
  necessary for MVP believability.

### Refined (originally vague, now specific)

- **Burst mode** — Original said "humans type in bursts" with no numbers. This spec
  defines trigger probability (8%), duration range (8–25 chars), speed multiplier (0.65×),
  and post-burst pause (200–600 ms).
- **Thinking pause** — Original said "300 ms–2000 ms, low probability." This spec
  defines the exact probability (1.5%), the interaction with state multipliers, and
  the stacking rules with word-boundary pauses.
- **Fatigue** — Original said "slower, more mistakes, more pauses" with no numbers.
  This spec defines the full warmup and fatigue curves with exact multiplier formulas.
- **Correction timing** — Original said errors are corrected with "added delay" but
  gave no correction pause values. This spec defines all pause distributions.

---

*End of HType 2.0 MVP Specification*
*Document version: 1.0 | Based on research through 2025*


---

# ROADMAP.md

# ROADMAP.md — Development Phases

> Six phases. Each phase has explicit deliverables and done criteria.
> Phases must not be merged or collapsed.

---

## Phase 0 — Repository Setup

**Goal:** The repository is navigable, CI runs, and all planning documents are in place. No source code.

### Deliverables

- [x] `README.md` — project overview (exists, to be expanded)
- [ ] `LICENSE` — MIT license file
- [ ] `.gitignore` — Rust/Cargo patterns (`/target`, `Cargo.lock` for library, etc.)
- [ ] All seven planning documents:
  - [ ] `AGENT.md`
  - [ ] `PROJECT.md`
  - [ ] `ARCHITECTURE.md`
  - [ ] `DECISIONS.md`
  - [ ] `ROADMAP.md` (this file)
  - [ ] `TECH_DEBT.md`
  - [ ] `CONTRIBUTING.md`
- [ ] `spec/htype-2-mvp-spec.md` — copy of MVP specification
- [ ] `.github/workflows/ci.yml` — CI pipeline skeleton (build + test + clippy + fmt)

### Done Criteria

- Repository has all planning documents committed and readable.
- `.gitignore` correctly excludes Cargo build artifacts.
- CI pipeline triggers on push/PR and passes (even if tests are minimal).
- No source code exists yet — this phase is documentation only.

---

## Phase 1 — Library Skeleton

**Goal:** The project structure is created, all library modules are stubbed, the public API is defined, and the build system works.

### Deliverables

- [ ] Root `Cargo.toml` — workspace manifest with `clack-core` and `clack-cli` members
- [ ] `clack-core/Cargo.toml` — library crate manifest with `rand`, `rand_chacha`, `rand_distr` dependencies
- [ ] `clack-cli/Cargo.toml` — binary crate manifest with `clack-core` and `clap` dependencies
- [ ] `clack-core/src/lib.rs` — public API fully defined and documented:
  - `ClackConfig` struct with all fields and defaults
  - `ClackEvent` struct
  - `BehavioralState` enum
  - `StateTransition` struct
  - `ConfigError` enum
  - `ClackEngine` struct with `new()`, `feed()`, `next_event()`, `finish()` method signatures
  - All types marked `#[non_exhaustive]` where specified in DECISIONS.md
- [ ] `clack-core/src/constants.rs` — all §15 constants defined as `pub const` with spec citations
- [ ] `clack-core/src/rng.rs` — `ClackRng` struct wrapping `ChaCha8Rng` with:
  - `new(seed: Option<u64>)` constructor
  - `sample_log_normal(mu: f64, sigma: f64) -> f64`
  - `sample_uniform(min: f64, max: f64) -> f64`
  - `sample_uniform_int(min: usize, max: usize) -> usize`
  - `sample_bool(probability: f64) -> bool`
- [ ] `clack-core/src/timing.rs` — stub with documented function signatures
- [ ] `clack-core/src/state.rs` — stub with `BehavioralState` enum and transition matrix defined
- [ ] `clack-core/src/pause.rs` — stub with documented function signatures
- [ ] `clack-core/src/keyboard.rs` — stub with documented function signatures
- [ ] `clack-core/src/error.rs` — stub with error type enum and documented function signatures
- [ ] `clack-core/src/correction.rs` — stub with documented function signatures
- [ ] `clack-core/src/language.rs` — stub with common word list defined
- [ ] `clack-core/src/session.rs` — stub with documented function signatures
- [ ] `clack-core/src/tokenizer.rs` — stub with documented function signatures
- [ ] `clack-cli/src/main.rs` — stub that:
  - Parses CLI arguments with clap
  - Creates a `ClackEngine` with default config
  - Reads stdin and calls `engine.feed()` / `engine.next_event()`
  - Compiles and runs (even if output is trivial)
- [ ] One test: calls `ClackEngine::new()` with valid config, calls `finish()`, asserts no panic

### Done Criteria

- `cargo build` succeeds for both crates.
- `cargo test` passes (at least one test).
- `cargo clippy -- -D warnings` passes.
- The library crate does NOT import `std::io`.
- The CLI binary exists at `target/debug/clack` and runs (even if it does nothing useful).
- Every module file exists with documented function signatures (may return defaults/stubs).
- The library/driver split is enforced by the Cargo workspace structure.

---

## Phase 2 — MVP Feature Implementation

**Goal:** All 45 tasks are complete. The simulation engine produces realistic human typing output. `--seed` produces byte-identical output.

### Task List (Implementation Order)

Dependencies are respected: each task builds on prior tasks.

#### Foundation (Tasks 01–05)

| # | Task | Spec Section | Module | Depends On |
|---|---|---|---|---|
| 01 | RNG subsystem with `--seed` support | §4, §15 | `rng.rs` | — |
| 02 | WPM-to-IKI conversion | §5.1, §2.3 | `timing.rs` | — |
| 03 | Log-normal IKI sampler | §5.1, §2.1 | `timing.rs` | 01 |
| 04 | Hard floor clamp (60 ms) | §5.1 | `timing.rs` | 03 |
| 05 | Momentum smoothing | §5.2 | `timing.rs` | 04 |

#### Keyboard Model (Tasks 06–12)

| # | Task | Spec Section | Module | Depends On |
|---|---|---|---|---|
| 06 | QWERTY keyboard layout table | §8.1 | `keyboard.rs` | — |
| 07 | Euclidean distance calculator | §8.2 | `keyboard.rs` | 06 |
| 08 | Distance-to-modifier mapping | §8.3 | `keyboard.rs` | 07 |
| 09 | Hand assignment table | §8.4 | `keyboard.rs` | 06 |
| 10 | Hand alternation modifier | §8.5 | `keyboard.rs` | 09 |
| 11 | Shift key penalty | §8.6 | `keyboard.rs` | 01 |
| 12 | Unknown character fallback | §8.7 | `keyboard.rs` | 06 |

#### Per-Character Timing Pipeline (Task 13)

| # | Task | Spec Section | Module | Depends On |
|---|---|---|---|---|
| 13 | Full per-character IKI pipeline | §5.3, §5.4 | `timing.rs` | 05, 08, 10, 11, 12 |

#### State Machine (Tasks 14–16)

| # | Task | Spec Section | Module | Depends On |
|---|---|---|---|---|
| 14 | Behavioral state machine — states and parameters | §6.1, §6.2 | `state.rs` | — |
| 15 | State transition logic and dwell times | §6.3, §6.4 | `state.rs` | 14, 01 |
| 16 | State output flag | §6.5 | `state.rs` | 15 |

#### Pause System (Tasks 17–21)

| # | Task | Spec Section | Module | Depends On |
|---|---|---|---|---|
| 17 | Word boundary pause | §7.1 | `pause.rs` | 01 |
| 18 | Sentence boundary pause | §7.2 | `pause.rs` | 01 |
| 19 | Stochastic thinking pause | §7.3 | `pause.rs` | 01, 15 |
| 20 | Line-start hesitation | §7.4 | `pause.rs` | 01 |
| 21 | Pause stacking logic | §7.5 | `pause.rs` | 17, 18, 19, 20 |

#### Burst Mode (Task 22)

| # | Task | Spec Section | Module | Depends On |
|---|---|---|---|---|
| 22 | Burst mode | §5.6 | `timing.rs` | 13, 15 |

#### Language Model (Tasks 23–25)

| # | Task | Spec Section | Module | Depends On |
|---|---|---|---|---|
| 23 | Common word list and acceleration | §11.1 | `language.rs` | — |
| 24 | Difficult word detection and slowdown | §11.2 | `language.rs` | — |
| 25 | Tokenizer — feeds tasks 17–24 | §11.3 | `tokenizer.rs` | 23, 24 |

#### Error System (Tasks 26–32)

| # | Task | Spec Section | Module | Depends On |
|---|---|---|---|---|
| 26 | Error type selection | §9.2 | `error.rs` | 01 |
| 27 | Substitution error | §9.3 | `error.rs` | 06, 26 |
| 28 | Insertion error | §9.4 | `error.rs` | 06, 26 |
| 29 | Omission error | §9.5 | `error.rs` | 26 |
| 30 | Transposition error | §9.6 | `error.rs` | 26 |
| 31 | Doubling error | §9.7 | `error.rs` | 26 |
| 32 | Error rate cap (35%) | §9.8 | `error.rs` | 26 |

#### Correction System (Tasks 33–36)

| # | Task | Spec Section | Module | Depends On |
|---|---|---|---|---|
| 33 | Correction strategy selection | §10.1, §10.5 | `correction.rs` | 01 |
| 34 | Immediate correction | §10.2 | `correction.rs` | 33 |
| 35 | Delayed correction with lookahead buffer | §10.3 | `correction.rs` | 33, 34 |
| 36 | Uncorrected errors | §10.4 | `correction.rs` | 33 |

#### Session Model (Tasks 37–40)

| # | Task | Spec Section | Module | Depends On |
|---|---|---|---|---|
| 37 | Warmup curve | §12.2 | `session.rs` | — |
| 38 | Fatigue curve | §12.3 | `session.rs` | — |
| 39 | Fatigue lapse events | §12.6 | `session.rs` | 38, 01, 15 |
| 40 | Session progress tracking | §12.1 | `session.rs` | 37, 38 |

#### Integration (Tasks 41–45)

| # | Task | Spec Section | Module | Depends On |
|---|---|---|---|---|
| 41 | Full CLI flag parsing | §3.2 | `main.rs` | — |
| 42 | Stdin handling and EOF behavior | §3.3, §13.4, §13.5 | `main.rs`, `lib.rs` | 35 |
| 43 | Backspace and output encoding | §13.1, §13.2, §13.3 | `correction.rs` | 34, 35 |
| 44 | Edge case handling — all entries in §14 | §14 | various | all |
| 45 | All constants from §15 wired to named constants | §15 | `constants.rs` | — |

### Done Criteria

- All 45 tasks complete and tested.
- All edge cases in spec §14 produce the defined behavior.
- `echo "test input" | clack --seed 42` produces byte-identical output on repeated runs.
- `cargo test` passes all unit and integration tests.
- `cargo clippy -- -D warnings` passes.
- The library crate does NOT import `std::io`.

---

## Phase 3 — MVP Hardening

**Goal:** Production-quality release v1.0.0 with comprehensive tests, documentation, and performance validation.

### Deliverables

- [ ] **Deterministic output test suite:**
  - Multiple test cases with fixed seeds and fixed inputs
  - Byte-compare actual stdout against committed snapshot files
  - Test at various WPM settings (20, 60, 100, 120)
  - Test with errors enabled and disabled
  - Test with fatigue enabled and disabled

- [ ] **Edge case test coverage:**
  - Every row in spec §14 covered by a named test
  - Tests for: single character input, whitespace-only input, `--wpm 0` rejection, `--error-rate 0.0`, transposition at EOF, consecutive sentence boundaries, empty word (double space), burst + state transition interaction

- [ ] **Performance validation:**
  - No perceptible output lag at `--wpm 120` on standard Linux terminal
  - Measure actual character emission rate vs expected rate
  - Profile the per-character processing pipeline — must complete in < 1 ms per character

- [ ] **README.md expansion:**
  - Installation instructions (download binary, or `cargo install`)
  - Quick-start usage examples
  - Full CLI option documentation
  - Attribution to original project
  - Link to ARCHITECTURE.md and CONTRIBUTING.md

- [ ] **Man page:**
  - Generated using `clap_mangen` crate
  - Installed to `target/man/clack.1`
  - Documents all CLI options, stdin/stdout behavior, exit codes
  - Build step: `cargo run -p clack-cli --features mangen -- --generate-man`

- [ ] **First tagged release: v1.0.0**
  - GitHub release with prebuilt Linux binary
  - Changelog documenting all features

### Done Criteria

- v1.0.0 tag exists on `main` branch.
- CI passes with all tests (unit, integration, deterministic snapshots, edge cases).
- Man page builds and installs.
- README has complete installation and usage instructions.
- Performance check: no visible lag at 120 WPM.

---

## Phase 4 — Post-MVP CLI Features

**Goal:** Extend the CLI with features deferred from the MVP. Items from spec §1.2 only.

### Planned Features

| Feature | Source | Notes |
|---|---|---|
| Code-aware typing | spec §1.2 | Bracket depth parser, identifier slowdown. Requires a mini-parser. |
| Trigraph timing tables | spec §1.2 | Extension of the keyboard distance model to 3-character sequences. |
| Panic events | spec §1.2 | Prefix repetition (`gitgit`), stuck-key bursts. Rare events. |
| `--script` mode | spec §1.2 | Predefined state sequences — new input format. |
| Profile recording and playback | spec §1.2 | Record timing data during typing, replay from file. |
| Digraph timing table from file | spec §1.2 | User-supplied timing overrides. |
| Additional keyboard layouts | DECISIONS-007 | Dvorak, Colemak, AZERTY. Layout from configuration file. |

### Implementation Order

Not defined at this stage. Each feature should be scoped and planned individually before implementation.

### Done Criteria

- Each feature is specified, implemented, tested, and released as a minor version bump.
- No feature in this phase changes the `clack-core` public API contract.

---

## Phase 5 — GUI

**Goal:** A graphical user interface that uses the same `clack-core` library to visualize human typing simulation in real time.

### Scope

To be defined at the end of Phase 3. The only constraint defined now:

> **The `clack-core` library API must not change to accommodate the GUI.** The GUI adapts to the library, not the other way around.

### Candidate GUI Frameworks

Given the language choice of Rust, reasonable candidates include: [PROPOSED]

| Framework | Notes |
|---|---|
| **egui + eframe** | Immediate-mode GUI. Pure Rust. Easy to prototype. Good for real-time visualization. |
| **iced** | Elm-inspired, reactive. Pure Rust. Strong layout system. |
| **Tauri** | Web-based UI (HTML/CSS/JS) with Rust backend. Familiar for web developers. |
| **gtk4-rs** | GTK 4 bindings. Native look on Linux. Heavier dependency. |

No framework is chosen at this time. The decision will be made based on the visualization requirements defined at the end of Phase 3.

### Potential GUI Features

- Real-time text area showing typed output with cursor
- Behavioral state indicator (color-coded: FOCUSED, FLOW, THINKING, DISTRACTED, FATIGUED)
- Typing speed graph (WPM over time)
- Error overlay showing where errors occurred and how they were corrected
- Configuration panel for all `ClackConfig` fields
- Session progress bar with warmup/fatigue zones

### API Evolution

If the GUI requires richer events (e.g., semantic "delete character at position N" instead of raw `\x08\x20\x08` bytes), this should be handled by:
1. Adding a new method to `ClackEngine` (e.g., `next_semantic_event()`) alongside the existing `next_event()`, OR
2. Adding a post-processing layer in the GUI crate that parses raw byte events into semantic events.

Option 2 is preferred to avoid changing the library API. See TECH_DEBT.md for details.


---

# TECH_DEBT.md

# TECH_DEBT.md — Known Limitations and Future Risks

> This document tracks three categories of technical debt:
> 1. Inherited limitations from the original project's approach
> 2. MVP scope compressions (things simplified for the MVP release)
> 3. Future API stability risks (things that may need to change for the GUI)

---

## 1. Inherited Limitations

These are aspects of the original htype project that Clack deliberately handles differently. The original approach was limited in the ways described below.

### 1.1 No Behavioral State Machine [VERIFIED — spec §6]

The original project uses simple random jitter with no concept of behavioral states. A real typist shifts between focused, flowing, thinking, distracted, and fatigued modes — each with distinct timing profiles.

**Clack's approach:** Five-state behavioral state machine with empirically derived transition probabilities, minimum dwell times, and per-state multipliers for speed, error rate, thinking pauses, and correction rate.

### 1.2 Uniform Random Timing Distribution [VERIFIED — spec §5.1]

The original project likely uses uniform or Gaussian random delays. Real inter-key intervals follow a right-skewed distribution (skewness ≈ 1.98 per Dhakal et al.).

**Clack's approach:** Log-normal sampling with mean-preserving parameterization, per PMC8606350 establishing log-normal as the best fit for human keystroke timing.

### 1.3 No Error Simulation [INFERRED]

The original project passes input through without generating typing errors. Real typing includes substitution, insertion, omission, transposition, and doubling errors at empirically observed rates.

**Clack's approach:** Five error types with published frequency distribution (Dhakal et al., Clarkson 2005, CHI 2025), three correction modes (immediate, delayed, uncorrected), and two backspace strategies (character-by-character vs. held).

### 1.4 No Session Dynamics [INFERRED]

The original project treats every character identically regardless of position in the input. Real typing exhibits warmup (slower at start) and fatigue (slower and more error-prone toward end).

**Clack's approach:** Warmup curve (30% slower during first 10% of session) and fatigue curve (up to 25% slower and 50% more errors during final 40%), plus fatigue lapse events (sudden long pauses) modeled per PMC6049040 and PMC7537853.

### 1.5 No Keyboard Geometry Model [INFERRED]

The original project likely applies the same timing regardless of which keys are involved. Real typing is faster for hand-alternating bigrams and slower for same-finger repetitions and distant key pairs.

**Clack's approach:** Full QWERTY coordinate grid with Euclidean distance, Fitts's Law-derived distance-to-modifier mapping, hand alternation effect (0.82×), same-finger penalty (1.30×), and shift-key penalty.

### 1.6 No Momentum Smoothing [INFERRED]

The original project likely samples each delay independently, producing jerky timing that jumps randomly between fast and slow. Real typing has inertia — speed changes are gradual.

**Clack's approach:** Exponential smoothing with momentum factor 0.35, blending each new IKI with the previous delivered IKI.

### 1.7 No Library/Driver Split [INFERRED]

The original project is a single C file (or a few files) where timing logic is interleaved with I/O. This prevents reuse in a GUI or as a library.

**Clack's approach:** Strict separation. `clack-core` is a pure library with zero I/O. `clack-cli` is a thin driver. The split is enforced by the Cargo workspace boundary.

---

## 2. MVP Scope Compressions

These are aspects of typing simulation that are simplified or omitted in the MVP. Each is a deliberate decision, not an oversight.

### 2.1 Hold Time Not Modeled Separately from IKI [VERIFIED — spec §2.2]

Real keystrokes have both flight time (time between releasing one key and pressing the next) and hold/dwell time (how long a key is held down). The spec notes: "HType uses the IKI value directly as the sleep between character emissions and does not model hold time separately in the MVP."

**Impact:** The simulation is slightly less realistic in the timing of fast consecutive keystrokes. The dwell time data (mean 76.9 ms, SD 22.2 ms from Dhakal et al.) is available for a future refinement.

**Future path:** `next_key_delay = IKI - hold_time_of_previous_key` (spec §2.2 formula). Add a `HoldTimeModel` to `timing.rs`.

### 2.2 Code-Awareness Deferred [VERIFIED — spec §1.2]

The MVP does not slow down for identifiers, bracket matching, or code structure. All input is treated as natural language text.

**Impact:** Typing code through Clack will look slightly more uniform than a real programmer typing the same code. Bracket depth parsing and identifier detection are significant implementation scope.

**Future path:** Add a `CodeAwareModel` to `language.rs` in Phase 4 (v2.1).

### 2.3 Trigraph Timing Tables Deferred [VERIFIED — spec §1.2]

The MVP models digraph-level timing (two consecutive characters) via the keyboard distance model but does not use trigraph (three-character) tables.

**Impact:** Some three-character sequences that are particularly fast or slow to type (e.g., `the`, `ing`) are not modeled at the trigraph level. The common-word acceleration partially compensates for this.

**Future path:** Add trigraph timing lookup table to `keyboard.rs` in Phase 4.

### 2.4 Shift Penalty Not Split by Context [VERIFIED — spec §16]

The shift-key penalty is applied uniformly to all shifted characters. In reality, sentence-initial capitals (where the typist expects to press Shift) may have a different penalty than mid-word capitals (e.g., `GitHub`, `iPhone`) which are more surprising.

**Impact:** Sentence-initial capitals are slightly over-penalized; mid-word capitals are slightly under-penalized.

**Future path:** Spec §16 notes this as a v2.1 refinement: "distinguishing sentence-initial capitals from mid-word capitals."

### 2.5 QWERTY Only [VERIFIED — spec §8.1]

The MVP hardcodes the QWERTY keyboard layout. Users with Dvorak, Colemak, or AZERTY layouts cannot get layout-accurate timing.

**Impact:** The keyboard distance model is inaccurate for non-QWERTY users. The timing will still be plausible but not layout-faithful.

**Future path:** Add layout configuration to `keyboard.rs`. The module already computes distance from a coordinate table — swapping the table for a different layout is straightforward.

### 2.6 No Panic Events [VERIFIED — spec §1.2]

Panic events (typing `gitgit` instead of `git`, or a stuck-key burst like `ppppproject`) are rare by definition. They add authenticity for very long sessions but their absence does not reduce overall believability.

**Future path:** Add as a rare stochastic event in the state machine in Phase 4.

### 2.7 No Digraph Timing from User-Supplied File [VERIFIED — spec §1.2]

Users cannot supply their own digraph timing tables from recorded typing data.

**Future path:** Add `--digraph-file` flag and a file parser in Phase 4.

### 2.8 No Profile Recording [VERIFIED — spec §1.2]

The MVP cannot record a user's actual typing patterns for later playback or model fitting.

**Future path:** Add `--record` flag that writes timing data to a file, and `--profile` flag that reads it, in Phase 4.

### 2.9 Thinking Pause Only at Word Boundaries [VERIFIED — spec §7.3]

Thinking pauses are evaluated only at word boundaries (space characters). A real typist might also pause mid-word when encountering a difficult sequence.

**Impact:** Mid-word hesitations are not modeled. The difficult-word slowdown (§11.2) partially compensates by increasing IKI for all characters within a difficult word.

**Future path:** Add a small probability of mid-word hesitation in the timing model, possibly triggered by the same difficult-word detection logic.

### 2.10 No External State Observation Beyond stderr [VERIFIED — spec §1.2]

The MVP's `--state-output` flag writes a simple text format to stderr. More sophisticated observation (named pipe, structured JSON, etc.) is deferred.

**Future path:** Add a machine-readable event stream to a named pipe or Unix socket in Phase 4.

---

## 3. Future API Stability Risks

These are aspects of the current `clack-core` library API that may need to evolve when the GUI is built. Each risk has a mitigation strategy.

### 3.1 Raw Byte Output vs. Semantic Events

**Risk:** The current `ClackEvent.bytes` field contains raw terminal bytes including `\x08\x20\x08` backspace sequences. A GUI needs semantic events like "delete character at position N" or "insert character 'x' at position N". The GUI would have to parse raw byte sequences to understand what happened.

**Mitigation (now):**
- Use `#[non_exhaustive]` on `ClackEvent` so fields can be added later.
- The library internally tracks the logical state (what the "correct" output string looks like). This state is available but not exposed in the MVP API.

**Mitigation (when GUI is built):**
- **Option A (preferred):** Add a post-processing layer in the GUI crate that parses `ClackEvent.bytes` into semantic events. This keeps `clack-core` unchanged.
- **Option B:** Add a new method `next_semantic_event() -> Option<SemanticEvent>` to `ClackEngine` that returns higher-level events. This is a non-breaking addition to the API.

**Assessment:** Low risk. Option A is straightforward and does not require any library changes.

### 3.2 Delay Interpretation

**Risk:** `ClackEvent.delay_ms` is currently interpreted by the CLI as "sleep this many milliseconds before writing." A GUI might need sub-millisecond precision or might want to animate based on cumulative time rather than per-event delays.

**Mitigation (now):**
- `delay_ms` is `u64` which provides millisecond precision — sufficient for both CLI and GUI.
- The GUI can accumulate delays and use them as animation keyframes.

**Mitigation (when GUI is built):**
- If sub-millisecond precision is needed, change `delay_ms` to `delay_us: u64` (microseconds) or `delay: Duration`. This is a breaking change but can be managed with a major version bump.

**Assessment:** Very low risk. Millisecond precision is likely sufficient for GUI animation.

### 3.3 Feed/Pull vs. Push API

**Risk:** The current API is pull-based: the driver calls `feed()` then `next_event()`. A GUI event loop might prefer a callback/push-based API where the engine calls a closure for each event.

**Mitigation (now):**
- The pull-based API is the most flexible primitive. A push-based wrapper is trivially built on top:
  ```rust
  fn run_with_callback(engine: &mut ClackEngine, input: &[u8], callback: impl FnMut(ClackEvent)) {
      engine.feed(input);
      while let Some(event) = engine.next_event() {
          callback(event);
      }
  }
  ```

**Assessment:** No risk. Pull-based is strictly more flexible than push-based.

### 3.4 Config Extensibility

**Risk:** `ClackConfig` may need new fields for GUI-specific options (e.g., animation speed, visual error highlighting mode).

**Mitigation (now):**
- `ClackConfig` is marked `#[non_exhaustive]`, so adding fields is a non-breaking change.
- Implement `Default` for `ClackConfig` so new fields have sensible defaults.

**Assessment:** No risk. The `#[non_exhaustive]` pattern handles this cleanly.

### 3.5 State Machine Observability

**Risk:** The GUI may want to observe the current behavioral state at any time, not just on transitions. The current API only reports state changes via `ClackEvent.state_transition`.

**Mitigation (now):**
- Add a `pub fn current_state(&self) -> BehavioralState` method to `ClackEngine`. This is a non-breaking addition. [PROPOSED]

**Assessment:** Low risk. Adding a getter method is trivial and non-breaking.

### 3.6 Thread Safety

**Risk:** A GUI might want to run the engine on a background thread and send events to the UI thread. `ClackEngine` currently has no `Send` or `Sync` guarantees.

**Mitigation (now):**
- `ClackEngine` contains only owned data (no `Rc`, no raw pointers, no thread-local state). It should automatically implement `Send` in Rust. Verify this during Phase 1.

**Assessment:** Very low risk. Rust's ownership model makes `Send` automatic for types with only owned data.


---

# DECISIONS.md

# DECISIONS.md — Architectural Decision Log

> Every decision that shapes the project is recorded here with full reasoning.
> Future contributors and AI agents must read this before proposing changes.
> Each entry uses confidence markers: **[VERIFIED]** (from MVP spec), **[INFERRED]** (from evidence), **[PROPOSED]** (recommendation).

---

## DECISION-001: Language and Toolchain Selection

**Status:** NEW
**Confidence:** [PROPOSED]

**Context:**
Clack is a greenfield implementation of a human typing simulator. Three languages were evaluated: C, Rust, and Go. The decision must optimize for: (1) clean library/driver split, (2) log-normal sampling without exotic dependencies, (3) single distributable binary, (4) no timing artifacts at 120 WPM (IKI as low as 60 ms), and (5) solo-developer maintainability.

**Decision:** Rust, using Cargo workspaces.

**Evaluation:**

| Criterion | C | Rust | Go |
|---|---|---|---|
| Library/driver split | Makefile discipline + `.a` archive. No language-enforced boundary. | Cargo workspace: `clack-core` (lib) + `clack-cli` (bin). Compiler-enforced crate boundary. | Go packages. Workable but less idiomatic for this split. |
| Log-normal sampling | No stdlib support. Requires manual Box-Muller + `exp()`. Must embed a PRNG (stdlib `rand()` is poor). | `rand_distr::LogNormal` is production-grade and deterministic with `rand_chacha::ChaCha8Rng`. | `gonum/stat/distuv` has LogNormal. Adds a dependency but workable. |
| Single binary | Yes, trivially. ~50-100 KB. | Yes, via `cargo build --release`. ~2-4 MB (acceptable for CLI). | Yes, natively. ~5-8 MB. |
| Timing precision at 60 ms IKI | `nanosleep` is a direct syscall. Best possible. | `std::thread::sleep(Duration)` uses `nanosleep` on Linux. Equivalent to C. | `time.Sleep` is precise but GC pauses introduce non-deterministic jitter. At 60 ms IKI, a 1 ms GC spike is ~1.7% timing error. Not catastrophic but undermines `--seed` timing guarantees. |
| Solo maintainability | Manual memory management for lookahead buffer, tokenizer state, correction engine. High bug surface. | Ownership model eliminates memory bugs. Enums with data are ideal for the 5-state machine and 5 error types. Built-in testing via `cargo test`. | Simple language, but concurrency strengths are irrelevant here. Not the idiomatic home for Unix pipe tools. |

**Consequences:**
- All source files use `.rs` extension.
- Build system is Cargo (no Makefile needed).
- Project structure uses Cargo workspace with two member crates.
- External crates: `rand`, `rand_chacha`, `rand_distr`, `clap`.
- Binary size is ~2-4 MB (vs ~100 KB for C). Acceptable tradeoff.
- Compile times are longer than C but the project is small (~3-5K lines).

**Alternatives considered:**
- **C:** Rejected. The library/driver split requires manual discipline with no compiler enforcement. The RNG and log-normal distribution must be implemented from scratch. Memory management for the delayed correction lookahead buffer and tokenizer state is high-risk for a solo developer.
- **Go:** Rejected. GC pauses introduce non-deterministic timing jitter that conflicts with the `--seed` byte-identical output guarantee. Go's concurrency model provides no benefit for a single-threaded stream processor. The language is not the natural home for this kind of Unix pipe tool.

---

## DECISION-002: Library/Driver Architectural Split

**Status:** NEW
**Confidence:** [VERIFIED]

**Context:**
A GUI is planned for post-MVP. The simulation engine must be reusable across both CLI and GUI drivers. The user prompt explicitly states: "Any design that couples simulation logic to stdin, stdout, terminal I/O, or any specific output medium is architecturally wrong."

**Decision:** The project is a Cargo workspace with two crates:

- **`clack-core`** — A pure library crate containing all simulation logic. Zero I/O. No dependency on `std::io`, `std::fs`, or any terminal library. It accepts input bytes and returns output events (delay + bytes to emit). It never sleeps, never reads, never writes.

- **`clack-cli`** — A thin binary crate that depends on `clack-core`. It handles: argument parsing (`clap`), reading from stdin, calling `clack-core` for each input, sleeping for computed delays, writing output to stdout, and emitting state transitions to stderr.

**Consequences:**
- The library crate's `Cargo.toml` must not depend on `clap`, `std::io::stdin`, or any I/O facility.
- The CLI crate's `main.rs` must contain zero simulation logic — no IKI computation, no error generation, no state transitions.
- A future GUI crate (e.g., `clack-gui`) can be added as a third workspace member with zero changes to `clack-core`.
- Testing the simulation engine does not require stdin/stdout mocking — it is tested as a pure function.

**Alternatives considered:**
- **Single crate with feature flags:** Rejected. Feature flags do not enforce architectural boundaries at compile time. A developer could easily add I/O calls inside `#[cfg(feature = "cli")]` blocks within the library.
- **Single binary with internal module split:** Rejected. No compiler-enforced boundary. The "library" modules could access stdout freely.

---

## DECISION-003: IKI Timing Distribution (Log-Normal)

**Status:** INHERITED-FROM-SPEC
**Confidence:** [VERIFIED]

**Context:**
The MVP spec §5.1 mandates log-normal sampling for inter-key intervals. The spec cites PMC8606350 establishing that human IKI distributions are log-normal, not Gaussian.

**Decision:** Use `rand_distr::LogNormal` with mean-preserving parameterization:

```
sigma = jitter × 0.4
mu    = ln(IKI_target) - (sigma² / 2)
IKI_raw = exp(normal_sample(mu, sigma))
IKI_raw = max(IKI_raw, 60)   // hard floor per spec §5.1
```

This ensures `E[IKI_raw] = IKI_target` while producing right-skewed samples (skewness ≈ 1.98 at default jitter). [VERIFIED]

**Consequences:**
- Requires `rand_distr` crate (provides `LogNormal` distribution).
- The `--jitter` flag controls the sigma parameter, not the standard deviation directly.
- At `--jitter 0.0`, sigma = 0, and all IKI samples equal `IKI_target` exactly.
- The hard floor of 60 ms is applied after sampling, not before.

**Alternatives considered:**
- **Gaussian distribution:** Rejected per spec. Real IKI data is right-skewed, not symmetric.
- **Ex-Gaussian distribution:** Viable per the cited research but more complex to parameterize. Log-normal is spec-mandated.

---

## DECISION-004: RNG Strategy

**Status:** INHERITED-FROM-SPEC
**Confidence:** [VERIFIED]

**Context:**
The spec §4 requires: "All components share a single RNG instance seeded by `--seed`. The RNG state is threaded through every random decision so that a fixed seed produces byte-identical output across runs."

**Decision:** Use `rand_chacha::ChaCha8Rng` as the single RNG instance. It is:
- Deterministic given the same seed (cross-platform reproducible).
- Cryptographically derived (ChaCha family) but ChaCha8 is the fast variant.
- The `rand` crate's `SeedableRng` trait provides `ChaCha8Rng::seed_from_u64(seed)`.

When `--seed` is not provided, seed from system entropy via `ChaCha8Rng::from_entropy()`.

The RNG instance is owned by `ClackEngine` and passed by mutable reference to every subsystem that needs randomness. No subsystem creates its own RNG.

**Consequences:**
- `rand_chacha` is a required dependency of `clack-core`.
- Every random decision (state transition, pause duration, error generation, correction strategy, burst trigger) draws from the same RNG in a fixed order.
- Changing the order of random draws is a breaking change for deterministic output. Tests must catch this.
- Cross-platform determinism is guaranteed by ChaCha (unlike platform-dependent PRNGs).

**Alternatives considered:**
- **`StdRng`:** This is an alias for ChaCha12Rng in current `rand` versions. Using ChaCha8Rng directly is faster and equally deterministic.
- **`Pcg64`:** Faster than ChaCha but not cryptographic. Since we don't need crypto properties and ChaCha8 is fast enough, either works. ChaCha8 chosen for its widespread use in Rust deterministic testing.
- **`thread_rng()`:** Rejected. Not seedable, not deterministic.

---

## DECISION-005: Backspace Encoding

**Status:** INHERITED-FROM-SPEC
**Confidence:** [VERIFIED]

**Context:**
Spec §13.2: "Backspace erases are encoded as the three-byte sequence: `\x08 \x20 \x08` (backspace, space, backspace). This is compatible with ANSI terminals and most terminal emulators."

**Decision:** All backspace operations in the correction engine emit the literal byte sequence `[0x08, 0x20, 0x08]` per erased character. The `0x20` (space) in this sequence is flagged as an erase-space and must not trigger word-boundary pause logic. [VERIFIED]

**Consequences:**
- The correction module produces `Vec<u8>` output containing these raw bytes.
- The space byte `0x20` within a backspace sequence must be distinguishable from a real space character. The library marks it internally so the pause system skips it.
- Output is compatible with dumb terminals and raw pipes.

**Alternatives considered:**
- **ANSI escape `\x1b[1D`:** Rejected per spec §13.2. ANSI is not guaranteed in dumb terminals or raw pipes.
- **Single `\x08` byte:** Insufficient. On most terminals, backspace alone moves the cursor but does not erase the character visually.

---

## DECISION-006: No ANSI Escape Sequences in Output

**Status:** INHERITED-FROM-SPEC
**Confidence:** [VERIFIED]

**Context:**
Spec §13.2: "ANSI escape sequences (e.g., `\x1b[1D`) are not used in the MVP because clack must work in dumb terminals and raw pipes where ANSI is not guaranteed."

**Decision:** The output stream contains only:
- Printable characters (the typed text)
- `\x08 \x20 \x08` sequences (backspace-erase)
- `\n` (newlines, passed through from input)
- `\t` (tabs, passed through from input)

No ANSI escape codes, no color codes, no cursor movement sequences. [VERIFIED]

**Consequences:**
- Output can be piped to any program without ANSI-aware parsing.
- The `--state-output` flag writes to stderr, not stdout, and is the only place where structured text appears.
- Future GUI drivers render corrections differently (not via byte sequences), which is fine because the library returns semantic events, not terminal bytes.

**Alternatives considered:**
- None. Spec is explicit.

---

## DECISION-007: QWERTY-Only Keyboard Layout in MVP

**Status:** INHERITED-FROM-SPEC
**Confidence:** [VERIFIED]

**Context:**
Spec §8.1: "The MVP supports QWERTY only." The layout is defined as a coordinate grid with staggered row offsets.

**Decision:** The keyboard model in `keyboard.rs` hardcodes the QWERTY layout as a static lookup table mapping characters to `(row, col)` coordinates with row-specific stagger offsets. Hand assignment is hardcoded per spec §8.4. [VERIFIED]

The layout data structure:
```rust
struct KeyPosition {
    row: u8,
    col: u8,
    hand: Hand,       // Left or Right
    finger: Finger,   // Index, Middle, Ring, Pinky
}
```

**Consequences:**
- No layout configuration file or `--layout` flag in MVP.
- Adding Dvorak, Colemak, or AZERTY in post-MVP requires adding new layout tables but no architectural change — the `keyboard.rs` module already computes distance and hand assignment from a layout table.
- Unknown characters (emoji, non-ASCII) receive baseline modifiers per spec §8.7.

**Alternatives considered:**
- **Layout from file:** Deferred to post-MVP. Adds a file format and parser for no MVP benefit.
- **Multiple hardcoded layouts with a flag:** Deferred. QWERTY covers the vast majority of users.

---

## DECISION-008: Build and Distribution Target

**Status:** NEW
**Confidence:** [PROPOSED]

**Context:**
The tool must produce a single distributable binary with no runtime dependencies.

**Decision:** The primary distribution artifact is a statically linked Linux binary produced by:

```bash
cargo build --release --target x86_64-unknown-linux-gnu
```

The binary is named `clack`. For maximum portability, consider building with `RUSTFLAGS="-C target-feature=+crt-static"` for full static linking (no glibc dependency). [PROPOSED]

Cross-compilation for macOS and Windows is deferred to post-MVP but is supported by Cargo's cross-compilation infrastructure.

**Consequences:**
- No installer needed. Users download and run the binary.
- The binary includes `clap`'s help/version output.
- Man page generation is a Phase 3 deliverable (can use `clap_mangen` crate).
- CI builds the release binary and runs tests on every push.

**Alternatives considered:**
- **Distribution via `cargo install`:** Viable as a secondary channel but the primary artifact is a prebuilt binary.
- **Homebrew formula:** Deferred to post-MVP.
- **Dynamic linking:** Rejected. Introduces runtime dependency on specific glibc versions.

---

## DECISION-009: Test Strategy

**Status:** NEW
**Confidence:** [PROPOSED]

**Context:**
The spec requires `--seed` to produce byte-identical output across runs. This is the foundation of the test strategy.

**Decision:** Three-tier testing approach:

1. **Unit tests** (inline `#[cfg(test)]` in each module):
   - Test individual functions: IKI computation, distance calculation, state transition, error generation.
   - Use a fixed-seed RNG for deterministic assertions.

2. **Library integration tests** (`clack-core/tests/`):
   - Feed known input to `ClackEngine` with a fixed seed.
   - Assert the exact sequence of `ClackEvent` outputs (delay values and byte sequences).
   - Cover all §14 edge cases.

3. **CLI end-to-end tests** (`clack-cli/tests/`):
   - Run the `clack` binary with `--seed` and fixed stdin input.
   - Capture stdout and compare byte-for-byte against expected output snapshots.
   - Use the `assert_cmd` crate for process invocation.

Snapshot files are committed to the repository. Any change that alters deterministic output for a given seed is a breaking change and must update the snapshots explicitly.

**Consequences:**
- Random draw order is part of the contract. Reordering draws breaks tests.
- New features that add random draws must be inserted at the end of the per-character draw sequence, or existing snapshots must be regenerated.
- CI runs all three test tiers on every push.

**Alternatives considered:**
- **Property-based testing (proptest):** Valuable but complementary, not primary. The deterministic snapshot approach is simpler and directly validates the `--seed` contract.
- **Statistical testing (run N seeds, check distribution):** Useful for validation but not for regression. Added as a Phase 3 hardening task.

---

## DECISION-010: GUI Deferral and Library API Stability Contract

**Status:** NEW
**Confidence:** [PROPOSED]

**Context:**
A GUI is planned for post-MVP. The library API must be stable enough that the GUI can be built without changing `clack-core`.

**Decision:** The `clack-core` public API is:

```rust
// Configuration
pub struct ClackConfig { /* all CLI-mappable fields */ }

// Output event
pub struct ClackEvent {
    pub delay_ms: u64,         // How long to wait before emitting
    pub bytes: Vec<u8>,        // What to emit (may include backspace sequences)
    pub state_transition: Option<StateTransition>,  // For --state-output
}

// Engine
pub struct ClackEngine { /* opaque internal state */ }

impl ClackEngine {
    pub fn new(config: ClackConfig) -> Self;
    pub fn feed(&mut self, input: &[u8]);          // Feed input bytes
    pub fn next_event(&mut self) -> Option<ClackEvent>;  // Pull next output event
    pub fn finish(&mut self);                       // Signal EOF, flush pending corrections
}
```

This API is I/O-agnostic. The CLI interprets `bytes` as raw stdout output and `delay_ms` as `thread::sleep` duration. A GUI could interpret `bytes` as display text and `delay_ms` as animation timing.

**API stability contract:**
- `ClackConfig` fields may be added (non-breaking with `#[non_exhaustive]`).
- `ClackEvent` fields may be added (non-breaking with `#[non_exhaustive]`).
- `ClackEngine` method signatures must not change without a major version bump.
- The `bytes` field in `ClackEvent` contains raw terminal bytes (including `\x08\x20\x08` for backspace). A future GUI may need semantic events (e.g., "delete character at position N") instead. This is a known API evolution point — see TECH_DEBT.md.

**Consequences:**
- Use `#[non_exhaustive]` on `ClackConfig`, `ClackEvent`, and `BehavioralState` to allow additive changes.
- The GUI will need to parse `\x08\x20\x08` sequences from `bytes` to understand corrections semantically. This is acceptable for MVP; a richer event type is a post-MVP enhancement.

**Alternatives considered:**
- **Semantic event type (Insert, Delete, MoveCursor):** More GUI-friendly but over-engineers the MVP. The CLI needs raw bytes; adding semantic events now doubles the API surface with no MVP consumer.
- **Callback-based API:** Rejected. Pull-based (iterator-style) is simpler, more testable, and more idiomatic in Rust.

---

## DECISION-011: External Crate Selection

**Status:** NEW
**Confidence:** [PROPOSED]

**Context:**
Rust's ecosystem provides high-quality crates for RNG, distributions, and CLI parsing. The selection must be minimal and stable.

**Decision:**

| Crate | Version | Used by | Purpose |
|---|---|---|---|
| `rand` | `0.8` | `clack-core` | Core RNG traits (`Rng`, `SeedableRng`) |
| `rand_chacha` | `0.3` | `clack-core` | `ChaCha8Rng` deterministic PRNG |
| `rand_distr` | `0.4` | `clack-core` | `LogNormal`, `Uniform` distributions |
| `clap` | `4` | `clack-cli` | CLI argument parsing with derive macros |
| `clap_mangen` | `0.2` | `clack-cli` (build) | Man page generation (Phase 3) |
| `assert_cmd` | `2` | `clack-cli` (dev) | End-to-end CLI testing |

**Consequences:**
- `clack-core` has exactly 3 dependencies (rand ecosystem). No I/O crates.
- `clack-cli` adds `clap` for argument parsing. This is the only CLI-specific dependency.
- All crates are from the `rust-lang` or widely-trusted ecosystem maintainers.

**Alternatives considered:**
- **`structopt`:** Superseded by `clap` 4's derive macros.
- **`argh`:** Lighter than clap but less featureful. Clap is the Rust ecosystem standard.
- **No external RNG (hand-roll PRNG):** Rejected. `rand_chacha` is battle-tested and provides the exact determinism guarantee needed.

---

## DECISION-012: Error Handling Strategy

**Status:** NEW
**Confidence:** [PROPOSED]

**Context:**
Clack is a stream processor. Errors are either configuration errors (caught at startup) or I/O errors (stdin read failure).

**Decision:**
- `clack-core` is infallible after construction. `ClackEngine::new()` validates config and returns `Result<ClackEngine, ClackError>`. After successful construction, `feed()` and `next_event()` never fail — they are pure computation on in-memory data.
- `clack-cli` handles I/O errors. Stdin read errors exit with code 2. Invalid options exit with code 1. Both per spec §3.4. [VERIFIED]

**Consequences:**
- The library API is simple: no `Result` returns on hot-path methods.
- Config validation happens once at startup, not per-character.
- The CLI driver wraps stdin reads in error handling and maps to exit codes.

**Alternatives considered:**
- **Result on every method:** Rejected. The engine is a pure state machine operating on in-memory data. There is no failure mode after initialization.

---

## DECISION-013: UTF-8 Handling

**Status:** INHERITED-FROM-SPEC
**Confidence:** [VERIFIED]

**Context:**
Spec §3.3: "Stdin must be text (UTF-8). Non-UTF-8 bytes are passed through unchanged with no delay." Spec §14: "Non-ASCII UTF-8 character: Emitted with baseline IKI and no keyboard model; no error generated on it."

**Decision:** The engine processes input byte-by-byte. ASCII bytes (0x00–0x7F) are handled with full simulation (keyboard model, errors, etc.). Non-ASCII bytes (0x80–0xFF, which are continuation bytes or leading bytes of multi-byte UTF-8 sequences) are passed through with baseline IKI, no keyboard modifier, no error generation. [VERIFIED]

**Consequences:**
- The engine does not need a full UTF-8 decoder. It only needs to distinguish ASCII from non-ASCII bytes.
- Emoji and accented characters pass through with baseline timing.
- The keyboard model only maps ASCII characters to positions.
- This approach is correct for the MVP and can be refined in post-MVP to handle multi-byte characters as single units.

**Alternatives considered:**
- **Full UTF-8 decoding:** Over-engineered for MVP. The spec explicitly treats non-ASCII as passthrough.
- **Rejecting non-UTF-8:** Rejected per spec. Non-UTF-8 bytes are passed through, not rejected.

