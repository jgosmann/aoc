use crate::datastructures::grid::GridView;
use crate::solvers::{Solution, Solver};
use anyhow::anyhow;
use std::collections::{BTreeMap, BTreeSet};

pub struct SolverImpl<'input> {
    grid: GridView<&'input [u8]>,
    start_col: usize,
}

impl<'input> Solver<'input> for SolverImpl<'input> {
    fn new(input: &'input str) -> anyhow::Result<Self> {
        let grid = GridView::from_separated(b'\n', input.as_bytes());
        let start_col = grid
            .row(0)
            .iter()
            .enumerate()
            .find(|&(_, c)| c == b'S')
            .ok_or_else(|| anyhow!("missing start marker"))?
            .0;
        Ok(Self { grid, start_col })
    }

    fn solve_part_1(&self) -> anyhow::Result<Solution> {
        let mut beam_cols = BTreeSet::new();
        beam_cols.insert(self.start_col);
        let mut n_splits = 0;
        for row in 1..self.grid.height() {
            let mut next_beam_cols = BTreeSet::new();
            for beam_col in beam_cols.into_iter() {
                if self.grid[(row, beam_col)] == b'^' {
                    n_splits += 1;
                    if let Some(left_split) = beam_col.checked_sub(1) {
                        next_beam_cols.insert(left_split);
                    }
                    if beam_col + 1 < self.grid.width() {
                        next_beam_cols.insert(beam_col + 1);
                    }
                } else {
                    next_beam_cols.insert(beam_col);
                }
            }
            beam_cols = next_beam_cols;
        }
        Ok(Solution::with_description(
            "Beam splits",
            n_splits.to_string(),
        ))
    }

    fn solve_part_2(&self) -> anyhow::Result<Solution> {
        let n_timelines =
            QuantumTachyonManifold::new(&self.grid).count_timelines((0, self.start_col));
        Ok(Solution::with_description(
            "Timelines",
            n_timelines.to_string(),
        ))
    }
}

struct QuantumTachyonManifold<'input> {
    grid: &'input GridView<&'input [u8]>,
    cache: BTreeMap<(usize, usize), u64>,
}

impl<'input> QuantumTachyonManifold<'input> {
    fn new(grid: &'input GridView<&'input [u8]>) -> Self {
        Self {
            grid,
            cache: BTreeMap::new(),
        }
    }

    fn count_timelines(&mut self, beam_pos: (usize, usize)) -> u64 {
        if beam_pos.0 >= self.grid.height() {
            return 1;
        }
        if let Some(&cached) = self.cache.get(&beam_pos) {
            return cached;
        }
        let mut n_timelines = 0;
        if self.grid[beam_pos] == b'^' {
            if let Some(left_split) = beam_pos.1.checked_sub(1) {
                n_timelines += self.count_timelines((beam_pos.0 + 1, left_split));
            }
            if beam_pos.1 + 1 < self.grid.width() {
                n_timelines += self.count_timelines((beam_pos.0 + 1, beam_pos.1 + 1));
            }
        } else {
            n_timelines = self.count_timelines((beam_pos.0 + 1, beam_pos.1))
        }
        self.cache.insert(beam_pos, n_timelines);
        n_timelines
    }
}

#[cfg(test)]
mod test {
    use super::SolverImpl;
    use crate::solvers::Solver;

    #[test]
    fn test_example_part_1() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day7-1.example"))?;
        assert_eq!(solver.solve_part_1()?.solution, "21");
        Ok(())
    }

    #[test]
    fn test_example_part_2() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day7-1.example"))?;
        assert_eq!(solver.solve_part_2()?.solution, "40");
        Ok(())
    }
}
