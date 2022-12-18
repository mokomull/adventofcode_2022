use prelude::log::debug;
use prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Solution {
    cubes: Vec<(i32, i32, i32)>,
}

#[wasm_bindgen]
impl Solution {
    pub fn new(input: &str) -> Self {
        init();

        let cubes = input
            .lines()
            .map(|line| {
                let split = line.split(',').collect_vec();
                (
                    split[0].parse().unwrap(),
                    split[1].parse().unwrap(),
                    split[2].parse().unwrap(),
                )
            })
            .collect();

        debug!("parsed {cubes:#?}");

        Solution { cubes }
    }
}
