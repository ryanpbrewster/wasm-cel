use std::collections::HashMap;

use crate::methods;
use crate::model::{Error, EvalResult, Expression, Identifier, Literal, Op, Value};
use std::cmp::Ordering;
use std::rc::Rc;
use std::sync::Mutex;

#[derive(Default, Clone)]
pub struct EvalContext<'a> {
    parent: Option<&'a EvalContext<'a>>,
    pub binding: Option<(Identifier, EvalResult)>,
    bytes_processed: Rc<Mutex<usize>>,
}

const BYTES_PROCESSED_LIMIT: usize = 1 << 20;
impl<'a> EvalContext<'a> {
    pub fn with_binding(&self, name: Identifier, result: EvalResult) -> EvalContext {
        EvalContext {
            parent: Some(self),
            binding: Some((name, result)),
            bytes_processed: self.bytes_processed.clone(),
        }
    }
    fn check_limits(&self) -> Result<(), Error> {
        if *self.bytes_processed.lock().unwrap() > BYTES_PROCESSED_LIMIT {
            return Err(Error::EvaluationTooLarge);
        }
        Ok(())
    }
    pub fn evaluate(&'a self, expr: Expression) -> EvalResult {
        self.check_limits()?;
        let result = match expr {
            Expression::LetBinding { id, value, body } => {
                let value = self.evaluate(*value);
                self.with_binding(id, value).evaluate(*body)
            }
            Expression::Ternary {
                condition,
                true_branch,
                else_branch,
            } => match self.evaluate(*condition) {
                Ok(Value::Bool(true)) => self.evaluate(*true_branch),
                _ => self.evaluate(*else_branch),
            },
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
            Expression::Or(children) => {
                let mut vs = Vec::new();
                for child in children {
                    match self.evaluate(child) {
                        Ok(Value::Bool(true)) => return Ok(Value::Bool(true)),
                        other => vs.push(other),
                    };
                }
                for v in vs {
                    match v? {
                        Value::Bool(b) => assert!(!b),
                        other => return Err(Error::InvalidTypeForOperator(other.kind(), Op::Or)),
                    };
                }
                Ok(Value::Bool(false))
            }
            Expression::And(children) => {
                let mut vs = Vec::new();
                for child in children {
                    match self.evaluate(child) {
                        Ok(Value::Bool(false)) => return Ok(Value::Bool(false)),
                        other => vs.push(other),
                    };
                }
                for v in vs {
                    match v? {
                        Value::Bool(b) => assert!(b),
                        other => return Err(Error::InvalidTypeForOperator(other.kind(), Op::Or)),
                    };
                }
                Ok(Value::Bool(true))
            }
            Expression::Eq(a, b) => {
                let a = self.evaluate(*a)?;
                let b = self.evaluate(*b)?;
                Ok(Value::Bool(a == b))
            }
            Expression::Neq(a, b) => {
                let a = self.evaluate(*a)?;
                let b = self.evaluate(*b)?;
                Ok(Value::Bool(a != b))
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
                    Some(ord) => Ok(Value::Bool(ord == Ordering::Less || a == b)),
                }
            }
            Expression::Gte(a, b) => {
                let a = self.evaluate(*a)?;
                let b = self.evaluate(*b)?;
                match a.partial_cmp(&b) {
                    None => Err(Error::InvalidTypesForOperator(a.kind(), b.kind(), Op::Gte)),
                    Some(ord) => Ok(Value::Bool(ord == Ordering::Greater || a == b)),
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
                    (Value::Bytes(a), Value::Bytes(b)) => {
                        Ok(Value::Bytes(a.into_iter().chain(b.into_iter()).collect()))
                    }
                    (Value::List(a), Value::List(b)) => {
                        Ok(Value::List(a.into_iter().chain(b.into_iter()).collect()))
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
            Expression::Binding(name) => self.lookup_binding(name),
            Expression::Member(e, name) => {
                let e = self.evaluate(*e)?;
                match e {
                    Value::Map(mut fields) => {
                        Ok(fields.remove(&name.0).ok_or(Error::NoSuchMember(name))?)
                    }
                    other => Err(Error::InvalidTypeForOperator(
                        other.kind(),
                        Op::Member(name),
                    )),
                }
            }
            Expression::Method(e, name, args) => {
                let e = self.evaluate(*e)?;
                let args = args
                    .into_iter()
                    .map(|a| self.evaluate(a))
                    .collect::<Result<Vec<_>, _>>()?;
                methods::evaluate_method(name, e, args)
            }
        };

        if let Ok(ref value) = result {
            *self.bytes_processed.lock().unwrap() += value.size();
        }
        result
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
            Literal::Map(kvs) => {
                let mut m = HashMap::new();
                for (k, v) in kvs {
                    let k = self.evaluate(k)?;
                    let v = self.evaluate(v)?;
                    match k {
                        Value::String(k) => {
                            if m.insert(k.clone(), v).is_some() {
                                return Err(Error::DuplicateMapKey(k));
                            }
                        }
                        other => return Err(Error::InvalidMapKey(other.kind())),
                    }
                }
                Ok(Value::Map(m))
            }
        }
    }

    fn lookup_binding(&self, name: Identifier) -> EvalResult {
        if let Some((ref id, ref value)) = self.binding {
            if *id == name {
                return value.clone();
            }
        }
        if let Some(parent) = self.parent {
            return parent.lookup_binding(name);
        }
        Err(Error::NoSuchBinding(name))
    }
}

#[cfg(test)]
mod test {
    use crate::model::{Error, EvalResult, Identifier, Kind, Op, Value};
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
    fn bytes_addition() {
        let input = r#" b"asdf" + b"pqrs" + b"tuvw" == b"asdfpqrstuvw" "#;
        assert_eq!(evaluate(input), Ok(Value::Bool(true)));
    }

