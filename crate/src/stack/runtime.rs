use crate::model::{Value, EvalResult, Error, Op};
use crate::stack::Operation;

fn evaluate(mut program: Vec<Operation>) -> EvalResult {
    let mut stack = Vec::new();
    while let Some(op) = program.pop() {
        match op {
            Operation::Lit(v) => stack.push(Ok(v)),
            Operation::MakeList(n) => {
                let mut acc = Ok(Vec::new());
                for _ in 0 .. n {
                    let v = stack.pop().unwrap();
                    if let Ok(ref mut vs) = acc {
                        match v {
                            Ok(v) => vs.push(v),
                            Err(e) => acc = Err(e),
                        }
                    }
                }
                stack.push(acc.map(|vs| Value::List(vs)));
            }
            Operation::MakeMap(_) => unimplemented!(),
            Operation::Add => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                match (a, b) {
                    (Ok(x), Ok(y)) => stack.push(add(x, y)),
                    (Err(e), _) => stack.push(Err(e)),
                    (_, Err(e)) => stack.push(Err(e)),
                }
            }
            Operation::Sub => unimplemented!(),
            Operation::Mul => unimplemented!(),
            Operation::Div => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                match (a, b) {
                    (Ok(x), Ok(y)) => stack.push(div(x, y)),
                    (Err(e), _) => stack.push(Err(e)),
                    (_, Err(e)) => stack.push(Err(e)),
                }
            }
            Operation::Mod => unimplemented!(),
            Operation::Neg => unimplemented!(),
            Operation::Not => unimplemented!(),
            Operation::Or => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(or(a, b));
            }
            Operation::And => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(and(a, b));
            }
        }
    }
    assert_eq!(stack.len(), 1);
    stack.pop().expect("valid programs always result in a single value on the stack")
}

fn add(a: Value, b: Value) -> EvalResult {
    match (a, b) {
        (Value::I64(a), Value::I64(b)) => Ok(Value::I64(a + b)),
        (Value::F64(a), Value::F64(b)) => Ok(Value::F64(a + b)),
        (a, b) => Err(Error::InvalidTypesForOperator(a.kind(), b.kind(), Op::Plus))
    }
}

fn div(a: Value, b: Value) -> EvalResult {
    match (a, b) {
        (Value::I64(a), Value::I64(b)) => {
            if b == 0 {
                Err(Error::DivisionByZero)
            } else {
                Ok(Value::I64(a / b))
            }
        },
        (Value::F64(a), Value::F64(b)) => {
            if b == 0.0 {
                Err(Error::DivisionByZero)
            } else {
                Ok(Value::F64(a / b))
            }
        },
        (a, b) => Err(Error::InvalidTypesForOperator(a.kind(), b.kind(), Op::Plus))
    }
}

fn or(a: EvalResult, b: EvalResult) -> EvalResult {
    match (a, b) {
        (Ok(Value::Bool(true)), _) | (_, Ok(Value::Bool(true))) => {
            Ok(Value::Bool(true))
        },
        (Ok(Value::Bool(false)), Ok(Value::Bool(false))) => {
            Ok(Value::Bool(false))
        },
        (Err(e), _) | (_, Err(e)) => Err(e),
        (Ok(a), Ok(b)) => Err(Error::InvalidTypesForOperator(a.kind(), b.kind(), Op::Or))
    }
}

fn and(a: EvalResult, b: EvalResult) -> EvalResult {
    match (a, b) {
        (Ok(Value::Bool(false)), _) | (_, Ok(Value::Bool(false))) => {
            Ok(Value::Bool(false))
        },
        (Ok(Value::Bool(true)), Ok(Value::Bool(true))) => {
            Ok(Value::Bool(true))
        },
        (Err(e), _) | (_, Err(e)) => Err(e),
        (Ok(a), Ok(b)) => Err(Error::InvalidTypesForOperator(a.kind(), b.kind(), Op::And))
    }
}

#[cfg(test)]
mod test {
    use crate::parser::parse;
    use crate::stack::walker::linearize;

    use super::*;

    #[test]
    fn eval_add() {
        let program = linearize(parse(r#" 1 + 1 "#).unwrap());
        assert_eq!(
            evaluate(program),
            Ok(Value::I64(1 + 1))
        );
    }

    #[test]
    fn eval_list_empty() {
        let program = linearize(parse(r#" [] "#).unwrap());
        assert_eq!(
            evaluate(program),
            Ok(Value::List(vec![]))
        );
    }

    #[test]
    fn eval_list() {
        let program = linearize(parse(r#" [1, 2 + 3] "#).unwrap());
        assert_eq!(
            evaluate(program),
            Ok(Value::List(vec![Value::I64(1), Value::I64(2 + 3)]))
        );
    }

    #[test]
    fn eval_list_err() {
        let program = linearize(parse(r#" [1, 2 / 0, 3] "#).unwrap());
        assert_eq!(
            evaluate(program),
            Err(Error::DivisionByZero),
        );
    }

    #[test]
    fn eval_or_simple() {
        let program = linearize(parse(r#" true || false "#).unwrap());
        assert_eq!(
            evaluate(program),
            Ok(Value::Bool(true))
        );
    }

    #[test]
    fn eval_or_error() {
        let program = linearize(parse(r#" false || (1/0) "#).unwrap());
        assert_eq!(
            evaluate(program),
            Err(Error::DivisionByZero),
        );
    }

    #[test]
    fn eval_or_error_recovery() {
        let program = linearize(parse(r#" true || (1/0) "#).unwrap());
        assert_eq!(
            evaluate(program),
            Ok(Value::Bool(true)),
        );
    }

    #[test]
    fn eval_and_simple() {
        let program = linearize(parse(r#" true && false "#).unwrap());
        assert_eq!(
            evaluate(program),
            Ok(Value::Bool(false))
        );
    }

    #[test]
    fn eval_and_errand() {
        let program = linearize(parse(r#" true && (1/0) "#).unwrap());
        assert_eq!(
            evaluate(program),
            Err(Error::DivisionByZero),
        );
    }

    #[test]
    fn eval_and_errand_recovery() {
        let program = linearize(parse(r#" false && (1/0) "#).unwrap());
        assert_eq!(
            evaluate(program),
            Ok(Value::Bool(false)),
        );
    }
}
