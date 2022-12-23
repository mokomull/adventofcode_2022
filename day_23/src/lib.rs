use prelude::log::debug;
use prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Solution {
    elves: HashSet<(i32, i32)>,
}

#[wasm_bindgen]
impl Solution {
    pub fn new(input: &str) -> Self {
        init();

        let elves = input
            .lines()
            .enumerate()
            .flat_map(|(row, line)| {
                line.chars().enumerate().filter_map(move |(column, c)| {
                    if c == '#' {
                        Some((row as i32, column as i32))
                    } else {
                        None
                    }
                })
            })
            .collect();
        debug!("parsed: {elves:#?}");

        Self { elves }
    }
}
