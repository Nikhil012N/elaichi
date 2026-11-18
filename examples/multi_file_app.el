import "./math_module.el";

function main() {
    terminal.println("=== Testing Multi-File Module Import ===");

    let h = computeHypot(3.0, 4.0);
    let s = sumRange(10);

    terminal.println("Hypot squared (3, 4): " + h);
    terminal.println("Sum range (1..10): " + s);

    if (h == 25.0 && s == 55) {
        terminal.println("[PASS] Multi-file import executed successfully!");
        return 0;
    } else {
        terminal.error("[FAIL] Multi-file import calculations failed.");
        return 1;
    }
}
