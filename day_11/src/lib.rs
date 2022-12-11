use std::rc::Rc;

use prelude::log::debug;
use prelude::*;
use wasm_bindgen::prelude::*;

#[derive(Clone)]
struct Monkey {
    items: Vec<u32>,
    operation: Rc<dyn Fn(u32) -> u32>,
    divisor: u32,
    true_target: usize,
    false_target: usize,
}

#[wasm_bindgen]
pub struct Solution {
    monkeys: Vec<Monkey>,
}

#[wasm_bindgen]
impl Solution {
    pub fn new(input: &str) -> Self {
        init();

        let mut lines = input.lines();
        let mut monkeys = vec![];

        while let Some(line) = lines.next() {
            debug!("line: {:?}", line);

            let monkey_id = line
                .strip_prefix("Monkey ")
                .expect("expected a monkey!")
                .strip_suffix(":")
                .unwrap()
                .parse::<usize>()
                .unwrap();
            // make sure we aren't given monkeys out of order
            assert_eq!(monkey_id, monkeys.len());

            let items = lines
                .next()
                .expect("unexpected EOF")
                .strip_prefix("  Starting items: ")
                .unwrap()
                .split(", ")
                .map(|word| word.parse().unwrap())
                .collect();

            let operation_str = lines
                .next()
                .expect("unexpected EOF")
                .strip_prefix("  Operation: new = old ")
                .unwrap();
            let operation: Rc<dyn Fn(u32) -> u32> = if operation_str == "* old" {
                Rc::new(|old| old * old)
            } else if let Some(other) = operation_str.strip_prefix("* ") {
                let other = other.parse::<u32>().unwrap();
                Rc::new(move |old| old * other)
            } else if let Some(other) = operation_str.strip_prefix("+ ") {
                let other = other.parse::<u32>().unwrap();
                Rc::new(move |old| old + other)
            } else {
                panic!("could not parse operation {:?}", operation_str);
            };

            let divisor = lines
                .next()
                .expect("unexpected EOF")
                .strip_prefix("  Test: divisible by ")
                .unwrap()
                .parse()
                .unwrap();

            let true_target = lines
                .next()
                .expect("unexpected EOF")
                .strip_prefix("    If true: throw to monkey ")
                .unwrap()
                .parse()
                .unwrap();

            let false_target = lines
                .next()
                .expect("unexpected EOF")
                .strip_prefix("    If false: throw to monkey ")
                .unwrap()
                .parse()
                .unwrap();

            match lines.next() {
                None => (),
                Some("") => (),
                Some(x) => panic!("expected empty line, got {:?}", x),
            }

            monkeys.push(Monkey {
                items,
                operation,
                divisor,
                true_target,
                false_target,
            })
        }

        Solution { monkeys }
    }

    pub fn part1(&self) -> u32 {
        let mut monkeys = self.monkeys.clone();
        let mut activity = vec![0; monkeys.len()];

        for _round in 0..20 {
            for i in 0..monkeys.len() {
                for item in std::mem::take(&mut monkeys[i].items) {
                    activity[i] += 1;

                    let item = (monkeys[i].operation)(item);
                    let item = item / 3;

                    let target = if item % monkeys[i].divisor == 0 {
                        monkeys[i].true_target
                    } else {
                        monkeys[i].false_target
                    };

                    monkeys[target].items.push(item);
                }
            }
        }

        activity.sort_by_key(|&i| std::cmp::Reverse(i));
        activity.iter().take(2).product()
    }

    pub fn part2(&self) -> u32 {
        let mut monkeys = self.monkeys.clone();
        let mut activity = vec![0; monkeys.len()];

        // we'll do all our math modulo the product of all the divisors, since they seem to be small
        // primes.
        //
        // TODO: find the theorem that says we can do math modulo p1*p2*p3*...
        let modulus: u32 = self.monkeys.iter().map(|i| i.divisor).product();

        for _round in 0..20 {
            for i in 0..monkeys.len() {
                for item in std::mem::take(&mut monkeys[i].items) {
                    activity[i] += 1;

                    let item = (monkeys[i].operation)(item);
                    let item = item % modulus;

                    let target = if item % monkeys[i].divisor == 0 {
                        monkeys[i].true_target
                    } else {
                        monkeys[i].false_target
                    };

                    monkeys[target].items.push(item);
                }
            }

            debug!("activity: {:?}", activity);
        }

        activity.sort_by_key(|&i| std::cmp::Reverse(i));
        activity.iter().take(2).product()
    }
}
