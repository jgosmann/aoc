use crate::datastructures::grid::GridView;
use crate::datastructures::iterators::NeighborIterator2d;
use crate::solvers::{Solution, Solver};
use std::collections::{BTreeSet, VecDeque};

pub struct SolverImpl<'input> {
    grid: GridView<&'input [u8]>,
    distance_grid: GridView<Vec<(usize, (usize, usize))>>,
    start_pos: (usize, usize),
    target: (usize, usize),
}

impl<'input> Solver<'input> for SolverImpl<'input> {
    fn new(input: &'input str) -> anyhow::Result<Self> {
        let grid = GridView::from_separated(b'\n', input.as_bytes());
        let start_pos = grid
            .iter()
            .position(|c| c == b'S')
            .map(|i| grid.nth_index(i))
            .expect("No start position found");
        let target = grid
            .iter()
            .position(|c| c == b'E')
            .map(|i| grid.nth_index(i))
            .expect("No target found");

        let distances = vec![(0usize, target); grid.width() * grid.height()];
        let mut distance_grid = GridView::from_vec(grid.width(), 0, distances);

        let mut to_visit = VecDeque::new();
        to_visit.push_back((target, 0, target));
        let mut visited = BTreeSet::new();

        while let Some((pos, distance, prev_pos)) = to_visit.pop_front() {
            if visited.contains(&pos) {
                continue;
            }
            visited.insert(pos);

            distance_grid[pos] = (distance, prev_pos);

            for neighbor in NeighborIterator2d::new(pos, grid.size()) {
                if grid[neighbor] != b'#' {
                    to_visit.push_back((neighbor, distance + 1, pos));
                }
            }
        }

        Ok(Self {
            grid,
            start_pos,
            target,
            distance_grid,
        })
    }

    fn solve_part_1(&self) -> anyhow::Result<Solution> {
        Ok(Solution::with_description(
            "Part 1",
            self.count_cheats(2, 100).to_string(),
        ))
    }

    fn solve_part_2(&self) -> anyhow::Result<Solution> {
        Ok(Solution::with_description(
            "Part 2",
            self.count_cheats(20, 100).to_string(),
        ))
    }
}

impl SolverImpl<'_> {
    fn count_cheats(&self, max_cheat_ps: isize, saved_ps_threshold_to_count: usize) -> usize {
        let mut pos = self.start_pos;
        let mut num_cheats = 0;
        while pos != self.target {
            let (distance, prev_pos) = self.distance_grid[pos];

            for dx in -max_cheat_ps..=max_cheat_ps {
                for dy in -max_cheat_ps..=max_cheat_ps {
                    let cheat_steps: usize = dx.abs() as usize + dy.abs() as usize;
                    if cheat_steps > max_cheat_ps as usize || cheat_steps < 2 {
                        continue;
                    }

                    if let Some(cheat_target) = pos
                        .0
                        .checked_add_signed(dx)
                        .and_then(|x| pos.1.checked_add_signed(dy).map(|y| (x, y)))
                    {
                        if cheat_target.0 >= self.grid.height()
                            || cheat_target.1 >= self.grid.width()
                        {
                            continue;
                        }
                        if self.grid[cheat_target] != b'#' {
                            if let Some(saving) = distance
                                .checked_sub(self.distance_grid[cheat_target].0 + cheat_steps)
                            {
                                if saving >= saved_ps_threshold_to_count {
                                    num_cheats += 1;
                                }
                            }
                        }
                    }
                }
            }

            pos = prev_pos;
        }

        num_cheats
    }
}

#[cfg(test)]
mod test {
    use super::SolverImpl;
    use crate::solvers::Solver;

    #[test]
    fn test_example_part_1() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day20-1.example"))?;
        assert_eq!(solver.count_cheats(2, 64), 1);
        assert_eq!(solver.count_cheats(2, 40), 2);
        assert_eq!(solver.count_cheats(2, 38), 3);
        assert_eq!(solver.count_cheats(2, 36), 4);
        assert_eq!(solver.count_cheats(2, 20), 5);
        assert_eq!(solver.count_cheats(2, 12), 8);
        assert_eq!(solver.count_cheats(2, 10), 10);
        assert_eq!(solver.count_cheats(2, 8), 14);
        assert_eq!(solver.count_cheats(2, 6), 16);
        assert_eq!(solver.count_cheats(2, 4), 30);
        assert_eq!(solver.count_cheats(2, 2), 44);
        Ok(())
    }

    #[test]
    fn test_example_part_2() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day20-1.example"))?;
        assert_eq!(solver.count_cheats(20, 76), 3);
        assert_eq!(solver.count_cheats(20, 74), 7);
        assert_eq!(solver.count_cheats(20, 72), 29);
        assert_eq!(solver.count_cheats(20, 70), 41);
        assert_eq!(solver.count_cheats(20, 68), 55);
        assert_eq!(solver.count_cheats(20, 66), 67);
        assert_eq!(solver.count_cheats(20, 64), 86);
        assert_eq!(solver.count_cheats(20, 62), 106);
        assert_eq!(solver.count_cheats(20, 60), 129);
        assert_eq!(solver.count_cheats(20, 58), 154);
        assert_eq!(solver.count_cheats(20, 56), 193);
        assert_eq!(solver.count_cheats(20, 54), 222);
        assert_eq!(solver.count_cheats(20, 52), 253);
        assert_eq!(solver.count_cheats(20, 50), 285);
        Ok(())
    }
}
