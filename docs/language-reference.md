# Language Reference

This document provides the formal grammar, keyword definitions, and semantics for the Elaichi programming language.

---

## 1. Identifiers & Scoping

Variable identifiers must begin with an alphabetic character or underscore, followed by alphanumeric characters or underscores. 

All bindings must be explicitly introduced using `const` or `let`:

```javascript
// Constant: reassignment produces a compiler error
const bufferSize = 1024;

// Mutable binding
let offset = 0;
offset = offset + 16;
```

---

## 2. Primitive Types

Elaichi defines the following primitive types for statically typed declarations:

* `i64`: 64-bit signed integer.
* `f64`: 64-bit double-precision IEEE 754 floating point.
* `bool`: Boolean value (`true` or `false`).
* `string`: Dynamic UTF-8 string sequence.

Type annotations follow the variable identifier separated by a colon:

```javascript
const maxLimit: i64 = 500;
let ratio: f64 = 0.75;
let active: bool = true;
```

---

## 3. Functions

### Static Functions

Functions prefixed with `static function` enforce static compilation. Type signatures are mandatory:

```javascript
static function vectorDot(x1: f64, y1: f64, x2: f64, y2: f64): f64 {
    const dx: f64 = x1 * x2;
    const dy: f64 = y1 * y2;
    return dx + dy;
}
```

### Dynamic Functions

Dynamic functions do not require explicit type annotations and run on the bytecode virtual machine:

```javascript
function formatOutput(header, payload) {
    terminal.println(header);
    terminal.println(payload);
}
```

---

## 4. Control Flow Statements

### Conditionals

Parentheses around conditions and braces around blocks are mandatory:

```javascript
if (count == 0) {
    terminal.println("Zero elements");
} else if (count > 0) {
    terminal.println("Positive elements");
} else {
    terminal.println("Negative elements");
}
```

### While Loops

```javascript
let i = 0;
while (i < 10) {
    if (i == 4) {
        i = i + 1;
        continue;
    }
    terminal.println(i);
    i = i + 1;
}
```

### For-In Range Loops

Iterates over a half-open numerical range `start..end`:

```javascript
for (let i in 0..10) {
    if (i == 7) {
        break;
    }
    terminal.println(i);
}
```

---

## 5. Composite Data Structures

### Arrays

Arrays are dynamically sized and indexed via bracket syntax:

```javascript
let numbers = [10, 20, 30, 40];
numbers[1] = 25;
terminal.println(numbers[1]); // 25
```

### Dictionaries

Key-value structures with dot-notation field access and in-place field updates:

```javascript
let node = {
    address: "10.0.0.1",
    port: 443,
    secure: true
};

node.port = 8443;
terminal.println(node.port); // 8443
```

---

## 6. Type Casting (`as`)

Converts expressions to a specified primitive type:

```javascript
const value = 50;
const asFloat = value as f64;
const asText = value as string;
```

---

## 7. Terminal I/O Subsystem

The `terminal` namespace provides native bytecode stream operations:

* `terminal.print(expr)`: Emits output without a trailing newline.
* `terminal.println(expr)`: Emits output followed by a newline.
* `terminal.warn(expr)`: Emits output prefixed with warning logging tags.
* `terminal.error(expr)`: Emits output to the standard error stream.
* `terminal.clear()`: Sends an ANSI clear-screen command to the terminal.
