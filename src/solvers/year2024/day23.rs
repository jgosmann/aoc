use crate::solvers::{Solution, Solver};
use std::collections::BTreeSet;

struct Graph {
    num_vertices: usize,
    adjacency: Vec<bool>,
}

impl Graph {
    pub fn new(num_vertices: usize) -> Self {
        Self {
            num_vertices,
            adjacency: vec![false; num_vertices * num_vertices],
        }
    }

    pub fn add_edge(&mut self, from: usize, to: usize) {
        let index = self.connection_index(from, to);
        self.adjacency[index] = true;
        let index = self.connection_index(to, from);
        self.adjacency[index] = true;
    }

    pub fn has_edge(&self, from: usize, to: usize) -> bool {
        self.adjacency[self.connection_index(from, to)]
    }

    fn connection_index(&self, from: usize, to: usize) -> usize {
        from * self.num_vertices + to
    }
}

fn computer_index(computer: &[u8]) -> usize {
    (computer[0] - b'a') as usize * 26 + (computer[1] - b'a') as usize
}

fn computer_name(index: usize) -> String {
    format!(
        "{}{}",
        ((index / 26) as u8 + b'a') as char,
        ((index % 26) as u8 + b'a') as char
    )
}

pub struct SolverImpl {
    graph: Graph,
}

impl<'input> Solver<'input> for SolverImpl {
    fn new(input: &'input str) -> anyhow::Result<Self> {
        let mut graph = Graph::new(26 * 26);
        for line in input.lines() {
            if let Some((from, to)) = line.split_once('-') {
                let from = computer_index(from.trim().as_bytes());
                let to = computer_index(to.trim().as_bytes());
                graph.add_edge(from, to);
            }
        }
        Ok(Self { graph })
    }

    fn solve_part_1(&self) -> anyhow::Result<Solution> {
        let mut count = 0usize;
        for node0 in computer_index(b"ta")..=computer_index(b"tz") {
            for node1 in 0..26 * 26 {
                if (computer_index(b"ta")..=node0).contains(&node1) {
                    continue;
                }
                for node2 in node1 + 1..26 * 26 {
                    if (computer_index(b"ta")..=node0).contains(&node2) {
                        continue;
                    }
                    if self.graph.has_edge(node0, node1)
                        && self.graph.has_edge(node1, node2)
                        && self.graph.has_edge(node2, node0)
                    {
                        count += 1;
                    }
                }
            }
        }
        Ok(Solution::with_description("Part 1", count.to_string()))
    }

    fn solve_part_2(&self) -> anyhow::Result<Solution> {
        let mut largest_clique = BTreeSet::new();
        for seed_node in 0..26 * 26 {
            let mut clique = BTreeSet::new();
            clique.insert(seed_node);
            for node in seed_node + 1..26 * 26 {
                if clique
                    .iter()
                    .all(|&c_node| self.graph.has_edge(c_node, node))
                {
                    clique.insert(node);
                }
            }
            if clique.len() > largest_clique.len() {
                largest_clique = clique;
            }
        }
        let mut node_names: Vec<_> = largest_clique.iter().copied().map(computer_name).collect();
        node_names.sort_unstable();
        Ok(Solution::with_description(
            "Part 2",
            node_names.join(",").to_string(),
        ))
    }
}

#[cfg(test)]
mod test {
    use super::SolverImpl;
    use crate::solvers::Solver;

    #[test]
    fn test_example_part_1() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day23-1.example"))?;
        assert_eq!(solver.solve_part_1()?.solution, "7");
        Ok(())
    }

    #[test]
    fn test_example_part_2() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day23-1.example"))?;
        assert_eq!(solver.solve_part_2()?.solution, "co,de,ka,ta");
        Ok(())
    }
}
