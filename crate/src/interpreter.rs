use crate::model::{Expression, Literal};

type EvalResult = Result<Literal, String>;

#[derive(Default)]
pub struct EvalContext;

fn evaluate(expr: Expression) -> EvalResult {
    EvalContext::default().evaluate(expr)
}

impl EvalContext {
    pub fn evaluate(&self, expr: Expression) -> EvalResult {
        match expr {
            Expression::Lit(literal) => Ok(literal),
            Expression::Neg(e) => match evaluate(*e)? {
                Literal::I64(x) => Ok(Literal::I64(-x)),
                Literal::F64(x) => Ok(Literal::F64(-x)),
                _ => Err(String::from("invalid types")),
            },
            Expression::Not(e) => match evaluate(*e)? {
                Literal::Bool(x) => Ok(Literal::Bool(!x)),
                _ => Err(String::from("invalid types")),
            },
            Expression::Eq(a, b) => {
                let a = evaluate(*a)?;
                let b = evaluate(*b)?;
                match (a, b) {
                    (Literal::I64(a), Literal::I64(b)) => Ok(Literal::Bool(a == b)),
                    (Literal::String(a), Literal::String(b)) => Ok(Literal::Bool(a == b)),
                    (Literal::Bytes(a), Literal::Bytes(b)) => Ok(Literal::Bool(a == b)),
                    _ => Err(String::from("invalid types")),
                }
            }
            Expression::Neq(a, b) => evaluate(Expression::Not(Box::new(Expression::Eq(a, b)))),
            Expression::Lt(a, b) => {
                let a = evaluate(*a)?;
                let b = evaluate(*b)?;
                match (a, b) {
                    (Literal::I64(a), Literal::I64(b)) => Ok(Literal::Bool(a < b)),
                    (Literal::String(a), Literal::String(b)) => Ok(Literal::Bool(a < b)),
                    (Literal::Bytes(a), Literal::Bytes(b)) => Ok(Literal::Bool(a < b)),
                    _ => Err(String::from("invalid types")),
                }
            }
            Expression::Lte(a, b) => {
                let a = evaluate(*a)?;
                let b = evaluate(*b)?;
                match (a, b) {
                    (Literal::I64(a), Literal::I64(b)) => Ok(Literal::Bool(a <= b)),
                    (Literal::String(a), Literal::String(b)) => Ok(Literal::Bool(a <= b)),
                    (Literal::Bytes(a), Literal::Bytes(b)) => Ok(Literal::Bool(a <= b)),
                    _ => Err(String::from("invalid types")),
                }
            }
            Expression::Gte(a, b) => evaluate(Expression::Not(Box::new(Expression::Lt(a, b)))),
            Expression::Gt(a, b) => evaluate(Expression::Not(Box::new(Expression::Lte(a, b)))),
            Expression::Add(a, b) => {
                let a = evaluate(*a)?;
                let b = evaluate(*b)?;
                match (a, b) {
                    (Literal::I64(a), Literal::I64(b)) => Ok(Literal::I64(a + b)),
                    (Literal::F64(a), Literal::F64(b)) => Ok(Literal::F64(a + b)),
                    (Literal::String(a), Literal::String(b)) => {
                        Ok(Literal::String(a.chars().chain(b.chars()).collect()))
                    }
                    _ => Err(String::from("invalid types")),
                }
            }
            Expression::Sub(a, b) => {
                let a = evaluate(*a)?;
                let b = evaluate(*b)?;
                match (a, b) {
                    (Literal::I64(a), Literal::I64(b)) => Ok(Literal::I64(a - b)),
                    _ => Err(String::from("invalid types")),
                }
            }
            Expression::Mul(a, b) => {
                let a = evaluate(*a)?;
                let b = evaluate(*b)?;
                match (a, b) {
                    (Literal::I64(a), Literal::I64(b)) => Ok(Literal::I64(a * b)),
                    (Literal::F64(a), Literal::F64(b)) => Ok(Literal::F64(a * b)),
                    _ => Err(String::from("invalid types")),
                }
            }
            Expression::Div(a, b) => {
                let a = evaluate(*a)?;
                let b = evaluate(*b)?;
                match (a, b) {
                    (Literal::I64(a), Literal::I64(b)) => {
                        if b != 0 {
                            Ok(Literal::I64(a / b))
                        } else {
                            Err(String::from("divide by zero"))
                        }
                    }
                    (Literal::F64(a), Literal::F64(b)) => {
                        if b != 0.0 {
                            Ok(Literal::F64(a / b))
                        } else {
                            Err(String::from("divide by zero"))
                        }
                    }
                    _ => Err(String::from("invalid types")),
                }
            }
            Expression::Mod(a, b) => {
                let a = evaluate(*a)?;
                let b = evaluate(*b)?;
                match (a, b) {
                    (Literal::I64(a), Literal::I64(b)) => Ok(Literal::I64(a % b)),
                    _ => Err(String::from("invalid types")),
                }
            }
            Expression::Binding(name) => Err(format!("no such binding: {:?}", name)),
            Expression::Member(e, name) => {
                let e = evaluate(*e)?;
                Err(format!("method {:?} not implemented on {:?}", name, e))
            }
            Expression::Method(e, name, args) => Err(format!("method {:?} not implemented", name)),
        }
    }
}

#[cfg(test)]
mod test {
    use super::evaluate;
    use crate::model::Literal;
    use crate::parser::parse;

    fn assert_eval_true(input: &str) {
        assert_eq!(
            evaluate(parse(input).unwrap()).unwrap(),
            Literal::Bool(true)
        );
    }

