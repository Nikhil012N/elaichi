# Getting Started

This guide walks you step-by-step through setting up Elaichi on your computer and running your first program.

---

## 1. What You Need

Elaichi compiles to a single, standalone executable file (`elaichi.exe` on Windows, or `elaichi` on Linux/Mac). 

If you want to build it from the source code, you only need the standard Rust tool:
* [Download Rust](https://www.rust-lang.org/tools/install) (this includes `cargo` and `rustc`).

---

## 2. Compiling the Elaichi Runner

1. Open your terminal or command prompt and clone the repository:
   ```bash
   git clone https://github.com/Nikhil012N/elaichi.git
   cd elaichi
   ```

2. Build the project using Cargo:
   ```bash
   cargo build --release
   ```

3. Your standalone binary is now ready inside `target/release/`:
   * **Windows**: `target/release/elaichi.exe`
   * **Linux & macOS**: `target/release/elaichi`

---

## 3. Writing Your First Program

Create a new file called `hello.el` in your project folder:

```javascript
function main() {
    terminal.println("Welcome to Elaichi!");

    let count = 3;
    while (count > 0) {
        terminal.println("Countdown: " + count);
        count = count - 1;
    }

    terminal.println("Liftoff!");
    return 0;
}
```

---

## 4. Running Your Program

Use the `elaichi run` command followed by the filename:

```bash
# Windows
.\target\release\elaichi.exe run hello.el

# Linux & macOS
./target/release/elaichi run hello.el
```

**Output:**
```text
Welcome to Elaichi!
Countdown: 3
Countdown: 2
Countdown: 1
Liftoff!
```

---

## 5. Helpful Commands

### Check the Version
```bash
elaichi --version
```

### See How the Compiler Splits Your Code
Elaichi includes a command called `dump-ast` that lets you see which functions are running in high-speed static mode versus normal dynamic mode:
```bash
elaichi dump-ast hello.el
```
