use crate::datastructures::iterators::NeighborIterator2d;
use crate::solvers::{Solution, Solver};
use anyhow::anyhow;
use std::collections::{BTreeSet, VecDeque};

type Pos = (usize, usize);

pub struct SolverImpl {
    byte_positions: Vec<Pos>,
}

impl<'input> Solver<'input> for SolverImpl {
    fn new(input: &'input str) -> anyhow::Result<Self> {
        let byte_positions = input
            .lines()
            .filter_map(|line| {
                if let Some((x, y)) = line.split_once(",") {
                    Some((x.parse::<usize>().ok()?, y.parse::<usize>().ok()?))
                } else {
                    None
                }
            })
            .collect();
        Ok(Self { byte_positions })
    }

    fn solve_part_1(&self) -> anyhow::Result<Solution> {
        Ok(Solution::with_description(
            "Part 1",
            self.solve_part_1_general((71, 71), 1024)?.to_string(),
        ))
    }

    fn solve_part_2(&self) -> anyhow::Result<Solution> {
        let (x, y) = self.solve_part_2_general((71, 71));
        Ok(Solution::with_description("Part 2", format!("{},{}", x, y)))
    }
}

impl SolverImpl {
    fn solve_part_1_general(
        &self,
        grid_size: (usize, usize),
        n_fallen: usize,
    ) -> anyhow::Result<usize> {
        let mut to_visit = VecDeque::new();
        to_visit.push_back((0, (0, 0)));
        let mut visited: BTreeSet<_> = self.byte_positions[..n_fallen].iter().copied().collect();
        visited.insert((0, 0));
        while let Some((steps, pos)) = to_visit.pop_front() {
            if pos == (grid_size.0 - 1, grid_size.1 - 1) {
                return Ok(steps);
            }

            for neighbor in NeighborIterator2d::new(pos, grid_size) {
                if visited.contains(&neighbor) {
                    continue;
                }
                visited.insert(neighbor);
                to_visit.push_back((steps + 1, neighbor));
            }
        }
        Err(anyhow!("No path found"))
    }

    fn solve_part_2_general(&self, grid_size: (usize, usize)) -> Pos {
        let mut left_bound = 0;
        let mut right_bound = self.byte_positions.len();
        while left_bound < right_bound {
            let mid = (left_bound + right_bound) / 2;
            if self.solve_part_1_general(grid_size, mid).is_ok() {
                left_bound = mid + 1;
            } else {
                right_bound = mid;
            }
        }
        self.byte_positions[left_bound - 1]
    }
}

#[cfg(test)]
mod test {
    use super::SolverImpl;
    use crate::solvers::Solver;

    #[test]
    fn test_example_part_1() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day18-1.example"))?;
        assert_eq!(solver.solve_part_1_general((7, 7), 12)?, 22);
        Ok(())
    }

    #[test]
    fn test_example_part_2() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day18-1.example"))?;
        assert_eq!(solver.solve_part_2_general((7, 7)), (6, 1));
        Ok(())
    }
}
