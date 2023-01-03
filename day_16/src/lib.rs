use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::multi::separated_list1;
use nom::IResult;
use prelude::log::{debug, info};
use prelude::*;

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

pub struct Solution {
    valves: HashMap<String, Valve>,
}

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

    pub fn part1(&self) -> i32 {
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

        max_flow_after_visiting(
            30,
            0,
            "AA",
            &HashSet::new(),
            &self.valves,
            &nonzero_valves,
            &distances,
        )
    }

    pub fn part2(&self) -> i32 {
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

        info!("nonzero valves: {}", nonzero_valves.len());

        let mut max_flow = 0;

        // choose that "myself" will visit the highest-order valve, without loss of generality; a
        // zero in one bit represents that "myself" will take it, a one in a bit represents that
        // "elephant" will take it
        for i in 0..(1 << (nonzero_valves.len() - 1)) {
            let mut myself = vec![];
            let mut elephant = vec![];

            for (j, valve) in nonzero_valves.iter().copied().enumerate() {
                if i & (1 << j) != 0 {
                    elephant.push(valve);
                } else {
                    myself.push(valve);
                }

                max_flow = std::cmp::max(
                    max_flow,
                    max_flow_after_visiting(
                        26,
                        0,
                        "AA",
                        &HashSet::new(),
                        &self.valves,
                        &myself,
                        &distances,
                    ) + max_flow_after_visiting(
                        26,
                        0,
                        "AA",
                        &HashSet::new(),
                        &self.valves,
                        &elephant,
                        &distances,
                    ),
                )
            }
        }

        max_flow
    }
}

fn max_flow_after_visiting(
    time_remaining: i32,
    released: i32,
    location: &str,
    already_visited: &HashSet<&str>,
    valves: &HashMap<String, Valve>,
    nonzero_valves: &[&str],
    distances: &HashMap<(&str, &str), i32>,
) -> i32 {
    // if we took longer than 30 minutes to get to this node, then this doesn't count as a maximum
    // path at all
    if time_remaining < 0 {
        return 0;
    }

    let mut visited = already_visited.clone();
    visited.insert(location);
    let visited = visited;

    let mut max_released = released;

    for &next in nonzero_valves {
        if visited.contains(&next) {
            continue;
        }

        // subtract 1 minute to account for turning `next` on.
        let time_remaining = time_remaining - distances[&(location, next)] - 1;
        max_released = std::cmp::max(
            max_released,
            max_flow_after_visiting(
                time_remaining,
                released + (time_remaining * valves[next].flow_rate as i32),
                next,
                &visited,
                valves,
                nonzero_valves,
                distances,
            ),
        );
    }

    debug!("max_flow_after_visiting {location}: time_remaining={time_remaining}, released={released}, already visited {already_visited:?} -> {max_released}");

    max_released
}