    #[test]
    fn list_addition() {
        let input = r#" [1,2,3] + ["a", "b", "c"] == [1, 2, 3, "a", "b", "c"] "#;
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
    fn map_len() {
        let input = r#" { "a": 1, "b": 2 }.len() "#;
        assert_eq!(evaluate(input), Ok(Value::I64(2)));
    }

    #[test]
    fn map_keys() {
        let input = r#" { "a": 1, "b": 2 }.keys() "#;
        assert_eq!(
            evaluate(input),
            Ok(Value::List(vec![
                Value::String("a".to_owned()),
                Value::String("b".to_owned())
            ]))
        );
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

    #[test]
    fn eq_mismatched_types() {
        let input = r#" 1 == false "#;
        assert_eq!(evaluate(input), Ok(Value::Bool(false)));
    }

    #[test]
    fn eq_mismatched_values() {
        let input = r#" 1 == 2 "#;
        assert_eq!(evaluate(input), Ok(Value::Bool(false)));
    }

    #[test]
    fn neq_mismatched_types() {
        let input = r#" 1 != false "#;
        assert_eq!(evaluate(input), Ok(Value::Bool(true)));
    }

    #[test]
    fn neq_mismatched_values() {
        let input = r#" 1 != 2 "#;
        assert_eq!(evaluate(input), Ok(Value::Bool(true)));
    }

    #[test]
    fn neq_matching_values() {
        let input = r#" 1 != 1 "#;
        assert_eq!(evaluate(input), Ok(Value::Bool(false)));
    }

    #[test]
    fn map_literal() {
        let input = r#" { "foo": "bar" }.len() "#;
        assert_eq!(evaluate(input), Ok(Value::I64(1)));
    }

    #[test]
    fn map_unbound_key() {
        let input = r#" { foo: "bar" }.len() "#;
        assert_eq!(
            evaluate(input),
            Err(Error::NoSuchBinding(Identifier::new("foo")))
        );
    }

    #[test]
    fn map_non_string_key() {
        let input = r#" { 1: "bar" }.len() "#;
        assert_eq!(evaluate(input), Err(Error::InvalidMapKey(Kind::I64)));
    }

    #[test]
    fn map_expression_key() {
        let input = r#" { ("asdf" + "pqrs"): "bar" }.len() "#;
        assert_eq!(evaluate(input), Ok(Value::I64(1)));
    }

    #[test]
    fn map_duplicate_keys() {
        let input = r#" { "a": 0, "a": 1 }.len() "#;
        assert_eq!(evaluate(input), Err(Error::DuplicateMapKey("a".to_owned())));
    }

    #[test]
    fn map_member_present() {
        let input = r#" { "a": 0, "b": 1 }.a "#;
        assert_eq!(evaluate(input), Ok(Value::I64(0)));
    }

    #[test]
    fn map_member_missing() {
        let input = r#" { "a": 0, "b": 1 }.c "#;
        assert_eq!(
            evaluate(input),
            Err(Error::NoSuchMember(Identifier::new("c")))
        );
    }

    #[test]
    fn non_map_member() {
        let input = r#" 42.a "#;
        assert_eq!(
            evaluate(input),
            Err(Error::InvalidTypeForOperator(
                Kind::I64,
                Op::Member(Identifier::new("a"))
            ))
        );
    }

    #[test]
    fn let_binding_smoke() {
        let input = r#" let x = 42; x "#;
        assert_eq!(evaluate(input), Ok(Value::I64(42)));
    }

    #[test]
    fn let_binding_error() {
        let input = r#" let x = 1 / 0; x == 4 || true "#;
        assert_eq!(evaluate(input), Ok(Value::Bool(true)));
    }

    #[test]
    fn multiple_let_bindings() {
        let input = r#" let a = 1; let b = 2; let c = a + b; a + b + c "#;
        assert_eq!(evaluate(input), Ok(Value::I64(1 + 2 + 3)));
    }

    #[test]
    fn let_rebinding() {
        let input = r#" let x = 42; let x = x + x; x "#;
        assert_eq!(evaluate(input), Ok(Value::I64(42 + 42)));
    }

    #[test]
    fn ternary_smoke() {
        let input = r#" true ? 1 : 2 "#;
        assert_eq!(evaluate(input), Ok(Value::I64(1)));
    }

    #[test]
    fn ternary_chain() {
        let input = r#"
            let x = "c";
            x == "a" ? 0 :
            x == "b" ? 1 :
            x == "c" ? 2 :
            x == "d" ? 3 :
            x == "e" ? 4 :
                       5
        "#;
        assert_eq!(evaluate(input), Ok(Value::I64(2)));
    }

    #[test]
    fn ternary_operator_map_key() {
        let input = r#" { true ? "a" : "b" : "foo"  } "#;
        assert_eq!(evaluate(input), evaluate(r#" { "a": "foo" } "#));
    }

    #[test]
    fn ternary_operator_map_value() {
        let input = r#" { "a": true ? "foo" : "bar"  } "#;
        assert_eq!(evaluate(input), evaluate(r#" { "a": "foo" } "#));
    }

    #[test]
    fn value_size_explosion() {
        // 16 ** 8 == 2 ** 32 values, should _definitely_ overflow
        let input = r#"
        let x = [0];
        let x = x + x + x + x + x + x + x + x + x + x + x + x + x + x + x + x;
        let x = x + x + x + x + x + x + x + x + x + x + x + x + x + x + x + x;
        let x = x + x + x + x + x + x + x + x + x + x + x + x + x + x + x + x;
        let x = x + x + x + x + x + x + x + x + x + x + x + x + x + x + x + x;
        let x = x + x + x + x + x + x + x + x + x + x + x + x + x + x + x + x;
        let x = x + x + x + x + x + x + x + x + x + x + x + x + x + x + x + x;
        let x = x + x + x + x + x + x + x + x + x + x + x + x + x + x + x + x;
        let x = x + x + x + x + x + x + x + x + x + x + x + x + x + x + x + x;
        x
        "#;
        assert_eq!(evaluate(input), Err(Error::EvaluationTooLarge));
    }
}
