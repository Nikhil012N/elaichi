# Language Reference

This reference is a complete guide to writing Elaichi code, from basic syntax to built-in helper tools.

---

## 1. Variables (`const` & `let`)

In Elaichi, every variable must be declared with either `const` or `let`:

* **`const`**: Use when the value should never change. Reassigning it causes an error.
  ```javascript
  const siteUrl = "https://example.com";
  ```
* **`let`**: Use when you need to change or update the value later.
  ```javascript
  let counter = 0;
  counter = counter + 1;
  ```

---

## 2. Primitive Types

When writing high-performance `static` functions, Elaichi asks you to specify the type of your data:

| Type | What it represents | Example |
| :--- | :--- | :--- |
| `i64` | Whole numbers (integers, positive or negative) | `42`, `-10`, `0` |
| `f64` | Decimal numbers (floating point) | `3.14`, `-0.5`, `100.0` |
| `bool` | True or False flags | `true`, `false` |
| `string` | Text characters inside quotes | `"Hello World"` |

**Example of declaring typed variables:**
```javascript
const maxSpeed: i64 = 120;
let temperature: f64 = 98.6;
let isReady: bool = true;
```

---

## 3. Functions

### Dynamic Functions (General Purpose)
Standard functions don't require you to declare data types:

```javascript
function greetUser(name) {
    terminal.println("Hello, " + name + "!");
}
```

### Static Functions (Maximum Speed)
Add `static` in front of the function and provide types for all inputs and the output. The computer runs this code at raw hardware speed:

```javascript
static function multiply(x: i64, y: i64): i64 {
    return x * y;
}
```

---

## 4. Control Flow

### `if` / `else if` / `else`
Parentheses `(...)` around conditions and curly braces `{...}` around blocks are always required:

```javascript
let balance = 150;

if (balance > 100) {
    terminal.println("Sufficient funds");
} else if (balance > 0) {
    terminal.println("Low funds");
} else {
    terminal.println("Account empty");
}
```

### `while` Loops
Repeats code as long as a condition remains true:

```javascript
let n = 5;
while (n > 0) {
    terminal.println("Remaining: " + n);
    n = n - 1;
}
```

### `for` Range Loops
Iterate over a sequence of numbers using `start..end`:

```javascript
// Prints 0, 1, 2, 3, 4
for (let i in 0..5) {
    terminal.println("Item " + i);
}
```

### Controlling Loops (`break` & `continue`)
* **`break;`**: Stops the loop immediately.
* **`continue;`**: Skips the rest of the current loop turn and moves to the next one.

```javascript
for (let i in 1..10) {
    if (i == 4) {
        continue; // skip number 4
    }
    if (i == 8) {
        break;    // stop completely when reaching 8
    }
    terminal.println(i);
}
```

---

## 5. Collections (Lists & Dictionaries)

### Lists (Arrays)
Store ordered collections of data:

```javascript
let colors = ["red", "green", "blue"];

// Read an element (starts at index 0)
terminal.println(colors[0]); // red

// Update an element
colors[1] = "yellow";
terminal.println(colors[1]); // yellow
```

### Dictionaries (Objects)
Store data as key-value pairs. You can read and change fields using dot notation:

```javascript
let server = {
    host: "localhost",
    port: 8080,
    online: true
};

// Accessing fields
terminal.println("Server running on port: " + server.port);

// Modifying fields in place
server.port = 9000;
terminal.println("Port changed to: " + server.port);
```

---

## 6. Type Conversion (`as`)

Convert a value from one type to another explicitly using the `as` keyword:

```javascript
let count = 42;
let asDecimal = count as f64;      // converts to 42.0
let asText = count as string;       // converts to "42"
```

---

## 7. The Terminal System

Instead of `console.log`, Elaichi provides native `terminal` commands:

* `terminal.print("text")`: Prints text on the current line (no newline at the end).
* `terminal.println("text")`: Prints text and moves to the next line.
* `terminal.warn("alert")`: Prints an alert with a warning tag.
* `terminal.error("message")`: Prints an error message.
* `terminal.clear()`: Clears the terminal screen.

---

## 8. Built-in Helper Functions

Elaichi standardizes all identifiers using **camelCase** for variables and functions, and **PascalCase** for Types and Structs:

| Function | Description | Example |
| :--- | :--- | :--- |
| `typeOf(val)` | Returns the runtime type as a string | `typeOf(42)` -> `"int"` |
| `toUpper(str)` | Converts string to uppercase | `toUpper("hello")` -> `"HELLO"` |
| `toLower(str)` | Converts string to lowercase | `toLower("WORLD")` -> `"world"` |
| `startsWith(str, prefix)` | Checks string prefix | `startsWith("elaichi", "el")` -> `true` |
| `endsWith(str, suffix)` | Checks string suffix | `endsWith("elaichi", "chi")` -> `true` |
| `replace(str, from, to)` | Replaces occurrences in string | `replace("a-b", "-", "+")` -> `"a+b"` |
| `split(str, sep)` | Splits string into a list | `split("a,b", ",")` -> `["a", "b"]` |
| `join(list, sep)` | Joins list items into string | `join(["a", "b"], ":")` -> `"a:b"` |
| `slice(coll, start, end)` | Slices a list or string | `slice([1, 2, 3], 0, 2)` -> `[1, 2]` |
| `reverse(coll)` | Reverses a list or string | `reverse([1, 2, 3])` -> `[3, 2, 1]` |
| `trim(str)` | Removes leading/trailing spaces | `trim("  hi  ")` -> `"hi"` |
| `sort(list)` | Sorts a list in ascending order | `sort([3, 1, 2])` -> `[1, 2, 3]` |
| `timeNow()` | Current timestamp in milliseconds | `timeNow()` -> `1700000000000` |
| `sqrt(n)` | Computes square root | `sqrt(144.0)` -> `12.0` |
| `pow(base, exp)` | Computes base to exponent power | `pow(2.0, 8.0)` -> `256.0` |
| `floor(n)` | Truncates to lower integer | `floor(3.9)` -> `3` |
| `ceil(n)` | Rounds up to upper integer | `ceil(3.1)` -> `4` |
| `round(n)` | Rounds to nearest integer | `round(4.6)` -> `5` |
| `abs(n)` | Turns negative numbers into positive | `abs(-5)` -> `5` |
| `min(a, b)` | Returns the smaller of two numbers | `min(10, 20)` -> `10` |
| `max(a, b)` | Returns the larger of two numbers | `max(10, 20)` -> `20` |
| `len(coll)` | Returns count of items or length | `len([1, 2])` -> `2` |
| `push(list, item)` | Adds an item to the end of a list | `push([1], 2)` -> `[1, 2]` |
| `filter(list, fnName)` | Filters elements matching predicate function | `filter([1, 2], "isEven")` -> `[2]` |
| `mapList(list, fnName)` | Maps list elements through transform function | `mapList([1, 2], "doubleNum")` -> `[2, 4]` |
| `reduce(list, fnName, initial)` | Accumulates list into a single value | `reduce([1, 2, 3], "sum", 0)` -> `6` |
| `clamp(val, min, max)` | Constrains value between min and max bounds | `clamp(150, 0, 100)` -> `100` |
| `find(list, fnName)` | Returns first element matching predicate | `find([1, 2], "isEven")` -> `2` |
| `keys(dict)` | Returns list of key names in a map | `keys(user)` -> `["id", "name"]` |
| `values(dict)` | Returns list of values in a map | `values(user)` -> `[1, "Alex"]` |
| `contains(coll, item)` | Checks item or substring membership | `contains(list, "a")` -> `true` |
