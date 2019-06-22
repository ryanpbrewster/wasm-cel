use crate::interpreter::EvalContext;
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

    JsValue::from_serde(&explore(&EvalContext::default(), ast)).expect("serialize")
}

fn explore(ctx: &EvalContext, expr: Expression) -> EvaluatedAst {
    let value = ctx.evaluate(expr.clone());
    let op = expr.op();
    let children = match expr {
        Expression::LetBinding { id, value, body } => {
            let value = ctx.evaluate(*value);
            let mut child_ctx = ctx.clone();
            child_ctx.bindings.insert(id, value);
            return explore(&child_ctx, *body);
        }
        Expression::Or(cs) => cs.into_iter().map(|c| explore(ctx, c)).collect(),
        Expression::And(cs) => cs.into_iter().map(|c| explore(ctx, c)).collect(),
        Expression::Eq(a, b) => vec![explore(ctx, *a), explore(ctx, *b)],
        Expression::Neq(a, b) => vec![explore(ctx, *a), explore(ctx, *b)],
        Expression::Lt(a, b) => vec![explore(ctx, *a), explore(ctx, *b)],
        Expression::Lte(a, b) => vec![explore(ctx, *a), explore(ctx, *b)],
        Expression::Gte(a, b) => vec![explore(ctx, *a), explore(ctx, *b)],
        Expression::Gt(a, b) => vec![explore(ctx, *a), explore(ctx, *b)],
        Expression::Add(a, b) => vec![explore(ctx, *a), explore(ctx, *b)],
        Expression::Sub(a, b) => vec![explore(ctx, *a), explore(ctx, *b)],
        Expression::Mul(a, b) => vec![explore(ctx, *a), explore(ctx, *b)],
        Expression::Div(a, b) => vec![explore(ctx, *a), explore(ctx, *b)],
        Expression::Mod(a, b) => vec![explore(ctx, *a), explore(ctx, *b)],
        Expression::Neg(a) => vec![explore(ctx, *a)],
        Expression::Not(a) => vec![explore(ctx, *a)],
        Expression::Member(a, _) => vec![explore(ctx, *a)],
        Expression::Method(a, _, args) => {
            let mut cs = vec![explore(ctx, *a)];
            for arg in args {
                cs.push(explore(ctx, arg));
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
