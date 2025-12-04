use crate::datastructures::grid::GridView;
use crate::datastructures::iterators::SurroundIterator2d;
use crate::solvers::{Solution, Solver};

pub struct SolverImpl<'input> {
    grid: GridView<&'input [u8]>,
}

impl<'input> Solver<'input> for SolverImpl<'input> {
    fn new(input: &'input str) -> anyhow::Result<Self> {
        let grid = GridView::from_separated(b'\n', input.as_bytes());
        Ok(Self { grid })
    }

    fn solve_part_1(&self) -> anyhow::Result<Solution> {
        let n_accessible = self
            .grid
            .iter()
            .enumerate()
            .filter(|&(i, value)| {
                value == b'@'
                    && SurroundIterator2d::new(self.grid.nth_index(i), self.grid.size())
                        .filter(|&neighbor_idx| self.grid[neighbor_idx] == b'@')
                        .count()
                        < 4
            })
            .count();
        Ok(Solution::with_description(
            "Number of accessible paper rolls",
            n_accessible.to_string(),
        ))
    }

    fn solve_part_2(&self) -> anyhow::Result<Solution> {
        let mut grid = self.grid.to_owned();
        let mut total_removed = 0;
        let mut has_removed = true;
        while has_removed {
            has_removed = false;
            let to_remove: Vec<_> = grid
                .iter()
                .enumerate()
                .filter(|&(i, value)| {
                    value == b'@'
                        && SurroundIterator2d::new(grid.nth_index(i), grid.size())
                            .filter(|&neighbor_idx| grid[neighbor_idx] == b'@')
                            .count()
                            < 4
                })
                .map(|(i, _)| grid.nth_index(i))
                .collect();
            total_removed += to_remove.len();
            for idx in to_remove {
                grid[idx] = b'.';
                has_removed = true;
            }
        }
        Ok(Solution::with_description(
            "Part 2",
            total_removed.to_string(),
        ))
    }
}

#[cfg(test)]
mod test {
    use super::SolverImpl;
    use crate::solvers::Solver;

    #[test]
    fn test_example_part_1() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day4-1.example"))?;
        assert_eq!(solver.solve_part_1()?.solution, "13");
        Ok(())
    }

    #[test]
    fn test_example_part_2() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day4-1.example"))?;
        assert_eq!(solver.solve_part_2()?.solution, "43");
        Ok(())
    }
}
