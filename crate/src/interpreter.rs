use std::collections::HashMap;

use crate::methods;
use crate::model::{Error, EvalResult, Expression, Identifier, Literal, Op, Value};
use std::cmp::Ordering;

#[derive(Default)]
pub struct EvalContext {
    bindings: HashMap<Identifier, Value>,
}

impl EvalContext {
    pub fn evaluate(&self, expr: Expression) -> EvalResult {
        match expr {
            Expression::Lit(lit) => self.evaluate_literal(lit),
            Expression::Neg(e) => match self.evaluate(*e)? {
                Value::I64(x) => Ok(Value::I64(-x)),
                Value::F64(x) => Ok(Value::F64(-x)),
                other => Err(Error::InvalidTypeForOperator(other.kind(), Op::Neg)),
            },
            Expression::Not(e) => match self.evaluate(*e)? {
                Value::Bool(x) => Ok(Value::Bool(!x)),
                other => Err(Error::InvalidTypeForOperator(other.kind(), Op::Not)),
            },
            Expression::Or(a, b) => {
                let a = match self.evaluate(*a) {
                    Ok(Value::Bool(true)) => return Ok(Value::Bool(true)),
                    other => other,
                };
                let b = match self.evaluate(*b) {
                    Ok(Value::Bool(true)) => return Ok(Value::Bool(true)),
                    other => other,
                };
                let a = a?;
                let b = b?;
                match (a, b) {
                    (Value::Bool(a), Value::Bool(b)) => Ok(Value::Bool(a || b)),
                    (a, b) => Err(Error::InvalidTypesForOperator(a.kind(), b.kind(), Op::Or)),
                }
            }
            Expression::And(a, b) => {
                let a = match self.evaluate(*a) {
                    Ok(Value::Bool(false)) => return Ok(Value::Bool(false)),
                    other => other,
                };
                let b = match self.evaluate(*b) {
                    Ok(Value::Bool(false)) => return Ok(Value::Bool(false)),
                    other => other,
                };
                let a = a?;
                let b = b?;
                match (a, b) {
                    (Value::Bool(a), Value::Bool(b)) => Ok(Value::Bool(a && b)),
                    (a, b) => Err(Error::InvalidTypesForOperator(a.kind(), b.kind(), Op::And)),
                }
            }
            Expression::Eq(a, b) => {
                let a = self.evaluate(*a)?;
                let b = self.evaluate(*b)?;
                match a.partial_cmp(&b) {
                    None => Err(Error::InvalidTypesForOperator(a.kind(), b.kind(), Op::Eq)),
                    Some(ord) => Ok(Value::Bool(ord == Ordering::Equal)),
                }
            }
            Expression::Neq(a, b) => {
                let a = self.evaluate(*a)?;
                let b = self.evaluate(*b)?;
                match a.partial_cmp(&b) {
                    None => Err(Error::InvalidTypesForOperator(a.kind(), b.kind(), Op::Neq)),
                    Some(ord) => Ok(Value::Bool(ord != Ordering::Equal)),
                }
            }
            Expression::Lt(a, b) => {
                let a = self.evaluate(*a)?;
                let b = self.evaluate(*b)?;
                match a.partial_cmp(&b) {
                    None => Err(Error::InvalidTypesForOperator(a.kind(), b.kind(), Op::Lt)),
                    Some(ord) => Ok(Value::Bool(ord == Ordering::Less)),
                }
            }
            Expression::Lte(a, b) => {
                let a = self.evaluate(*a)?;
                let b = self.evaluate(*b)?;
                match a.partial_cmp(&b) {
                    None => Err(Error::InvalidTypesForOperator(a.kind(), b.kind(), Op::Lte)),
                    Some(ord) => Ok(Value::Bool(ord == Ordering::Less || ord == Ordering::Equal)),
                }
            }
            Expression::Gte(a, b) => {
                let a = self.evaluate(*a)?;
                let b = self.evaluate(*b)?;
                match a.partial_cmp(&b) {
                    None => Err(Error::InvalidTypesForOperator(a.kind(), b.kind(), Op::Gte)),
                    Some(ord) => Ok(Value::Bool(
                        ord == Ordering::Greater || ord == Ordering::Equal,
                    )),
                }
            }
            Expression::Gt(a, b) => {
                let a = self.evaluate(*a)?;
                let b = self.evaluate(*b)?;
                match a.partial_cmp(&b) {
                    None => Err(Error::InvalidTypesForOperator(a.kind(), b.kind(), Op::Gt)),
                    Some(ord) => Ok(Value::Bool(ord == Ordering::Greater)),
                }
            }
            Expression::Add(a, b) => {
                let a = self.evaluate(*a)?;
                let b = self.evaluate(*b)?;
                match (a, b) {
                    (Value::I64(a), Value::I64(b)) => Ok(Value::I64(a + b)),
                    (Value::F64(a), Value::F64(b)) => Ok(Value::F64(a + b)),
                    (Value::String(a), Value::String(b)) => {
                        Ok(Value::String(a.chars().chain(b.chars()).collect()))
                    }
                    (a, b) => Err(Error::InvalidTypesForOperator(a.kind(), b.kind(), Op::Plus)),
                }
            }
            Expression::Sub(a, b) => {
                let a = self.evaluate(*a)?;
                let b = self.evaluate(*b)?;
                match (a, b) {
                    (Value::I64(a), Value::I64(b)) => Ok(Value::I64(a - b)),
                    (a, b) => Err(Error::InvalidTypesForOperator(
                        a.kind(),
                        b.kind(),
                        Op::Minus,
                    )),
                }
            }
            Expression::Mul(a, b) => {
                let a = self.evaluate(*a)?;
                let b = self.evaluate(*b)?;
                match (a, b) {
                    (Value::I64(a), Value::I64(b)) => Ok(Value::I64(a * b)),
                    (Value::F64(a), Value::F64(b)) => Ok(Value::F64(a * b)),
                    (a, b) => Err(Error::InvalidTypesForOperator(
                        a.kind(),
                        b.kind(),
                        Op::Times,
                    )),
                }
            }
            Expression::Div(a, b) => {
                let a = self.evaluate(*a)?;
                let b = self.evaluate(*b)?;
                match (a, b) {
                    (Value::I64(a), Value::I64(b)) => {
                        if b != 0 {
                            Ok(Value::I64(a / b))
                        } else {
                            Err(Error::DivisionByZero)
                        }
                    }
                    (Value::F64(a), Value::F64(b)) => {
                        if b != 0.0 {
                            Ok(Value::F64(a / b))
                        } else {
                            Err(Error::DivisionByZero)
                        }
                    }
                    (a, b) => Err(Error::InvalidTypesForOperator(a.kind(), b.kind(), Op::Div)),
                }
            }
            Expression::Mod(a, b) => {
                let a = self.evaluate(*a)?;
                let b = self.evaluate(*b)?;
                match (a, b) {
                    (Value::I64(a), Value::I64(b)) => Ok(Value::I64(a % b)),
                    (a, b) => Err(Error::InvalidTypesForOperator(a.kind(), b.kind(), Op::Mod)),
                }
            }
            Expression::Binding(name) => match self.bindings.get(&name) {
                Some(value) => Ok(value.clone()),
                None => Err(Error::NoSuchBinding(name)),
            },
            Expression::Member(e, name) => {
                let e = self.evaluate(*e)?;
                Err(Error::Unknown(format!(
                    "method {:?} not implemented on {:?}",
                    name, e
                )))
            }
            Expression::Method(e, name, args) => {
                let e = self.evaluate(*e)?;
                let args = args
                    .into_iter()
                    .map(|a| self.evaluate(a))
                    .collect::<Result<Vec<_>, _>>()?;
                methods::evaluate_method(name, e, args)
            }
        }
    }

