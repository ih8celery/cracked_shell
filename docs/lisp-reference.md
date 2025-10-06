# Cracked Shell Lisp Reference

## Introduction

Cracked Shell implements a subset of Lisp designed specifically for shell
scripting. The language combines Lisp's powerful composition features with
practical shell integration, providing a cleaner alternative to bash's syntax
while maintaining high performance through Unix command execution.

This document specifies the grammar, data types, special forms, and shell
extensions that comprise the Cracked Shell Lisp dialect.

## Design Principles

1. **Simplicity**: Minimal core language, extensible via macros
2. **Shell Integration**: First-class Unix command execution and composition
3. **Performance**: Zero-cost abstractions via compile-time macro expansion
4. **Practicality**: Focus on common shell tasks, omit heavyweight Lisp features

## Grammar

### EBNF Notation

```ebnf
program     ::= expr*

expr        ::= atom
              | list
              | quoted
              | quasiquoted

atom        ::= symbol
              | string
              | number
              | boolean

symbol      ::= [a-zA-Z_+\-*/<>=!?][a-zA-Z0-9_+\-*/<>=!?]*

string      ::= '"' char* '"'
char        ::= [^"\\] | '\\' escape
escape      ::= 'n' | 't' | 'r' | '\\' | '"'

number      ::= integer | float
integer     ::= '-'? [0-9]+
float       ::= '-'? [0-9]+ '.' [0-9]+ ([eE] [+-]? [0-9]+)?

boolean     ::= '#t' | '#f'

list        ::= '(' expr* ')'
              | '(' expr+ '.' expr ')'  ; dotted pair

quoted      ::= "'" expr                  ; sugar for (quote expr)
quasiquoted ::= '`' expr                  ; sugar for (quasiquote expr)
              | ',' expr                  ; sugar for (unquote expr)
              | ',@' expr                 ; sugar for (unquote-splicing expr)

comment     ::= ';' [^\n]* '\n'
```

### Lexical Rules

- **Whitespace**: Spaces, tabs, newlines separate tokens but are otherwise ignored
- **Case Sensitivity**: Symbols are case-sensitive (`foo` ≠ `Foo`)
- **Comments**: `;` starts a line comment (ignored until newline)
- **Parentheses**: `(` and `)` delimit lists
- **String Escapes**: `\"`, `\\`, `\n`, `\t`, `\r`

### Examples

```lisp
; Atoms
42                ; integer
3.14              ; float
"hello"           ; string
foo               ; symbol
#t                ; boolean true
#f                ; boolean false

; Lists
()                ; empty list
(1 2 3)           ; list of integers
(+ 1 2)           ; function call
(define x 42)     ; special form

; Dotted pairs (cons cells)
(1 . 2)           ; pair
(1 2 . 3)         ; improper list

; Quoted forms
'x                ; (quote x)
'(1 2 3)          ; (quote (1 2 3))
`(a ,b c)         ; (quasiquote (a (unquote b) c))
```

## Data Types

### Primitive Types

#### Integer

- **Description**: Signed 64-bit integer
- **Range**: -9,223,372,036,854,775,808 to 9,223,372,036,854,775,807
- **Literals**: `42`, `-17`, `0`
- **Operations**: `+`, `-`, `*`, `/`, `%`, `<`, `>`, `=`, `<=`, `>=`

#### Float

- **Description**: IEEE 754 double-precision floating point
- **Literals**: `3.14`, `-0.5`, `1e6`, `2.5e-3`
- **Operations**: `+`, `-`, `*`, `/`, `<`, `>`, `=`, `<=`, `>=`

#### String

- **Description**: UTF-8 encoded text
- **Literals**: `"hello"`, `"world\n"`
- **Escapes**: `\"`, `\\`, `\n`, `\t`, `\r`
- **Operations**: `string-append`, `string-length`, `substring`

#### Boolean

- **Values**: `#t` (true), `#f` (false)
- **Falsy**: Only `#f` is false; everything else is truthy (including `0`, `""`, `()`)
- **Operations**: `and`, `or`, `not`

#### Symbol

- **Description**: Identifier or quoted name
- **Unevaluated**: Inside `quote`, symbols remain symbols
- **Evaluated**: In normal context, symbols look up variables
- **Examples**: `foo`, `+`, `my-var`

### Compound Types

#### List

