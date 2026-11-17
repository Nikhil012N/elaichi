# Compiler Internals

Technical reference for the compiler passes, bytecode representations, and the dual-stack execution engine of Elaichi.

---

## Source Organization

```
src/
├── ast.rs            # AST node definitions, statements, and type system
├── bifurcator.rs     # Partitioning pass: classifies AST into static vs dynamic IR
├── bridge.rs         # Type coercion, unboxing, and boxing implementations
├── lexer.rs          # Lexical scanner for tokens and operators
├── main.rs           # CLI driver and file loader
├── parser.rs         # Recursive-descent grammar parser
├── runtime.rs        # Cooperative dual-stack execution coordinator
├── static_engine.rs  # Unboxed static instruction set & frame executor
├── tests.rs          # Test suite
└── vm.rs             # Dynamic bytecode chunk, OpCodes, and tagged values
```

---

## 1. Lexical Analysis (`src/lexer.rs`)

The lexical scanner transforms UTF-8 source text into a sequential vector of `Token` instances:

* **Keywords**: `static`, `function`, `const`, `let`, `if`, `else`, `while`, `for`, `in`, `struct`, `return`, `as`, `break`, `continue`.
* **Terminal Primitives**: `terminal`, `print`, `println`, `warn`, `error`, `clear`.
* **Types**: `i64`, `f64`, `bool`, `string`.
* **Operators**: `+`, `-`, `*`, `/`, `%`, `==`, `!=`, `<`, `<=`, `>`, `>=`, `&&`, `||`, `!`, `..`.

---

## 2. Parser Architecture (`src/parser.rs`)

The parser uses a recursive-descent strategy combined with precedence climbing for binary expressions:

* **Precedence Hierarchy**:
  1. Primary expressions (literals, variables, grouped expressions)
  2. Postfix expressions (indexing `arr[i]`, property access `obj.field`, calls `fn()`)
  3. Prefix unary expressions (`!`, `-`)
  4. Multiplicative expressions (`*`, `/`, `%`)
  5. Additive expressions (`+`, `-`)
  6. Relational comparisons (`<`, `<=`, `>`, `>=`)
  7. Equality comparisons (`==`, `!=`)
  8. Logical conjunction (`&&`)
  9. Logical disjunction (`||`)

---

## 3. Bifurcation Pass (`src/bifurcator.rs`)

The bifurcator segregates the validated AST into static instruction streams and dynamic bytecode chunks:

```rust
pub struct Bifurcator {
    pub static_functions: HashMap<String, (Function, StaticFunctionCode)>,
    pub dynamic_functions: HashMap<String, (Function, DynamicBytecodeChunk)>,
}
```

During compilation:
* Loop jumps for `break` and `continue` statements are maintained on jump stacks (`break_jumps` and `continue_jumps`) and resolved to concrete bytecode indices upon loop completion.
* Unboxed local variable slots are bound sequentially in `StaticFunctionCode.local_types`.

---

## 4. Cooperative Runtime Engine (`src/runtime.rs`)

The runtime coordinates memory, variable frames, and inter-stack boundary calls:

```rust
pub struct Runtime {
    pub bifurcator: Bifurcator,
    pub dyn_globals: HashMap<String, DynValue>,
}
```

Execution begins at the specified entry function (default `main`). When an opcode requires cross-stack communication:
* The unboxing thunk converts dynamic tagged values (`DynValue`) into unboxed static variants (`StaticValue`).
* Execution branches directly into `static_engine.rs`.
* Upon completion, the static value is boxed back into a dynamic value and returned to the VM frame.
