use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

use crate::{
    datastructures::grid::GridView,
    solvers::{Solution, Solver},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
    Left,
    Right,
    Up,
    Down,
}

impl Dir {
    fn invert(&self) -> Dir {
        match self {
            Dir::Left => Dir::Right,
            Dir::Right => Dir::Left,
            Dir::Down => Dir::Up,
            Dir::Up => Dir::Down,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct VisitedKey {
    pos: (usize, usize),
    dir: Dir,
    steps_since_last_dir_change: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PathState {
    heatloss: usize,
    current_pos: (usize, usize),
    travel_direction: Dir,
    steps_since_last_dir_change: usize,
    target: (usize, usize),
}

impl PartialOrd for PathState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PathState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.min_heatloss_bound().cmp(&other.min_heatloss_bound())
    }
}

impl PathState {
    fn min_heatloss_bound(&self) -> usize {
        self.heatloss + (self.target.0 - self.current_pos.0) + (self.target.1 - self.current_pos.1)
    }

    fn is_valid_travel_dir(
        &self,
        dir: Dir,
        min_steps: Option<usize>,
        max_steps: usize,
        grid_size: (usize, usize),
    ) -> bool {
        if self.travel_direction.invert() == dir {
            return false;
        }
        if self.travel_direction == dir && self.steps_since_last_dir_change >= max_steps {
            return false;
        }
        if let Some(min_steps) = min_steps {
            if self.travel_direction != dir && self.steps_since_last_dir_change < min_steps {
                return false;
            }
        }
        match dir {
            Dir::Down => self.current_pos.0 < grid_size.0 - 1,
            Dir::Up => self.current_pos.0 > 0,
            Dir::Left => self.current_pos.1 > 0,
            Dir::Right => self.current_pos.1 < grid_size.1 - 1,
        }
    }
}

fn find_min_heatloss(
    grid: &GridView<&[u8]>,
    min_steps: Option<usize>,
    max_steps: usize,
) -> Option<usize> {
    let target = (grid.height() - 1, grid.width() - 1);
    let mut queue = BinaryHeap::from([
        Reverse(PathState {
            heatloss: 0,
            current_pos: (0, 0),
            travel_direction: Dir::Down,
            steps_since_last_dir_change: 0,
            target,
        }),
        Reverse(PathState {
            heatloss: 0,
            current_pos: (0, 0),
            travel_direction: Dir::Right,
            steps_since_last_dir_change: 0,
            target,
        }),
    ]);
    let mut visited: HashMap<VisitedKey, usize> = HashMap::new();
    while let Some(state) = queue.pop() {
        let state = state.0;
        if state.current_pos == target
            && state.steps_since_last_dir_change >= min_steps.unwrap_or_default()
        {
            return Some(state.heatloss);
        }

        let visited_key = VisitedKey {
            pos: state.current_pos,
            dir: state.travel_direction,
            steps_since_last_dir_change: state.steps_since_last_dir_change,
        };
        if let Some(&prior_heatloss) = visited.get(&visited_key) {
            if prior_heatloss <= state.heatloss {
                continue;
            }
        }
        visited.insert(visited_key, state.heatloss);

        for dir in [Dir::Left, Dir::Right, Dir::Down, Dir::Up] {
            if !state.is_valid_travel_dir(dir, min_steps, max_steps, grid.size()) {
                continue;
            }

            let new_pos = match dir {
                Dir::Up => (state.current_pos.0 - 1, state.current_pos.1),
                Dir::Down => (state.current_pos.0 + 1, state.current_pos.1),
                Dir::Left => (state.current_pos.0, state.current_pos.1 - 1),
                Dir::Right => (state.current_pos.0, state.current_pos.1 + 1),
            };

            queue.push(Reverse(PathState {
                heatloss: state.heatloss + ((grid[new_pos] - b'0') as usize),
                current_pos: new_pos,
                travel_direction: dir,
                steps_since_last_dir_change: if state.travel_direction == dir {
                    state.steps_since_last_dir_change + 1
                } else {
                    1
                },
                target,
            }));
        }
    }

    None
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
        let min_heatloss = find_min_heatloss(&self.grid, None, 3).expect("a solution should exist");
        Ok(Solution::with_description(
            "Minimal heat loss",
            min_heatloss.to_string(),
        ))
    }

    fn solve_part_2(&self) -> anyhow::Result<Solution> {
        let min_heatloss =
            find_min_heatloss(&self.grid, Some(4), 10).expect("a solution should exist");
        Ok(Solution::with_description(
            "Minimal heat loss with ultra crucible",
            min_heatloss.to_string(),
        ))
    }
}

#[cfg(test)]
mod test {
    use super::SolverImpl;
    use crate::solvers::Solver;

    #[test]
    fn test_example_part_1() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day17-1.example"))?;
        assert_eq!(solver.solve_part_1()?.solution, "102");
        Ok(())
    }

    #[test]
    fn test_example_part_2() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day17-1.example"))?;
        assert_eq!(solver.solve_part_2()?.solution, "94");
        Ok(())
    }

    #[test]
    fn test_example2_part_2() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day17-2.example"))?;
        assert_eq!(solver.solve_part_2()?.solution, "71");
        Ok(())
    }
}
