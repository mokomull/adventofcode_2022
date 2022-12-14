use prelude::log::debug;
use prelude::*;

pub struct Solution {
    input: Vec<i64>,
}

impl Solution {
    pub fn new(input: &str) -> Solution {
        init();

        let input = input.lines().map(|line| line.parse().unwrap()).collect();
        debug!("parsed: {input:?}");

        Solution { input }
    }

    pub fn part1(&self) -> i64 {
        let arrangement = self.mix(self.input.iter().collect(), 1);

        let zero_position = arrangement
            .iter()
            .position(|&&element| element == 0)
            .expect("couldn't find zero");

        arrangement[(zero_position + 1000) % arrangement.len()]
            + arrangement[(zero_position + 2000) % arrangement.len()]
            + arrangement[(zero_position + 3000) % arrangement.len()]
    }

    pub fn part2(&self) -> i64 {
        let mut arrangement = self.input.iter().collect();

        const KEY: i64 = 811589153;

        for _round in 0..10 {
            arrangement = self.mix(arrangement, KEY);
        }

        let zero_position = arrangement
            .iter()
            .position(|&&element| element == 0)
            .expect("couldn't find zero");

        KEY * (arrangement[(zero_position + 1000) % arrangement.len()]
            + arrangement[(zero_position + 2000) % arrangement.len()]
            + arrangement[(zero_position + 3000) % arrangement.len()])
    }

    fn mix<'a>(&'a self, mut arrangement: Vec<&'a i64>, factor: i64) -> Vec<&'a i64> {
        // arrangement is a Vec of references so we can always find *exactly* the one we're looking for.

        for number in &self.input {
            let position = arrangement
                .iter()
                .position(|&element| std::ptr::eq(element, number))
                .expect("somehow lost a thing");
            arrangement.remove(position);

            let target = (position as i64) + (*number * factor);
            // the % operator always returns something the same sign as its left-hand side
            let target = target % (arrangement.len() as i64);
            let target = if target <= 0 {
                target + (arrangement.len() as i64)
            } else {
                target
            };

            arrangement.insert(
                target.try_into().expect("target shouldn't be negative..."),
                number,
            );

            debug!("found {number} at {position}; moved it to {target}\narrangement is now {arrangement:?}");
        }

        debug!("final arrangement is {arrangement:?}");

        arrangement
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn example() {
        let solution = Solution::new(
            "1
2
-3
3
-2
0
4",
        );

        assert_eq!(solution.part1(), 3);
        assert_eq!(solution.part2(), 1623178306);
    }

    #[test]
    fn real_data() {
        let solution = Solution::new(include_str!("input.txt"));
        assert_eq!(solution.part1(), 3346);
        assert_eq!(solution.part2(), 4265712588168);
    }
}
