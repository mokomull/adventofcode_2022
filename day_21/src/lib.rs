use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::multi::separated_list0;
use nom::IResult;
use prelude::log::debug;
use prelude::*;
use wasm_bindgen::prelude::*;

use Monkey::*;

#[derive(Debug)]
enum Monkey {
    Literal(i32),
    Add(String, String),
    Subtract(String, String),
    Multiply(String, String),
    Divide(String, String),
}

fn parse_monkey(input: &str) -> IResult<&str, (String, Monkey)> {
    let (input, name) = alpha1(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, monkey) = alt((
        parse_details::literal,
        parse_details::add,
        parse_details::subtract,
        parse_details::multiply,
        parse_details::divide,
    ))(input)?;

    Ok((input, (name.to_owned(), monkey)))
}

mod parse_details {
    use super::*;

    pub(super) fn literal(input: &str) -> IResult<&str, Monkey> {
        let (input, number) = nom::character::complete::i32(input)?;
        Ok((input, Literal(number)))
    }

    pub(super) fn add(input: &str) -> IResult<&str, Monkey> {
        let (input, m1) = alpha1(input)?;
        let (input, _) = tag(" + ")(input)?;
        let (input, m2) = alpha1(input)?;
        Ok((input, Add(m1.to_owned(), m2.to_owned())))
    }

    pub(super) fn subtract(input: &str) -> IResult<&str, Monkey> {
        let (input, m1) = alpha1(input)?;
        let (input, _) = tag(" - ")(input)?;
        let (input, m2) = alpha1(input)?;
        Ok((input, Subtract(m1.to_owned(), m2.to_owned())))
    }

    pub(super) fn multiply(input: &str) -> IResult<&str, Monkey> {
        let (input, m1) = alpha1(input)?;
        let (input, _) = tag(" * ")(input)?;
        let (input, m2) = alpha1(input)?;
        Ok((input, Multiply(m1.to_owned(), m2.to_owned())))
    }

    pub(super) fn divide(input: &str) -> IResult<&str, Monkey> {
        let (input, m1) = alpha1(input)?;
        let (input, _) = tag(" / ")(input)?;
        let (input, m2) = alpha1(input)?;
        Ok((input, Divide(m1.to_owned(), m2.to_owned())))
    }
}

#[wasm_bindgen]
pub struct Solution {
    monkeys: HashMap<String, Monkey>,
}

#[wasm_bindgen]
impl Solution {
    pub fn new(input: &str) -> Self {
        init();

        let monkeys = input.lines().map(|line| parse_monkey(line).unwrap().1).collect();
        debug!("parsed: {:?}", monkeys);

        Solution { monkeys }
    }
}
