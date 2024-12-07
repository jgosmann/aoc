use crate::datastructures::grid::GridView;
use crate::solvers::{Solution, Solver};
use std::collections::HashSet;
use std::ops::{Deref, Index};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Default for Direction {
    fn default() -> Self {
        Self::North
    }
}

impl Direction {
    fn vector(&self) -> (isize, isize) {
        match self {
            Self::North => (-1, 0),
            Self::South => (1, 0),
            Self::East => (0, 1),
            Self::West => (0, -1),
        }
    }

    fn turn(&self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }
}

pub struct SolverImpl<'input> {
    input: &'input str,
}

impl<'input> Solver<'input> for SolverImpl<'input> {
    fn new(input: &'input str) -> anyhow::Result<Self> {
        Ok(Self { input })
    }

    fn solve_part_1(&self) -> anyhow::Result<Solution> {
        let grid = GridView::from_separated(b'\n', self.input.as_bytes());
        let starting_pos = Self::find_starting_pos(&grid)
            .ok_or_else(|| anyhow::anyhow!("no starting position found"))?;
        let mut pos = starting_pos;
        let mut direction = Direction::default();
        let mut distinct_positions = HashSet::with_capacity(grid.height() * grid.width());
        distinct_positions.insert(pos);
        while let Some((new_pos, new_direction)) = Self::next_pos(&grid, pos, direction) {
            pos = new_pos;
            direction = new_direction;
            distinct_positions.insert(pos);
        }
        Ok(Solution::with_description(
            "Part 1",
            distinct_positions.len().to_string(),
        ))
    }

    fn solve_part_2(&self) -> anyhow::Result<Solution> {
        let input: Vec<u8> = self.input.as_bytes().to_owned();
        let mut grid = GridView::from_separated_vec(b'\n', input);
        let starting_pos = Self::find_starting_pos(&grid)
            .ok_or_else(|| anyhow::anyhow!("no starting position found"))?;
        let mut pos = starting_pos;
        let mut direction = Direction::default();
        let mut obstructions = HashSet::new();
        let mut visited = HashSet::new();
        visited.insert(pos);
        while let Some((new_pos, new_direction)) = Self::next_pos(&grid, pos, direction) {
            grid[new_pos] = b'#';
            if !visited.contains(&new_pos) && self.check_is_loop(&grid, pos, direction) {
                obstructions.insert(new_pos);
            }
            grid[new_pos] = b'.';
            visited.insert(new_pos);
            pos = new_pos;
            direction = new_direction;
        }
        Ok(Solution::with_description(
            "Part 2",
            obstructions.len().to_string(),
        ))
    }
}

impl SolverImpl<'_> {
    fn find_starting_pos<T>(grid: &GridView<T>) -> Option<(usize, usize)>
    where
        T: Deref,
        T::Target: Index<usize, Output = u8>,
    {
        for row in 0..grid.height() {
            for col in 0..grid.width() {
                if grid[(row, col)] == b'^' {
                    return Some((row, col));
                }
            }
        }
        None
    }

    fn next_pos<T>(
        grid: &GridView<T>,
        pos: (usize, usize),
        direction: Direction,
    ) -> Option<((usize, usize), Direction)>
    where
        T: Deref,
        T::Target: Index<usize, Output = u8>,
    {
        let (row, col) = pos;
        let (drow, dcol) = direction.vector();
        row.checked_add_signed(drow).and_then(|new_row| {
            col.checked_add_signed(dcol).and_then(|new_col| {
                if new_row < grid.height() && new_col < grid.width() {
                    if grid[(new_row, new_col)] == b'#' {
                        return Self::next_pos(grid, pos, direction.turn());
                    }
                    Some(((new_row, new_col), direction))
                } else {
                    None
                }
            })
        })
    }

    fn check_is_loop(
        &self,
        grid: &GridView<Vec<u8>>,
        starting_pos: (usize, usize),
        direction: Direction,
    ) -> bool {
        let mut pos = starting_pos;
        let mut direction = direction;
        let mut distinct_positions = HashSet::with_capacity(grid.height() * grid.width());
        distinct_positions.insert((pos, direction));
        while let Some((new_pos, new_direction)) = Self::next_pos(grid, pos, direction) {
            pos = new_pos;
            direction = new_direction;
            if !distinct_positions.insert((pos, direction)) {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod test {
    use super::SolverImpl;
    use crate::solvers::Solver;

    #[test]
    fn test_example_part_1() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day6-1.example"))?;
        assert_eq!(solver.solve_part_1()?.solution, "41");
        Ok(())
    }

    #[test]
    fn test_example_part_2() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day6-1.example"))?;
        assert_eq!(solver.solve_part_2()?.solution, "6");
        Ok(())
    }
}
