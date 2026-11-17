#[cfg(test)]
mod tests {
    use crate::bifurcator::Bifurcator;
    use crate::lexer::Lexer;
    use crate::parser::Parser;
    use crate::runtime::Runtime;
    use crate::vm::DynValue;

    fn run_code(src: &str) -> Result<DynValue, String> {
        let mut lexer = Lexer::new(src);
        let tokens = lexer.tokenize()?;
        let mut parser = Parser::new(tokens);
        let program = parser.parse_program()?;
        let mut bifurcator = Bifurcator::new();
        bifurcator.compile_program(program)?;
        let mut runtime = Runtime::new(bifurcator);
        runtime.execute("main")
    }

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
                return absolute + maximum + minimum; // 50 + 250 + 100 = 400
            }
        "#;
        let res = run_code(code).unwrap();
        assert_eq!(res, DynValue::Int(400));
    }
}
