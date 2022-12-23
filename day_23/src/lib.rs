use itertools::MinMaxResult;
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

    pub fn part1(&self) -> i32 {
        let mut elves = self.elves.clone();

        for _round in 0..10 {
            let mut moves = HashMap::<(i32, i32), Vec<(i32, i32)>>::new();

            for &(from_row, from_col) in &elves {
                let check_north = [
                    (from_row - 1, from_col - 1),
                    (from_row - 1, from_col),
                    (from_row - 1, from_col + 1),
                ];
                let check_south = [
                    (from_row + 1, from_col - 1),
                    (from_row + 1, from_col),
                    (from_row + 1, from_col + 1),
                ];
                let check_west = [
                    (from_row - 1, from_col - 1),
                    (from_row, from_col - 1),
                    (from_row + 1, from_col - 1),
                ];
                let check_east = [
                    (from_row - 1, from_col + 1),
                    (from_row, from_col + 1),
                    (from_row + 1, from_col + 1),
                ];

                // if no elves are adjacent, we won't move at all
                let mut all_adjacent = check_north
                    .iter()
                    .chain(check_south.iter())
                    .chain(check_west.iter())
                    .chain(check_east.iter());
                if !all_adjacent.any(|pos| elves.contains(pos)) {
                    continue;
                }

                let can_move_north = !check_north.iter().any(|loc| elves.contains(loc));
                let can_move_south = !check_south.iter().any(|loc| elves.contains(loc));
                let can_move_west = !check_west.iter().any(|loc| elves.contains(loc));
                let can_move_east = !check_east.iter().any(|loc| elves.contains(loc));

                let to = if can_move_north {
                    (from_row - 1, from_col)
                } else if can_move_south {
                    (from_row + 1, from_col)
                } else if can_move_west {
                    (from_row, from_col - 1)
                } else if can_move_east {
                    (from_row, from_col + 1)
                } else {
                    // don't move at all; another elf won't want to move to this space anyway, since
                    // it'd be a false entry in that respective elf's can_move_<direction> iterator.
                    continue;
                };

                moves.entry(to).or_default().push((from_row, from_col));
            }

            for (to, froms) in moves {
                if froms.len() == 1 {
                    assert!(elves.remove(&froms[0]));
                    assert!(elves.insert(to));
                }
            }
        }

        let MinMaxResult::MinMax(min_row, max_row) = elves.iter().map(|(row, _)| row).minmax() else {
            panic!("ran out of elves?");
        };
        let MinMaxResult::MinMax(min_col, max_col) = elves.iter().map(|(_, col)| col).minmax() else {
            unreachable!();
        };

        (max_row - min_row) * (max_col - min_col) - elves.len() as i32
    }
}
