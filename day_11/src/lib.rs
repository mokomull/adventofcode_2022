use prelude::log::debug;
use prelude::*;
use wasm_bindgen::prelude::*;

struct Monkey {
    items: Vec<u32>,
    operation: Box<dyn Fn(u32) -> u32>,
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
            let operation: Box<dyn Fn(u32) -> u32> = if operation_str == "* old" {
                Box::new(|old| old * old)
            } else if let Some(other) = operation_str.strip_prefix("* ") {
                let other = other.parse::<u32>().unwrap();
                Box::new(move |old| old * other)
            } else if let Some(other) = operation_str.strip_prefix("+ ") {
                let other = other.parse::<u32>().unwrap();
                Box::new(move |old| old + other)
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
}
