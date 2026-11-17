// examples/app.el - JavaScript-style Syntax in Elaichi

// 1. High-performance unboxed Static Function (with static TypeScript-style types)
static function calculatePhysics(delta: f64, steps: i64): f64 {
    const rate: f64 = delta * 1.5;
    return rate;
}

// 2. Standard Dynamic Function (clean JavaScript structure)
function main() {
    console.log("=== Elaichi JavaScript-style Engine ===");

    // Use `const` for immutable variables
    const maxItems = 5;

    // Use `let` for mutable variables (NO `var`)
    let scores = [10, 20, 30];
    scores[1] = 42;

    console.log("Updated list element:");
    console.log(scores[1]);

    // For loops require proper parentheses (let i in ...) and braces { ... }
    let total = 0;
    for (let i in 1..5) {
        // If statements require proper condition parentheses ( ... )
        if (i % 2 == 0) {
            total = total + i;
        } else {
            total = total + 1;
        }
    }

    console.log("Calculated total:");
    console.log(total);

    // Dynamic to Static Bridge call
    const result = calculatePhysics(0.016, 60);
    console.log("Static stack physics calculation:");
    console.log(result);
}
