use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::IResult;
use prelude::log::debug;
use prelude::*;

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

    fn invert(&self, target: i64, monkeys: &HashMap<String, Monkey>) -> i64 {
        let (left, right) = match self {
            Add(l, r) | Subtract(l, r) | Multiply(l, r) | Divide(l, r) => {
                (&monkeys[l], &monkeys[r])
            }
            Unknown => return target,
            Literal(_) => panic!("can't invert a literal!"),
        };

        match (self, left.eval(monkeys), right.eval(monkeys)) {
            // Add and Multiply are commutative, so their new targets should be equal in either direction
            (Add(_, _), Some(x), None) => right.invert(target - x, monkeys),
            (Add(_, _), None, Some(x)) => left.invert(target - x, monkeys),
            (Multiply(_, _), Some(x), None) => {
                assert_eq!(target % x, 0);
                right.invert(target / x, monkeys)
            }
            (Multiply(_, _), None, Some(x)) => {
                assert_eq!(target % x, 0);
                left.invert(target / x, monkeys)
            }
            // Subtract and Divide are not
            (Subtract(_, _), Some(x), None) => right.invert(x - target, monkeys),
            (Subtract(_, _), None, Some(x)) => left.invert(target + x, monkeys),
            (Divide(_, _), Some(x), None) => {
                assert_eq!(x % target, 0);
                right.invert(x / target, monkeys)
            }
            (Divide(_, _), None, Some(x)) => left.invert(target * x, monkeys),
            // we already weeded these out immediately upon entering invert().
            (Literal(_), _, _) | (Unknown, _, _) => unreachable!(),
            // and if neither/both sides is unknown, that's a problem
            (_, None, None) => panic!("both sides are unknown!"),
            (_, Some(_), Some(_)) => panic!("both sides are known!"),
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

pub struct Solution {
    monkeys: HashMap<String, Monkey>,
}

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

        unknown.invert(target, &monkeys)
    }
}
