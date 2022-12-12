use std::fmt::Debug;

use prelude::log::debug;
use prelude::*;
use wasm_bindgen::prelude::*;

use Instruction::*;

#[derive(Debug)]
enum Instruction {
    Noop,
    Addx(i64),
}

impl TryFrom<&str> for Instruction {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value == "noop" {
            Ok(Noop)
        } else if let Some(num) = value.strip_prefix("addx ") {
            let operand = num
                .parse()
                .map_err(|e| format!("error parsing operand: {:?}", e))?;
            Ok(Addx(operand))
        } else {
            Err(format!(
                "expected noop or addx instruction, got {:?}",
                value
            ))
        }
    }
}

impl Instruction {
    fn eval<F>(&self, x: i64, mut f: F) -> i64
    where
        F: FnMut(i64),
    {
        match self {
            Noop => {
                f(x);
                x
            }
            Addx(value) => {
                f(x);
                f(x);
                x + value
            }
        }
    }
}

#[wasm_bindgen]
pub struct Solution {
    instructions: Vec<Instruction>,
}

#[wasm_bindgen]
impl Solution {
    pub fn new(input: &str) -> Self {
        init();

        let instructions = input
            .lines()
            .map(Instruction::try_from)
            .collect::<Result<_, _>>()
            .unwrap();
        debug!("parsed: {:#?}", instructions);

        Self { instructions }
    }

    pub fn part1(&self) -> i64 {
        let mut x = 1;
        let mut result = 0;
        let mut cycle = 0;

        for instruction in &self.instructions {
            x = instruction.eval(x, |x| {
                cycle += 1;

                if cycle % 40 == 20 {
                    debug!("cycle {}, x = {}", cycle, x);
                    result += cycle * x;
                }
            })
        }

        debug!("completed {} cycles", cycle);

        result
    }
}
