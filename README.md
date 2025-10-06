# Cracked Shell

A Unix shell with Lisp syntax that combines the composability of Lisp with the
performance of native Unix command execution.

## Vision

Bash provides poor abstraction for composing programs. Cracked Shell uses Lisp
s-expressions to provide:

- **Uniform Syntax**: Code and data share representation, enabling powerful macros
- **Composability**: First-class functions and higher-order combinators over command streams
- **Zero-Cost Abstractions**: Compile-time macro expansion eliminates runtime overhead
- **Performance**: Match or exceed bash with lazy evaluation and direct process management

## Status

🚧 **Early Development** - Phase 1 in progress (Foundation & Language Decision)

See [project-plan.md](project-plan.md) for the complete roadmap.

## Quick Start

### Prerequisites

Install Rust via [rustup](https://rustup.rs/):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### Build

```bash
cargo build
```

### Run

```bash
cargo run
```

### Test

```bash
cargo test
```

### Lint

```bash
cargo clippy
```

### Format

```bash
cargo fmt
```

## Documentation

- [Architecture](docs/architecture.md) - Design and technology choices
- [Lisp Reference](docs/lisp-reference.md) - Language specification
- [Project Plan](project-plan.md) - Development roadmap

## Example Syntax (Planned)

```lisp
; Simple command
(ls "-la")

; Pipeline
(pipe (ls "-la") (grep "txt"))

; Stream processing
(map string-upcase (lines (cat "file.txt")))

; Function definition
(define (count-lines file)
  (length (lines (cat file))))

; Macro definition
(defmacro when (cond . body)
  `(if ,cond (begin ,@body) #f))
```

## Project Structure

```
cracked_shell/
├── Cargo.toml           # Rust project manifest
├── README.md            # This file
├── project-plan.md      # Development roadmap
├── docs/                # Documentation
│   ├── architecture.md  # Design and architecture
│   └── lisp-reference.md # Language specification
├── src/                 # Source code
│   ├── main.rs          # CLI entry point
│   └── lib.rs           # Library exports
├── tests/               # Tests
│   ├── integration/     # End-to-end tests
│   └── unit/            # Unit tests
├── benches/             # Benchmarks
├── examples/            # Example scripts
└── lib/                 # Archived C++ prototype
```

## Development

### Running Tests

```bash
# All tests
cargo test

# Specific test
cargo test test_name

# With output
cargo test -- --nocapture
```

### Benchmarking

```bash
cargo bench
```

### Documentation

```bash
# Generate and open docs
cargo doc --open
```

## Contributing

Contributions welcome! Please see:

1. [Project Plan](project-plan.md) for current priorities
2. [Architecture](docs/architecture.md) for design guidelines
3. Run `cargo fmt` and `cargo clippy` before submitting

## License

Dual-licensed under MIT OR Apache-2.0

## Acknowledgments

- Inspired by Lisp's elegance and Unix's composability
- Built with Rust for memory safety and performance
