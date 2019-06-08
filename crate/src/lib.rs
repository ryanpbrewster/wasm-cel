extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn tokens(input: String) -> String {
  let ts: Vec<String> = input.split_whitespace().map(String::from).collect();
  serde_json::to_string(&ts).expect("serialize")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn smoke() {
    }
}