    fn evaluate_literal(&self, lit: Literal) -> EvalResult {
        match lit {
            Literal::Null => Ok(Value::Null),
            Literal::String(v) => Ok(Value::String(v)),
            Literal::Bytes(v) => Ok(Value::Bytes(v)),
            Literal::Bool(v) => Ok(Value::Bool(v)),
            Literal::I64(v) => Ok(Value::I64(v)),
            Literal::F64(v) => Ok(Value::F64(v)),
            Literal::List(elems) => {
                let vs = elems
                    .into_iter()
                    .map(|e| self.evaluate(e))
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(Value::List(vs))
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::model::{Error, EvalResult, Kind, Op, Value};
    use crate::parser::parse;

    fn evaluate(input: &str) -> EvalResult {
        super::EvalContext::default().evaluate(parse(input).expect("parse"))
    }

    #[test]
    fn smoke() {
        let input = r#" 1 + 2 + 3 + 4 + 5 "#;
        assert_eq!(evaluate(input), Ok(Value::I64(1 + 2 + 3 + 4 + 5)));
    }

    #[test]
    fn unary_negative_i64() {
        let input = r#" -5 + 8 "#;
        assert_eq!(evaluate(input), Ok(Value::I64(3)));
    }

    #[test]
    fn unary_negative_f64() {
        let input = r#" -5.1 + 8.1 "#;
        assert_eq!(evaluate(input), Ok(Value::F64(3.0)));
    }

    #[test]
    fn unary_not() {
        let input = r#" !(5 + 5 == 10) "#;
        assert_eq!(evaluate(input), Ok(Value::Bool(false)));
    }

    #[test]
    fn string_addition() {
        let input = r#" "asdf" + "pqrs" + "tuvw" == "asdfpqrstuvw" "#;
        assert_eq!(evaluate(input), Ok(Value::Bool(true)));
    }

    #[test]
    fn addition_and_subtraction() {
        let input = r#" 1 - 2 + 3 - 4 + 5 "#;
        assert_eq!(evaluate(input), Ok(Value::I64(1 - 2 + 3 - 4 + 5)));
    }

    #[test]
    fn multiplication_and_division() {
        let input = r#" 1.0 / 2.0 * 3.0 / 4.0 * 5.0 "#;
        assert_eq!(evaluate(input), Ok(Value::F64(1.0 / 2.0 * 3.0 / 4.0 * 5.0)));
    }

    #[test]
    fn string_len() {
        let input = r#" "asdf".len() + "pqrs".len() "#;
        assert_eq!(evaluate(input), Ok(Value::I64(8)));
    }

    #[test]
    fn bytes_len() {
        let input = r#" b"\xFF".len() "#;
        assert_eq!(evaluate(input), Ok(Value::I64(1)));
    }

    #[test]
    fn bytes_eq() {
        let input = r#" b"¢" == b'\xC2\xA2' "#;
        assert_eq!(evaluate(input), Ok(Value::Bool(true)));
    }

    #[test]
    fn bytes_cmp() {
        assert_eq!(evaluate(r#" b"\x00" < b"\x01" "#), Ok(Value::Bool(true)));
        assert_eq!(evaluate(r#" b"asdf" < b"pqrs" "#), Ok(Value::Bool(true)));
        assert_eq!(evaluate(r#" b"" < b"asdf" "#), Ok(Value::Bool(true)));
        assert_eq!(
            evaluate(r#" b"\xFE\xFF\xFF\xFF\xFF" < b"\xFF" "#),
            Ok(Value::Bool(true))
        );
    }

    #[test]
    fn int_pow() {
        let input = r#" 42.pow(2) "#;
        assert_eq!(evaluate(input), Ok(Value::I64(42 * 42)));
    }

    #[test]
    fn float_powf() {
        let input = r#" 3.1415926.pow(3.1415926) "#;
        assert_eq!(
            evaluate(input),
            Ok(Value::F64(3.1415926f64.powf(3.1415926))),
        );
    }

    #[test]
    fn float_powi() {
        let input = r#" 3.1415926.pow(2) "#;
        assert_eq!(evaluate(input), Ok(Value::F64(3.1415926f64.powf(2.0))));
    }

    #[test]
    fn list_len() {
        let input = r#" ["a", 3, false].len() "#;
        assert_eq!(evaluate(input), Ok(Value::I64(3)));
    }

    #[test]
    fn list_contains_true() {
        let input = r#" ["a", 3, false].contains(3) "#;
        assert_eq!(evaluate(input), Ok(Value::Bool(true)));
    }

    #[test]
    fn list_contains_false() {
        let input = r#" ["a", 3, false].contains(4) "#;
        assert_eq!(evaluate(input), Ok(Value::Bool(false)));
    }

    #[test]
    fn list_contains_true_with_error() {
        let input = r#" ["a", 3, 1 / 0].contains(3) "#;
        assert_eq!(evaluate(input), Err(Error::DivisionByZero));
    }

    #[test]
    fn list_contains_false_with_error() {
        let input = r#" ["a", 3, 1 / 0].contains(2) "#;
        assert_eq!(evaluate(input), Err(Error::DivisionByZero));
    }

    #[test]
    fn list_contains_error() {
        let input = r#" ["a", 3, false].contains(1 / 0) "#;
        assert_eq!(evaluate(input), Err(Error::DivisionByZero));
    }

    #[test]
    fn type_error_adding_string_and_int() {
        let input = r#" "asdf" + 5 "#;
        assert_eq!(
            evaluate(input),
            Err(Error::InvalidTypesForOperator(
                Kind::String,
                Kind::I64,
                Op::Plus,
            ))
        );
    }

    #[test]
    fn type_error_subtracting_strings() {
        let input = r#" "asdf" - "pqrs" "#;
        assert_eq!(
            evaluate(input),
            Err(Error::InvalidTypesForOperator(
                Kind::String,
                Kind::String,
                Op::Minus,
            ))
        );
    }

    #[test]
    fn eval_error_divide_by_zero_int() {
        let input = r#" 1 / 0 "#;
        assert_eq!(evaluate(input), Err(Error::DivisionByZero));
    }

    #[test]
    fn or_true_with_error() {
        let input = r#" true || 1 / 0 "#;
        assert_eq!(evaluate(input), Ok(Value::Bool(true)));
    }

    #[test]
    fn or_false_with_error() {
        let input = r#" false || 1 / 0 "#;
        assert_eq!(evaluate(input), Err(Error::DivisionByZero));
    }

    #[test]
    fn and_false_with_error() {
        let input = r#" false && 1 / 0 "#;
        assert_eq!(evaluate(input), Ok(Value::Bool(false)));
    }

    #[test]
    fn and_true_with_error() {
        let input = r#" true && 1 / 0 "#;
        assert_eq!(evaluate(input), Err(Error::DivisionByZero));
    }
}
