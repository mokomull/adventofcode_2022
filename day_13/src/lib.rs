use std::cmp::Ordering;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::multi::separated_list0;
use nom::IResult;
use prelude::log::debug;
use prelude::*;
use wasm_bindgen::prelude::*;

use Packet::*;

#[derive(Debug, PartialEq, Eq)]
enum Packet {
    Integer(u32),
    List(Vec<Packet>),
}

fn parse_packet(input: &str) -> IResult<&str, Packet> {
    alt((
        |input| -> IResult<_, _> {
            let (input, number) = nom::character::complete::u32(input)?;
            Ok((input, Integer(number)))
        },
        |input| -> IResult<_, _> {
            let (input, _) = tag("[")(input)?;
            let (input, packets) = separated_list0(tag(","), parse_packet)(input)?;
            let (input, _) = tag("]")(input)?;
            Ok((input, List(packets)))
        },
    ))(input)
}

impl PartialOrd<Packet> for Packet {
    fn partial_cmp(&self, other: &Packet) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Packet) -> Ordering {
        match (self, other) {
            (Integer(i), Integer(j)) => i.cmp(&j),
            (List(left), List(right)) => {
                let mut left = left.as_slice();
                let mut right = right.as_slice();

                while !left.is_empty() {
                    if right.is_empty() {
                        return Ordering::Greater;
                    }

                    match left[0].cmp(&right[0]) {
                        Ordering::Less => return Ordering::Less,
                        Ordering::Greater => return Ordering::Greater,
                        Ordering::Equal => (), // keep checking the rest of the list!
                    }

                    left = &left[1..];
                    right = &right[1..];
                }

                if right.is_empty() {
                    Ordering::Equal
                } else {
                    Ordering::Less
                }
            }
            (Integer(i), List(_)) => List(vec![Integer(*i)]).cmp(other),
            (List(_), Integer(i)) => self.cmp(&List(vec![Integer(*i)])),
        }
    }
}

#[wasm_bindgen]
pub struct Solution {
    packets: Vec<(Packet, Packet)>,
}

#[wasm_bindgen]
impl Solution {
    pub fn new(input: &str) -> Self {
        init();

        let mut packets = vec![];

        let mut lines = input.lines();
        while let Some(line) = lines.next() {
            let (remaining, packet) = parse_packet(line).expect("couldn't parse");
            assert!(remaining.is_empty());
            let line = lines.next().expect("need two packets");
            let (remaining, second_packet) = parse_packet(line).expect("couldn't parse");
            assert!(remaining.is_empty());

            packets.push((packet, second_packet));

            // eat the intervening newline
            let _ = lines.next();
        }

        debug!("parsed: {:#?}", packets);

        Solution { packets }
    }

    pub fn part1(&self) -> usize {
        self.packets
            .iter()
            .enumerate()
            .filter_map(|(index, (left, right))| if left < right { Some(index + 1) } else { None })
            .sum()
    }
}
