use prelude::log::debug;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Solution {
    elves: Vec<Vec<u64>>,
}

#[wasm_bindgen]
impl Solution {
    pub fn new(input: &str) -> Self {
        prelude::init();

        let mut elves = vec![];
        let mut elf = vec![];

        for line in input.lines() {
            prelude::log::debug!("got: {:?}", line);

            if line.is_empty() {
                elves.push(elf);
                elf = vec![];
                continue;
            }

            elf.push(line.parse().expect("input was bad"));
        }
        elves.push(elf);

        debug!("elves: {:?}", elves);

        Solution { elves }
    }

    pub fn part1(&self) -> u64 {
        self.elves
            .iter()
            .map(|elf| elf.iter().sum())
            .max()
            .expect("there weren't any elves?")
    }
}
