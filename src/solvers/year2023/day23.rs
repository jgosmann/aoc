use std::collections::HashSet;

use crate::{
    datastructures::{grid::GridView, iterators::NeighborIterator2d},
    solvers::{Solution, Solver},
};

pub struct SolverImpl {
    longest_path_len: usize,
}

impl<'input> Solver<'input> for SolverImpl {
    fn new(input: &'input str) -> anyhow::Result<Self> {
        let grid = GridView::from_separated(b'\n', input.as_bytes());

        let mut longest_path = HashSet::new();
        let mut queue = vec![((0, 1), HashSet::new())];
        while let Some((pos, mut visited)) = queue.pop() {
            if visited.contains(&pos) {
                continue;
            }

            if pos == (grid.height() - 1, grid.width() - 2) && visited.len() > longest_path.len() {
                longest_path = visited.clone();
            }

            visited.insert(pos);

            for neighbor in NeighborIterator2d::new(pos, grid.size()) {
                match grid[neighbor] {
                    b'.' => queue.push((neighbor, visited.clone())),
                    b'>' if pos.1 < neighbor.1 => queue.push((neighbor, visited.clone())),
                    b'<' if pos.1 > neighbor.1 => queue.push((neighbor, visited.clone())),
                    b'v' if pos.0 < neighbor.0 => queue.push((neighbor, visited.clone())),
                    b'^' if pos.0 > neighbor.0 => queue.push((neighbor, visited.clone())),
                    _ => (),
                }
            }
        }

        Ok(Self {
            longest_path_len: longest_path.len(),
        })
    }

    fn solve_part_1(&self) -> anyhow::Result<Solution> {
        Ok(Solution::with_description(
            "Longest hike",
            self.longest_path_len.to_string(),
        ))
    }

    fn solve_part_2(&self) -> anyhow::Result<Solution> {
        Ok(Solution::with_description(
            "Part 2",
            "not implemented".to_string(),
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
        assert_eq!(solver.solve_part_1()?.solution, "94");
        Ok(())
    }

    #[test]
    fn test_example_part_2() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day23-1.example"))?;
        assert_eq!(solver.solve_part_2()?.solution, "TODO");
        Ok(())
    }
}
