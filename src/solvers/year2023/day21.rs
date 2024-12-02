use std::collections::HashSet;

use anyhow::anyhow;

use crate::{
    datastructures::{grid::GridView, iterators::NeighborIterator2d},
    solvers::{Solution, Solver},
};

pub struct SolverImpl<'input> {
    grid: GridView<&'input [u8]>,
    start: (usize, usize),
}

impl SolverImpl<'_> {
    pub fn reachable_in_steps(
        &self,
        start: (usize, usize),
        steps: usize,
    ) -> HashSet<(usize, usize)> {
        let mut positions = HashSet::from([start]);
        for _ in 0..steps {
            positions = positions
                .iter()
                .flat_map(|&from| {
                    NeighborIterator2d::new(from, self.grid.size())
                        .filter(|&neighbor| self.grid[neighbor] != b'#')
                })
                .collect();
        }
        positions
    }

    pub fn reachable_in_steps_with_assumptions(&self, steps: usize) -> usize {
        assert!(self.grid.col(0).iter().all(|tile| tile != b'#'));
        assert!(self
            .grid
            .col(self.grid.width() - 1)
            .iter()
            .all(|tile| tile != b'#'));
        assert!(self.grid.col(0).iter().all(|tile| tile != b'#'));
        assert!(self
            .grid
            .col(self.grid.height() - 1)
            .iter()
            .all(|tile| tile != b'#'));
        assert!(self.grid.col(self.start.1).iter().all(|tile| tile != b'#'));
        assert!(self.grid.row(self.start.0).iter().all(|tile| tile != b'#'));
        assert_eq!(self.grid.width(), self.grid.height());

        let w = self.grid.width();

        assert_eq!((steps - w / 2) % w, 0);
        assert_eq!(((steps - w / 2) / w) % 2, 0);

        let replication_steps = (steps - self.grid.width() / 2) / self.grid.width();

        let top_left = (0, 0);
        let top_mid = (0, w / 2);
        let top_right = (0, w - 1);
        let mid_left = (w / 2, 0);
        let mid_right = (w / 2, w - 1);
        let bottom_left = (w - 1, 0);
        let bottom_mid = (w - 1, w / 2);
        let bottom_right = (w - 1, w - 1);

        let steps_half_tile = w / 2;
        let steps_full_tile = w;

        let down = self.reachable_in_steps(top_mid, steps_full_tile - 1);
        let up = self.reachable_in_steps(bottom_mid, steps_full_tile - 1);
        let right = self.reachable_in_steps(mid_left, steps_full_tile - 1);
        let left = self.reachable_in_steps(mid_right, steps_full_tile - 1);

        let upper_left_diag =
            self.reachable_in_steps(bottom_right, steps_full_tile + steps_half_tile - 1);
        let upper_right_diag =
            self.reachable_in_steps(bottom_left, steps_full_tile + steps_half_tile - 1);
        let lower_left_diag =
            self.reachable_in_steps(top_right, steps_full_tile + steps_half_tile - 1);
        let lower_right_diag =
            self.reachable_in_steps(top_left, steps_full_tile + steps_half_tile - 1);

        let inner_odd = self.reachable_in_steps(top_mid, 3 * w - 1);
        let inner_even = &self.reachable_in_steps(top_mid, 2 * w - 1);
        let center = &self.reachable_in_steps(self.start, 2 * w + w / 2);

        let corner_upper_left = self.reachable_in_steps(bottom_right, steps_half_tile - 1);
        let corner_upper_right = self.reachable_in_steps(bottom_left, steps_half_tile - 1);
        let corner_lower_left = self.reachable_in_steps(top_right, steps_half_tile - 1);
        let corner_lower_right = self.reachable_in_steps(top_left, steps_half_tile - 1);

        let n = replication_steps - 1;
        center.len()
            + down.len()
            + up.len()
            + right.len()
            + left.len()
            + n * (upper_left_diag.len()
                + upper_right_diag.len()
                + lower_left_diag.len()
                + lower_right_diag.len())
            + replication_steps
                * (corner_lower_left.len()
                    + corner_lower_right.len()
                    + corner_upper_left.len()
                    + corner_upper_right.len())
            + (n * n - 1) * inner_odd.len()
            + replication_steps * replication_steps * inner_even.len()
    }
}

impl<'input> Solver<'input> for SolverImpl<'input> {
    fn new(input: &'input str) -> anyhow::Result<Self> {
        let grid = GridView::from_separated(b'\n', input.as_bytes());
        let start = grid
            .iter()
            .enumerate()
            .find_map(|(i, tile)| match tile {
                b'S' => Some(grid.nth_index(i)),
                _ => None,
            })
            .ok_or_else(|| anyhow!("Start position required."))?;

        Ok(Self { grid, start })
    }

    fn solve_part_1(&self) -> anyhow::Result<Solution> {
        Ok(Solution::with_description(
            "Garden plots reachable in 64 steps",
            self.reachable_in_steps(self.start, 64).len().to_string(),
        ))
    }

    fn solve_part_2(&self) -> anyhow::Result<Solution> {
        Ok(Solution::with_description(
            "Garden plots reachable in 26501365",
            self.reachable_in_steps_with_assumptions(26501365)
                .to_string(),
        ))
    }
}

#[cfg(test)]
mod test {
    use super::SolverImpl;
    use crate::solvers::Solver;

    #[test]
    fn test_example_part_1() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day21-1.example"))?;
        assert_eq!(solver.reachable_in_steps(solver.start, 6).len(), 16);
        Ok(())
    }
}
