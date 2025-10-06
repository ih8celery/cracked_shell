# Cracked Shell Architecture

## Overview

Cracked Shell is a Unix shell that replaces traditional shell syntax with Lisp
s-expressions, enabling powerful composition of Unix commands through Lisp's
functional programming features while maintaining competitive performance.

## Language Evaluation & Decision

### Requirements Analysis

A shell implementation requires:

1. **Memory Safety**: Shells handle untrusted input (user commands, file paths,
   environment variables). Memory vulnerabilities can lead to arbitrary code
   execution.
2. **Performance**: Must match or exceed bash performance for common workflows
   (pipelines, file processing, loops).
3. **Concurrency**: Modern shells benefit from parallel command execution and
   async I/O.
4. **Systems Programming**: Direct access to POSIX APIs (fork, exec, pipe,
   signals).
5. **Ecosystem**: Libraries for parsing, REPL, async runtime, testing.

### C++ vs Rust Comparison

#### Memory Safety

**C++**:
- Manual memory management via RAII, smart pointers
- Still susceptible to use-after-free, double-free, buffer overflows
- Requires careful code review and sanitizers (AddressSanitizer, UBSan)
- Existing prototype (~100 lines) has no memory bugs yet, but doesn't handle
  complex scenarios

**Rust**:
- Ownership system prevents use-after-free, data races at compile time
- Borrow checker enforces lifetime safety
- Unsafe code isolated to FFI boundaries
- Shell security critical: parsing untrusted input, executing commands with user
  privileges

**Winner: Rust** - Memory safety is non-negotiable for a shell. The ownership
system eliminates entire bug classes that plague C++ shells.

#### Performance

**C++**:
- Mature optimizers (GCC, Clang) with decades of tuning
- Zero-cost abstractions via templates
- Minimal runtime overhead
- Proven track record in systems programming

**Rust**:
- LLVM-based compiler (same backend as Clang)
- Zero-cost abstractions via traits and monomorphization
- No garbage collection overhead
- Benchmarks show Rust matching or exceeding C++ in systems programming tasks
- Example: ripgrep (Rust) faster than grep (C)

**Winner: Tie** - Both offer excellent performance. Rust's LLVM backend and
zero-cost abstractions match C++.

#### Concurrency

**C++**:
- std::thread, std::async for parallelism
- Manual synchronization (mutexes, condition variables)
- Data races possible despite threading primitives
- C++20 coroutines exist but library support immature

**Rust**:
- Ownership prevents data races at compile time ("fearless concurrency")
- Async/await with mature ecosystems (tokio, async-std)
- Rayon for data parallelism
- Channels (mpsc) and Arc/Mutex for shared state
- Type system guarantees Send/Sync safety

**Winner: Rust** - Compile-time data race prevention is a game-changer for
concurrent shell operations (parallel pipelines, background jobs).

#### Systems Programming

**C++**:
- Direct C API access (fork, exec, pipe, signal)
- Extensive POSIX bindings
- Mature libraries (Boost, POCO)

**Rust**:
- libc crate provides raw POSIX bindings
- nix crate offers safe Rust wrappers for Unix APIs
- std::process for command execution
- tokio for async process management
- Growing ecosystem for systems programming

**Winner: C++** (historically) but **Rust closing fast** - Rust's nix and
tokio::process provide excellent POSIX support. The gap is minimal.

#### Ecosystem

**C++**:
- No standard build system (CMake, Make, Bazel fragmentation)
- No standard package manager (conan, vcpkg, manual dependency management)
- Parsing: Boost.Spirit, PEGTL (header-only)
- REPL: readline (C library), custom solutions
- Testing: Google Test, Catch2

**Rust**:
- Cargo: standard build system and package manager
- crates.io: centralized package repository
- Parsing: nom (parser combinators), pest (PEG), lalrpop (LR)
- REPL: rustyline (pure Rust readline alternative)
- Testing: built-in test framework, proptest for property testing
- Async: tokio, async-std (mature runtimes)

**Winner: Rust** - Cargo's unified tooling and crates.io dramatically reduce
dependency friction. The ecosystem has matured rapidly.

#### Learning Curve & Developer Productivity

**C++**:
- Team familiarity assumed (based on existing prototype)
- Mature tooling (IDEs, debuggers, profilers)
- Large developer base and resources
- Complexity: templates, SFINAE, manual memory management

