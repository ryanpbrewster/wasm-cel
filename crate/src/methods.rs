use crate::model::{Error, EvalResult, Identifier, Kind, Value};

const METHOD_CONTAINS: &str = "contains";
const METHOD_KEYS: &str = "keys";
const METHOD_LEN: &str = "len";
const METHOD_POW: &str = "pow";

fn arg_kinds(args: Vec<Value>) -> Vec<Kind> {
    args.into_iter().map(|arg| arg.kind()).collect()
}

pub fn evaluate_method(method: Identifier, operand: Value, args: Vec<Value>) -> EvalResult {
    match method.0.as_ref() {
        METHOD_CONTAINS => evaluate_method_contains(operand, args),
        METHOD_KEYS => evaluate_method_keys(operand, args),
        METHOD_LEN => evaluate_method_len(operand, args),
        METHOD_POW => evaluate_method_pow(operand, args),
        _ => Err(Error::NoMethod(method)),
    }
}

fn evaluate_method_contains(operand: Value, args: Vec<Value>) -> EvalResult {
    if args.len() != 1 {
        return Err(Error::NoMethodWithSignature(
            operand.kind(),
            Identifier::new(METHOD_CONTAINS),
            arg_kinds(args),
        ));
    }
    let arg = args.into_iter().next().unwrap();
    match operand {
        Value::List(elems) => Ok(Value::Bool(elems.contains(&arg))),
        other => Err(Error::NoMethodOnType(
            other.kind(),
            Identifier::new(METHOD_CONTAINS),
        )),
    }
}

fn evaluate_method_keys(operand: Value, args: Vec<Value>) -> EvalResult {
    if args.len() != 0 {
        return Err(Error::NoMethodWithSignature(
            operand.kind(),
            Identifier::new(METHOD_KEYS),
            arg_kinds(args),
        ));
    }
    match operand {
        Value::Map(fields) => {
            let mut keys: Vec<String> = fields.keys().cloned().collect();
            keys.sort();
            Ok(Value::List(
                keys.into_iter().map(|s| Value::String(s)).collect(),
            ))
        }
        other => Err(Error::NoMethodOnType(
            other.kind(),
            Identifier::new(METHOD_LEN),
        )),
    }
}

fn evaluate_method_len(operand: Value, args: Vec<Value>) -> EvalResult {
    if args.len() != 0 {
        return Err(Error::NoMethodWithSignature(
            operand.kind(),
            Identifier::new(METHOD_LEN),
            arg_kinds(args),
        ));
    }
    match operand {
        Value::List(elems) => Ok(Value::I64(elems.len() as i64)),
        Value::String(s) => Ok(Value::I64(s.chars().count() as i64)),
        Value::Bytes(b) => Ok(Value::I64(b.len() as i64)),
        Value::Map(fields) => Ok(Value::I64(fields.len() as i64)),
        other => Err(Error::NoMethodOnType(
            other.kind(),
            Identifier::new(METHOD_LEN),
        )),
    }
}

fn evaluate_method_pow(operand: Value, args: Vec<Value>) -> EvalResult {
    if args.len() != 1 {
        return Err(Error::NoMethodWithSignature(
            operand.kind(),
            Identifier::new(METHOD_POW),
            arg_kinds(args),
        ));
    }
    let arg = args.into_iter().next().unwrap();
    match operand {
        Value::I64(base) => match arg {
            Value::I64(exp) => Ok(Value::I64(i64::pow(base, exp as u32))),
            other => Err(Error::NoMethodWithSignature(
                Kind::I64,
                Identifier::new(METHOD_POW),
                vec![other.kind()],
            )),
        },
        Value::F64(base) => match arg {
            Value::I64(exp) => Ok(Value::F64(f64::powf(base, exp as f64))),
            Value::F64(exp) => Ok(Value::F64(f64::powf(base, exp))),
            other => Err(Error::NoMethodWithSignature(
                Kind::F64,
                Identifier::new(METHOD_POW),
                vec![other.kind()],
            )),
        },
        other => Err(Error::NoMethodOnType(
            other.kind(),
            Identifier::new(METHOD_POW),
        )),
    }
}
