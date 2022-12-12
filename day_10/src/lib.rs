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
}
