static function computeHypot(a: f64, b: f64): f64 {
    return a * a + b * b;
}

static function sumRange(n: i64): i64 {
    let total: i64 = 0;
    let i: i64 = 1;
    while (i <= n) {
        total = total + i;
        i = i + 1;
    }
    return total;
}
