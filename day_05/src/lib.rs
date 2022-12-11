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

    pub fn render(&self, document: Document, target: &HtmlDivElement) -> Result<Renderer, JsValue> {
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

                if crates.insert((stack_id, crate_height), div).is_some() {
                    panic!(
                        "inserted stack_id {}, height {} into crates twice",
                        stack_id, crate_height
                    );
                }
            }
        }

        Ok(Renderer {
            stacks: self.initial.iter().map(Vec::len).collect(),
            crate_divs: crates,
            steps: self.steps.clone().into(),
            currently_moving: vec![],
            current_count: 0,
            current_from: 0,
            current_to: 0,
            generation: 1,
        })
    }
}

#[wasm_bindgen]
pub struct Renderer {
    stacks: Vec<usize>,                               // height of each stack
    crate_divs: HashMap<(usize, usize), HtmlElement>, // (stack id, height) => element representing a crate
    steps: VecDeque<(usize, usize, usize)>,

    currently_moving: Vec<HtmlElement>,
    current_count: usize,
    current_from: usize,
    current_to: usize,

    generation: usize,
}

#[wasm_bindgen]
impl Renderer {
    // despite all the storage of from/to being one-indexed, do_move is called with *zero*-indexed
    // stack ids.
    fn tick<F>(&mut self, generation: Option<String>, mut do_move: F) -> Result<(), JsValue>
    where
        F: FnMut(&mut Self, usize, usize, usize) -> Result<Vec<HtmlElement>, JsValue>,
    {
        debug!(
            "top: called for generation {:?}, moving {} from {} to {}",
            generation, self.current_count, self.current_from, self.current_to
        );
        debug!("element is {:?}", self.currently_moving);

        if let Some(g) = generation {
            if g != self.generation.to_string() {
                debug!("called for a mismatching generation {:?}, skipping", g);
                return Ok(());
            }
        }
        self.generation += 1;
        debug!("generation is now {}", self.generation);

        for current_crate in self.currently_moving.drain(..) {
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

        self.currently_moving = do_move(
            self,
            self.current_count,
            self.current_from - 1,
            self.current_to - 1,
        )?;

        Ok(())
    }

    // from and to are *zero*-based stack ids
    fn move_crates(
        &mut self,
        count: usize,
        from: usize,
        to: usize,
        animation_callback: &Function,
    ) -> Result<Vec<HtmlElement>, JsValue> {
        self.current_count -= count;

        self.stacks[from] -= count;
        // heights are zero-indexed, so take these snapshots while the lengths of both stacks are
        // already at their shortest
        let from_height = self.stacks[from];
        let to_height = self.stacks[to];
        self.stacks[to] += count;

        let mut moving_divs = vec![];

        for i in 0..count {
            let moving_div = self
                .crate_divs
                .remove(&(from, from_height + i))
                .expect("tried to move a crate that isn't in the HashMap");
            let style = moving_div.style();
            style.set_property("--newStack", &to.to_string())?;
            style.set_property("--newIndex", &(to_height + i).to_string())?;
            moving_div.set_class_name("moving");

            // TODO: does a clone()d HtmlElement actually copy the JS-side DOM object?  Or does it
            // copy the reference to the same object?
            if self
                .crate_divs
                .insert((to, (to_height + i)), moving_div.clone())
                .is_some()
            {
                panic!("tried to reinsert a crate where another crate already exists");
            }

            moving_divs.push(moving_div);
        }
        // only set the callback on *one* of the elements, and use that one as the indication to
        // progress to the next animation
        moving_divs
            .get(0)
            .expect("somehow didn't move any crates")
            .set_onanimationend(Some(&animation_callback.bind1(
                &JsValue::NULL,
                &JsValue::from_str(&self.generation.to_string()),
            )));

        Ok(moving_divs)
    }

    pub fn tick_part1(
        &mut self,
        generation: Option<String>,
        callback: &Function,
    ) -> Result<(), JsValue> {
        self.tick(
            generation,
            move |this: &mut Self, _count, from, to| -> Result<Vec<HtmlElement>, JsValue> {
                this.move_crates(1, from, to, callback)
            },
        )
    }

    pub fn tick_part2(
        &mut self,
        generation: Option<String>,
        callback: &Function,
    ) -> Result<(), JsValue> {
        self.tick(generation, move |this, count, from, to| {
            this.move_crates(count, from, to, callback)
        })
    }
}
