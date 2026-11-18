<p align="center">
  <img src="./elaichi-logo.svg" alt="elaichi" width="160" />
</p>

# Elaichi

A general-purpose programming language that partitions execution between two runtime stacks: an unboxed native static stack and a dynamic bytecode stack.

---

## Architecture

Most runtimes evaluate code through a single execution strategy. A language is either compiled ahead-of-time (Rust, C, Go) or evaluated dynamically inside an interpreter (Python, JavaScript). Languages that attempt gradual typing typically box all values into heap objects, meaning typed code still incurs dynamic dispatch overhead.

Elaichi partitions the Abstract Syntax Tree (AST) at compile time via a bifurcation pass:

```
                     Source File (.el)
                            │
                    Frontend Compiler
                    (Lexer / Parser)
                            │
                    Bifurcation Pass
                    ┌───────┴───────┐
                    ▼               ▼
            [Static AST]      [Dynamic AST]
                    │               │
               Native Codegen  Bytecode Compiler
                    │               │
                    ▼               ▼
            ┌───────────────┐ ┌───────────────┐
            │ Static Stack  │ │ Dynamic Stack │
            │ (unboxed CPU  │ │ (tagged NaN   │
            │  call frames) │ │  VM stack)    │
            └───────┬───────┘ └───────┬───────┘
                    │                 │
                    └──► Bridge Thunk ◄┘
                            │
                       OS / Thread
```

### Static Stack
- Functions marked `static function` and typed bindings compile to unboxed call frames.
- Primitives (`i64`, `f64`, `bool`) occupy raw stack slots and registers with no heap allocations or GC metadata.
- Function calls execute as direct machine jumps without hash table lookups or inline cache misses.

### Dynamic Stack
- Standard `function` definitions execute on a register bytecode virtual machine.
- Variables declared with `const` or `let` without explicit type signatures are stored as tagged values.
- Objects and arrays are dynamically sized and evaluated at runtime.

### The Inter-Stack Bridge
When a dynamic function invokes a static function, the compiler inserts a bridge thunk:
1. Dynamic arguments are inspected against target static parameter types.
2. If arguments match or can be coerced, values are unboxed directly into the static call frame.
3. If an argument violates the expected type signature, a runtime type error is emitted.
4. On return, unboxed static return values are boxed into tagged values and placed back on the caller's evaluation stack.

---

## Syntax

### Variables

Elaichi enforces `const` and `let` declarations:

```javascript
// Immutable dynamic binding
const maxRetries = 5;

// Mutable dynamic binding
let counter = 0;
counter = counter + 1;

// Statically typed bindings (unboxed)
const speed: f64 = 9.81;
let total: i64 = 0;
```

### Functions

```javascript
// Statically typed native function
static function multiply(a: i64, b: i64): i64 {
    const result: i64 = a * b;
    return result;
}

// Dynamic function
function calculateTotal(items) {
    let sum = 0;
    for (let i in 0..len(items)) {
        sum = sum + items[i];
    }
    return sum;
}
```

### Control Flow

Parentheses around conditions and braces around blocks are required:

```javascript
if (threshold > 100) {
    terminal.println("Threshold exceeded");
} else {
    terminal.println("Within limits");
}

while (count < 10) {
    count = count + 1;
}

for (let i in 0..5) {
    terminal.println(i);
}
```

### Terminal I/O

I/O operations use the `terminal` interface:

```javascript
// Print without newline
terminal.print("Loading: ");
terminal.print(50);
terminal.println("%");

// Diagnostic output
terminal.warn("Memory consumption elevated");
terminal.error("Connection rejected");

// Clear console
terminal.clear();
```

---

## Toolchain & CLI

### Installation

Building the compiler requires Rust 1.75+:

```bash
git clone https://github.com/nikhil/elaichi.git
cd elaichi
cargo build --release
```

Binary location: `target/release/elaichi`

### CLI Commands

Run a source file:
```bash
elaichi run app.el
```

Dump AST bifurcation classification:
```bash
elaichi dump-ast app.el
```

Check version:
```bash
elaichi --version
```

---

## Repository Structure

```
.
├── compiler/
│   ├── ast.rs            # AST definitions and nodes
│   ├── bifurcator.rs     # AST partitioning into static vs dynamic IR
│   ├── lexer.rs          # Token scanner
│   ├── parser.rs         # Recursive descent parser
│   ├── static_engine.rs  # Unboxed static instruction set
│   └── vm.rs             # Dynamic bytecode VM and opcodes
├── runtime/
│   ├── bridge.rs         # Marshaling and unboxing thunks
│   └── runtime.rs        # Stack execution coordinator
├── docs/                 # Documentation website source
├── examples/             # Sample source programs
├── elaichi-logo.svg      # Official vector logo
├── Cargo.toml
└── README.md
```

---

## License

MIT
