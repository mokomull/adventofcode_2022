use prelude::log::debug;
use prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Solution {
    input: Vec<i32>,
}

#[wasm_bindgen]
impl Solution {
    pub fn new(input: &str) -> Solution {
        init();

        let input = input.lines().map(|line| line.parse().unwrap()).collect();
        debug!("parsed: {input:?}");

        Solution {input}
    }
}
