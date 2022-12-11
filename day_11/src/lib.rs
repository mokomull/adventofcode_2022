use prelude::log::debug;
use prelude::*;
use wasm_bindgen::prelude::*;

struct Monkey {}

#[wasm_bindgen]
pub struct Solution {
    monkeys: Vec<Monkey>,
}

#[wasm_bindgen]
impl Solution {
    pub fn new(input: &str) -> Self {
        unimplemented!()
    }
}
