// extern crate pest;
// extern crate pest_derive;
// extern crate serde;
// extern crate serde_json;
// extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

mod model;
mod parser;

/// Parse `input` into an AST, then serialize it as JSON.
#[wasm_bindgen]
pub fn parse_to_ast(input: String) -> String {
    match parser::parse(&input) {
        Ok(parsed) => serde_json::to_string_pretty(&parsed).expect("serialize"),
        Err(err) => format!("could not parse input: {:?}", err),
    }
}
