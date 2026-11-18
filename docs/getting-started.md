# Getting Started

This guide walks you step-by-step through installing Elaichi globally, writing code, and compiling or running your programs.

---

## 1. Quick Global Installation

Elaichi comes with a zero-friction global installer for Windows, macOS, and Linux.

### Windows (PowerShell or Command Prompt)
Run the one-click installer script in the Elaichi root directory:
```powershell
# Using PowerShell
.\install.ps1

# Or using Command Prompt (.bat)
install.bat
```

### Via npm (Universal)
If you have Node.js / npm installed:
```bash
npm install -g .
```

Once installed, the `elaichi` command is available everywhere in your terminal. Verify with:
```bash
elaichi --version
```
*Output:*
```text
Elaichi v0.1.0 (Dual-Stack Engine)
```

---

## 2. Writing Your First Program

Create a file named `hello.el`:

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

## 3. Running Your Program

Execute your `.el` file using the `run` command:

```bash
elaichi run hello.el
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

## 4. Native Compilation Commands

Elaichi includes multiple compilation targets to achieve bare-metal C and Rust speed:

### Compile to LLVM IR (Machine Code Backend)
Turn your unboxed code into typed SSA LLVM IR for native execution via LLVM/inkwell:
```bash
elaichi emit-llvm hello.el
```

### Compile to Optimized C99
Generate clean, vectorized C99 source code for Clang or GCC `-O3` compilation:
```bash
elaichi emit-c hello.el
```

### Inspect Bifurcation Classification
See how the compiler partitions your functions between the high-speed static stack and the dynamic stack:
```bash
elaichi dump-ast hello.el
```

---

## 5. Editor Support: Language Server Protocol (LSP)

Elaichi includes a built-in JSON-RPC 2.0 LSP daemon. To configure your editor (VS Code, Neovim, Helix, Sublime Text) to automatically validate types and diagnostics:

```bash
elaichi lsp
```
Configure your editor's LSP client to spawn `elaichi lsp` on `.el` files.
