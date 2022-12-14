use prelude::log::debug;
use prelude::*;
use wasm_bindgen::JsValue;

use Move::*;

#[derive(Debug)]
enum Move {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32),
}

impl TryFrom<&str> for Move {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match &value.split(' ').collect_vec()[..] {
            &[direction, count] => {
                let count = count
                    .parse()
                    .map_err(|e| format!("couldn't parse count: {:?}", e))?;
                match direction {
                    "U" => Ok(Up(count)),
                    "D" => Ok(Down(count)),
                    "L" => Ok(Left(count)),
                    "R" => Ok(Right(count)),
                    x => Err(format!("unexpected direction: {:?}", x)),
                }
            }
            v => Err(format!("expected a direction and a count, got {:?}", v)),
        }
    }
}

pub struct Solution {
    directions: Vec<Move>,
}

impl Solution {
    pub fn new(input: &str) -> Result<Solution, JsValue> {
        init();

        let directions = input
            .lines()
            .map(Move::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        debug!("parsed: {:#?}", directions);

        Ok(Solution { directions })
    }

    pub fn part1(&self) -> usize {
        self.simulate(2)
    }

    pub fn part2(&self) -> usize {
        self.simulate(10)
    }

    fn simulate(&self, knots: usize) -> usize {
        let mut positions = vec![(0_i32, 0_i32); knots];
        let mut visited = HashSet::new();

        for instruction in &self.directions {
            let (dx, dy, &count) = match instruction {
                Up(x) => (0, 1, x),
                Down(x) => (0, -1, x),
                Left(x) => (-1, 0, x),
                Right(x) => (1, 0, x),
            };

            for _ in 0..count {
                positions[0] = (positions[0].0 + dx, positions[0].1 + dy);

                // reconcile the position of each knot ('tail') to follow the knot in front of it
                // ('head')
                for tail_idx in 1..positions.len() {
                    let (head, tail) = positions.split_at_mut(tail_idx);
                    let head = head.last_mut().unwrap();
                    let tail = tail.first_mut().unwrap();

                    if head.0 == tail.0 {
                        // same row
                        if head.1 > tail.1 + 1 {
                            tail.1 += 1;
                        } else if head.1 < tail.1 - 1 {
                            tail.1 -= 1;
                        }
                    } else if head.1 == tail.1 {
                        // same column
                        if head.0 > tail.0 + 1 {
                            tail.0 += 1;
                        } else if head.0 < tail.0 - 1 {
                            tail.0 -= 1;
                        }
                    } else {
                        // something diagonal may happen
                        if (head.0 - tail.0).abs() > 1 || (head.1 - tail.1).abs() > 1 {
                            // they're not touching, so tail needs to move diagonally

                            // we know they're not in the same row or column if we get here, so these
                            // will always move tail left-or-right *and* up-or-down.
                            if head.0 > tail.0 {
                                tail.0 += 1;
                            } else {
                                tail.0 -= 1;
                            }

                            if head.1 > tail.1 {
                                tail.1 += 1;
                            } else {
                                tail.1 -= 1;
                            }
                        }
                    }
                }

                visited.insert(*positions.last().unwrap());
            }
        }

        visited.len()
    }
}
