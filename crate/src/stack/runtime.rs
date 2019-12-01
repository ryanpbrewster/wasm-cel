use crate::model::{Value, EvalResult, Error, Op};
use crate::stack::Operation;

fn evaluate(mut program: Vec<Operation>) -> EvalResult {
    let mut stack = Vec::new();
    while let Some(op) = program.pop() {
        match op {
            Operation::Lit(v) => stack.push(Ok(v)),
            Operation::MakeList(n) => {
                let mut acc = Ok(Value::List(Vec::new()));
                for _ in 0 .. n {
                    let v = stack.pop().unwrap();
                    if let Ok(Value::List(ref mut vs)) = acc {
                        match v {
                            Ok(v) => vs.push(v),
                            Err(e) => acc = Err(e),
                        }
                    }
                }
                stack.push(acc);
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
            Operation::Div => unimplemented!(),
            Operation::Mod => unimplemented!(),
            Operation::Neg => unimplemented!(),
            Operation::Not => unimplemented!(),
        }
    }
    assert_eq!(stack.len(), 1);
    stack.pop().expect("valid programs always result in a single value on the stack")
}

fn add(a: Value, b: Value) -> EvalResult {
    match (&a, &b) {
        (Value::I64(a), Value::I64(b)) => Ok(Value::I64(a + b)),
        (Value::F64(a), Value::F64(b)) => Ok(Value::F64(a + b)),
        _ => Err(Error::InvalidTypesForOperator(a.kind(), b.kind(), Op::Plus))
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
}
