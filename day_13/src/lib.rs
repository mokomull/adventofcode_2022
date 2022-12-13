use prelude::log::debug;
use prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Solution {
}


#[wasm_bindgen]
impl Solution {
    pub fn new(input: &str) -> Self {
        init();

        Solution {  }
    }
}