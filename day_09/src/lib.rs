use prelude::log::debug;
use prelude::*;
use wasm_bindgen::prelude::*;

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

#[wasm_bindgen]
pub struct Solution {}

#[wasm_bindgen]
impl Solution {
    pub fn new(input: &str) -> Result<Solution, JsValue> {
        init();
        
        let directions = input
            .lines()
            .map(Move::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        debug!("parsed: {:#?}", directions);

        Ok(Solution {})
    }
}
