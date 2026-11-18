<div style="text-align: center; padding: 48px 0 36px; border-bottom: 1px solid #1e2227;">
  <img src="elaichi-logo.svg" alt="Elaichi" width="100" height="100" style="margin-bottom: 20px; display: inline-block;">
  <h1 style="font-size: 3rem; font-weight: 800; letter-spacing: -1.2px; margin: 0 0 12px; border: none; color: #ffffff;">Elaichi</h1>
  <p style="font-size: 1.25rem; color: #94a3b8; max-width: 640px; margin: 0 auto 28px; line-height: 1.6;">
    A fast, modern JavaScript-syntax runtime designed for everyone.<br>Develop dynamically, execute compute-heavy loops at raw machine speed.
  </p>
  <div style="display: flex; justify-content: center; gap: 12px; flex-wrap: wrap; margin-bottom: 28px;">
    <a href="#/getting-started" style="background: #B5E853; color: #0c0d0e; font-weight: 700; padding: 11px 24px; border-radius: 2px; text-decoration: none; font-size: 0.95rem;">Quickstart Guide →</a>
    <a href="#/language-reference" style="background: #16181d; color: #f1f5f9; border: 1px solid #282c34; font-weight: 600; padding: 11px 24px; border-radius: 2px; text-decoration: none; font-size: 0.95rem;">API Reference</a>
    <a href="https://github.com/Nikhil012N/elaichi" target="_blank" style="background: #16181d; color: #94a3b8; border: 1px solid #282c34; font-weight: 600; padding: 11px 20px; border-radius: 2px; text-decoration: none; font-size: 0.95rem;">GitHub</a>
  </div>
  <div style="background: #121417; border: 1px solid #22262e; border-radius: 2px; padding: 12px 24px; display: inline-flex; align-items: center; gap: 10px; font-family: 'JetBrains Mono', monospace; font-size: 0.92rem; color: #f1f5f9;">
    <span style="color: #64748b; user-select: none;">$</span>
    <span>elaichi run app.el</span>
  </div>
</div>

<div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(280px, 1fr)); gap: 16px; margin: 36px 0;">
  <div style="background: #121417; border: 1px solid #1e2227; border-radius: 2px; padding: 22px;">
    <div style="font-family: 'JetBrains Mono', monospace; font-size: 0.72rem; color: #B5E853; font-weight: 700; margin-bottom: 8px;">DUAL-STACK RUNTIME</div>
    <h3 style="margin: 0 0 8px; color: #ffffff; font-size: 1.1rem; font-weight: 700;">Static Speed When You Need It</h3>
    <p style="margin: 0; color: #94a3b8; font-size: 0.9rem; line-height: 1.6;">Add <code>static</code> to any function for zero-overhead, unboxed CPU register computation without garbage collection lag.</p>
  </div>
  <div style="background: #121417; border: 1px solid #1e2227; border-radius: 2px; padding: 22px;">
    <div style="font-family: 'JetBrains Mono', monospace; font-size: 0.72rem; color: #B5E853; font-weight: 700; margin-bottom: 8px;">FAMILIAR SYNTAX</div>
    <h3 style="margin: 0 0 8px; color: #ffffff; font-size: 1.1rem; font-weight: 700;">Standard JavaScript Feel</h3>
    <p style="margin: 0; color: #94a3b8; font-size: 0.9rem; line-height: 1.6;">Build easily with <code>const</code>, <code>let</code>, dictionary objects, dynamic lists, range loops, and straightforward control flow.</p>
  </div>
  <div style="background: #121417; border: 1px solid #1e2227; border-radius: 2px; padding: 22px;">
    <div style="font-family: 'JetBrains Mono', monospace; font-size: 0.72rem; color: #B5E853; font-weight: 700; margin-bottom: 8px;">ZERO DEPENDENCIES</div>
    <h3 style="margin: 0 0 8px; color: #ffffff; font-size: 1.1rem; font-weight: 700;">Self-Contained Executable</h3>
    <p style="margin: 0; color: #94a3b8; font-size: 0.9rem; line-height: 1.6;">Compiles directly to a single binary with zero external runtime requirements. Download and run anywhere.</p>
  </div>