- **Description**: Linked list of values
- **Constructor**: `(list 1 2 3)` or `'(1 2 3)`
- **Empty List**: `()` or `'()` (also represents "nil")
- **Operations**: `car`, `cdr`, `cons`, `append`, `length`, `reverse`

#### Pair (Cons Cell)

- **Description**: Two-element structure (dotted pair)
- **Constructor**: `(cons 1 2)` → `(1 . 2)`
- **Access**: `(car pair)` → first element, `(cdr pair)` → second element
- **List as Pairs**: `(1 2 3)` = `(1 . (2 . (3 . ())))`

### Shell-Specific Types

#### Stream

- **Description**: Lazy sequence of lines/bytes from command output
- **Constructor**: Returned by command execution
- **Operations**: `map`, `filter`, `reduce`, `take`, `drop`, `collect`
- **Example**: `(map string-upcase (lines "/etc/hosts"))`

#### Process

- **Description**: Handle to a running/completed Unix process
- **Constructor**: `(ls "-la")` spawns process
- **Operations**: `wait`, `status`, `stdout`, `stderr`, `success?`
- **Example**: `(status (ls "/nonexistent"))` → exit code

#### Function

- **Description**: First-class function (builtin or user-defined)
- **Constructor**: `(lambda (x) (* x x))` or builtin
- **Application**: `(func arg1 arg2 ...)`
- **Examples**: `+`, `map`, user-defined lambdas

## Special Forms

Special forms are syntactic constructs evaluated differently from normal
function calls. The evaluator recognizes them by name and applies custom logic.

### `quote` - Prevent Evaluation

**Syntax**: `(quote expr)` or `'expr`

**Description**: Returns `expr` without evaluating it.

**Examples**:
```lisp
(quote x)         ; → symbol x (not variable lookup)
'(1 2 3)          ; → list (1 2 3)
(quote (+ 1 2))   ; → list (+ 1 2), not 3
```

### `if` - Conditional Execution

**Syntax**: `(if condition then-expr else-expr)`

**Description**: Evaluates `condition`. If truthy, evaluates and returns
`then-expr`; otherwise, evaluates and returns `else-expr`.

**Examples**:
```lisp
(if (> x 0) "positive" "non-positive")
(if (file-exists? "foo.txt")
    (read-file "foo.txt")
    "")
```

**Notes**:
- `else-expr` is optional; defaults to `#f` if omitted
- Only the taken branch is evaluated (short-circuit)

### `define` - Variable Binding

**Syntax**: `(define name value)`

**Description**: Binds `name` to evaluated `value` in the current environment.

**Examples**:
```lisp
(define x 42)
(define greet (lambda (name) (string-append "Hello, " name)))
```

**Function Shorthand**:
```lisp
(define (square x) (* x x))
; Equivalent to: (define square (lambda (x) (* x x)))
```

### `lambda` - Anonymous Function

**Syntax**: `(lambda (param1 param2 ...) body)`

**Description**: Creates an anonymous function with parameters and body. The
body is evaluated when the function is called, with parameters bound to
arguments.

**Examples**:
```lisp
(lambda (x) (* x x))
((lambda (x y) (+ x y)) 3 4)  ; → 7
(map (lambda (x) (* x 2)) '(1 2 3))  ; → (2 4 6)
```

**Closure**: Lambdas capture their lexical environment.

```lisp
(define (make-adder n)
  (lambda (x) (+ x n)))
(define add5 (make-adder 5))
(add5 10)  ; → 15
```

### `let` - Local Binding

**Syntax**: `(let ((var1 val1) (var2 val2) ...) body)`

**Description**: Creates a new scope with local bindings. Evaluates `body` with
bindings in scope.

**Examples**:
```lisp
(let ((x 10) (y 20))
  (+ x y))  ; → 30

(let ((twice (lambda (x) (* x 2))))
  (twice 21))  ; → 42
```

**Sequential Binding** (`let*`):
```lisp
(let* ((x 10) (y (+ x 5)))
  y)  ; → 15
; Regular let would fail since x is not in scope for y's initializer
```

### `begin` - Sequential Execution

**Syntax**: `(begin expr1 expr2 ... exprN)`

**Description**: Evaluates expressions in order, returns the value of the last
expression. Used for side effects.

**Examples**:
```lisp
(begin
  (print "Starting...")
  (define x 42)
  (print "Done")
  x)  ; → 42 (and prints "Starting..." and "Done")
```

### `defmacro` - Macro Definition