**Rust**:
- Initial learning curve for ownership/borrowing
- Excellent compiler error messages guide learning
- cargo doc, clippy, rustfmt provide quality tooling
- Strong community and documentation (The Rust Book, Rust by Example)
- Productivity gains after initial investment: fewer bugs, faster refactoring

**Winner: C++ (short-term), Rust (long-term)** - Rust's learning curve is
offset by long-term productivity from fewer bugs and better tooling.

### Decision: Rust

**Rationale**:

1. **Security**: Memory safety is paramount for a shell handling untrusted
   input. Rust's ownership system prevents entire bug classes (use-after-free,
   data races).
2. **Concurrency**: Fearless concurrency enables parallel command execution and
   async I/O without data race risks.
3. **Ecosystem**: Cargo and crates.io provide excellent dependency management
   and mature libraries (tokio, rustyline, nom).
4. **Performance**: LLVM-based optimizer matches C++; zero-cost abstractions
   preserve performance.
5. **Low Migration Cost**: Existing C++ prototype is minimal (~100 lines). Core
   abstractions (ShellData, ShellEnv) translate cleanly to Rust enums and
   structs.

**Alternatives Considered**: Continuing C++ would preserve existing code but
sacrifice memory safety and modern concurrency. The prototype's simplicity makes
rewrite cost negligible compared to long-term benefits.

**Trade-offs Accepted**:
- Learning curve for team members unfamiliar with Rust
- Smaller pool of Rust developers vs C++ (mitigated by excellent docs and
  community)

### Migration Notes from C++ Prototype

The existing C++ code (`lib/include/cracked_shell.h`, `lib/src/cracked_shell.cpp`)
provides valuable design insights:

**Preserved Concepts**:

1. **ShellData**: Type-tagged union for runtime values
   - C++: `void*` with `ShellDataType` enum
   - Rust: `enum Value { String(String), Int(i64), Float(f64), List(Vec<Value>),
     Function(...), ... }`
   - Rust's tagged unions are safer and more ergonomic

2. **ShellEnv**: Environment with variable storage and stack
   - C++: `std::unordered_map<const char*, ShellData*>` + stack
   - Rust: `HashMap<String, Rc<Value>>` + `Vec<Rc<Value>>` stack
   - Rust's ownership eliminates lifetime issues

3. **CrackedShellApp**: REPL loop
   - C++: Manual `read_line()`, `tokenize_line()`, `parse_tokens()`, `execute_shell_program()`
   - Rust: `rustyline` for readline, `nom` for parsing, custom evaluator
   - Rust ecosystem provides battle-tested components

**Design Flaws Corrected**:

1. **C++ Memory Management**:
   - Problem: `ShellData` stores raw `void*`, unclear ownership
   - Solution: Rust `Rc<Value>` or `Arc<Value>` for shared ownership

2. **C++ Error Handling**:
   - Problem: No error handling visible (functions return `void` or `int`)
   - Solution: Rust `Result<T, E>` for explicit error propagation

3. **C++ Type Safety**:
   - Problem: `to_string()`, `to_integer()` may fail at runtime
   - Solution: Rust `match` on `Value` enum ensures exhaustive handling

**Lessons Learned**:

- Stack-based evaluation works well for Lisp (REPL pushes/pops values)
- Separate AST and runtime value types may be beneficial (C++ conflates them)
- REPL loop needs robust error handling to avoid crashes on parse/eval errors

## High-Level Architecture

