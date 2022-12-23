use std::collections::{BTreeSet, VecDeque};

use itertools::MinMaxResult;
use prelude::log::debug;
use prelude::*;
use wasm_bindgen::prelude::*;

use Direction::*;

#[wasm_bindgen]
pub struct Solution {
    elves: HashSet<(i32, i32)>,
}

enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn can_move(&self, from: (i32, i32), elves: &BTreeSet<(i32, i32)>) -> bool {
        let (from_row, from_col) = from;
        let check = match self {
            North => [
                (from_row - 1, from_col - 1),
                (from_row - 1, from_col),
                (from_row - 1, from_col + 1),
            ],
            South => [
                (from_row + 1, from_col - 1),
                (from_row + 1, from_col),
                (from_row + 1, from_col + 1),
            ],
            West => [
                (from_row - 1, from_col - 1),
                (from_row, from_col - 1),
                (from_row + 1, from_col - 1),
            ],
            East => [
                (from_row - 1, from_col + 1),
                (from_row, from_col + 1),
                (from_row + 1, from_col + 1),
            ],
        };

        !check.iter().any(|to| elves.contains(to))
    }

    fn move_elf(&self, from: (i32, i32)) -> (i32, i32) {
        let (from_row, from_col) = from;
        match self {
            North => (from_row - 1, from_col),
            South => (from_row + 1, from_col),
            West => (from_row, from_col - 1),
            East => (from_row, from_col + 1),
        }
    }
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
        let mut elves = self.elves.iter().copied().collect::<BTreeSet<_>>();
        let mut directions = VecDeque::from(vec![North, South, West, East]);

        for _round in 0..10 {
            let moves = calculate_moves(&elves, &directions);

            for (to, froms) in moves {
                if froms.len() == 1 {
                    assert!(elves.remove(&froms[0]));
                    assert!(elves.insert(to));
                }
            }

            debug!("after {_round}: {elves:#?}");

            let first = directions.pop_front().unwrap();
            directions.push_back(first);
        }

        let MinMaxResult::MinMax(min_row, max_row) = elves.iter().map(|(row, _)| row).minmax() else {
            panic!("ran out of elves?");
        };
        let MinMaxResult::MinMax(min_col, max_col) = elves.iter().map(|(_, col)| col).minmax() else {
            unreachable!();
        };

        (max_row - min_row + 1) * (max_col - min_col + 1) - elves.len() as i32
    }
}

fn calculate_moves(
    elves: &BTreeSet<(i32, i32)>,
    directions: &VecDeque<Direction>,
) -> HashMap<(i32, i32), Vec<(i32, i32)>> {
    let mut moves = HashMap::<(i32, i32), Vec<(i32, i32)>>::new();

    for &from in elves {
        // if we can move in all four directions, then there aren't any elves adjacent and we shouldn't move
        if directions.iter().all(|d| d.can_move(from, &elves)) {
            continue;
        }

        let to = directions.iter().find_map(|d| {
            if d.can_move(from, &elves) {
                Some(d.move_elf(from))
            } else {
                None
            }
        });

        if let Some(to) = to {
            moves.entry(to).or_default().push(from);
        }
    }

    moves
}
