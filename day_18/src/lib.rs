use prelude::log::debug;
use prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Solution {
    cubes: HashSet<(i32, i32, i32)>,
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

    pub fn part1(&self) -> u32 {
        let mut result = 0;

        for &(i, j, k) in &self.cubes {
            for (a, b, c) in [
                (i - 1, j, k),
                (i + 1, j, k),
                (i, j - 1, k),
                (i, j + 1, k),
                (i, j, k - 1),
                (i, j, k + 1),
            ] {
                if !self.cubes.contains(&(a, b, c)) {
                    result += 1;
                }
            }
        }

        result
    }
}
