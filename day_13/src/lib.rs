use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::multi::separated_list1;
use nom::IResult;
use prelude::log::debug;
use prelude::*;
use wasm_bindgen::prelude::*;

use Packet::*;

#[derive(Debug)]
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
            let (input, packets) = separated_list1(tag(","), parse_packet)(input)?;
            let (input, _) = tag("]")(input)?;
            Ok((input, List(packets)))
        },
    ))(input)
}

#[wasm_bindgen]
pub struct Solution {}

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

        Solution {}
    }
}