</div>

---

## What is Elaichi?

Elaichi is a general-purpose programming language built in Rust that unites **developer productivity** and **execution performance**.

Most environments force a trade-off:
* **Dynamic languages** (like standard JavaScript and Python) are intuitive to write, but every value is wrapped in extra memory ("boxed"), causing math and loop bottlenecks.
* **Systems languages** (like Rust, C, or C++) are fast, but require manual memory handling, complex setups, and steep learning curves.

**Elaichi solves this with AST Bifurcation:**
1. **Dynamic Track**: Write standard, expressive application logic using everyday variables and objects.
2. **Static Track**: Mark compute-intensive algorithms with `static function` to execute at raw CPU hardware speeds.
3. **The Inter-Stack Bridge**: Dynamic and static routines communicate seamlessly. You can call static functions directly from regular code without writing foreign function bindings.

---

## Quick Example

Here is a complete program demonstrating how dynamic code interacts with a high-performance static routine:

```javascript
// 1. High-Performance Static Routine (Runs at raw CPU speed)
static function calculateFibonacci(n: i64): i64 {
    if (n <= 1) {
        return n;
    }
    return calculateFibonacci(n - 1) + calculateFibonacci(n - 2);
}

// 2. Dynamic Application Logic (Familiar and flexible)
function main() {
    terminal.println("=== Elaichi Execution Engine ===");

    // Call static computation seamlessly
    const terms = 20;
    terminal.print("Calculating fibonacci(" + terms + ")... ");
    const result = calculateFibonacci(terms);
    terminal.println(result);

    // Dynamic dictionary mutation
    let report = {
        task: "Fibonacci Calculation",
        input: terms,
        output: result,
        status: "complete"
    };

    terminal.println("Status: " + report.status);
    return 0;
}
```

---

## Architectural Comparison

| Feature | Elaichi | Standard Dynamic Engines | Systems Languages |
| :--- | :--- | :--- | :--- |
| **Learning Curve** | **Beginner-Friendly (JS-Style)** | Beginner-Friendly | Steep / Strict |
| **Static Execution** | **Unboxed Hardware Registers** | Deoptimized / JIT Bailouts | Compiled Machine Code |
| **Dynamic Execution** | **Bytecode Virtual Machine** | Bytecode VM / Interpreter | Not Supported (Static Only) |
| **Value Boxing** | **Zero boxing in `static` scopes** | Full Object Boxing | Zero Boxing |
| **I/O Engine** | **First-class `terminal.*` opcodes** | Standard Library (`console.log`) | System Calls / Streams |
| **Distribution** | **Single standalone binary** | Large runtime directory | Single standalone binary |

---

## Standard Library Primitives

Elaichi includes essential utility functions out-of-the-box:

* **Collections**: `len(item)` returns list or dictionary size; `push(list, value)` appends an item.
* **Dictionary Reflection**: `keys(dict)` returns sorted key names; `values(dict)` returns property values.
* **Membership**: `contains(collection, target)` checks for items in lists, keys in dictionaries, or substrings in text.
* **Math Primitives**: `abs(number)` returns the absolute value; `min(a, b)` and `max(a, b)` evaluate boundaries.
* **Terminal I/O**: `terminal.print()`, `terminal.println()`, `terminal.warn()`, `terminal.error()`, and `terminal.clear()`.

---

## Documentation Index

Explore the rest of the documentation:
* [Getting Started Guide](getting-started.md) — Toolchain installation, compilation, and project creation.
* [Language Reference](language-reference.md) — Complete specifications for syntax, types, loops, and operators.
* [How It Works Under the Hood](architecture.md) — Explanation of AST bifurcation and the inter-stack bridge.
* [Compiler Internals](compiler-internals.md) — Rust source code structure, parser grammar, and bytecode opcodes.
