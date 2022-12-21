use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::IResult;
use prelude::log::debug;
use prelude::*;
use wasm_bindgen::prelude::*;

use Monkey::*;

#[derive(Debug, Clone)]
enum Monkey {
    Literal(i64),
    Add(String, String),
    Subtract(String, String),
    Multiply(String, String),
    Divide(String, String),
    Unknown,
}

impl Monkey {
    fn eval(&self, monkeys: &HashMap<String, Monkey>) -> Option<i64> {
        match self {
            Literal(i) => Some(*i),
            Add(m1, m2) => monkeys[m1].eval(monkeys).and_then(|m1_result| {
                monkeys[m2]
                    .eval(monkeys)
                    .map(|m2_result| m1_result + m2_result)
            }),
            Subtract(m1, m2) => monkeys[m1].eval(monkeys).and_then(|m1_result| {
                monkeys[m2]
                    .eval(monkeys)
                    .map(|m2_result| m1_result - m2_result)
            }),
            Multiply(m1, m2) => monkeys[m1].eval(monkeys).and_then(|m1_result| {
                monkeys[m2]
                    .eval(monkeys)
                    .map(|m2_result| m1_result * m2_result)
            }),
            Divide(m1, m2) => monkeys[m1].eval(monkeys).and_then(|m1_result| {
                monkeys[m2]
                    .eval(monkeys)
                    .map(|m2_result| m1_result / m2_result)
            }),
            Unknown => None,
        }
    }
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
        let (input, number) = nom::character::complete::i64(input)?;
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

        let monkeys = input
            .lines()
            .map(|line| parse_monkey(line).unwrap().1)
            .collect();
        debug!("parsed: {:?}", monkeys);

        Solution { monkeys }
    }

    pub fn part1(&self) -> i64 {
        self.monkeys["root"].eval(&self.monkeys).unwrap()
    }

    pub fn part2(&self) -> i64 {
        let (left, right) = match &self.monkeys["root"] {
            Add(l, r) | Subtract(l, r) | Multiply(l, r) | Divide(l, r) => {
                (&self.monkeys[l], &self.monkeys[r])
            }
            _ => panic!("root monkey doesn't have two sides"),
        };

        let mut monkeys = self.monkeys.clone();
        monkeys
            .insert("humn".to_owned(), Unknown)
            .expect("humn was not previously in the list?!");
        let monkeys = monkeys;

        let (target, unknown) = match (left.eval(&monkeys), right.eval(&monkeys)) {
            (Some(x), None) => (x, right),
            (None, Some(x)) => (x, left),
            (None, None) => panic!("neither side is a known value"),
            (Some(_), Some(_)) => panic!("both sides are known!"),
        };
        debug!("will try to make one side equal {target}");

        unimplemented!()
    }
}
