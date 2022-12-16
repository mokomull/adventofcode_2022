use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::multi::separated_list1;
use nom::IResult;
use prelude::log::debug;
use prelude::*;
use wasm_bindgen::prelude::*;

#[derive(Debug)]
struct Valve {
    flow_rate: u32,
    neighbors: Vec<String>,
}

fn parse_valve(input: &str) -> IResult<&str, (String, Valve)> {
    let (input, _) = tag("Valve ")(input)?;
    let (input, name) = alpha1(input)?;
    let (input, _) = tag(" has flow rate=")(input)?;
    let (input, flow_rate) = nom::character::complete::u32(input)?;
    // because of course the input grammar has to be singular when there's only one valve
    let (input, neighbors) = alt((
        |input| {
            let (input, _) = tag("; tunnels lead to valves ")(input)?;
            separated_list1(tag(", "), alpha1)(input)
        },
        |input| {
            let (input, _) = tag("; tunnel leads to valve ")(input)?;
            let (input, neighbor) = alpha1(input)?;
            Ok((input, vec![neighbor]))
        },
    ))(input)?;

    Ok((
        input,
        (
            name.to_owned(),
            Valve {
                flow_rate,
                neighbors: neighbors.into_iter().map(str::to_owned).collect(),
            },
        ),
    ))
}

#[wasm_bindgen]
pub struct Solution {
    valves: HashMap<String, Valve>,
}

#[wasm_bindgen]
impl Solution {
    pub fn new(input: &str) -> Self {
        init();

        let valves = input
            .lines()
            .map(|line| {
                let (rest, valve) = parse_valve(line).expect("invalid input");
                assert!(rest.is_empty());
                valve
            })
            .collect();

        debug!("parsed: {:#?}", valves);

        Solution { valves }
    }
}
