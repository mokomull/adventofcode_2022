use prelude::log::debug;
use prelude::*;
use wasm_bindgen::prelude::*;

#[derive(Clone)]
enum Instruction {
    Noop,
    Addx(i64),
}

#[wasm_bindgen]
pub struct Solution {
    instructions: Vec<Instruction>,
}

#[wasm_bindgen]
impl Solution {
    pub fn new(input: &str) -> Self {
        init();
        unimplemented!()
    }
}
