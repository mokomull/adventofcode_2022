use prelude::log::debug;
use prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::{HtmlDivElement, Text};

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

        let steps = lines[separator_index + 1..]
            .iter()
            .map(|line| {
                let split = line.split(' ').collect_vec();
                (
                    split[1].parse().expect("count"),
                    split[3].parse().expect("from"),
                    split[5].parse().expect("to"),
                )
            })
            .collect();

        debug!("steps: {:?}", steps);

        Solution {
            initial: stacks,
            steps,
        }
    }

    pub fn part1(&self) -> String {
        let mut stacks = self.initial.clone();

        for &(count, from, to) in &self.steps {
            for _ in 0..count {
                let container = stacks[from - 1].pop().expect("ran out of containers!");
                stacks[to - 1].push(container);
            }
        }

        let result = stacks
            .iter()
            .map(|stack| stack.last().expect("empty stack is a surprise"))
            .copied()
            .collect_vec();
        String::from_utf8(result).expect("invalid UTF-8 somehow!")
    }

    pub fn part2(&self) -> String {
        let mut stacks = self.initial.clone();

        for &(count, from, to) in &self.steps {
            let from = &mut stacks[from - 1];
            let mut containers = from.split_off(from.len() - count);
            stacks[to - 1].append(&mut containers);
        }

        let result = stacks
            .iter()
            .map(|stack| stack.last().expect("empty stack is a surprise"))
            .copied()
            .collect_vec();
        String::from_utf8(result).expect("invalid UTF-8 somehow!")
    }

    pub fn render(&self, target: &HtmlDivElement) -> Result<(), JsValue> {
        let text = Text::new_with_data("roflcopter")?;
        target.append_child(&text)?;

        Ok(())
    }
}
