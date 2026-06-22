# Clack (htype)

Clack is a highly realistic human typing simulator. It acts as a standard UNIX pipe, reading text from `stdin` and emitting it to `stdout` with human-like delays, stochastic typing errors, corrections, and fatigue modeling.

This project is a Rust port and expansion of the original `htype` concept, designed with an architecture that cleanly separates the simulation engine (`clack-core`) from the CLI driver (`clack-cli`).

## Installation

### From Source using Cargo
If you have Rust installed, you can build and install Clack directly from source:
```bash
cargo install --path clack-cli
```
This will install the `clack` binary to your `~/.cargo/bin` directory.

### Pre-built Binaries
You can download pre-built binaries for your platform from the [Releases](#) page. Simply extract the binary and place it in your `$PATH`.

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

## Documentation

For a detailed understanding of how Clack works under the hood, refer to the [ARCHITECTURE.md](ARCHITECTURE.md) document.

If you're interested in contributing to the project, please read [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines on code style, testing, and pull requests.

## Attribution
This project is based on the original `htype` typing simulator specification. It implements the exact probabilistic and cognitive models outlined in the original research to simulate realistic keyboard usage patterns.
