# Clack

Clack is an extremely realistic typing simulator that translates static text or standard input into a dynamic, human-like keystroke stream. 

*Clack is heavily inspired by and credits its core idea to **htype** (https://github.com/lord-of-the-strings/htype), originally created by **Aadity Setu**.* 

If you are generating screencasts, terminal recordings, or automated UI demonstrations, regular typing automation tools like `xdotool` or basic sleep scripts look robotic and fake. Clack utilizes probabilistic models to simulate how a real human types, complete with motor-control panics, realistic typing speeds, error rates, and layout-aware key travel distances.




https://github.com/user-attachments/assets/282e4487-f12c-4f81-9dcd-22b81fb3f5a9

---



## ✨ New in V2: Graphical User Interface!

Clack 2.0 introduces a brand new, fully-featured **GTK4 Graphical Interface** for Linux! You no longer need to rely purely on the command line to configure your virtual typist.

- **Live Tuning:** Adjust WPM, Error Rates, and Correction probability via sliders and instantly see the results.
- **Typing Presets:** Switch between pre-configured typing styles like *Casual*, *Hunt & Peck*, *Proficient*, and *Fast* on the fly.
- **No Leftover Mistakes:** A highly-requested toggle that ensures your final text is 100% correct, while still authentically simulating the process of making and fixing mistakes as it types.
- **Global Shortcuts (WIP):** Pause, resume, or trigger speed-boosts globally using keyboard shortcuts without needing to focus the application window.
- **Cross-Window Compatibility:** Type directly into any target application window using secure `/dev/uinput` injection.


<img width="1070" height="553" alt="ClackGUI1 (Edit)" src="https://github.com/user-attachments/assets/ef53276b-cd4d-4d2e-a660-6824cc682bb5" />

---

## 🌟 Features

Clack doesn't just add a random delay between keypresses. It simulates an entire virtual typist sitting at a keyboard. It employs several advanced mechanics to achieve this:

- **Bimodal Timing Distributions**: Human typing has two distinct rhythms. We type adjacent letters in a word quickly, but pause longer at word boundaries, punctuation, and capital letters. Clack models this using Gaussian distributions.
- **Euclidean Key Distances**: Clack understands the physical layout of your keyboard (QWERTY, Dvorak, Colemak, AZERTY). Moving your finger from 'a' to 'z' is faster than moving from 'a' to 'p'. Clack dynamically calculates the physical travel distance between consecutive keys to adjust the inter-key timing.
- **Probabilistic Errors & Corrections**: Humans make mistakes! Clack can simulate typos (hitting an adjacent key), transpositions (swapping two letters), and omissions. It then realistically realizes the mistake, pauses, backspaces, and re-types the correct characters.
- **Panic Events**: Sometimes, a typist's brain glitches. Clack simulates rare "Panic Events" where the virtual typist accidentally holds down a key for too long (a stuck-key burst) or gets confused and re-types the prefix of the current word before backspacing to fix it.
- **Code-Aware Typing Mode**: Typing source code requires more cognitive load than typing prose. When enabled, Clack tracks the depth of nested brackets `{} [] ()` and slows down the typing speed for deeper scopes. It also detects complex identifiers (`snake_case`, `camelCase`) and types them more carefully to reflect real programming behavior.

---

## 🚀 Setup and Running

Clack is built in Rust for maximum performance, with a modern GTK4 Python frontend. 

### Option 1: Debian/Ubuntu Linux (.deb)
If you are using a Debian-based Linux distribution (Ubuntu, Mint, Pop!_OS, etc.), the easiest way to install Clack is using the `.deb` package. This installs the `clack` CLI tool and the `clack-gui` Desktop Application.

1. Download the `clack_v2_amd64.deb` file from the Releases page.
2. Install the package via terminal:
   ```bash
   sudo dpkg -i clack_v2_amd64.deb
   sudo apt-get install -f # To resolve any missing dependencies automatically
   ```
3. Open "Clack" from your desktop application launcher, or run `clack-gui` in the terminal!

### Option 2: Pre-compiled Tarball (.tar.gz)
If you are using Arch, Fedora, Alpine, or prefer portable executables, use the `.tar.gz` archive.

1. Download `clack-2.0.0-x86_64-linux-gnu.tar.gz`.
2. Extract the archive and ensure you have `ydotool` and `python3-gi` installed on your system.
3. Run the CLI directly or execute the `gui` python module.

### Option 3: Compile from Source

If you have Rust and Cargo installed, you can compile Clack natively:

```bash
git clone https://github.com/ThisWasAryan/clack.git
cd clack
cargo build --release
```

To run the GUI from source:
```bash
sudo apt install ydotool python3-gi python3-gi-cairo gir1.2-gtk-4.0
# Start the ydotool daemon with open socket permissions so your user can access it
sudo ydotoold --socket-perm 0666 &
python3 -m gui
```

---

## 💻 CLI Usage

If you prefer the command line, Clack still supports its powerful CLI backend. It accepts input from standard input (stdin) and outputs the simulated keystroke timing stream to standard output (stdout).

**Pipe a string into clack:**
```bash
echo "Hello, world! I am a simulated human." | clack
```

**Simulate a very fast, erratic typist who makes lots of mistakes:**
```bash
echo "I'm typing really fast but making so many typos!" | clack --wpm 140 --jitter 0.3 --error-rate 0.1
```

**Simulate a slow, careful programmer typing Dvorak:**
```bash
cat main.rs | clack --wpm 45 --layout dvorak --code-mode --error-rate 0.01
```

### Full CLI Options

| Flag | Type | Default | Description |
|---|---|---|---|
| `--wpm` | float | `60.0` | Target average words per minute |
| `--jitter` | float | `0.15` | IKI (Inter-Key Interval) jitter coefficient (0.0 = none, 1.0 = extreme) |
| `--error-rate` | float | `0.04` | Base probability of generating an error per character (0.0–1.0) |
| `--correction-rate` | float | `0.85` | Fraction of generated errors that get corrected |
| `--no-errors` | flag | off | Disable all error generation |
| `--seed` | int | (random) | RNG seed for deterministic output. When set, behavior is fully reproducible |
| `--session-length` | int | `500` | Expected total character count; used to compute warmup/fatigue curves |
| `--no-fatigue` | flag | off | Disable warmup and fatigue session modelling |
| `--max-pause` | int | `5000` | Maximum any single pause may be in milliseconds |
| `--thinking-pause-prob` | float | `0.015` | Probability of a stochastic thinking pause between any two words |
| `--state-output` | flag | off | Emit behavioral state transitions to stderr |
| `--code-mode` | flag | off | Slows down for nested brackets and complex identifiers |
| `--layout` | string | `qwerty` | Target keyboard layout (`qwerty`, `dvorak`, `colemak`, `azerty`) |
| `--version` | flag | — | Print version string and exit |

---

## 🔬 Empirical Foundation & Research

The default parameters and modeling techniques in Clack are derived from extensive academic research on human keystroke dynamics. Rather than relying on simple sleep delays, every value traces its origins back to research:

### 1. Inter-Key Interval (IKI) Distribution
The IKI distribution is **right-skewed**, not Gaussian. We model the delay between keypresses as a **log-normal distribution**, ensuring extreme realism. Fast typists average ~120 ms between keys, while slower typists average ~480 ms, with an absolute physical minimum (hard floor) of 60 ms.

### 2. Error Rates and Type Distribution
Errors in fast typing naturally occur at a rate of 4%–8%, broken down by precise human fallibility ratios:
- **Substitution (adjacent key):** ~39%
- **Insertion (extra character):** ~33%
- **Omission (missing character):** ~21%
- **Transposition (swap two adjacent):** ~5% (76% occurring across hands)
- **Doubling (letter doubled):** ~2%

### 3. Euclidean Distance & Fitts's Law
Clack computes the physical Euclidean distance between keys on the selected layout (e.g., QWERTY, Dvorak). Transitioning from `A` to `Z` is significantly faster than `A` to `P`. Distances map to specific IKI multipliers. Hand-alternating bigrams (e.g., left hand `E` → right hand `R`) are mathematically enforced to be up to 18% faster than same-hand sequences.

### 4. Cognitive Pauses & Fatigue
Pauses are separated into specific cognitive events:
- **Word Boundary Pauses:** Log-normal pause at the end of a word (~80 ms mean).
- **Sentence Boundary Pauses:** End-of-thought pauses after `.` `?` `!` (~600 ms mean).
- **Line-start Hesitation:** Brief re-orientation before starting a new line.
- **Session Fatigue:** Fatigue models gradual IKI multiplier increases and spikes in errors. Lapses ("mental blocks") with very long IKIs begin appearing late in typing sessions.

### Sources
- Dhakal et al., "Observations on Typing from 136 Million Keystrokes", CHI 2018. URL: https://userinterfaces.aalto.fi/136Mkeystrokes/
- Clarkson (2005) mini-QWERTY study; CHI 2025 "Simulating Errors in Touchscreen Typing" (arXiv 2502.03560).
- "On the shape of timings distributions in free-text keystroke dynamics profiles", PMC8606350.
- "Age Modulates the Effects of Mental Fatigue on Typewriting", Frontiers in Psychology 2018 (PMC6049040).
- "Dynamics in typewriting performance reflect mental fatigue during real-life office work", PLOS ONE 2020 (PMC7537853).

---

## 🏗️ Architecture & Simulation Flow

Clack ensures *O(1)* per-character processing overhead, meaning it can generate timing events infinitely without memory scaling issues. It is strictly deterministic when given a seed.

### System Data Flow
```text
stdin
  │
  ▼
[Reader]
  │  character stream
  ▼
[Tokenizer]  — splits into: regular chars, word boundaries, sentence boundaries, newlines
  │
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

### Correction Strategies
Clack implements multiple dynamic correction strategies.
- **Immediate Correction (~70%):** A typo is immediately recognized. A realistic backspace delay sequence is emitted (`\x08 \x20 \x08`) followed by the correct characters.
- **Delayed Correction (~30%):** The typist makes a mistake but continues typing 3-12 characters before "noticing" the error, pausing in confusion, erasing everything up to the mistake, and re-typing the rest of the word correctly.

---

## 📁 File Structure

The repository is highly modularized to separate core simulation logic from platform-dependent I/O operations and graphical user interfaces.

```text
clack/
├── Cargo.toml                  # Workspace manifest
├── clack-core/                 # Highly-optimized Rust Library — 0% I/O, 100% Simulation
│   └── src/
│       ├── lib.rs              # Public API: ClackEngine, ClackConfig, ClackEvent
│       ├── timing.rs           # Log-normal sampling, base IKI logic, bursts
│       ├── state.rs            # Behavioral state transitions and matrices
│       ├── keyboard.rs         # Layout grids, Euclidean distance maps, hand-alternation rules
│       ├── error.rs            # Probabilistic error generation (sub, add, omit, trans, double)
│       ├── pause.rs            # Sentence, word, and stochastic thinking pauses
│       ├── session.rs          # Session progress, warmup curve, fatigue curve, lapses
│       ├── language.rs         # Common word lists, difficult word heuristics
│       └── correction.rs       # Backspacing, immediate vs delayed correction state machines
├── clack-cli/                  # Thin Rust CLI driver
│   └── src/main.rs             # Stdin → Engine → Stdout stream plumbing (100% I/O)
└── gui/                        # GTK4 Python Frontend
    ├── __main__.py             # Entrypoint
    ├── app.py                  # Main GTK Application loop and style contexts
    ├── window.py               # Main window layout, controllers, and components
    ├── injector.py             # Secure uinput daemon / ydotoold wrapper for external injection
    ├── focus_watcher.py        # System-wide window focus tracking
    ├── clack_runner.py         # Subprocess interaction tying the Rust CLI to the GUI
    ├── settings.py             # Advanced settings panel UI
    └── presets.py              # Tuned presets (Hunt&Peck, Casual, Fast) and modifiers
```

---

<div align="center">
  <b>Made with ♥ by <a href="https://github.com/ThisWasAryan" style="text-decoration:none;">ThisWasAryan</a> | <a href="https://github.com/ThisWasAryan/clack" style="text-decoration:none;">Repository</a></b>
</div>
