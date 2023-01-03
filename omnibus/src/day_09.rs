use super::*;

#[wasm_bindgen]
pub struct Day09(::day_09::Solution);

#[wasm_bindgen]
impl Day09 {
    pub fn new(input: &str) -> Result<Day09, JsValue> {
        Ok(Self(::day_09::Solution::new(input)?))
    }

    pub fn part1(&self) -> usize {
        self.0.part1()
    }

    pub fn part2(&self) -> usize {
        self.0.part2()
    }
}
