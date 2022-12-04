use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Solution;

#[wasm_bindgen]
impl Solution {
    pub fn new() -> Self {
        prelude::init();
        Solution
    }
}
