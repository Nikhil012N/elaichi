# Elaichi Standardization & Project Specification

This document outlines the standard architecture, project layout conventions, and compiler pipeline for the Elaichi programming language.

---

## 1. Standard Project Layout

Every production Elaichi package or application follows a predictable convention:

```text
my-app/
├── elaichi.json          # Project manifest & configuration
├── src/
│   ├── main.el           # Executable entrypoint (or lib.el for libraries)
│   ├── routes.el         # Domain logic
│   └── math/
│       └── matrix.el     # Nested modules
├── tests/
│   └── test_math.el      # Automated test suites
└── README.md
```

### The `elaichi.json` Manifest

```json
{
  "$schema": "https://elaichi.dev/schemas/manifest.v1.json",
  "name": "my-app",
  "version": "0.1.0",
  "edition": "2026",
  "entry": "src/main.el",
  "compiler": {
    "target": "dual-stack",
    "opt_level": 3
  }
}
```

---

## 2. Compiler Pipeline & Source Code Architecture

The Elaichi compiler and runtime are structured into three professional, decoupled engine modules:

```text
src/
├── frontend/             # Lexical analysis & AST generation
│   ├── mod.rs
│   ├── ast.rs            # Typed AST node definitions
│   ├── lexer.rs          # Token scanner with byte spans
│   └── parser.rs         # Recursive-descent grammar parser
├── compiler/             # AST Bifurcation & Code Generation
│   ├── mod.rs
│   ├── bifurcator.rs     # Dual-track classifier & bytecode emitter
│   ├── bridge.rs         # Zero-cost dynamic <-> static marshaller
│   ├── static_engine.rs  # Unboxed CPU register instructions
│   └── vm.rs             # Tagged dynamic VM opcodes & value types
├── runtime/              # Execution engine & Standard Library
│   ├── mod.rs
│   ├── engine.rs         # Execution dispatcher & stack frame loops
│   └── stdlib.rs         # Standard library functions & collections
├── tests.rs              # Self-testing validation test suite
└── main.rs               # Command-line interface (CLI)
```

---

## 3. Language Structure & Execution Tracks

Elaichi standardizes code into two cooperative execution models:

### Dynamic Track
* Intended for application flow, string formatting, configuration, and API handlers.
* Variables: `const` and `let` with dynamic typing.
* Example:
  ```javascript
  function handleRequest(payload) {
      terminal.println("Processing payload: " + payload);
      return { status: 200, done: true };
  }
  ```

### Static Track
* Intended for math-heavy algorithms, physics, matrices, image processing, and tight loops.
* Explicit primitive types (`i64`, `f64`, `bool`, `string`).
* Direct hardware register speeds with zero GC pauses.
* Example:
  ```javascript
  static function computeSum(n: i64): i64 {
      let total: i64 = 0;
      let i: i64 = 1;
      while (i <= n) {
          total = total + i;
          i = i + 1;
      }
      return total;
  }
  ```

---

## 4. Standard Library Namespaces

The standard library provides core capabilities under well-defined modules:

* `terminal`: Standard input/output, formatted logs, colors, and diagnostics.
* `math`: Arithmetic functions, trigonometric algorithms, and random number utilities.
* `fs`: File system I/O (reading, writing, directory walking).
* `time`: High-resolution timers, benchmarking, and timestamps.
* `string`: String utilities and character manipulation.
