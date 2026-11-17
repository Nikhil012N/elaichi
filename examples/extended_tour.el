// examples/extended_tour.el - Comprehensive Tour of Extended Elaichi Features

// 1. Static Struct Definition
struct Vector2 {
    x: i64,
    y: i64,
}

// 2. High-performance unboxed static function
static function dot_product(x1: i64, y1: i64, x2: i64, y2: i64): i64 {
    const dx: i64 = x1 * x2;
    const dy: i64 = y1 * y2;
    return dx + dy;
}

// 3. Dynamic coordinator
function main() {
    terminal.println("=== Elaichi Feature Showcase ===");

    // A. Lists, Maps, and Field Mutation
    const scores = [85, 92, 78, 96];
    terminal.println("Scores count: " + scores.length);

    let system_node = {
        name: "Cluster-Alpha",
        port: 8080,
        active: true
    };
    terminal.println("Initial Node Port: " + system_node.port);
    system_node.port = 9000;
    terminal.println("Updated Node Port: " + system_node.port);

    // B. Loop control: break and continue
    let evens_sum = 0;
    for (let i in 1..20) {
        if (i % 2 != 0) {
            continue;
        }
        if (i > 10) {
            break;
        }
        evens_sum = evens_sum + i;
    }
    terminal.println("Sum of even integers up to 10: " + evens_sum);

    // C. Explicit Type Casting (as f64)
    const raw_val = 15;
    const ratio = (raw_val as f64) / 2.0;
    terminal.println("Casted ratio: " + ratio);

    // D. Seamless Bridge into Static Stack
    const dp = dot_product(3, 4, 2, 5);
    terminal.println("Static engine dot product: " + dp);

    terminal.println("Execution completed successfully.");
    return 0;
}
