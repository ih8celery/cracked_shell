# Archived C++ Prototype

This directory documents the original C++ prototype of Cracked Shell. The
implementation has been superseded by the Rust version in the root `src/`
directory.

## Note on Archived Files

The original C++ source files (`include/cracked_shell.h` and
`src/cracked_shell.cpp`) existed in the initial commit but have been removed
from the working tree as part of the migration to Rust. They can be viewed in
the git history at commit `12be03c6` (initial commit).

The files are not preserved in this directory because:
1. They were minimal prototypes (~150 lines total)
2. The design is fully documented below
3. Git history provides permanent reference

## Key Design Elements Preserved

### ShellData Type System

The C++ prototype used a type-tagged union approach that informed the Rust
`Value` enum:

**C++ Approach**:
```cpp
enum class ShellDataType {
  STRING, ARRAY, HASH, NUM, INT, FUNCTION
};

class ShellData {
  private:
    ShellDataType data_type;
    void * data;  // Type-unsafe pointer
};
```

**Rust Translation**:
```rust
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    String(String),
    Integer(i64),
    Float(f64),
    List(Vec<Rc<Value>>),
    // ... etc
}
```

The Rust version eliminates the unsafe `void*` pointer and provides type safety
through tagged unions with compile-time exhaustiveness checking.

### Environment & Variable Scoping

The C++ prototype used a hash map for variable storage with a separate stack:

**C++ Approach**:
```cpp
class ShellEnv {
  private:
    std::unordered_map<const char*, ShellData*> vars;
    ShellData** stack;
    int stackSize;
};
```

**Rust Translation**:
```rust
pub struct Environment {
    bindings: HashMap<String, Rc<Value>>,
    parent: Option<Rc<RefCell<Environment>>>,
}
```

The Rust version uses lexical scope chaining (parent pointers) rather than a
global variable map, enabling proper closure support.

### REPL Loop Structure

The C++ prototype sketched the REPL flow:

**C++ Approach**:
```cpp
int CrackedShellApp::run() {
    do {
        std::cout << prompt_str;
        line = read_line();
        args = tokenize_line(line);
        tree = parse_tokens(args);
        status = execute_shell_program(tree);
    } while (status);
}
```

**Rust Translation** (planned):
```rust
pub fn run(&mut self) -> Result<()> {
    loop {
        let line = self.read_line()?;
        let expr = parse_str(&line)?;
        let value = eval(&expr, &mut self.env)?;
        println!("{}", value);
    }
}
```

The Rust version uses `Result` for explicit error handling rather than integer
status codes.

## Lessons Learned

### Issues in C++ Prototype

1. **Memory Management**: Raw pointers (`void*`, `ShellData*`) required manual
   lifetime tracking. No clear ownership model.

2. **Error Handling**: Functions return `void` or `int` with no indication of
   failure modes. Errors would likely propagate as crashes.

3. **Incomplete Implementation**: Functions like `read_line()`, `tokenize_line()`,
   `parse_tokens()`, `execute_shell_program()` were declared but not defined.

4. **Type Safety**: `void*` storage in `ShellData` allows type confusion bugs.
   Conversion functions (`to_string()`, `to_integer()`) have no error handling.

5. **Const Correctness**: `const char*` keys in hash map but unclear ownership
   (string literals? allocated?).

### Improvements in Rust Version

1. **Ownership**: `Rc<Value>` provides clear shared ownership with automatic
   cleanup.

2. **Error Handling**: `Result<T, Error>` types make errors explicit and
   recoverable.

3. **Type Safety**: Enums eliminate type confusion; pattern matching ensures
   exhaustive handling.

4. **Concurrency**: Ownership rules prevent data races; async support built-in.

5. **Ecosystem**: Cargo, crates.io, and mature libraries (nom, rustyline, tokio)
   accelerate development.

## Migration Notes for Future Reference

If any C++ design patterns are needed, refer to this archive. However, the Rust
version is the canonical implementation going forward.

### Performance Comparison (Future Work)

Once the Rust implementation is complete, benchmark against a theoretical C++
version to validate performance claims in `docs/architecture.md`.

## Archival Date

2025-10-05 - Archived during Phase 1 (Task 1.5)
