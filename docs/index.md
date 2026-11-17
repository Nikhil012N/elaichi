# Elaichi Documentation

Welcome to the official documentation for **Elaichi** (इलायची), a general-purpose programming language designed around the **Dual-Stack Execution Model (DSEM)**.

---

## What is Elaichi?

Elaichi solves the classic trade-off between **high-performance systems languages** (like Rust, C, and Go) and **flexible scripting languages** (like Python and JavaScript).

Instead of forcing your entire codebase into a single execution model or relying on fragile Foreign Function Interfaces (FFI), Elaichi splits your program into two cooperative stacks within the exact same process:

1. **The Static Stack (`#static`):** Unboxed native execution with deterministic memory layout and zero type-tag overhead.
2. **The Dynamic Stack (`#dynamic`):** Dynamically typed, bytecode-driven execution with fast runtime dispatch, duck typing, and prototyping freedom.
3. **The Inter-Stack Bridge:** Compiler-generated boundary thunks that validate, unbox, and box data as it moves between stacks.

```
                  ┌─────────────────────────────────────────┐
                  │            Source Code (.el)            │
                  └────────────────────┬────────────────────┘
                                       │
                               Lexer & Parser
                                       │
                                Bifurcation Pass
                                ┌──────┴──────┐
                                ▼             ▼
                        [Static Nodes]  [Dynamic Nodes]
                                │             │
                        ┌───────▼──────┐ ┌────▼────────┐
                        │ Static Stack │ │Dynamic Stack│
                        │ (Unboxed)    │ │(Tagged VM)  │
                        └───────▲──────┘ └────▲────────┘
                                └──────┬──────┘
                                       │
                               Runtime Bridge
```

---

## Core Philosophy

- **Zero-Cost Static Primitives:** Statically typed variables and functions compile to raw registers and stack offsets. No GC tags or heap boxing.
- **Natural Scripting Ergonomics:** High-level dynamic blocks allow rapid prototyping, dynamic maps, duck-typed handlers, and REPL evaluation.
- **Transparent Interoperability:** A dynamic routine can call a static function just like any local function; the compiler generates the type guards and unboxing automatically.

---

## Navigation

- [Getting Started](./getting-started.md) — Installation, building from source, and your first program.
- [Architecture & The Dual Stack](./architecture.md) — Deep dive into how the bifurcator, memory model, and runtime bridge work.
- [Language Reference](./language-reference.md) — Syntax, types, control flow, functions, and standard conventions.
- [Compiler Internals](./compiler-internals.md) — AST structures, bytecode instructions, static opcodes, and runtime execution.
