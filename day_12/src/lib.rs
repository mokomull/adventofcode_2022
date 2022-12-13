use petgraph::graph::Node;
use petgraph::prelude::*;
use prelude::log::debug;
use prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Solution {
    lines: Vec<String>,
}

type NodeName = (usize, usize);

#[wasm_bindgen]
impl Solution {
    pub fn new(input: &str) -> Self {
        init();

        Solution {
            lines: input.lines().map(str::to_owned).collect(),
        }
    }

    pub fn part1(&self) -> u32 {
        let (graph, start, end) = self.to_graph();

        let costs = petgraph::algo::dijkstra(&graph, end, Some(start), |_| 1);
        *costs.get(&start).unwrap()
    }

    pub fn part2(&self) -> u32 {
        let (graph, _, end) = self.to_graph();
        let costs = petgraph::algo::dijkstra(&graph, end, None, |_| 1);

        debug!("costs: {:#?}", costs);

        let mut min_cost = u32::MAX;

        for (i, row) in self.lines.iter().enumerate() {
            for (j, cell) in row.bytes().enumerate() {
                if cell == b'S' || cell == b'a' {
                    debug!("looking at {:?}", (i, j));
                    if let Some(&cost) = costs.get(&(i, j)) {
                        min_cost = std::cmp::min(min_cost, cost)
                    }
                }
            }
        }

        min_cost
    }

    fn to_graph(&self) -> (DiGraphMap<NodeName, ()>, NodeName, NodeName) {
        let mut graph = DiGraphMap::new();
        let mut start = None;
        let mut end = None;

        for (i, row) in self.lines.iter().enumerate() {
            for (j, cell) in row.bytes().enumerate() {
                let height = match cell {
                    b'S' => {
                        start = Some((i, j));
                        b'a'
                    }
                    b'E' => {
                        end = Some((i, j));
                        b'z'
                    }
                    x => x,
                };

                for (other_i, other_j) in [
                    (i.wrapping_sub(1), j),
                    (i + 1, j),
                    (i, j.wrapping_sub(1)),
                    (i, j + 1),
                ] {
                    let Some(&other_cell) = self.lines.get(other_i).and_then(|line| line.as_bytes().get(other_j)) else {
                        continue;
                    };
                    let other_height = match other_cell {
                        b'S' => b'a',
                        b'E' => b'z',
                        x => x,
                    };
                    if other_height <= height + 1 {
                        graph.add_edge((other_i, other_j), (i, j), ());
                    }
                }
            }
        }

        (
            graph,
            start.expect("didn't find S"),
            end.expect("didn't find E"),
        )
    }
}
