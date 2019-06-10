use crate::model::{EvalResult, Identifier, Value};

const METHOD_CONTAINS: &str = "contains";
const METHOD_LEN: &str = "len";
const METHOD_POW: &str = "pow";

pub fn evaluate_method(method: Identifier, operand: Value, args: Vec<Value>) -> EvalResult {
    match method.0.as_ref() {
        METHOD_CONTAINS => evaluate_method_contains(operand, args),
        METHOD_LEN => evaluate_method_len(operand, args),
        METHOD_POW => evaluate_method_pow(operand, args),
        _ => Err(format!(
            "method {:?}/{} not implemented on {:?}",
            method,
            args.len(),
            operand
        )),
    }
}

fn evaluate_method_contains(operand: Value, args: Vec<Value>) -> EvalResult {
    if args.len() != 1 {
        return Err(format!(
            "wrong number of arguments for .contains(): {}",
            args.len()
        ));
    }
    let arg = args.into_iter().next().unwrap();
    match operand {
        Value::List(elems) => Ok(Value::Bool(elems.contains(&arg))),
        Value::String(_) => Err(format!("no method .len() on string")),
        Value::Bytes(_) => Err(format!("no method .len() on bytes")),
        Value::Bool(_) => Err(format!("no method .len() on bool")),
        Value::I64(_) => Err(format!("no method .len() on i64")),
        Value::F64(_) => Err(format!("no method .len() on f64")),
        Value::Null => Err(format!("no method .len() on null")),
    }
}

fn evaluate_method_len(operand: Value, args: Vec<Value>) -> EvalResult {
    if args.len() != 0 {
        return Err(format!(
            "wrong number of arguments for .len(): {}",
            args.len()
        ));
    }
    match operand {
        Value::List(elems) => Ok(Value::I64(elems.len() as i64)),
        Value::String(s) => Ok(Value::I64(s.chars().count() as i64)),
        Value::Bytes(b) => Ok(Value::I64(b.len() as i64)),
        Value::Bool(_) => Err(format!("no method .len() on bool")),
        Value::I64(_) => Err(format!("no method .len() on i64")),
        Value::F64(_) => Err(format!("no method .len() on f64")),
        Value::Null => Err(format!("no method .len() on null")),
    }
}

fn evaluate_method_pow(operand: Value, args: Vec<Value>) -> EvalResult {
    if args.len() != 1 {
        return Err(format!(
            "wrong number of arguments for .pow(): {}",
            args.len()
        ));
    }
    let arg = args.into_iter().next().unwrap();
    match operand {
        Value::I64(base) => match arg {
            Value::I64(exp) => Ok(Value::I64(i64::pow(base, exp as u32))),
            other => Err(format!("bad argument for .pow(): {:?}", other)),
        },
        Value::F64(base) => match arg {
            Value::I64(exp) => Ok(Value::F64(f64::powf(base, exp as f64))),
            Value::F64(exp) => Ok(Value::F64(f64::powf(base, exp))),
            other => Err(format!("bad argument for .pow(): {:?}", other)),
        },
        Value::List(_) => Err(format!("no method .len() on list")),
        Value::String(_) => Err(format!("no method .pow() on string")),
        Value::Bytes(_) => Err(format!("no method .pow() on bytes")),
        Value::Bool(_) => Err(format!("no method .pow() on bool")),
        Value::Null => Err(format!("no method .pow() on null")),
    }
}
