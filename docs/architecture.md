# How Elaichi Works: Under the Hood

This page explains how Elaichi runs your code under the hood in simple, clear terms.

---

## 1. The Big Picture: Why Dual-Stack?

In most popular programming languages, you run into a trade-off:
* **Dynamically typed code** (like normal JavaScript) gives you great flexibility, but the computer has to constantly check what type every value is at runtime, creating memory overhead and slower math calculations.
* **Statically typed code** (like C or Rust) runs at maximum hardware speed because the types are locked in, but it can feel rigid and take more time to build.

**Elaichi solves this with AST Bifurcation.**

"Bifurcation" is just a fancy word for **splitting something into two branches**. When Elaichi reads your source code file (`.el`), it reads every function and automatically divides them into two separate execution tracks:

```text
                  Your Code File (.el)
                          │
                   Elaichi Compiler
                  (Reads the syntax)
                          │
                  The Bifurcation Split
                 ┌────────┴────────┐
                 ▼                 ▼
         Static Functions    Dynamic Functions
         (Marked 'static')   (Normal functions)
                 │                 │
                 ▼                 ▼
          [Static Stack]    [Dynamic Stack]
         (Direct CPU speed)  (Flexible VM)
                 │                 │
                 └────────┬────────┘
                          │
                  The Bridge Thunk
              (Converts values safely)
```

---

## 2. Track 1: The Static Stack (Raw Speed)

When you define a function with `static`, you are telling Elaichi: *"I know the exact numbers and types here. Don't waste time checking them at runtime."*

```javascript
static function calculateSum(a: i64, b: i64): i64 {
    return a + b;
}
```

* **What happens behind the scenes:**
  1. The compiler takes `a` and `b` and places them directly into computer CPU memory slots (registers).
  2. No memory garbage is created.
  3. The addition happens in a single CPU instruction cycle.

---

## 3. Track 2: The Dynamic Stack (Everyday Flexibility)

When you write a normal function without `static`, you get the convenience of everyday scripting:

```javascript
function greetUser(profile) {
    terminal.println("Welcome back, " + profile.name);
}
```

* **What happens behind the scenes:**
  1. The function compiles into bytecode instructions.
  2. Values are stored as tagged objects (`DynValue`) that can change freely.
  3. You can create lists, dictionaries, and nested objects without worrying about memory management.

---

## 4. The Bridge: How the Two Tracks Talk to Each Other

What happens when your flexible, dynamic code wants to use a high-speed static function?

```javascript
static function fastMath(n: i64): i64 {
    return n * 2;
}

function main() {
    let input = 21;
    // Calling a static function from normal code:
    let output = fastMath(input);
    terminal.println("Result: " + output);
}
```

Elaichi handles this through an **Inter-Stack Bridge**:
1. When `main()` calls `fastMath(input)`, the bridge checks that `input` is a valid integer.
2. It strips away the dynamic wrapper (unboxing) and passes the raw number directly into the CPU register.
3. The static function calculates `21 * 2 = 42` instantly.
4. The bridge wraps the result (`42`) and hands it right back to `main()`.

You never have to write messy conversion code or foreign function wrappers yourself—the compiler does it for you automatically.
