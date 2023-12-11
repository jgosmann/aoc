use crate::datastructures::grid::GridView;
use crate::solvers::{Solution, Solver};
use std::collections::HashSet;

fn to_range(a: usize, b: usize) -> std::ops::Range<usize> {
    if a < b {
        a..b
    } else {
        b..a
    }
}

pub struct SolverImpl {
    galaxies: Vec<(usize, usize)>,
    galaxy_rows: HashSet<usize>,
    galaxy_cols: HashSet<usize>,
}

impl SolverImpl {
    pub fn sum_shortest_paths(&self, cosmological_constant: usize) -> usize {
        self.galaxies
            .iter()
            .enumerate()
            .map(|(i, galaxy_a)| {
                self.galaxies[i + 1..]
                    .iter()
                    .map(|galaxy_b| {
                        let row_range = to_range(galaxy_a.0, galaxy_b.0);
                        let col_range = to_range(galaxy_a.1, galaxy_b.1);
                        row_range
                            .into_iter()
                            .map(|row| {
                                if self.galaxy_rows.contains(&row) {
                                    1
                                } else {
                                    cosmological_constant
                                }
                            })
                            .sum::<usize>()
                            + col_range
                                .into_iter()
                                .map(|col| {
                                    if self.galaxy_cols.contains(&col) {
                                        1
                                    } else {
                                        cosmological_constant
                                    }
                                })
                                .sum::<usize>()
                    })
                    .sum::<usize>()
            })
            .sum::<usize>()
    }
}

impl<'input> Solver<'input> for SolverImpl {
    fn new(input: &'input str) -> anyhow::Result<Self> {
        let grid = GridView::from_separated(b'\n', input.as_bytes());
        let galaxies: Vec<_> = grid
            .iter()
            .enumerate()
            .filter_map(|(i, cell)| {
                if cell == b'#' {
                    Some(grid.nth_index(i))
                } else {
                    None
                }
            })
            .collect();
        let galaxy_rows: HashSet<_> = galaxies.iter().map(|galaxy| galaxy.0).collect();
        let galaxy_cols: HashSet<_> = galaxies.iter().map(|galaxy| galaxy.1).collect();

        Ok(Self {
            galaxies,
            galaxy_rows,
            galaxy_cols,
        })
    }

    fn solve_part_1(&self) -> anyhow::Result<Solution> {
        Ok(Solution::with_description(
            "Part 1",
            self.sum_shortest_paths(2).to_string(),
        ))
    }

    fn solve_part_2(&self) -> anyhow::Result<Solution> {
        Ok(Solution::with_description(
            "Part 2",
            self.sum_shortest_paths(1_000_000).to_string(),
        ))
    }
}

#[cfg(test)]
mod test {
    use super::SolverImpl;
    use crate::solvers::Solver;

    #[test]
    fn test_example_part_1() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day11-1.example"))?;
        assert_eq!(solver.solve_part_1()?.solution, "374");
        Ok(())
    }

    #[test]
    fn test_example_part_2a() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day11-1.example"))?;
        assert_eq!(solver.sum_shortest_paths(10), 1030);
        Ok(())
    }

    #[test]
    fn test_example_part_2b() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day11-1.example"))?;
        assert_eq!(solver.sum_shortest_paths(100), 8410);
        Ok(())
    }
}
