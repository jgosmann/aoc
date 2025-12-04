use std::collections::{BinaryHeap, HashMap, HashSet};

use crate::{
    datastructures::{grid::GridView, iterators::NeighborIterator2d},
    solvers::{Solution, Solver},
};

#[derive(Clone, PartialEq, Eq)]
struct QueueItem(usize, (usize, usize), HashSet<(usize, usize)>);

impl Ord for QueueItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl PartialOrd for QueueItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub struct SolverImpl<'input> {
    grid: GridView<&'input [u8]>,
}

impl<'input> Solver<'input> for SolverImpl<'input> {
    fn new(input: &'input str) -> anyhow::Result<Self> {
        let grid = GridView::from_separated(b'\n', input.as_bytes());

        Ok(Self { grid })
    }

    fn solve_part_1(&self) -> anyhow::Result<Solution> {
        let mut longest_path = HashSet::new();
        let mut queue = vec![((0, 1), HashSet::new())];
        while let Some((pos, mut visited)) = queue.pop() {
            if visited.contains(&pos) {
                continue;
            }

            if pos == (self.grid.height() - 1, self.grid.width() - 2)
                && visited.len() > longest_path.len()
            {
                longest_path = visited.clone();
            }

            visited.insert(pos);

            for neighbor in NeighborIterator2d::new(pos, self.grid.size()) {
                match self.grid[neighbor] {
                    b'.' => queue.push((neighbor, visited.clone())),
                    b'>' if pos.1 < neighbor.1 => queue.push((neighbor, visited.clone())),
                    b'<' if pos.1 > neighbor.1 => queue.push((neighbor, visited.clone())),
                    b'v' if pos.0 < neighbor.0 => queue.push((neighbor, visited.clone())),
                    b'^' if pos.0 > neighbor.0 => queue.push((neighbor, visited.clone())),
                    _ => (),
                }
            }
        }

        Ok(Solution::with_description(
            "Longest hike with icy patches",
            longest_path.len().to_string(),
        ))
    }

    fn solve_part_2(&self) -> anyhow::Result<Solution> {
        let graph = self.construct_graph();

        let mut longest_path_len = 0;
        let mut queue = BinaryHeap::from([QueueItem(0, (0, 1), HashSet::new())]);
        while let Some(QueueItem(path_len, pos, mut visited)) = queue.pop() {
            if visited.contains(&pos) {
                continue;
            }

            if pos == (self.grid.height() - 1, self.grid.width() - 2) && path_len > longest_path_len
            {
                longest_path_len = path_len;
            }

            visited.insert(pos);

            for &(neighbor, weight) in graph.get(&pos).unwrap_or(&vec![]) {
                queue.push(QueueItem(path_len + weight, neighbor, visited.clone()))
            }
        }

        Ok(Solution::with_description(
            "Longest hike",
            longest_path_len.to_string(),
        ))
    }
}

type Nodes = HashMap<(usize, usize), Vec<((usize, usize), usize)>>;

impl SolverImpl<'_> {
    fn construct_graph(&self) -> Nodes {
        let mut nodes = HashMap::new();

        let mut queue = vec![(0, 1)];
        while let Some(start) = queue.pop() {
            if nodes.contains_key(&start) {
                continue;
            }

            for neighbor in self.path_neighbors(start) {
                let mut length: usize = 0;
                let mut current = start;
                let mut prev;

                let mut next_candidates = vec![neighbor];
                while next_candidates.len() == 1 {
                    length += 1;
                    prev = current;
                    current = next_candidates.first().copied().unwrap();
                    next_candidates = self
                        .path_neighbors(current)
                        .filter(|&p| p != prev)
                        .collect();
                }

                queue.push(current);
                let entry = nodes.entry(start).or_insert(vec![]);
                entry.push((current, length));
            }
        }

        nodes
    }

    #[allow(unused)]
    fn print_dot_graph(nodes: &Nodes) {
        for (from, edges) in nodes {
            for (to, length) in edges {
                if from < to {
                    println!("  \"{from:?}\" -- \"{to:?}\" [label=\"{length}\"];");
                }
            }
        }
        println!("}}");
    }

    fn path_neighbors(&self, pos: (usize, usize)) -> impl Iterator<Item = (usize, usize)> + '_ {
        NeighborIterator2d::new(pos, self.grid.size()).filter(move |&p| self.grid[p] != b'#')
    }
}

#[cfg(test)]
mod test {
    use super::SolverImpl;
    use crate::solvers::Solver;

    #[test]
    fn test_example_part_1() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day23-1.example"))?;
        assert_eq!(solver.solve_part_1()?.solution, "94");
        Ok(())
    }

    #[test]
    fn test_example_part_2() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day23-1.example"))?;
        assert_eq!(solver.solve_part_2()?.solution, "154");
        Ok(())
    }
}
