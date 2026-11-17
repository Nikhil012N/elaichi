# Architecture & The Dual-Stack Execution Model

This document outlines the internal architecture of the Elaichi execution model, explaining how AST bifurcation coordinates unboxed static execution and tagged dynamic bytecode evaluation.

---

## The AST Bifurcation Pipeline

Elaichi uses a bifurcation compiler pass to resolve the gradual typing overhead problem. Rather than boxing all values or relying on foreign function interfaces (FFI), the front-end compiler partitions program subtrees into two independent runtime representations:

```
                       Source Code (.el)
                              │
                    Lexer & AST Parser
                              │
                    AST Validation Pass
                              │
                      Bifurcation Pass
                     ┌────────┴────────┐
                     ▼                 ▼
             Static Functions   Dynamic Functions
                     │                 │
             Static Codegen     Bytecode Compiler
                     │                 │
                     ▼                 ▼
            StaticFunctionCode  DynamicBytecodeChunk
                     │                 │
             ┌───────▼───────┐ ┌───────▼───────┐
             │  Static Stack │ │ Dynamic Stack │
             │  (Direct ABI) │ │  (Tagged VM)  │
             └───────┬───────┘ └───────┬───────┘
                     └────────┬────────┘
                              │
                     Inter-Stack Bridge
```

---

## 1. Static Execution Stack

Functions designated with the `static` keyword are compiled into unboxed machine frames:

* **Direct Frame Mapping**: Local variables are indexed by integer register offsets.
* **Unboxed Primitives**: Values of type `i64`, `f64`, and `bool` reside directly in contiguous memory slots without pointers, headers, or runtime tags.
* **Scope-Bound Deallocation**: Memory for stack frames is allocated and released deterministically upon function entry and exit.
* **Instruction Dispatch**: Evaluates linear sequences of `StaticInst` opcodes.

### Static Instruction Set (`StaticInst`)

```rust
pub enum StaticInst {
    ConstI64(i64),
    ConstF64(f64),
    ConstBool(bool),
    LoadLocal(usize),
    StoreLocal(usize),
    I64Add,
    I64Sub,
    I64Mul,
    I64Div,
    I64Mod,
    I64Eq,
    I64Neq,
    I64Lt,
    I64Le,
    I64Gt,
    I64Ge,
    JumpIfFalse(usize),
    Jump(usize),
    Call(String, usize),
    Return,
}
```

---

## 2. Dynamic Execution Stack

Standard function blocks compile to bytecode executed by a register virtual machine:

* **Dynamic Value Variant**: Stored using the tagged enum `DynValue`:
  ```rust
  pub enum DynValue {
      Nil,
      Bool(bool),
      Int(i64),
      Float(f64),
      Str(String),
      List(Vec<DynValue>),
      Map(HashMap<String, DynValue>),
  }
  ```
* **Dynamic Bytecode Opcodes (`OpCode`)**:
  * Variable Access: `LoadLocal`, `StoreLocal`, `LoadGlobal`, `StoreGlobal`.
  * Collections: `BuildList`, `BuildMap`, `IndexGet`, `IndexSet`, `FieldGet`, `FieldSet`.
  * Conversions: `Cast(ElaichiType)`.
  * Control Flow: `Jump`, `JumpIfFalse`, `Call`, `Return`.
  * I/O Subsystem: `TerminalPrint`, `TerminalPrintln`, `TerminalWarn`, `TerminalError`, `TerminalClear`.

---

## 3. The Inter-Stack Bridge

The boundary between the dynamic stack and static stack is managed by the `Bridge` subsystem (`src/bridge.rs`):

1. **Invocation from Dynamic Scopes**: When a dynamic function executes a `Call` to a static subroutine, the bridge checks the argument types against the target static parameters.
2. **Unboxing**: Converts tagged `DynValue` variants into raw `StaticValue` primitives.
3. **Execution**: Evaluates the static frame to completion.
4. **Boxing**: Converts the return `StaticValue` into a `DynValue` and pushes it onto the dynamic VM stack.
