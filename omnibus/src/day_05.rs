use js_sys::Function;
use web_sys::{Document, HtmlDivElement};

use crate::*;

#[wasm_bindgen]
pub struct Day05(::day_05::Solution);

#[wasm_bindgen]
pub struct Day05Renderer(::day_05::Renderer);

#[wasm_bindgen]
impl Day05 {
    pub fn new(input: &str) -> Self {
        Self(::day_05::Solution::new(input))
    }

    pub fn make_renderer(
        &self,
        document: Document,
        target: &HtmlDivElement,
    ) -> Result<Day05Renderer, JsValue> {
        Ok(Day05Renderer(self.0.render(document, target)?))
    }
}

#[wasm_bindgen]
impl Day05Renderer {
    pub fn tick_part1(&mut self, callback: &Function) -> Result<(), JsValue> {
        self.0.tick_part1(callback)
    }

    pub fn tick_part2(&mut self, callback: &Function) -> Result<(), JsValue> {
        self.0.tick_part2(callback)
    }
}