```
┌─────────────────────────────────────────────────────────┐
│                         REPL                            │
│         (rustyline: readline, history, completion)      │
└────────────────────┬────────────────────────────────────┘
                     │ String
                     ▼
┌─────────────────────────────────────────────────────────┐
│                   Lexer/Parser                          │
│        (nom: tokenize, parse s-expressions)             │
└────────────────────┬────────────────────────────────────┘
                     │ Expr (AST)
                     ▼
┌─────────────────────────────────────────────────────────┐
│                 Macro Expander                          │
│      (expand defmacro, quasiquote before eval)          │
└────────────────────┬────────────────────────────────────┘
                     │ Expr (expanded)
                     ▼
┌─────────────────────────────────────────────────────────┐
│                    Evaluator                            │
│  - Symbol resolution (environment lookup)               │
│  - Function application (user/builtin)                  │
│  - Special form dispatch (if, let, lambda, etc.)        │
└────────────────────┬────────────────────────────────────┘
                     │ Value
          ┌──────────┴──────────┐
          ▼                     ▼
┌─────────────────┐   ┌─────────────────────┐
│  Builtins       │   │  Process Manager    │
│  (cd, export,   │   │  (tokio::process,   │
│   alias, etc.)  │   │   pipes, signals)   │
└─────────────────┘   └──────────┬──────────┘
                                 │ Stream<T>
                                 ▼
                      ┌──────────────────────┐
                      │  Stream Abstraction  │
                      │  (async iterators,   │
                      │   map/filter/reduce) │
                      └──────────────────────┘
```

## Core Subsystems

### 1. REPL (Read-Eval-Print Loop)

**Responsibilities**:
- Display prompt (customizable via environment variable)
- Read user input with history and completion
- Handle multi-line input (incomplete s-expressions)
- Catch and display errors without crashing
- Manage job control (Ctrl+C, Ctrl+Z)

**Dependencies**:
- `rustyline` crate for readline functionality
- `colored` or `termcolor` for syntax highlighting

**Interface**:
```rust
pub struct Repl {
    env: Environment,
    history_path: PathBuf,
}

impl Repl {
    pub fn new(env: Environment) -> Self;
    pub fn run(&mut self) -> Result<()>;
    fn read_line(&mut self) -> Result<String>;
    fn eval_print(&mut self, input: &str) -> Result<()>;
}
```

### 2. Lexer/Parser

**Responsibilities**:
- Tokenize input into atoms, parens, quotes
- Parse tokens into AST (s-expressions)
- Handle syntax errors with helpful messages
- Support reader macros (e.g., `'x` → `(quote x)`)

**Dependencies**:
- `nom` parser combinator library

**Data Types**:
```rust
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    LParen,
    RParen,
    Symbol(String),
    String(String),
    Integer(i64),
    Float(f64),
    Quote,
    Quasiquote,
    Unquote,
    UnquoteSplicing,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Symbol(String),
    String(String),
    Integer(i64),
    Float(f64),
    List(Vec<Expr>),
    Quote(Box<Expr>),
    Quasiquote(Box<Expr>),
    Unquote(Box<Expr>),
}
```

**Interface**:
```rust
pub fn tokenize(input: &str) -> Result<Vec<Token>>;
pub fn parse(tokens: &[Token]) -> Result<Expr>;
pub fn parse_str(input: &str) -> Result<Expr> {
    let tokens = tokenize(input)?;
    parse(&tokens)
}
```

### 3. Environment & Symbol Resolution

**Responsibilities**:
- Store variable bindings (global and lexical scopes)
- Lookup symbols with parent scope chain
- Support `define`, `let` bindings
- Handle undefined symbol errors

**Data Types**:
```rust
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Environment {
    bindings: HashMap<String, Rc<Value>>,
    parent: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Self;
    pub fn with_parent(parent: Rc<RefCell<Environment>>) -> Self;
    pub fn define(&mut self, name: String, value: Rc<Value>);
    pub fn lookup(&self, name: &str) -> Result<Rc<Value>>;
    pub fn set(&mut self, name: &str, value: Rc<Value>) -> Result<()>;
}
```

### 4. Evaluator

**Responsibilities**:
- Evaluate AST nodes to values
- Dispatch special forms (`if`, `define`, `lambda`, `let`, `quote`)
- Apply functions (builtin and user-defined)
- Tail call optimization for recursion
- Error handling with stack traces

**Data Types**:
```rust
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    String(String),
    Integer(i64),
    Float(f64),
    Bool(bool),
    List(Vec<Rc<Value>>),
    Symbol(String),  // Unevaluated symbol (for quote)
    Builtin(BuiltinFn),
    Lambda {
        params: Vec<String>,
        body: Expr,
        env: Rc<RefCell<Environment>>,
    },
    Stream(Stream),
    Process(Process),
}

pub type BuiltinFn = fn(&[Rc<Value>], &mut Environment) -> Result<Rc<Value>>;
```

