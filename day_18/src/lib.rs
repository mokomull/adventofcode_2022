use std::collections::VecDeque;

use prelude::log::debug;
use prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Solution {
    cubes: HashSet<(i32, i32, i32)>,
}

#[wasm_bindgen]
impl Solution {
    pub fn new(input: &str) -> Self {
        init();

        let cubes = input
            .lines()
            .map(|line| {
                let split = line.split(',').collect_vec();
                (
                    split[0].parse().unwrap(),
                    split[1].parse().unwrap(),
                    split[2].parse().unwrap(),
                )
            })
            .collect();

        debug!("parsed {cubes:#?}");

        Solution { cubes }
    }

    pub fn part1(&self) -> u32 {
        let mut result = 0;

        for &(i, j, k) in &self.cubes {
            for (a, b, c) in [
                (i - 1, j, k),
                (i + 1, j, k),
                (i, j - 1, k),
                (i, j + 1, k),
                (i, j, k - 1),
                (i, j, k + 1),
            ] {
                if !self.cubes.contains(&(a, b, c)) {
                    result += 1;
                }
            }
        }

        result
    }

    pub fn part2(&self) -> u32 {
        let mut result = 0;

        let mut trapped = Trapped {
            cubes: &self.cubes,
            already_seen: Default::default(),
            maximum_dimension: self
                .cubes
                .iter()
                .flat_map(|&(i, j, k)| [i, j, k])
                .max()
                .unwrap()
                + 1,
        };

        for &(i, j, k) in &self.cubes {
            for (a, b, c) in [
                (i - 1, j, k),
                (i + 1, j, k),
                (i, j - 1, k),
                (i, j + 1, k),
                (i, j, k - 1),
                (i, j, k + 1),
            ] {
                if !self.cubes.contains(&(a, b, c)) && !trapped.is_trapped(a, b, c) {
                    result += 1;
                }
            }
        }

        result
    }
}

struct Trapped<'a> {
    cubes: &'a HashSet<(i32, i32, i32)>,
    already_seen: HashMap<(i32, i32, i32), bool>,
    maximum_dimension: i32,
}

impl<'a> Trapped<'a> {
    fn is_trapped(&mut self, i: i32, j: i32, k: i32) -> bool {
        if let Some(&already) = self.already_seen.get(&(i, j, k)) {
            return already;
        }

        let mut to_visit = VecDeque::from(vec![(i, j, k)]);
        let mut visited = [(i, j, k)].into_iter().collect::<HashSet<_>>();

        while let Some((i, j, k)) = to_visit.pop_front() {
            if (i, j, k) == (0, 0, 0) || self.already_seen.get(&(i, j, k)) == Some(&false) {
                // we've reached the outside, definitively, or something we already know was on the
                // outside
                for (a, b, c) in visited.into_iter() {
                    if let Some(old) = self.already_seen.insert((a, b, c), false) {
                        assert!(
                            !old,
                            "tried to flip the cached state of {:?} to false",
                            (a, b, c)
                        );
                    }
                }
                return false;
            }

            if self.already_seen.get(&(i, j, k)) == Some(&true) {
                // we've reached something we definitively know is trapped, so we must be trapped
                for (a, b, c) in visited.into_iter() {
                    if let Some(old) = self.already_seen.insert((a, b, c), true) {
                        assert!(
                            old,
                            "tried to flip the cached state of {:?} to true",
                            (a, b, c)
                        );
                    }
                }
                return true;
            }

            for (a, b, c) in [
                (i - 1, j, k),
                (i + 1, j, k),
                (i, j - 1, k),
                (i, j + 1, k),
                (i, j, k - 1),
                (i, j, k + 1),
            ] {
                if self.cubes.contains(&(a, b, c)) {
                    // we can't move through a cube
                    continue;
                }

                // confine ourselves to an NxNxN cube, since our input seems to be small integers;
                // even if it's something like 100x100x100 then we still only have at most a million
                // iterations
                if a > self.maximum_dimension
                    || b > self.maximum_dimension
                    || c > self.maximum_dimension
                    || a < 0
                    || b < 0
                    || c < 0
                {
                    continue;
                }

                if visited.contains(&(a, b, c)) {
                    continue;
                }

                visited.insert((a, b, c));
                to_visit.push_back((a, b, c));
            }
        }

        // if we've exhausted all our paths, and we still didn't reach (0,0,0), then we're trapped.
        for (a, b, c) in visited.into_iter() {
            if let Some(old) = self.already_seen.insert((a, b, c), true) {
                assert!(
                    old,
                    "tried to flip the cached state of {:?} to true",
                    (a, b, c)
                );
            }
        }
        return true;
    }
}
