// test_math.el - Testing Elaichi Dual-Stack Execution

// 1. Pure static function: runs on the native unboxed static stack
static fn add_fast(a: i64, b: i64) -> i64 {
    let result: i64 = a + b;
    return result;
}

// 2. High-level dynamic function: runs on the dynamic stack VM
fn main() {
    print("Welcome to Elaichi! [Dual-Stack Engine]");

    // Dynamic variables
    var x = 15;
    var y = 27;

    // Crossing the bridge:
    // Dynamic values (x, y) are coerced and passed to static fn add_fast()
    var sum = add_fast(x, y);

    print("Result computed across the Inter-Stack Bridge:");
    print(sum);
}
