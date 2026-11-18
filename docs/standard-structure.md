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

## 2. Compiler Pipeline Architecture

The Elaichi compiler and runtime are structured into five core stages:

```text
.el Source Code
      │
      ▼
1. Frontend Scanner (Lexer)
      │  Tokens with line/column byte spans
      ▼
2. Recursive Descent Parser
      │  Typed Abstract Syntax Tree (AST)
      ▼
3. AST Bifurcator
      ├── Static Branch   ──► Native / Unboxed Register Engine
      └── Dynamic Branch  ──► Stack Bytecode VM
      ▲
      │
4. Inter-Stack Bridge
      │  Zero-cost unboxing & bidirectional marshalling
      ▼
5. Unified Runtime Engine
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
