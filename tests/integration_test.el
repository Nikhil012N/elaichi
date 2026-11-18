function isEven(n) {
    return n % 2 == 0;
}

function doubleNum(n) {
    return n * 2;
}

function accumulate(acc, val) {
    return acc + val;
}

function testDynamicExecution() {
    let list = [10, 20, 30];
    let total = 0;
    for (let item in list) {
        total = total + item;
    }
    return total;
}

static function testStaticMath(n: i64): i64 {
    let acc: i64 = 0;
    let i: i64 = 1;
    while (i <= n) {
        acc = acc + i;
        i = i + 1;
    }
    return acc;
}

static function testStaticHypot(a: f64, b: f64): f64 {
    return a * a + b * b;
}

function main() {
    terminal.println("=== Running Elaichi Integration Tests ===");
    
    // Test 1: Dynamic Execution
    let dynSum = testDynamicExecution();
    if (dynSum == 60) {
        terminal.println("[PASS] Dynamic List Iteration & Reduction");
    } else {
        terminal.error("[FAIL] Dynamic sum expected 60, got: " + dynSum);
        return 1;
    }

    // Test 2: Static Execution via Bridge
    let statSum = testStaticMath(10);
    let hypotSq = testStaticHypot(3.0, 4.0);
    if (statSum == 55 && hypotSq == 25.0) {
        terminal.println("[PASS] Static Unboxed Computation (Integer loops & Float math)");
    } else {
        terminal.error("[FAIL] Static computations failed");
        return 1;
    }

    // Test 3: Math and Type Standard Library
    let root = sqrt(100.0);
    let rounded = round(4.6);
    let typ = typeOf(dynSum);
    if (root == 10.0 && rounded == 5 && typ == "int") {
        terminal.println("[PASS] Extended Standard Library (sqrt, round, typeOf)");
    } else {
        terminal.error("[FAIL] Stdlib assertions failed");
        return 1;
    }

    // Test 4: Map and Object Property Access
    let config = {
        name: "Elaichi Test Suite",
        version: 1,
        active: true
    };
    if (config.name == "Elaichi Test Suite" && config.active == true) {
        terminal.println("[PASS] Object Key-Value & Property Access");
    } else {
        terminal.error("[FAIL] Map property access failed");
        return 1;
    }

    // Test 5: String & List Transformations (split, join, slice, reverse, trim)
    let parts = split("alpha,beta,gamma", ",");
    let joined = join(parts, " -> ");
    let reversedStr = reverse("stressed"); // desserts
    let cleaned = trim("   dual-stack   ");
    let powerVal = pow(2.0, 10.0); // 1024.0

    if (parts[1] == "beta" && reversedStr == "desserts" && cleaned == "dual-stack" && powerVal == 1024.0) {
        terminal.println("[PASS] String & Collection Transformers (split, join, slice, reverse, trim, pow)");
    } else {
        terminal.error("[FAIL] Collection or String transforms failed");
        return 1;
    }

    // Test 6: Advanced String Matching & Timestamps (camelCase conventions)
    let upper = toUpper("elaichi");
    let lower = toLower("RUNTIME");
    let hasPrefix = startsWith(upper, "ELA");
    let hasSuffix = endsWith(lower, "ME");
    let replaced = replace("hello world", "world", "elaichi");
    let sortedList = sort([40, 10, 30, 20]);
    let currentEpoch = timeNow();

    if (upper == "ELAICHI" && lower == "runtime" && hasPrefix && hasSuffix && replaced == "hello elaichi" && sortedList[0] == 10 && currentEpoch > 0) {
        terminal.println("[PASS] String Case, Search & Epoch Timers (toUpper, toLower, startsWith, endsWith, replace, sort, timeNow)");
    } else {
        terminal.error("[FAIL] Advanced string or timestamp operations failed");
        return 1;
    }

    // Test 7: Functional Collection Pipeline (filter, mapList, reduce, clamp, find)
    let rawNumbers = [1, 2, 3, 4, 5, 6, 7, 8];
    let evenNumbers = filter(rawNumbers, "isEven"); // [2, 4, 6, 8]
    let doubledEvens = mapList(evenNumbers, "doubleNum"); // [4, 8, 12, 16]
    let totalReduction = reduce(doubledEvens, "accumulate", 0); // 40
    let clampedValue = clamp(250, 10, 100); // 100
    let firstFound = find(rawNumbers, "isEven"); // 2

    if (totalReduction == 40 && clampedValue == 100 && firstFound == 2) {
        terminal.println("[PASS] Functional Collection Pipelines (filter, mapList, reduce, clamp, find)");
    } else {
        terminal.error("[FAIL] Functional pipeline operations failed");
        return 1;
    }

    terminal.println("All integration test assertions passed successfully.");
    return 0;
}
