use wasm_bindgen::prelude::*;

mod interpreter;
mod methods;
mod model;
mod parser;

/// Parse `input` into an AST, then serialize it as JSON.
#[wasm_bindgen]
pub fn parse_to_ast(input: String) -> JsValue {
    match parser::parse(&input) {
        Ok(parsed) => JsValue::from_serde(&parsed).expect("serialize"),
        Err(err) => JsValue::from_str(&err),
    }
}

/// Parse `input` into an AST, then serialize it as JSON.
#[wasm_bindgen]
pub fn evaluate(input: String) -> JsValue {
    match parser::parse(&input) {
        Ok(parsed) => match interpreter::EvalContext::default().evaluate(parsed) {
            Ok(value) => JsValue::from_serde(&value).expect("serialize"),
            Err(err) => JsValue::from_str(&format!("{:?}", err)),
        },
        Err(err) => JsValue::from_str(&err),
    }
}
