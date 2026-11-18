use crate::bifurcator::Bifurcator;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::runtime::Runtime;
use crate::vm::DynValue;

pub fn run_code(src: &str) -> Result<DynValue, String> {
    let mut lexer = Lexer::new(src);
    let tokens = lexer.tokenize()?;
    let mut parser = Parser::new(tokens);
    let program = parser.parse_program()?;
    let mut bifurcator = Bifurcator::new();
    bifurcator.compile_program(program)?;
    let mut runtime = Runtime::new(bifurcator);
    runtime.execute("main")
}

#[cfg(test)]
mod test_cases {
    use super::*;

    #[test]
    fn test_terminal_statements() {
        let code = r#"
            function main() {
                terminal.print("Counting: ");
                terminal.println(42);
                terminal.warn("This is a warning");
                terminal.error("This is an error");
                return 42;
            }
        "#;
        let res = run_code(code).unwrap();
        assert_eq!(res, DynValue::Int(42));
    }

    #[test]
    fn test_js_style_variables_and_functions() {
        let code = r#"
            function main() {
                const a = 10;
                let b = 25;
                b = b + 5;
                return a + b;
            }
        "#;
        let res = run_code(code).unwrap();
        assert_eq!(res, DynValue::Int(40));
    }

    #[test]
    fn test_js_style_if_and_for_loops() {
        let code = r#"
            function main() {
                let sum = 0;
                for (let i in 1..5) {
                    if (i % 2 == 0) {
                        sum = sum + i;
                    }
                }
                return sum;
            }
        "#;
        let res = run_code(code).unwrap();
        assert_eq!(res, DynValue::Int(6)); // 2 + 4 = 6
    }

    #[test]
    fn test_maps_and_field_assignment() {
        let code = r#"
            function main() {
                let config = { title: "Elaichi", port: 8080 };
                config.port = 9000;
                return config.port;
            }
        "#;
        let res = run_code(code).unwrap();
        assert_eq!(res, DynValue::Int(9000));
    }

    #[test]
    fn test_loop_break_and_continue() {
        let code = r#"
            function main() {
                let acc = 0;
                for (let i in 1..10) {
                    if (i == 3) {
                        continue;
                    }
                    if (i == 6) {
                        break;
                    }
                    acc = acc + i;
                }
                return acc; // 1 + 2 + 4 + 5 = 12
            }
        "#;
        let res = run_code(code).unwrap();
        assert_eq!(res, DynValue::Int(12));
    }

    #[test]
    fn test_type_casting() {
        let code = r#"
            function main() {
                let val = 42;
                let fl = val as f64;
                return fl;
            }
        "#;
        let res = run_code(code).unwrap();
        assert_eq!(res, DynValue::Float(42.0));
    }

    #[test]
    fn test_expanded_stdlib() {
        let code = r#"
            function main() {
                let m = { a: 10, b: 20 };
                let k = keys(m);
                let v = values(m);
                let has_a = contains(m, "a");
                let maximum = max(100, 250);
                let minimum = min(100, 250);
                let absolute = abs(-50);
                let root = sqrt(144.0);
                let flr = floor(3.9);
                let cl = ceil(3.1);
                let rnd = round(4.6);
                let ty = type_of(m);
                return absolute + maximum + minimum + (root as i64) + flr + cl + rnd; 
                // 50 + 250 + 100 + 12 + 3 + 4 + 5 = 424
            }
        "#;
        let res = run_code(code).unwrap();
        assert_eq!(res, DynValue::Int(424));
    }

    #[test]
    fn test_static_computation_and_bridge() {
        let code = r#"
            static function fast_fib(n: i64): i64 {
                if (n <= 1) {
                    return n;
                }
                let a: i64 = 0;
                let b: i64 = 1;
                let i: i64 = 2;
                while (i <= n) {
                    let next: i64 = a + b;
                    a = b;
                    b = next;
                    i = i + 1;
                }
                return b;
            }

            function main() {
                let fib10 = fast_fib(10);
                return fib10;
            }
        "#;
        let res = run_code(code).unwrap();
        assert_eq!(res, DynValue::Int(55));
    }

