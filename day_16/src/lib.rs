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

    pub fn part1(&self) -> u32 {
        let mut graph = petgraph::graphmap::DiGraphMap::new();
        for (name, valve) in &self.valves {
            for target in &valve.neighbors {
                graph.add_edge(name.as_str(), target.as_str(), ());
            }
        }

        let distances = petgraph::algo::floyd_warshall(&graph, |_| 1).unwrap();
        debug!("computed distances:\n{distances:#?}");

        let nonzero_valves = self
            .valves
            .iter()
            .filter_map(|(name, valve)| (valve.flow_rate > 0).then_some(name.as_str()))
            .collect_vec();

        let mut result = 0;

        // is O(N!) really the way to do this?!
        'order: for ordering in nonzero_valves
            .iter()
            .copied()
            .permutations(nonzero_valves.len())
        {
            debug!("checking {ordering:?}");

            let mut time_remaining: i32 = 30;
            let mut released: i32 = 0;

            for (from, to) in ["AA"]
                .into_iter()
                .chain(ordering.iter().copied())
                .tuple_windows()
            {
                debug!("moving {} to get from {} to {}", distances[&(from, to)], from, to);
                // move to `to`
                time_remaining -= distances[&(from, to)];
                // open `to`
                time_remaining -= 1;

                debug!("time_remaining: {} * flow_rate: {}", time_remaining, self.valves[to].flow_rate);
                released += time_remaining * self.valves[to].flow_rate as i32;

                if time_remaining < 0 {
                    debug!("actually managed to run out of time: {ordering:?}");
                    // we can't use more than 30 minutes to visit all the valves!
                    continue 'order;
                }
            }

            result = std::cmp::max(result, released);
        }

        result as u32
    }
}
