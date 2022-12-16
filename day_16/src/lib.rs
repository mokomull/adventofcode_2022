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
        #[derive(Debug)]
        struct State<'a> {
            location: &'a str,
            opened_yet: bool,
            opened_valves: HashSet<&'a str>,
            visited: HashSet<&'a str>, // assume we'll never come back to a valve we've passed-up
            released: u32,
        }

        let mut states = vec![State {
            location: "AA",
            opened_yet: false,
            opened_valves: HashSet::new(),
            visited: HashSet::new(),
            released: 0,
        }];

        for minute in 0..30 {
            let mut new_states = vec![];

            log::info!(
                "Beginning of minute {}, there are currently {} states",
                minute,
                states.len()
            );
            debug!("Beginning of minute {minute}, states:\n{states:#?}");

            for state in states.into_iter() {
                let released = state.released
                    + state
                        .opened_valves
                        .iter()
                        .map(|&name| self.valves[name].flow_rate)
                        .sum::<u32>();

                let mut visited = state.visited;
                visited.insert(state.location);
                let visited = visited;

                if state.opened_yet {
                    let mut found_neighbor = false;
                    for next in &self.valves[state.location].neighbors {
                        if !visited.contains(next.as_str()) {
                            found_neighbor = true;
                            new_states.push(State {
                                location: next,
                                opened_yet: false,
                                released,
                                visited: visited.clone(),
                                opened_valves: state.opened_valves.clone(),
                            })
                        }
                    }

                    // and we're always allowed to stay put, but it only makes sense to do that if
                    // we've already opened the valve we're at
                    if !found_neighbor {
                        new_states.push(State {
                            released,
                            visited,
                            ..state
                        });
                    }
                } else {
                    // what if we moved on without opening it?
                    for next in &self.valves[state.location].neighbors {
                        if !visited.contains(next.as_str()) {
                            new_states.push(State {
                                location: next,
                                opened_yet: false,
                                released,
                                visited: visited.clone(),
                                opened_valves: state.opened_valves.clone(),
                            })
                        }
                    }

                    // but also what if we stuck around to open it?
                    let mut opened_valves = state.opened_valves;
                    opened_valves.insert(state.location);
                    new_states.push(State {
                        location: state.location,
                        opened_yet: true,
                        released,
                        visited,
                        opened_valves,
                    })
                }
            }

            states = new_states;
        }

        states
            .into_iter()
            .map(|state| state.released)
            .max()
            .expect("no states!?")
    }
}
