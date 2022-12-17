use std::cmp::max;
use std::ops::ControlFlow;

use prelude::log::debug;
use prelude::*;
use wasm_bindgen::prelude::*;

use RockType::*;

#[derive(Clone)]
enum RockType {
    Underscore,
    Plus,
    Ell,
    Pipe,
    Square,
}

struct Rock {
    rock_type: RockType,
    left_pos: u32,
    bottom_pos: u32,
}

impl Rock {
    fn move_left(&mut self) {
        if self.left_pos > 0 {
            self.left_pos -= 1;
        }
    }

    fn move_right(&mut self) {
        let width = match self.rock_type {
            Underscore => 4,
            Plus | Ell => 3,
            Pipe => 1,
            Square => 2,
        };

        if self.left_pos + width < 7 {
            self.left_pos += 1;
        }
    }

    // Break(u32) represents one-past-the-top of the height of the brick, if it ossifies.
    fn move_down(&mut self, chamber: &mut HashSet<(u32, u32)>) -> ControlFlow<u32> {
        if self.bottom_pos == 0 {
            return self.ossify(chamber);
        }

        // offsets from the row *below* this rock, so we can continue working in u32s, in
        // (horizontal, vertical) order.
        let below_offsets: &[(u32, u32)] = match self.rock_type {
            Underscore => &[(0, 0), (1, 0), (2, 0), (3, 0)],
            Plus => &[(0, 1), (1, 0), (2, 1)],
            Ell => &[(0, 0), (1, 0), (2, 0)],
            Pipe => &[(0, 0)],
            Square => &[(0, 0), (1, 0)],
        };

        if below_offsets
            .into_iter()
            .any(|&(left, up)| chamber.contains(&(self.left_pos + left, self.bottom_pos - 1 + up)))
        {
            return self.ossify(chamber);
        }

        self.bottom_pos -= 1;
        ControlFlow::Continue(())
    }

    fn ossify(&self, chamber: &mut HashSet<(u32, u32)>) -> ControlFlow<u32> {
        // offsets from (left_pos, bottom_pos)
        let offsets: &[(u32, u32)] = match self.rock_type {
            Underscore => &[(0, 0), (1, 0), (2, 0), (3, 0)],
            Plus => &[(0, 1), (1, 2), (1, 1), (1, 0), (2, 1)],
            Ell => &[(0, 0), (1, 0), (2, 2), (2, 1), (2, 0)],
            Pipe => &[(0, 3), (0, 2), (0, 1), (0, 0)],
            Square => &[(0, 1), (0, 0), (1, 1), (1, 0)],
        };

        let height = match self.rock_type {
            Underscore => 1,
            Plus | Ell => 3,
            Pipe => 4,
            Square => 2,
        };

        for &(left, up) in offsets {
            chamber.insert((self.left_pos + left, self.bottom_pos + up));
        }

        ControlFlow::Break(self.bottom_pos + height)
    }
}

#[wasm_bindgen]
pub struct Solution {
    input: String,
}

#[wasm_bindgen]
impl Solution {
    pub fn new(input: &str) -> Self {
        init();

        Solution {
            input: input.to_owned(),
        }
    }

    pub fn part1(&self) -> u32 {
        let mut count = 0;
        let mut next_rock_type = [Underscore, Plus, Ell, Pipe, Square].into_iter().cycle();
        let mut input = self.input.chars().cycle();
        let mut chamber = HashSet::new();
        let mut max_height = 0;

        let mut rock = Rock {
            rock_type: next_rock_type.next().unwrap(),
            left_pos: 2,
            bottom_pos: 3,
        };

        loop {
            match input.next().expect("ran out of input") {
                '<' => rock.move_left(),
                '>' => rock.move_right(),
                x => panic!("unexpected character {x:?}"),
            }

            match rock.move_down(&mut chamber) {
                ControlFlow::Continue(()) => (),
                ControlFlow::Break(top) => {
                    max_height = max(max_height, top);
                    count += 1;
                    if count == 2022 {
                        return max_height;
                    }

                    rock = Rock {
                        rock_type: next_rock_type.next().unwrap(),
                        left_pos: 2,
                        bottom_pos: max_height + 3,
                    };
                }
            }
        }
    }
}
