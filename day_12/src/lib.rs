use petgraph::prelude::*;
use prelude::log::debug;
use prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Solution {
    lines: Vec<String>,
}

#[wasm_bindgen]
impl Solution {
    pub fn new(input: &str) -> Self {
        init();

        Solution {
            lines: input.lines().map(str::to_owned).collect(),
        }
    }

    pub fn part1(&self) -> u32 {
        let (graph, start, end, _) = self.to_graph();

        let costs = petgraph::algo::dijkstra(&graph, end, Some(start), |e| *e.weight());
        *costs.get(&start).unwrap()
    }

    pub fn part2(&self) -> u32 {
        let (graph, _, end, indexes) = self.to_graph();
        let costs = petgraph::algo::dijkstra(&graph, end, None, |e| *e.weight());

        debug!("costs: {:#?}", costs);

        let mut min_cost = u32::MAX;

        for (i, row) in self.lines.iter().enumerate() {
            for (j, cell) in row.bytes().enumerate() {
                if cell == b'S' || cell == b'a' {
                    debug!("looking at {:?}", (i, j));
                    min_cost = std::cmp::min(
                        min_cost,
                        costs[&indexes[i][j]],
                    )
                }
            }  
        }

        min_cost
    }

    fn to_graph(&self) -> (DiGraph<u32, u32>, NodeIndex, NodeIndex, Vec<Vec<NodeIndex>>) {
        let mut graph = DiGraph::new();
        let mut indexes = vec![];
        let mut start = None;
        let mut end = None;

        for row in &self.lines {
            let mut row_indexes = vec![];
            for _cell in row.bytes() {
                row_indexes.push(graph.add_node(0));
            }
            indexes.push(row_indexes);
        }
        let indexes = indexes;

        for (i, row) in self.lines.iter().enumerate() {
            for (j, cell) in row.bytes().enumerate() {
                let height = match cell {
                    b'S' => {
                        start = Some(indexes[i][j]);
                        b'a'
                    }
                    b'E' => {
                        end = Some(indexes[i][j]);
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
                        graph.add_edge(indexes[other_i][other_j], indexes[i][j], 1);
                    }
                }
            }
        }

        (
            graph,
            start.expect("didn't find S"),
            end.expect("didn't find E"),
            indexes,
        )
    }
}