**Interface**:
```rust
pub fn eval(expr: &Expr, env: &mut Environment) -> Result<Rc<Value>>;

// Special forms
fn eval_if(args: &[Expr], env: &mut Environment) -> Result<Rc<Value>>;
fn eval_define(args: &[Expr], env: &mut Environment) -> Result<Rc<Value>>;
fn eval_lambda(args: &[Expr], env: &mut Environment) -> Result<Rc<Value>>;
fn eval_let(args: &[Expr], env: &mut Environment) -> Result<Rc<Value>>;
fn eval_quote(args: &[Expr]) -> Result<Rc<Value>>;

// Function application
fn apply(func: Rc<Value>, args: &[Rc<Value>], env: &mut Environment) -> Result<Rc<Value>>;
```

### 5. Macro Expander

**Responsibilities**:
- Expand `defmacro` definitions before evaluation
- Handle quasiquote/unquote for code generation
- Prevent variable capture (hygiene, if feasible)

**Interface**:
```rust
pub fn expand_macros(expr: &Expr, env: &Environment) -> Result<Expr>;
fn expand_quasiquote(expr: &Expr, env: &Environment) -> Result<Expr>;
```

### 6. Process Manager

**Responsibilities**:
- Spawn Unix commands via fork/exec
- Set up pipes between processes
- Capture stdout/stderr as streams
- Handle exit codes and signals
- Manage background jobs

**Dependencies**:
- `tokio::process` for async process management
- `nix` crate for low-level POSIX APIs (signals, pipes)

**Data Types**:
```rust
use tokio::process::Child;
use std::process::Stdio;

pub struct Process {
    child: Child,
    stdout: Option<Stream>,
    stderr: Option<Stream>,
}

impl Process {
    pub async fn spawn(cmd: &str, args: &[String]) -> Result<Self>;
    pub async fn wait(self) -> Result<i32>;  // Exit code
    pub fn stdout(&mut self) -> Option<Stream>;
    pub fn stderr(&mut self) -> Option<Stream>;
}
```

**Interface**:
```rust
pub async fn execute_command(cmd: &str, args: &[String]) -> Result<Process>;
pub async fn pipe_commands(cmds: Vec<Command>) -> Result<Process>;
```

### 7. Stream Abstraction

**Responsibilities**:
- Lazy iteration over command output
- Support line-based and byte-based reading
- Compose with map/filter/reduce
- Handle backpressure and errors

**Data Types**:
```rust
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::ChildStdout;

pub struct Stream {
    // Internal: BufReader<ChildStdout> or similar
}

impl Stream {
    pub async fn lines(self) -> impl futures::Stream<Item = Result<String>>;
    pub async fn bytes(self) -> impl futures::Stream<Item = Result<u8>>;
    pub fn map<F>(self, f: F) -> MappedStream<F>;
    pub fn filter<F>(self, f: F) -> FilteredStream<F>;
    pub async fn collect(self) -> Result<Vec<String>>;
}
```

### 8. Builtins

**Responsibilities**:
- Implement shell utilities (`cd`, `pwd`, `export`, `alias`)
- Arithmetic and comparison operators
- List operations (`car`, `cdr`, `cons`, `append`)
- I/O operations (`print`, `read`)

**Interface**:
```rust
pub fn register_builtins(env: &mut Environment);

// Example builtins
fn builtin_add(args: &[Rc<Value>], _env: &mut Environment) -> Result<Rc<Value>>;
fn builtin_cd(args: &[Rc<Value>], _env: &mut Environment) -> Result<Rc<Value>>;
fn builtin_pipe(args: &[Rc<Value>], env: &mut Environment) -> Result<Rc<Value>>;
```

## Data Flow

### Typical Command Execution

1. User enters: `(pipe (ls "-la") (grep "foo"))`
2. **REPL** reads input via rustyline
3. **Lexer** tokenizes: `[LParen, Symbol("pipe"), LParen, Symbol("ls"), String("-la"), RParen, ...]`
4. **Parser** builds AST: `List([Symbol("pipe"), List([Symbol("ls"), String("-la")]), ...])`
5. **Macro Expander** checks for macros (none in this case)
6. **Evaluator** recognizes `pipe` as builtin, evaluates args:
   - `(ls "-la")` → evaluates to `Process` with stdout stream
   - `(grep "foo")` → evaluates to `Process` connected to previous stdout
