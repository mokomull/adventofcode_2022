use prelude::log::debug;
use prelude::*;

pub struct Solution {
    rucksacks: Vec<(Vec<u8>, Vec<u8>)>,
}

fn priority(c: u8) -> u64 {
    match c {
        b'a'..=b'z' => (c - b'a') as u64 + 1,
        b'A'..=b'Z' => (c - b'A') as u64 + 27,
        _ => panic!("unexpected letter {:?}", c),
    }
}

impl Solution {
    pub fn new(input: &str) -> Self {
        init();

        let rucksacks = input
            .lines()
            .map(|line| {
                let line = line.as_bytes();
                let (first, second) = line.split_at(line.len() / 2);

                (first.to_vec(), second.to_vec())
            })
            .collect();

        debug!("rucksacks: {:x?}", rucksacks);

        Self { rucksacks }
    }

    pub fn part1(&self) -> u64 {
        self.rucksacks
            .iter()
            .map(|(first, second)| {
                let first = first.iter().collect::<HashSet<_>>();
                let second = second.iter().collect::<HashSet<_>>();

                let overlap = first.intersection(&second).collect_vec();
                assert_eq!(overlap.len(), 1);

                priority(**overlap[0])
            })
            .sum()
    }

    pub fn part2(&self) -> u64 {
        self.rucksacks
            .iter()
            .map(|(first, second)| first.iter().chain(second.iter()).collect::<HashSet<_>>())
            .tuples()
            .map(|(elf, alf, ulf)| {
                let all = elf.intersection(&alf).copied().collect::<HashSet<_>>();
                let all = all.intersection(&ulf).copied().collect_vec();
                debug!("all: {:?}", all);
                assert_eq!(all.len(), 1);

                priority(*all[0])
            })
            .sum()
    }
}
