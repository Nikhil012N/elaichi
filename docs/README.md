# Elaichi Language & Runtime Specification

Elaichi is a general-purpose programming language built around an Abstract Syntax Tree (AST) Bifurcation model. It evaluates programs across two cooperative execution stacks: an unboxed native static stack and a tagged bytecode dynamic virtual machine.

---

## Technical Overview

Programs written in Elaichi are parsed into a single unified syntax tree and partitioned during compilation:

1. **Static Execution Stack**: Subroutines declared with `static function` compile to linear arrays of unboxed opcodes (`StaticInst`). Variables reside in pre-allocated register slots without heap allocations, boxing headers, or garbage collection overhead.
2. **Dynamic Execution Stack**: Routines declared with standard `function` syntax compile to virtual machine bytecode (`OpCode`). Dynamic values are stored as tagged unions (`DynValue`) supporting runtime mutation, heterogenous lists, and dictionaries.
3. **Inter-Stack Boundary**: Calls between dynamic and static scopes pass through zero-copy type-checking thunks implemented in `src/bridge.rs`.

---

## Core Language Syntax

### Lexical Scoping and Variables

Elaichi prohibits implicit declarations and legacy variable hoisting. All identifiers must be bound using `const` or `let`:

```javascript
// Immutable dynamic binding
const maxRetries = 5;

// Mutable dynamic binding
let counter = 0;
counter = counter + 1;

// Statically typed bindings (unboxed in static scopes)
const step: i64 = 1;
let accumulator: f64 = 0.0;
```

### Static Functions

Functions marked with `static function` require explicit parameter and return types:

```javascript
static function dotProduct(x1: f64, y1: f64, x2: f64, y2: f64): f64 {
    const dx: f64 = x1 * x2;
    const dy: f64 = y1 * y2;
    return dx + dy;
}
```

### Dynamic Functions

Standard functions execute on the register VM:

```javascript
function calculateMetrics(records) {
    let total = 0;
    for (let i in 0..len(records)) {
        total = total + records[i];
    }
    return total;
}
```

### Control Flow

Parentheses are mandatory for loop and branch conditions:

```javascript
// Conditional branching
if (value > 100) {
    terminal.println("Threshold exceeded");
} else if (value == 100) {
    terminal.println("Threshold met");
} else {
    terminal.println("Within limits");
}

// While loop with continue
let n = 10;
while (n > 0) {
    if (n == 5) {
        n = n - 1;
        continue;
    }
    n = n - 1;
}

// Range iteration with break
for (let i in 0..100) {
    if (i == 50) {
        break;
    }
}
```

---

## Memory & Type System

### Scalar Types

| Type | Size | Allocation Target | Description |
| :--- | :--- | :--- | :--- |
| `i64` | 8 bytes | Static Stack | 64-bit two's complement signed integer |
| `f64` | 8 bytes | Static Stack | 64-bit IEEE 754 double-precision floating point |
| `bool` | 1 byte | Static Stack | Boolean logic flag (`true`, `false`) |
| `string` | Variable | Dynamic Heap | UTF-8 encoded string |
| `dyn` | Variable | Dynamic Stack | Tagged variant representing arbitrary runtime values |

### Dictionaries and In-Place Mutation

Dictionaries support dot-notation and key indexing:

```javascript
let config = {
    host: "127.0.0.1",
    port: 8080,
    active: true
};

// Field assignment
config.port = 9000;
terminal.println(config.port); // 9000
```

### Explicit Type Casting

Type casting is performed via the `as` operator:

```javascript
const raw = 42;
const asFloat = raw as f64;
const asText = raw as string;
```

---

## The Terminal Subsystem

Elaichi provides first-class I/O opcodes via the `terminal` namespace. Writes are flushed directly through the runtime standard streams:

```javascript
terminal.print("Processing batch... ");
terminal.println("Done.");

terminal.warn("Memory capacity near ceiling");
terminal.error("Unable to bind network interface");
terminal.clear();
```

---

## Standard Utility Library

* `len(collection)`: Returns the integer length of an array, string, or dictionary.
* `push(list, element)`: Appends an element to an array and returns the modified array.
* `str(value)`: Returns the string representation of any primitive or composite value.
* `keys(map)`: Returns an array of dictionary keys sorted alphabetically.
* `values(map)`: Returns an array of values ordered by sorted dictionary keys.
* `contains(collection, target)`: Verifies inclusion within an array, dictionary, or string.
* `abs(number)`: Computes the absolute value of a signed integer or floating-point value.
* `min(a, b)`: Returns the minimum of two numerical values.
* `max(a, b)`: Returns the maximum of two numerical values.