7. **Process Manager** sets up pipe(2) between processes
8. **Stream** yields lines from final process stdout
9. **REPL** prints stream to terminal

### Macro Expansion Example

1. User defines: `(defmacro when (cond . body) (list 'if cond (cons 'begin body) 'nil))`
2. User enters: `(when (> x 0) (print "positive") (print "done"))`
3. **Macro Expander** recognizes `when` as macro, expands:
   - Input: `(when (> x 0) (print "positive") (print "done"))`
   - Output: `(if (> x 0) (begin (print "positive") (print "done")) nil)`
4. **Evaluator** processes expanded `if` form

## Error Handling Strategy

### Principles

1. **Fail Fast**: Parse and type errors caught early
2. **Informative Messages**: Include source location, context, suggestions
3. **Recoverability**: REPL continues after errors
4. **Stack Traces**: Show call chain for debugging

### Error Types

```rust
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Parse error at {location}: {message}")]
    ParseError { location: SourceLocation, message: String },

    #[error("Undefined symbol: {0}")]
    UndefinedSymbol(String),

    #[error("Type error: expected {expected}, got {actual}")]
    TypeError { expected: String, actual: String },

    #[error("Arity error: {func} expects {expected} args, got {actual}")]
    ArityError { func: String, expected: usize, actual: usize },

    #[error("Process error: {0}")]
    ProcessError(#[from] std::io::Error),

    #[error("Runtime error: {0}")]
    RuntimeError(String),
}

pub type Result<T> = std::result::Result<T, Error>;
```

### Error Recovery

- **REPL**: Catch all errors, print, reset to prompt
- **Scripts**: Propagate errors to caller, optionally continue with error handler
- **Pipelines**: Failed command sets exit code, optionally stops pipeline

## Concurrency Model

### Async Runtime

Use **tokio** for:
- Async process spawning (`tokio::process`)
- Async I/O (reading streams)
- Parallel command execution
- Background job management

### Ownership & Sharing

- **Rc/RefCell** for single-threaded shared state (Environment, Values)
- **Arc/Mutex** if multi-threaded parallelism needed (future work)
- **Channels** (tokio::sync::mpsc) for inter-task communication

### Parallel Execution

```rust
// Example: (parallel cmd1 cmd2 cmd3)
async fn builtin_parallel(args: &[Rc<Value>], env: &mut Environment) -> Result<Rc<Value>> {
    let tasks: Vec<_> = args.iter().map(|arg| {
        tokio::spawn(eval_async(arg.clone(), env.clone()))
    }).collect();

    let results = futures::future::join_all(tasks).await;
    Ok(Rc::new(Value::List(results?)))
}
```

## Performance Considerations

### Lazy Evaluation

- **Streams**: Don't buffer entire command output; read line-by-line
- **Pipelines**: Start all processes immediately; data flows as produced

### Zero-Cost Abstractions

- **Macros**: Expand at parse time, no runtime overhead
- **Inlining**: Small functions (arithmetic, comparisons) inlined by LLVM
- **Monomorphization**: Generic stream combinators specialized per type

### Builtins vs External Commands

- **Builtins** (`cd`, `export`): Native code, no fork/exec overhead
- **External** (`ls`, `grep`): Fork/exec required
- **Optimization**: Detect common external commands, implement as builtins if
  performance-critical

### Memory Management

- **Rc** for value sharing: cheap clones (ref count increment)
- **Avoid allocations**: Reuse buffers, iterators over collect
- **Profiling**: Use `cargo flamegraph` to identify hot paths

## Testing Strategy

### Unit Tests

- **Lexer**: Token sequences for edge cases (nested strings, escapes)
- **Parser**: AST correctness, error messages
- **Evaluator**: Special forms, function application, errors
- **Builtins**: Arithmetic, comparisons, list ops
- **Process**: Mock commands, pipe setup
- **Stream**: Lazy iteration, combinators

### Integration Tests

- **Shell Scripts**: End-to-end execution of `.lisp` files
- **Pipelines**: Multi-stage commands, exit codes
- **REPL**: Interactive session simulation
- **Regression**: Known bugs remain fixed

### Property Testing

- **Parser**: Random s-expressions round-trip (parse → print → parse)
- **Evaluator**: Arithmetic properties (commutativity, associativity)

### Benchmarks

