use std::collections::VecDeque;

use js_sys::Function;
use prelude::log::debug;
use prelude::*;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{Document, HtmlDivElement, HtmlElement, Text};

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

    fn count_tallest_part2(&self) -> usize {
        let mut stacks = self.initial.clone();
        let mut tallest = 0;

        for &(count, from, to) in &self.steps {
            let from = &mut stacks[from - 1];
            let mut containers = from.split_off(from.len() - count);
            stacks[to - 1].append(&mut containers);
            tallest = std::cmp::max(tallest, stacks[to - 1].len());
        }

        tallest
    }

    pub fn part1(&self, document: Document, target: &HtmlDivElement) -> Result<Part1, JsValue> {
        while let Some(child) = target.first_child() {
            let _ = target.remove_child(&child); // if somehow the child already got removed, not my problem!
        }

        let height = self.count_tallest_part2();
        target
            .style()
            .set_property("--count", &height.to_string())?;

        let mut crates = HashMap::new();

        for (stack_id, stack) in self.initial.iter().enumerate() {
            for (crate_height, &krate) in stack.iter().enumerate() {
                let div = document.create_element("div")?.dyn_into::<HtmlElement>()?;
                let span = document.create_element("span")?;
                div.append_child(&span)?;

                let crate_name_utf8 = &[krate];
                let crate_name = std::str::from_utf8(crate_name_utf8)
                    .map_err(|e| format!("somehow the crate did not have a UTF-8 name: {:?}", e))?;

                let text = Text::new_with_data(crate_name)?;
                span.append_child(&text)?;

                target.append_child(&div)?;

                let style = div.style();
                style.set_property("--stack", &stack_id.to_string())?;
                style.set_property("--index", &crate_height.to_string())?;

                if let Some(_other) = crates.insert(krate, div) {
                    return Err(
                        format!("crate named {:?} appeared multiple times", crate_name).into(),
                    );
                }
            }
        }

        Ok(Part1 {
            stacks: self.initial.clone(),
            crate_divs: crates,
            steps: self.steps.clone().into(),
            current_crate: None,
            current_count: 0,
            current_from: 0,
            current_to: 0,
        })
    }
}

#[wasm_bindgen]
pub struct Part1 {
    stacks: Vec<Vec<u8>>,
    crate_divs: HashMap<u8, HtmlElement>,
    steps: VecDeque<(usize, usize, usize)>,

    current_crate: Option<HtmlElement>,
    current_count: usize,
    current_from: usize,
    current_to: usize,
}

#[wasm_bindgen]
impl Part1 {
    pub fn tick(&mut self, callback: &Function) -> Result<(), JsValue> {
        debug!(
            "top: {} from {} to {}",
            self.current_count, self.current_from, self.current_to
        );
        debug!("element is {:?}", self.current_crate);

        if let Some(ref current_crate) = self.current_crate.take() {
            current_crate.set_onanimationend(None);
            let style = current_crate.style();

            style.set_property("--stack", &style.get_property_value("--newStack")?)?;
            style.set_property("--index", &style.get_property_value("--newIndex")?)?;
            current_crate.set_class_name("");

            // force reflow so that the just-completed animation will reset, in
            // case we pick up the same crate later
            current_crate.offset_height();
        }

        if self.current_count == 0 {
            let Some((count, from, to)) = self.steps.pop_front() else {
                debug!("done!");
                return Ok(());
            };
            self.current_count = count;
            self.current_from = from;
            self.current_to = to;
            debug!(
                "refreshed, will move {} from {} to {}",
                self.current_count, self.current_from, self.current_to
            );
        }
        self.current_count -= 1;

        let moving = self.stacks[self.current_from - 1]
            .pop()
            .ok_or("empty stack")?;
        self.stacks[self.current_to - 1].push(moving);

        // TODO: does a clone()d HtmlElement actually copy the JS-side DOM object?  Or does it copy the reference to the same object?
        let moving_div = self
            .crate_divs
            .get(&moving)
            .expect("tried to move a crate that isn't in the HashMap")
            .clone();
        let style = moving_div.style();
        style.set_property("--newStack", &(self.current_to - 1).to_string())?;
        style.set_property(
            "--newIndex",
            &(self.stacks[self.current_to - 1].len() - 1).to_string(),
        )?;
        moving_div.set_class_name("moving");
        moving_div.set_onanimationend(Some(callback));
        self.current_crate = Some(moving_div);

        Ok(())
    }
}
