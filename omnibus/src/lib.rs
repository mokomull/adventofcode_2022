use wasm_bindgen::prelude::*;

macro_rules! common_day {
    ($krate: ident, $strukt: ident, $part1_result: ty, $part2_result: ty) => {
        #[wasm_bindgen]
        pub struct $strukt($krate::Solution);

        #[wasm_bindgen]
        impl $strukt {
            pub fn new(input: &str) -> Self {
                Self($krate::Solution::new(input))
            }

            pub fn part1(&self) -> $part1_result {
                self.0.part1()
            }

            pub fn part2(&self) -> $part2_result {
                self.0.part2()
            }
        }
    };
}

common_day!(day_01, Day01, u64, u64);
common_day!(day_02, Day02, i64, i64);
common_day!(day_03, Day03, u64, u64);
common_day!(day_07, Day07, Result<u64, JsValue>, Result<u64, JsValue>);
common_day!(day_11, Day11, u32, u64);
common_day!(day_12, Day12, u32, u32);
common_day!(day_13, Day13, usize, usize);
common_day!(day_17, Day17, u32, u64);
common_day!(day_18, Day18, u32, u32);
common_day!(day_20, Day20, i64, i64);
common_day!(day_21, Day21, i64, i64);
common_day!(day_23, Day23, i32, u32);