- **Criterion**: Microbenchmarks for hot paths (eval, parse)
- **Hyperfine**: End-to-end script comparison vs bash/zsh

## Security Considerations

### Threat Model

- **Untrusted Input**: User-provided commands, file paths, environment variables
- **Privilege**: Shell runs with user's privileges, must not escalate
- **Injection**: Prevent command injection via careful quoting/escaping

### Mitigations

1. **Command Execution**: Always use exec with argv array, never shell
   interpolation
2. **Path Handling**: Canonicalize paths, validate before use
3. **Environment**: Sanitize environment variables before passing to child
   processes
4. **Resource Limits**: Configurable limits on processes, memory (future work)
5. **Fuzzing**: AFL or libFuzzer on parser and evaluator

## Build System

### Cargo Configuration

**Cargo.toml**:
```toml
[package]
name = "cracked_shell"
version = "0.1.0"
edition = "2021"
authors = ["Adam Marshall"]

[dependencies]
tokio = { version = "1", features = ["full"] }
rustyline = "13"
nom = "7"
nix = { version = "0.27", features = ["process", "signal"] }
thiserror = "1"
colored = "2"

[dev-dependencies]
criterion = "0.5"
proptest = "1"

[[bin]]
name = "cracked"
path = "src/main.rs"

[[bench]]
name = "eval_benchmark"
harness = false
```

### Directory Structure

```
cracked_shell/
├── Cargo.toml
├── Cargo.lock
├── README.md
├── LICENSE
├── project-plan.md
├── docs/
│   ├── architecture.md      # This file
│   ├── lisp-reference.md    # Language spec (Task 1.3)
│   ├── user-guide.md        # User documentation (Phase 6)
│   └── bash-migration.md    # Migration guide (Phase 6)
├── src/
│   ├── main.rs              # CLI entry point
│   ├── lib.rs               # Library exports
│   ├── repl.rs              # REPL implementation
│   ├── lexer.rs             # Tokenization
│   ├── parser.rs            # S-expression parsing
│   ├── eval.rs              # Evaluator
│   ├── env.rs               # Environment/scope
│   ├── macro_expand.rs      # Macro expansion
│   ├── builtin.rs           # Built-in functions
│   ├── process.rs           # Unix process management
│   ├── stream.rs            # Lazy I/O streams
│   ├── value.rs             # Runtime value types
│   └── error.rs             # Error types
├── tests/
│   ├── integration/
│   │   ├── test_repl.rs
│   │   ├── test_pipelines.rs
│   │   └── scripts/         # .lisp test files
│   └── unit/
│       ├── test_lexer.rs
│       ├── test_parser.rs
│       └── test_eval.rs
├── benches/
│   └── eval_benchmark.rs
├── examples/
│   └── sample_scripts/
│       ├── hello.lisp
│       ├── pipeline.lisp
│       └── fibonacci.lisp
└── lib/                     # Archived C++ code
    └── archive/
        ├── include/
        └── src/
```

## CI/CD Pipeline

### GitHub Actions Workflow

```yaml
name: CI

on: [push, pull_request]

jobs:
  test:
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest]
        rust: [stable, nightly]
    steps:
      - uses: actions/checkout@v3
      - uses: dtolnay/rust-toolchain@master
        with:
          toolchain: ${{ matrix.rust }}
      - run: cargo build --verbose
      - run: cargo test --verbose
      - run: cargo clippy -- -D warnings
      - run: cargo fmt -- --check

  benchmark:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo bench
```

## Future Considerations

### Phase 2+ Enhancements

- **Reader Macros**: Custom syntax extensions (e.g., `#(1 2 3)` for arrays)
- **Module System**: Import/export for organizing large scripts
- **Foreign Function Interface**: Call C libraries from Lisp
- **Static Analysis**: Type checker, linter for shell scripts
- **Debugger**: Step-through debugging for Lisp code
- **Package Manager**: Install third-party Lisp libraries

### Ecosystem Integration

- **Shell Plugins**: LSP server for editor integration (autocomplete, jump-to-def)
- **Script Repository**: Curated collection of useful Lisp shell utilities
- **Cross-Platform**: Windows support via tokio (limited POSIX emulation)

## Revision History

- 2025-10-05: Initial architecture document created (Task 1.1 & 1.2)