**Syntax**: `(defmacro name (param1 param2 ...) body)`

**Description**: Defines a macro that transforms code at compile time. The body
receives unevaluated arguments and returns an expression to evaluate.

**Examples**:
```lisp
(defmacro when (cond . body)
  `(if ,cond (begin ,@body) #f))

(when (> x 0)
  (print "positive")
  (print "done"))
; Expands to: (if (> x 0) (begin (print "positive") (print "done")) #f)
```

### `quasiquote` - Template Substitution

**Syntax**: `` `expr`` (quasiquote), `,expr` (unquote), `,@expr`
(unquote-splicing)

**Description**: Like `quote`, but allows selective evaluation via `unquote`
(`,`) and list splicing via `unquote-splicing` (`,@`).

**Examples**:
```lisp
`(a b c)          ; → (a b c)
`(a ,b c)         ; b evaluated, others quoted
`(a ,@(list 1 2) c)  ; → (a 1 2 c)
```

**Macro Use**:
```lisp
(defmacro unless (cond . body)
  `(if (not ,cond) (begin ,@body) #f))
```

## Built-in Functions

### Arithmetic

```lisp
(+ a b ...)       ; Addition (variadic)
(- a b ...)       ; Subtraction
(* a b ...)       ; Multiplication
(/ a b ...)       ; Division (float if any operand is float)
(% a b)           ; Modulo (integer only)
```

**Examples**:
```lisp
(+ 1 2 3)         ; → 6
(- 10 3)          ; → 7
(* 2 3 4)         ; → 24
(/ 10 3)          ; → 3 (integer division)
(/ 10.0 3)        ; → 3.333... (float division)
(% 10 3)          ; → 1
```

### Comparison

```lisp
(= a b)           ; Equality
(< a b)           ; Less than
(> a b)           ; Greater than
(<= a b)          ; Less than or equal
(>= a b)          ; Greater than or equal
```

**Examples**:
```lisp
(= 42 42)         ; → #t
(< 5 10)          ; → #t
(> "b" "a")       ; → #t (string comparison)
```

### Boolean Logic

```lisp
(and a b ...)     ; Logical AND (short-circuit)
(or a b ...)      ; Logical OR (short-circuit)
(not a)           ; Logical NOT
```

**Examples**:
```lisp
(and #t #t)       ; → #t
(or #f #f)        ; → #f
(not #f)          ; → #t
(and (> x 0) (< x 10))  ; x in range (0, 10)
```

### List Operations

```lisp
(car lst)         ; First element (CAR = Contents of Address part of Register)
(cdr lst)         ; Rest of list (CDR = Contents of Decrement part of Register)
(cons a b)        ; Construct pair
(list a b ...)    ; Construct list
(append lst1 lst2 ...)  ; Concatenate lists
(length lst)      ; List length
(reverse lst)     ; Reverse list
(nth n lst)       ; Nth element (0-indexed)
```

**Examples**:
```lisp
(car '(1 2 3))    ; → 1
(cdr '(1 2 3))    ; → (2 3)
(cons 1 '(2 3))   ; → (1 2 3)
(list 1 2 3)      ; → (1 2 3)
(append '(1 2) '(3 4))  ; → (1 2 3 4)
(length '(a b c)) ; → 3
(reverse '(1 2 3)) ; → (3 2 1)
(nth 1 '(a b c))  ; → b
```

### String Operations

```lisp
(string-append s1 s2 ...)  ; Concatenate strings
(string-length s)          ; String length
(substring s start end)    ; Extract substring
(string-upcase s)          ; Convert to uppercase
(string-downcase s)        ; Convert to lowercase
(string-split s delim)     ; Split string into list
```

**Examples**:
```lisp
(string-append "hello" " " "world")  ; → "hello world"
(string-length "foo")                ; → 3
(substring "hello" 1 4)              ; → "ell"
(string-upcase "foo")                ; → "FOO"
(string-split "a,b,c" ",")           ; → ("a" "b" "c")
```

### Type Predicates

```lisp
(integer? x)
(float? x)
(string? x)
(boolean? x)
(symbol? x)
(list? x)
(pair? x)
(null? x)         ; True if empty list
(function? x)
(stream? x)
(process? x)
```

### I/O

```lisp
(print x)         ; Print value to stdout
(println x)       ; Print value with newline
(read-line)       ; Read line from stdin
(read-file path)  ; Read entire file as string
(write-file path content)  ; Write string to file
```

## Shell Extensions

### Command Execution

**Syntax**: `(command arg1 arg2 ...)`

**Description**: Any list whose first element is a symbol not bound to a
function/macro is treated as a Unix command. Arguments are passed as strings.

**Examples**:
```lisp
(ls "-la")                    ; Executes /bin/ls -la
(grep "foo" "file.txt")       ; Executes /bin/grep foo file.txt
(echo "hello" "world")        ; Executes /bin/echo hello world
```

**Return Value**: Returns a `Process` object with accessible stdout stream.

**Output Capture**:
```lisp
(define output (collect (stdout (ls "-la"))))
; output is a list of lines
```

### Pipe Composition

**Syntax**: `(pipe cmd1 cmd2 ... cmdN)` or `(| cmd1 cmd2 ... cmdN)`

**Description**: Connects stdout of each command to stdin of the next.

**Examples**:
```lisp
(pipe (ls "-la") (grep "txt"))
; Equivalent to: ls -la | grep txt

(| (cat "file.txt") (sort) (uniq))
; Equivalent to: cat file.txt | sort | uniq
```

**Return Value**: Returns the `Process` of the last command.

### Stream Operations

**Syntax**: `(map fn stream)`, `(filter pred stream)`, `(reduce fn init stream)`

**Description**: Higher-order functions for lazy stream processing.

**Examples**:
```lisp
; Convert file to uppercase
(map string-upcase (lines (cat "file.txt")))

; Filter non-empty lines
(filter (lambda (line) (> (string-length line) 0))
        (lines (cat "file.txt")))

; Sum numbers in file
(reduce + 0 (map string->integer (lines (cat "numbers.txt"))))
```

**Additional Stream Functions**:
```lisp
(lines proc)      ; Stream of lines from process stdout
(take n stream)   ; First n elements
(drop n stream)   ; Skip first n elements
(collect stream)  ; Collect entire stream into list
```

### Exit Code Handling

```lisp
(status proc)     ; Return exit code (integer)
(success? proc)   ; True if exit code is 0
(failed? proc)    ; True if exit code is non-zero
```

**Examples**:
```lisp
(if (success? (grep "foo" "file.txt"))
    (print "Found!")
    (print "Not found"))

(define exit-code (status (ls "/nonexistent")))
; exit-code is non-zero
```

### Environment Variables

```lisp
(getenv "VAR")         ; Get environment variable
(setenv "VAR" value)   ; Set environment variable
(export "VAR")         ; Export variable to child processes
```

**Examples**:
```lisp
(define path (getenv "PATH"))
(setenv "MY_VAR" "hello")
(export "MY_VAR")
(echo (getenv "MY_VAR"))  ; Child process sees MY_VAR
```

### Built-in Shell Utilities

```lisp
(cd path)         ; Change working directory
(pwd)             ; Print working directory
(alias name cmd)  ; Create command alias
(source path)     ; Evaluate Lisp file in current environment
```

**Examples**:
```lisp
(cd "/tmp")
(pwd)  ; → "/tmp"
(alias ll (lambda () (ls "-la")))
(source "~/.crackedrc")
```

## Macro Examples

### Custom Control Flow

```lisp
; when: execute body if condition is true
(defmacro when (cond . body)
  `(if ,cond (begin ,@body) #f))

(when (> x 0)
  (print "positive"))

; unless: execute body if condition is false
(defmacro unless (cond . body)
  `(if (not ,cond) (begin ,@body) #f))

(unless (file-exists? "foo.txt")
  (print "File not found"))
```

### Iteration

```lisp
; for-each: iterate over list
(defmacro for-each (var lst . body)
  `(map (lambda (,var) ,@body) ,lst))

(for-each line (lines (cat "file.txt"))
  (print line))
```

### Pipeline Sugar

```lisp
; -> threading macro (like Clojure)
(defmacro -> (x . forms)
  (if (null? forms)
      x
      `(-> (,(car (car forms)) ,x ,@(cdr (car forms)))
           ,@(cdr forms))))

(-> "/etc/hosts"
    (cat)
    (lines)
    (filter (lambda (line) (not (string-prefix? "#" line))))
    (collect))
; Equivalent to:
; (collect (filter ... (lines (cat "/etc/hosts"))))
```

## Complete Examples

### Simple Script

```lisp
#!/usr/bin/env cracked

; Print files larger than 1MB
(for-each file (lines (find "." "-type" "f"))
  (let ((size (string->integer (car (lines (du "-b" file))))))
    (when (> size 1048576)
      (println file))))
```

### Pipeline with Stream Processing

```lisp
; Count unique words in file
(define word-count
  (reduce (lambda (acc word) (+ acc 1))
          0
          (map string-downcase
               (filter (lambda (w) (> (string-length w) 0))
                       (string-split (read-file "doc.txt") " ")))))

(println word-count)
```

### Background Job

```lisp
; Run long command in background
(define job (background (sleep "10")))
(print "Job started")
(print "Doing other work...")
(wait job)
(print "Job finished")
```

### Error Handling

```lisp
; Try command, fallback on failure
(define result
  (if (success? (grep "pattern" "file.txt"))
      (collect (stdout (grep "pattern" "file.txt")))
      '()))

(if (null? result)
    (print "Pattern not found")
    (for-each line result
      (println line)))
```

## Standard Library

The standard library provides higher-level utilities built from primitives.
These are implemented in Lisp and loaded on startup.

### File Operations

```lisp
(file-exists? path)
(directory? path)
(read-lines path)      ; Read file as list of lines
(write-lines path lst) ; Write list of lines to file
```

### Functional Utilities

```lisp
(identity x)           ; Return x unchanged
(compose f g)          ; Function composition: (compose f g) = λx.f(g(x))
(partial f . args)     ; Partial application
```

### Stream Utilities

```lisp
(take-while pred stream)
(drop-while pred stream)
(filter-map fn stream)  ; Map then filter non-#f
(fold-left fn init stream)
```

## Differences from Standard Lisps

### Simplifications

1. **No CLOS**: No object system; use data structures and functions
2. **No Conditions**: Use `Result` types or simple error messages
3. **Limited Numeric Tower**: Only `int` and `float`, no rationals/complex
4. **No Tail Recursion Guarantee**: Tail calls optimized but not required by spec
5. **No Continuations**: No `call/cc`

### Extensions

1. **Shell Integration**: Commands, pipes, streams are first-class
2. **Async**: Background jobs and parallel execution
3. **Practical Defaults**: Mutable variables, simpler scoping

### Incompatibilities

- **Empty List**: `()` is the only falsy list (bash-like)
- **Symbol Case**: Case-sensitive (unlike some Lisps)
- **Macro Hygiene**: Not guaranteed (caveat emptor on variable capture)

## Error Messages

### Parse Errors

```
Parse error at line 3, column 5: unclosed string literal
  (define x "hello
            ^
```

### Runtime Errors

```
Undefined symbol: foo
  at eval (input:2:3)

Type error: expected integer, got string
  in function: +
  at eval (input:1:1)

Arity error: + expects at least 1 arg, got 0
  at eval (input:1:1)
```

## Reserved Words

The following symbols have special meaning and cannot be redefined:

- `quote`, `if`, `define`, `lambda`, `let`, `let*`, `begin`, `defmacro`
- `quasiquote`, `unquote`, `unquote-splicing`

## Naming Conventions

- **Predicates**: End with `?` (e.g., `null?`, `integer?`)
- **Mutators**: End with `!` (e.g., `set!`, `reverse!`) [if added]
- **Type Converters**: Use `->` (e.g., `string->integer`, `list->vector`)
- **Constants**: ALL_CAPS (e.g., `PI`, `MAX_INT`)

## Implementation Notes

### Evaluation Order

- **Function Arguments**: Evaluated left-to-right before function application
- **Special Forms**: Custom evaluation order (e.g., `if` short-circuits)
- **Macros**: Expanded before evaluation

### Scoping

- **Lexical Scoping**: Variables resolved in the lexical environment
- **Closures**: Lambdas capture their defining environment

### Tail Call Optimization

Tail calls are optimized when feasible (last expression in function), but not
guaranteed in all cases. Use iteration for deeply recursive algorithms if stack
overflow is a concern.

### Macro Expansion

Macros expand during the parse phase (before evaluation). This enables
compile-time optimization but means macros cannot be defined conditionally at
runtime.

## Future Extensions

### Potential Additions

- **Pattern Matching**: `(match expr (pattern1 result1) ...)`
- **Reader Macros**: `#[...]` custom syntax
- **Modules**: `(import foo)`, `(export bar)`
- **Static Types**: Optional type annotations and checking

## Revision History

- 2025-10-05: Initial Lisp reference specification created (Task 1.3)
