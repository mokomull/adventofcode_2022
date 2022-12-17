use std::cmp::max;
use std::collections::BTreeSet;
use std::ops::ControlFlow;

use prelude::log::debug;
use prelude::*;
use wasm_bindgen::prelude::*;

use RockType::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum RockType {
    Underscore,
    Plus,
    Ell,
    Pipe,
    Square,
}

#[derive(Debug)]
struct Rock {
    rock_type: RockType,
    left_pos: u32,
    bottom_pos: u32,
}

impl Rock {
    fn move_left(&mut self, chamber: &HashSet<(u32, u32)>) {
        if self.left_pos == 0 {
            return;
        }

        // offsets from (left_pos - 1, bottom_pos), so we can do all the math as u32
        let left_offsets: &[(u32, u32)] = match self.rock_type {
            Underscore => &[(0, 0)],
            Plus => &[(1, 0), (0, 1), (1, 2)],
            Ell => &[(0, 0), (2, 1), (2, 2)],
            Pipe => &[(0, 0), (0, 1), (0, 2), (0, 3)],
            Square => &[(0, 0), (0, 1)],
        };

        if left_offsets
            .into_iter()
            .any(|&(left, up)| chamber.contains(&(self.left_pos - 1 + left, self.bottom_pos + up)))
        {
            return;
        }

        self.left_pos -= 1;
    }

    fn move_right(&mut self, chamber: &HashSet<(u32, u32)>) {
        let width = match self.rock_type {
            Underscore => 4,
            Plus | Ell => 3,
            Pipe => 1,
            Square => 2,
        };

        if self.left_pos + width == 7 {
            return;
        }
        assert!(self.left_pos + width < 7);

        // offsets from (left_pos, bottom_pos), thankfully, since we can just add unsigned numbers
        let offsets: &[(u32, u32)] = match self.rock_type {
            Underscore => &[(4, 0)],
            Plus => &[(2, 2), (3, 1), (2, 0)],
            Ell => &[(3, 2), (3, 1), (3, 0)],
            Pipe => &[(1, 3), (1, 2), (1, 1), (1, 0)],
            Square => &[(2, 1), (2, 0)],
        };

        if offsets
            .into_iter()
            .any(|&(left, up)| chamber.contains(&(self.left_pos + left, self.bottom_pos + up)))
        {
            return;
        }

        self.left_pos += 1;
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
                '<' => rock.move_left(&chamber),
                '>' => rock.move_right(&chamber),
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

    pub fn part2(&self) -> u64 {
        let mut count = 0;
        let mut next_rock_type = [Underscore, Plus, Ell, Pipe, Square].into_iter().cycle();
        let mut input = self.input.chars().enumerate().cycle();
        let mut chamber = HashSet::new();
        let mut max_height = 0;

        // (last rock placed, top 10 rows with the indexes shifted down to the "floor") -> height of
        // the first time we saw that situation
        let mut seen = HashMap::new();

        let mut rock = Rock {
            rock_type: next_rock_type.next().unwrap(),
            left_pos: 2,
            bottom_pos: 3,
        };

        let cycle_base;
        let cycle_start;

        loop {
            let (i, direction) = input.next().unwrap();
            match direction {
                '<' => rock.move_left(&chamber),
                '>' => rock.move_right(&chamber),
                x => panic!("unexpected character {x:?}"),
            }

            match rock.move_down(&mut chamber) {
                ControlFlow::Continue(()) => (),
                ControlFlow::Break(top) => {
                    max_height = max(max_height, top);
                    count += 1;

                    if max_height > 10 {
                        let top_ten_rows: BTreeSet<(u32, u32)> = chamber
                            .iter()
                            .filter_map(|&(left, up)| {
                                if up > max_height - 10 {
                                    Some((left, up + 10 - max_height))
                                } else {
                                    None
                                }
                            })
                            .collect();

                        if let Some(&(old_count, old_height)) =
                            seen.get(&(i, rock.rock_type, top_ten_rows.clone()))
                        {
                            log::info!("cycle after {count} rocks, previous count {old_count}, height {old_height}!");
                            cycle_start = old_count;
                            cycle_base = old_height;
                            break;
                        }

                        seen.insert((i, rock.rock_type, top_ten_rows), (count, max_height));
                    }

                    rock = Rock {
                        rock_type: next_rock_type.next().unwrap(),
                        left_pos: 2,
                        bottom_pos: max_height + 3,
                    };
                }
            }
        }

        let cycle_count = count - cycle_start;
        let final_cycle_height = max_height;
        let cycle_height = max_height - cycle_base;

        rock = Rock {
            rock_type: next_rock_type.next().unwrap(),
            left_pos: 2,
            bottom_pos: 3,
        };

        let mut count = 0;
        let target_count = 1000000000000u64 - cycle_start;
        loop {
            match input.next().expect("ran out of input").1 {
                '<' => rock.move_left(&chamber),
                '>' => rock.move_right(&chamber),
                x => panic!("unexpected character {x:?}"),
            }

            match rock.move_down(&mut chamber) {
                ControlFlow::Continue(()) => (),
                ControlFlow::Break(top) => {
                    max_height = max(max_height, top);
                    count += 1;
                    if count == target_count % (cycle_count as u64) {
                        return (cycle_height as u64) * (target_count / cycle_count as u64)
                            + (max_height as u64 - final_cycle_height as u64)
                            + cycle_base as u64;
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
