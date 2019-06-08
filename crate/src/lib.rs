// extern crate pest;
// extern crate pest_derive;
// extern crate serde;
// extern crate serde_json;
// extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

mod model;
mod parser;

#[wasm_bindgen]
pub fn tokens(input: String) -> String {
    match parser::parse(&input) {
        Ok(parsed) => serde_json::to_string(&parsed).expect("serialize"),
        Err(err) => format!("could not parse input: {:?}", err),
    }
}
