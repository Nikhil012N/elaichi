# Compiler Internals

Technical reference for the compiler passes, type inference, SSA intermediate representation (MIR), and the dual-stack execution engine of Elaichi.

---

## 1. Modular Source Organization

```
src/
├── frontend/             # Lexer, Parser, AST & Multi-File Module Loader
│   ├── ast.rs            # AST node definitions, statements, and type system
│   ├── lexer.rs          # Token scanner with line/col and escape sequences
│   ├── parser.rs         # Recursive-descent grammar parser with diagnostics
│   └── loader.rs         # Recursive multi-file import resolver
├── compiler/             # High-Performance SSA & Type Inference Pipeline
│   ├── types.rs          # Hindley-Milner type system & TypeSchemes
│   ├── infer.rs          # Algorithm W type unification and occurs check
│   ├── mir.rs            # Virtual SSA registers (%0, %1) & Control Flow Graph
│   ├── mir_builder.rs    # AST to SSA IR lowering pass
│   ├── mir_opt.rs        # Constant folding, DCE, inlining & escape analysis
│   ├── mir_llvm.rs       # Native LLVM IR emitter
│   ├── mir_transpile.rs  # High-performance ISO C99 emitter
│   ├── lsp.rs            # Language Server Protocol daemon
│   ├── bifurcator.rs     # Static vs dynamic stack partitioning pass
│   ├── static_engine.rs  # Unboxed static instruction set & frame executor
│   └── vm.rs             # Dynamic bytecode chunk, OpCodes, and tagged values
├── runtime/              # Execution Loop, Primitives & Standard Library
│   ├── engine.rs         # Dual-stack runtime coordinator
│   └── stdlib.rs         # 25+ built-in math, string, and collection primitives
├── main.rs               # Rust CLI driver and file runner
└── tests.rs              # Comprehensive unit and integration test suite
```

---

## 2. Hindley-Milner Type Inference (Algorithm W)

Elaichi implements classical Hindley-Milner type inference in `src/compiler/types.rs` and `src/compiler/infer.rs`:

* **Zero Boxing Overhead**: Types are inferred ahead of time without requiring mandatory user type annotations.
* **Unification**: Structural equality unification with substitution composition ($S_2 \circ S_1$).
* **Occurs Check**: Prevents infinite recursive types ($\tau = [\tau]$).
* **Principal Types**: Resolves expressions to canonical CPU primitives (`i64`, `f64`, `bool`, `str`, `[T]`, `Fn(T) -> U`).

---

## 3. SSA Mid-Level IR (MIR)

The SSA compiler (`src/compiler/mir.rs`) transforms high-level syntax into a basic-block Control Flow Graph:

* **Registers**: Infinite virtual registers (`%0`, `%1`, `%2`...) storing pure 64-bit machine words.
* **Basic Blocks**: Linear sequences of instructions ending with a strict terminator (`Goto`, `Branch`, `Return`, `Unreachable`).
* **Phi Nodes**: Merges converging values across conditional branches without memory reads.

---

## 4. Compiler Optimization Passes (`src/compiler/mir_opt.rs`)

1. **Constant Folding**: Pre-computes arithmetic and logical operations at compile-time (`4 * 5` $\to$ `20`).
2. **Dead Code Elimination (DCE)**: Prunes unread registers and dead basic blocks.
3. **Escape Analysis**: 
   - Non-escaping lists/records are promoted directly to the CPU stack (`AllocStack`).
   - Escaping values cross to ARC without stop-the-world GC pauses.
4. **Leaf Function Inliner**: Inlines candidate functions directly into call sites to eliminate call-frame setup.

---

## 5. Emission Backends

* **LLVM IR Backend (`src/compiler/mir_llvm.rs`)**: Emits typed SSA LLVM IR matching the inkwell backend spec for native compilation.
* **C99 Transpiler (`src/compiler/mir_transpile.rs`)**: Emits ANSI C code for compilation with GCC or Clang `-O3`.
* **Dual-Stack Runtime**: Cooperative execution between unboxed native frames and dynamic bytecode chunks via `Bridge`.