    #[test]
    fn smoke() {
        let input = r#" 1 + 2 + 3 + 4 + 5 "#;
        assert_eq!(
            evaluate(parse(input).unwrap()),
            Ok(Literal::I64(1 + 2 + 3 + 4 + 5)),
        );
    }

    #[test]
    fn unary_negative_i64() {
        let input = r#" -5 + 8 "#;
        assert_eq!(evaluate(parse(input).unwrap()), Ok(Literal::I64(3)),);
    }

    #[test]
    fn unary_negative_f64() {
        let input = r#" -5.1 + 8.1 "#;
        assert_eq!(evaluate(parse(input).unwrap()), Ok(Literal::F64(3.0)),);
    }

    #[test]
    fn unary_not() {
        let input = r#" !(5 + 5 == 10) "#;
        assert_eq!(evaluate(parse(input).unwrap()), Ok(Literal::Bool(false)),);
    }

    #[test]
    fn string_addition() {
        let input = r#" "asdf" + "pqrs" + "tuvw" == "asdfpqrstuvw" "#;
        assert_eq!(evaluate(parse(input).unwrap()), Ok(Literal::Bool(true)),);
    }

    #[test]
    fn addition_and_subtraction() {
        let input = r#" 1 - 2 + 3 - 4 + 5 "#;
        assert_eq!(
            evaluate(parse(input).unwrap()),
            Ok(Literal::I64(1 - 2 + 3 - 4 + 5)),
        );
    }

    #[test]
    fn multiplication_and_division() {
        let input = r#" 1.0 / 2.0 * 3.0 / 4.0 * 5.0 "#;
        assert_eq!(
            evaluate(parse(input).unwrap()),
            Ok(Literal::F64(1.0 / 2.0 * 3.0 / 4.0 * 5.0)),
        );
    }

    #[test]
    fn string_len() {
        let input = r#" "asdf".len() + "pqrs".len() "#;
        assert_eq!(evaluate(parse(input).unwrap()), Ok(Literal::I64(8)),);
    }

    #[test]
    fn bytes_len() {
        let input = r#" b"\xFF".len() "#;
        assert_eq!(evaluate(parse(input).unwrap()), Ok(Literal::I64(1)),);
    }

    #[test]
    fn bytes_eq() {
        let input = r#" b"¢" == b'\xC2\xA2' "#;
        assert_eq!(evaluate(parse(input).unwrap()), Ok(Literal::Bool(true)));
    }

    #[test]
    fn bytes_cmp() {
        assert_eval_true(r#" b"\x00" < b"\x01" "#);
        assert_eval_true(r#" b"asdf" < b"pqrs" "#);
        assert_eval_true(r#" b"" < b"asdf" "#);
        assert_eval_true(r#" b"\xFE\xFF\xFF\xFF\xFF" < b"\xFF" "#);
    }

    #[test]
    fn int_pow() {
        let input = r#" 42.pow(2) "#;
        assert_eq!(evaluate(parse(input).unwrap()), Ok(Literal::I64(42 * 42)),);
    }

    #[test]
    fn float_powf() {
        let input = r#" 3.1415926.pow(3.1415926) "#;
        assert_eq!(
            evaluate(parse(input).unwrap()),
            Ok(Literal::F64(3.1415926f64.powf(3.1415926))),
        );
    }

    #[test]
    fn float_powi() {
        let input = r#" 3.1415926.pow(2) "#;
        assert_eq!(
            evaluate(parse(input).unwrap()),
            Ok(Literal::F64(3.1415926f64.powf(2.0))),
        );
    }

    #[test]
    fn list_len() {
        let input = r#" ["a", 3, false].len() "#;
        assert_eq!(evaluate(parse(input).unwrap()), Ok(Literal::I64(3)),);
    }

    #[test]
    fn list_contains_true() {
        let input = r#" ["a", 3, false].contains(3) "#;
        assert_eq!(evaluate(parse(input).unwrap()), Ok(Literal::Bool(true)),);
    }

    #[test]
    fn list_contains_false() {
        let input = r#" ["a", 3, false].contains(4) "#;
        assert_eq!(evaluate(parse(input).unwrap()), Ok(Literal::Bool(false)),);
    }

    #[test]
    fn list_contains_true_with_error() {
        let input = r#" ["a", 3, 1 / 0].contains(3) "#;
        assert_eq!(evaluate(parse(input).unwrap()), Ok(Literal::Bool(true)),);
    }

    #[test]
    fn list_contains_false_with_error() {
        let input = r#" ["a", 3, 1 / 0].contains(2) "#;
        assert_eq!(
            evaluate(parse(input).unwrap()),
            Err(String::from("divide by zero")),
        );
    }

    #[test]
    fn list_contains_error() {
        let input = r#" ["a", 3, false].contains(1 / 0) "#;
        assert_eq!(
            evaluate(parse(input).unwrap()),
            Err(String::from("divide by zero")),
        );
    }

    #[test]
    fn type_error_adding_string_and_int() {
        let input = r#" "asdf" + 5 "#;
        assert_eq!(
            evaluate(parse(input).unwrap()),
            Err(String::from("invalid types"))
        );
    }

    #[test]
    fn type_error_subtracting_strings() {
        let input = r#" "asdf" - "pqrs" "#;
        assert_eq!(
            evaluate(parse(input).unwrap()),
            Err(String::from("invalid types"))
        );
    }

    #[test]
    fn eval_error_divide_by_zero_int() {
        let input = r#" 1 / 0 "#;
        assert_eq!(
            evaluate(parse(input).unwrap()),
            Err(String::from("divide by zero"))
        );
    }
}
