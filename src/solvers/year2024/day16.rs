use crate::datastructures::grid::GridView;
use crate::solvers::{Solution, Solver};
use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashSet};

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, Ord, PartialOrd)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn delta(&self) -> (isize, isize) {
        match self {
            Direction::North => (-1, 0),
            Direction::South => (1, 0),
            Direction::East => (0, 1),
            Direction::West => (0, -1),
        }
    }

    fn lturn(&self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
            Direction::West => Direction::South,
        }
    }

    fn rturn(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::East => Direction::South,
            Direction::West => Direction::North,
        }
    }
}

pub struct SolverImpl<'input> {
    grid: GridView<&'input [u8]>,
    start_pos: (usize, usize),
}

impl<'input> Solver<'input> for SolverImpl<'input> {
    fn new(input: &'input str) -> anyhow::Result<Self> {
        let grid = GridView::from_separated(b'\n', input.as_bytes());
        let start_pos = grid
            .iter()
            .position(|c| c == b'S')
            .map(|p| grid.nth_index(p))
            .expect("no starting position");

        Ok(Self { grid, start_pos })
    }

    fn solve_part_1(&self) -> anyhow::Result<Solution> {
        Ok(Solution::with_description(
            "Part 1",
            self.find_lowest_score().to_string(),
        ))
    }

    fn solve_part_2(&self) -> anyhow::Result<Solution> {
        Ok(Solution::with_description(
            "Part 2",
            "not implemented".to_string(),
        ))
    }
}

impl SolverImpl<'_> {
    fn find_lowest_score(&self) -> usize {
        let mut to_visit = BinaryHeap::new();
        let mut visited = HashSet::new();
        to_visit.push((Reverse(0), self.start_pos, Direction::East));

        while let Some((Reverse(score), pos, dir)) = to_visit.pop() {
            if self.grid[pos] == b'E' {
                return score;
            }

            if self.grid[pos] == b'#' {
                continue;
            }

            if visited.contains(&(pos, dir)) {
                continue;
            }
            visited.insert((pos, dir));

            if let Some(forward_pos) = self.next_pos(pos, dir) {
                to_visit.push((Reverse(score + 1), forward_pos, dir));
            }
            to_visit.push((Reverse(score + 1000), pos, dir.lturn()));
            to_visit.push((Reverse(score + 1000), pos, dir.rturn()));
        }

        panic!("no path to exit");
    }

    fn next_pos(&self, pos: (usize, usize), dir: Direction) -> Option<(usize, usize)> {
        let forward_pos = (
            pos.0.checked_add_signed(dir.delta().0),
            pos.1.checked_add_signed(dir.delta().1),
        );
        let forward_pos = forward_pos
            .0
            .and_then(|x| forward_pos.1.and_then(|y| Some((x, y))));
        if let Some(forward_pos) = forward_pos {
            if forward_pos.0 < self.grid.height() && forward_pos.1 < self.grid.width() {
                return Some(forward_pos);
            }
        }
        None
    }
}

#[cfg(test)]
mod test {
    use super::SolverImpl;
    use crate::solvers::Solver;

    #[test]
    fn test_example_part_1_1() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day16-1.example"))?;
        assert_eq!(solver.solve_part_1()?.solution, "7036");
        Ok(())
    }

    #[test]
    fn test_example_part_1_2() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day16-2.example"))?;
        assert_eq!(solver.solve_part_1()?.solution, "11048");
        Ok(())
    }

    #[test]
    fn test_example_part_2_1() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day16-1.example"))?;
        assert_eq!(solver.solve_part_2()?.solution, "45");
        Ok(())
    }

    #[test]
    fn test_example_part_2_2() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day16-2.example"))?;
        assert_eq!(solver.solve_part_2()?.solution, "64");
        Ok(())
    }
}
