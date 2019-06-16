use crate::model::{EvalResult, Expression, Op, Value};
use serde::Serialize;
use wasm_bindgen::prelude::*;

mod interpreter;
mod methods;
mod model;
mod ordering;
mod parser;

/// Parse `input` into an AST, then serialize it as JSON.
#[wasm_bindgen]
pub fn parse_to_ast(input: String) -> JsValue {
    match parser::parse(&input) {
        Ok(parsed) => JsValue::from_serde(&parsed).expect("serialize"),
        Err(err) => JsValue::from_str(&err),
    }
}

/// Parse `input` into an AST, evaluate it fully, then serialize the resulting `EvaluatedAst` as JSON.
#[wasm_bindgen]
pub fn process(input: String) -> JsValue {
    let ast = match parser::parse(&input) {
        Ok(parsed) => parsed,
        Err(err) => return JsValue::from_str(&err),
    };

    JsValue::from_serde(&explore(ast)).expect("serialize")
}

fn explore(expr: Expression) -> EvaluatedAst {
    let value = interpreter::EvalContext::default().evaluate(expr.clone());
    let op = expr.op();
    let children = match expr {
        Expression::Or(a, b) => vec![explore(*a), explore(*b)],
        Expression::And(a, b) => vec![explore(*a), explore(*b)],
        Expression::Eq(a, b) => vec![explore(*a), explore(*b)],
        Expression::Neq(a, b) => vec![explore(*a), explore(*b)],
        Expression::Lt(a, b) => vec![explore(*a), explore(*b)],
        Expression::Lte(a, b) => vec![explore(*a), explore(*b)],
        Expression::Gte(a, b) => vec![explore(*a), explore(*b)],
        Expression::Gt(a, b) => vec![explore(*a), explore(*b)],
        Expression::Add(a, b) => vec![explore(*a), explore(*b)],
        Expression::Sub(a, b) => vec![explore(*a), explore(*b)],
        Expression::Mul(a, b) => vec![explore(*a), explore(*b)],
        Expression::Div(a, b) => vec![explore(*a), explore(*b)],
        Expression::Mod(a, b) => vec![explore(*a), explore(*b)],
        Expression::Neg(a) => vec![explore(*a)],
        Expression::Not(a) => vec![explore(*a)],
        Expression::Member(a, _) => vec![explore(*a)],
        Expression::Method(a, _, args) => {
            let mut cs = vec![explore(*a)];
            for arg in args {
                cs.push(explore(arg));
            }
            cs
        }
        Expression::Lit(_) => vec![],
        Expression::Binding(id) => vec![EvaluatedAst {
            op: Op::Lookup,
            value: Ok(Value::String(id.0)),
            children: vec![],
        }],
    };

    EvaluatedAst {
        op,
        value,
        children,
    }
}

#[derive(Serialize)]
pub struct EvaluatedAst {
    op: Op,
    value: EvalResult,
    children: Vec<EvaluatedAst>,
}
