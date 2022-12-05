use prelude::log::debug;
use prelude::*;
use std::cmp::Ordering::*;
use wasm_bindgen::prelude::*;
use Choice::*;

#[derive(Debug, PartialEq, Eq)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl std::cmp::PartialOrd for Choice {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::Ord for Choice {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (x, y) if x == y => Equal,
            (Rock, Scissors) | (Scissors, Paper) | (Paper, Rock) => Greater,
            (x, y) => y.cmp(x).reverse(),
        }
    }
}

impl TryFrom<&str> for Choice {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "A" | "X" => Ok(Rock),
            "B" | "Y" => Ok(Paper),
            "C" | "Z" => Ok(Scissors),
            _ => Err(()),
        }
    }
}

#[wasm_bindgen]
pub struct Solution {
    strategy_guide: Vec<(Choice, Choice)>,
}

#[wasm_bindgen]
impl Solution {
    pub fn new(input: &str) -> Self {
        prelude::init();

        let strategy_guide = input
            .lines()
            .map(|line| {
                let data = line.split(" ").collect_vec();
                let opponent = data[0].try_into().unwrap();
                let me = data[1].try_into().unwrap();
                (opponent, me)
            })
            .collect_vec();

        debug!("{:?}", strategy_guide);

        Solution { strategy_guide }
    }

    pub fn part1(&self) -> i64 {
        self.strategy_guide
            .iter()
            .map(|(opponent, me)| {
                let shape = match me {
                    Rock => 1,
                    Paper => 2,
                    Scissors => 3,
                };

                let result = match me.cmp(opponent) {
                    Less => 0,
                    Equal => 3,
                    Greater => 6,
                };

                debug!("{:?}", (shape, result));

                shape + result
            })
            .sum()
    }
}
