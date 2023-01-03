use js_sys::Function;

use crate::*;

#[wasm_bindgen]
pub struct Day10(::day_10::Solution);

#[wasm_bindgen]
impl Day10 {
    pub fn new(input: &str) -> Self {
        Self(::day_10::Solution::new(input))
    }

    pub fn part1(&self) -> i64 {
        self.0.part1()
    }

    pub fn part2(&self, light: Function) {
        self.0.part2(light)
    }
}
