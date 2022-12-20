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

        Solution { input }
    }

    pub fn part1(&self) -> i32 {
        // Vec of references so we can always find *exactly* the one we're looking for.
        let mut arrangement: Vec<&_> = self.input.iter().collect_vec();

        for number in &self.input {
            let position = arrangement
                .iter()
                .position(|&element| std::ptr::eq(element, number))
                .expect("somehow lost a thing");
            arrangement.remove(position);

            let target = (position as i32) + *number;
            // the % operator always returns something the same sign as its left-hand side
            let target = target % (self.input.len() as i32);
            let target = if target < 0 {
                target + (self.input.len() as i32)
            } else {
                target
            };

            arrangement.insert(
                target.try_into().expect("target shouldn't be negative..."),
                number,
            );
        }

        debug!("final arrangement is {arrangement:?}");

        unimplemented!()
    }
}
