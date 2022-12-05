use prelude::log::debug;
use prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Solution {
    initial: Vec<Vec<u8>>, // stacks of crates from the bottom up
    steps: Vec<(usize, usize, usize)>,
}

#[wasm_bindgen]
impl Solution {
    pub fn new(input: &str) -> Self {
        init();

        let lines = input.lines().collect_vec();
        let separator_index = lines
            .iter()
            .position(|line| line.is_empty())
            .expect("you should've given me an empty line at least");

        let stack_count = lines[separator_index - 1]
            .split(' ')
            .filter(|word| !word.is_empty())
            .count();
        let mut stacks = vec![vec![]; stack_count];
        for line in lines[..=separator_index - 2].iter().rev() {
            debug!("parsing line {:?}", line);
            for stack_index in 0..stack_count {
                let container = line.as_bytes().get(stack_index * 4 + 1);
                debug!("found maybe a container: {:x?}", container);
                match container {
                    None => continue,
                    Some(c @ b'A'..=b'Z') => {
                        stacks[stack_index].push(*c);
                    }
                    Some(_) => continue,
                }
            }
        }

        debug!("initial stacks: {:x?}", stacks);

        Solution {
            initial: stacks,
            steps: unimplemented!(),
        }
    }
}
