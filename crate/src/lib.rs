use wasm_bindgen::prelude::*;

mod interpreter;
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