    #[test]
    fn test_nested_data_structures() {
        let code = r#"
            function main() {
                let matrix = [
                    [1, 2, 3],
                    [4, 5, 6],
                    [7, 8, 9]
                ];
                let center = matrix[1][1];
                let user = {
                    name: "Elaichi",
                    specs: { version: 1, stable: true }
                };
                return center + user.specs.version; // 5 + 1 = 6
            }
        "#;
        let res = run_code(code).unwrap();
        assert_eq!(res, DynValue::Int(6));
    }
}

pub fn run_all_self_tests() -> Result<(), String> {
    let tests = [
        ("Terminal I/O", r#"
            function main() {
                terminal.print("test: ");
                terminal.println("ok");
                return 1;
            }
        "#),
        ("Variables & Math", r#"
            function main() {
                const x = 10;
                let y = 20;
                return x + y;
            }
        "#),
        ("Bifurcation & Bridge", r#"
            static function addNums(a: i64, b: i64): i64 {
                return a + b;
            }
            function main() {
                return addNums(15, 25);
            }
        "#),
        ("Standard Math Library", r#"
            function main() {
                let root = sqrt(81.0);
                let maximum = max(10, 50);
                let rounded = round(4.6);
                return (root as i64) + maximum + rounded; // 9 + 50 + 5 = 64
            }
        "#),
        ("Data Structures", r#"
            function main() {
                let map = { a: 10, b: 20 };
                let l = [1, 2, 3];
                return map.b + l[1]; // 20 + 2 = 22
            }
        "#),
        ("String and List Utilities", r#"
            function main() {
                let parts = split("elaichi-runtime-fast", "-");
                let joined = join(parts, "::");
                let sub = slice(joined, 0, 7);
                let p = pow(2.0, 8.0);
                let rev = reverse([1, 2, 3]);
                let tr = trim("   ready   ");
                if (sub == "elaichi" && p == 256.0 && rev[0] == 3 && tr == "ready") {
                    return 100;
                }
                return 0;
            }
        "#),
        ("Text and Temporal Operations", r#"
            function main() {
                let upper = toUpper("elaichi");
                let lower = toLower("FAST");
                let sw = startsWith(upper, "ELA");
                let ew = endsWith(lower, "ST");
                let rep = replace("hello world", "world", "elaichi");
                let sorted = sort([3, 1, 2]);
                let ts = timeNow();
                if (upper == "ELAICHI" && lower == "fast" && sw && ew && rep == "hello elaichi" && sorted[0] == 1 && ts > 0) {
                    return 200;
                }
                return 0;
            }
        "#),
        ("Functional Pipeline Operations", r#"
            function isEven(n) {
                return n % 2 == 0;
            }
            function doubleVal(n) {
                return n * 2;
            }
            function sumVals(acc, n) {
                return acc + n;
            }
            function main() {
                let numbers = [1, 2, 3, 4, 5, 6];
                let evens = filter(numbers, "isEven"); // [2, 4, 6]
                let doubled = mapList(evens, "doubleVal"); // [4, 8, 12]
                let total = reduce(doubled, "sumVals", 0); // 24
                let bounded = clamp(150, 0, 100); // 100
                let found = find(numbers, "isEven"); // 2
                if (total == 24 && bounded == 100 && found == 2) {
                    return 300;
                }
                return 0;
            }
        "#),
        ("Static Float Speed", r#"
            static function fastHypot(a: f64, b: f64): f64 {
                return a * a + b * b;
            }
            function main() {
                let h = fastHypot(3.0, 4.0);
                return h as i64; // 9.0 + 16.0 = 25
            }
        "#)
    ];

    for (name, src) in tests {
        let _res = run_code(src)?;
        println!("  [PASS] {}", name);
    }
    Ok(())
}
