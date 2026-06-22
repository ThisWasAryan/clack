# Clack

Clack is an extremely realistic typing simulator that translates static text or standard input into a dynamic, human-like keystroke stream. 

*Clack is heavily inspired by and credits its core idea to **htype** (https://github.com/lord-of-the-strings/htype), originally created by **Aadity Setu**.* 

If you are generating screencasts, terminal recordings, or automated UI demonstrations, regular typing automation tools like `xdotool` or basic sleep scripts look robotic and fake. Clack utilizes probabilistic models to simulate how a real human types, complete with motor-control panics, realistic typing speeds, error rates, and layout-aware key travel distances.

---

## 🌟 How it Works

Clack doesn't just add a random delay between keypresses. It simulates an entire virtual typist sitting at a keyboard. It employs several advanced mechanics to achieve this:

- **Bimodal Timing Distrubutions**: Human typing has two distinct rhythms. We type adjacent letters in a word quickly, but pause longer at word boundaries, punctuation, and capital letters. Clack models this using Gaussian distributions.
- **Euclidean Key Distances**: Clack understands the physical layout of your keyboard (QWERTY, Dvorak, Colemak, AZERTY). Moving your finger from 'a' to 'z' is faster than moving from 'a' to 'p'. Clack dynamically calculates the physical travel distance between consecutive keys to adjust the inter-key timing.
- **Probabilistic Errors & Corrections**: Humans make mistakes! Clack can simulate typos (hitting an adjacent key), transpositions (swapping two letters), and omissions. It then realistically realizes the mistake, pauses, backspaces, and re-types the correct characters.
- **Panic Events**: Sometimes, a typist's brain glitches. Clack simulates rare "Panic Events" where the virtual typist accidentally holds down a key for too long (a stuck-key burst) or gets confused and re-types the prefix of the current word before backspacing to fix it.
- **Code-Aware Typing Mode**: Typing source code requires more cognitive load than typing prose. When enabled, Clack tracks the depth of nested brackets `{} [] ()` and slows down the typing speed for deeper scopes. It also detects complex identifiers (`snake_case`, `camelCase`) and types them more carefully to reflect real programming behavior.

---

## 🚀 Installation

Clack is built in Rust for maximum performance and cross-platform compatibility.

To install from source, ensure you have Rust and Cargo installed, then run:

```bash
git clone https://github.com/ThisWasAryan/clack.git
cd clack
cargo install --path clack-cli
```

---

## 💻 Usage

Clack accepts input from standard input (stdin) and outputs the simulated keystroke timing stream to standard output (stdout) or directly to another program.

### Basic Usage

Pipe a file or string into `clack`:

```bash
echo "Hello, world! I am a simulated human." | clack
```

```bash
cat my_script.sh | clack
```

### Configuration Flags

You can customize the simulated typist's behavior using CLI arguments:

- `--wpm <f64>`: Set the target Words Per Minute (default: `80.0`).
- `--error-rate <f64>`: Probability of making a mistake per character (default: `0.03`, i.e., 3%).
- `--correction-rate <f64>`: Probability of correcting a mistake (default: `1.0`, i.e., 100%).
- `--jitter <f64>`: Variance in the typing speed (default: `0.1`). Higher is more erratic.
- `--layout <string>`: The physical keyboard layout to simulate. Options: `qwerty` (default), `dvorak`, `colemak`, `azerty`.
- `--code-mode`: Enable scope-depth tracking and identifier slowdowns for realistic code typing.

### Examples

**Simulate a very fast, erratic typist who makes lots of mistakes:**
```bash
echo "I'm typing really fast but making so many typos!" | clack --wpm 140 --jitter 0.3 --error-rate 0.1
```

**Simulate a slow, careful programmer typing Dvorak:**
```bash
cat main.rs | clack --wpm 45 --layout dvorak --code-mode --error-rate 0.01
```

---

## 🏗️ Technical Architecture

Clack is split into two components to enforce a strict boundary between the simulation logic and the command-line interface.

- **`clack-core`**: The engine library. It maintains a deterministic internal event queue, processes characters via probabilistic distributions (`rand`), manages layout coordinate mapping, and handles the correction state machine. It guarantees reproducibility given the same seed.
- **`clack-cli`**: The user-facing binary. It parses command-line arguments (using `clap`), initializes the `ClackEngine`, and streams the generated `ClackEvent` timings to stdout or coordinates real-time delays.

Clack ensures *O(1)* per-character processing overhead, meaning it can generate timing events infinitely without memory scaling issues.
