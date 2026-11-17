// examples/terminal_demo.el - Demonstrating Elaichi's first-class terminal I/O

static function calculateFactorial(n: i64): i64 {
    let mut total: i64 = 1;
    let mut current: i64 = n;
    while (current > 1) {
        total = total * current;
        current = current - 1;
    }
    return total;
}

function main() {
    // 1. terminal.clear() to clear the screen
    // terminal.clear();

    // 2. terminal.println() - prints with a newline
    terminal.println("=== Elaichi Terminal I/O Engine ===");

    // 3. terminal.print() - prints inline without a trailing newline
    terminal.print("Processing items: ");
    for (let i in 1..4) {
        terminal.print(i);
        terminal.print(".. ");
    }
    terminal.println("Done!");

    // 4. Structured diagnostics
    const limit = 100;
    let usage = 85;

    if (usage > 80) {
        terminal.warn("Memory threshold high: usage at 85%");
    }

    if (usage >= limit) {
        terminal.error("Process limit exceeded!");
    }

    // 5. Calling static routine and printing output
    const fact = calculateFactorial(5);
    terminal.print("Factorial of 5 (calculated on static stack): ");
    terminal.println(fact);
}
