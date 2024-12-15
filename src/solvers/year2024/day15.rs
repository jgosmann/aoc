use crate::datastructures::grid::GridView;
use crate::solvers::{Solution, Solver};
use anyhow::anyhow;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl TryFrom<u8> for Direction {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'^' => Ok(Direction::Up),
            b'v' => Ok(Direction::Down),
            b'<' => Ok(Direction::Left),
            b'>' => Ok(Direction::Right),
            _ => Err(anyhow!("invalid direction")),
        }
    }
}

impl Direction {
    fn delta(self) -> (isize, isize) {
        match self {
            Self::Up => (-1, 0),
            Self::Down => (1, 0),
            Self::Left => (0, -1),
            Self::Right => (0, 1),
        }
    }
}

fn push(grid: &mut GridView<Vec<u8>>, pos: (usize, usize), direction: Direction) -> bool {
    let (dx, dy) = direction.delta();
    let push_target = (pos.0.wrapping_add_signed(dx), pos.1.wrapping_add_signed(dy));
    match grid[push_target] {
        b'#' => false,
        b'O' => {
            if push(grid, push_target, direction) {
                grid[(
                    push_target.0.wrapping_add_signed(dx),
                    push_target.1.wrapping_add_signed(dy),
                )] = b'O';
                grid[push_target] = b'.';
                true
            } else {
                false
            }
        }
        _ => true,
    }
}

fn push_wide(
    grid: &mut GridView<Vec<u8>>,
    pos: (usize, usize),
    direction: Direction,
    dry_run: bool,
) -> bool {
    let (dx, dy) = direction.delta();
    let push_target = (pos.0.wrapping_add_signed(dx), pos.1.wrapping_add_signed(dy));
    match grid[push_target] {
        b'#' => false,
        b'[' | b']' => {
            if direction == Direction::Right || direction == Direction::Left {
                if push_wide(grid, push_target, direction, dry_run) {
                    if !dry_run {
                        grid[(
                            push_target.0.wrapping_add_signed(dx),
                            push_target.1.wrapping_add_signed(dy),
                        )] = grid[push_target];
                        grid[push_target] = b'.';
                    }
                    true
                } else {
                    false
                }
            } else {
                let delta_other_half: isize = if grid[push_target] == b'[' { 1 } else { -1 };
                if push_wide(grid, push_target, direction, dry_run)
                    && push_wide(
                        grid,
                        (
                            push_target.0,
                            push_target.1.wrapping_add_signed(delta_other_half),
                        ),
                        direction,
                        dry_run,
                    )
                {
                    if !dry_run {
                        grid[(
                            push_target.0.wrapping_add_signed(dx),
                            push_target.1.wrapping_add_signed(dy),
                        )] = grid[push_target];
                        grid[push_target] = b'.';
                        let neighbor_target = (
                            push_target.0,
                            push_target.1.wrapping_add_signed(delta_other_half),
                        );
                        grid[(
                            push_target.0.wrapping_add_signed(dx),
                            push_target.1.wrapping_add_signed(dy + delta_other_half),
                        )] = grid[neighbor_target];
                        grid[neighbor_target] = b'.';
                    }
                    true
                } else {
                    false
                }
            }
        }
        _ => true,
    }
}

pub struct SolverImpl {
    grid: GridView<Vec<u8>>,
    movements: Vec<Direction>,
}

impl<'input> Solver<'input> for SolverImpl {
    fn new(input: &'input str) -> anyhow::Result<Self> {
        let mut lines_iter = input.lines().peekable();
        let width = lines_iter.peek().map(|line| line.len()).unwrap_or(0);
        let grid_lines = lines_iter
            .by_ref()
            .take_while(|line| !line.trim().is_empty())
            .flat_map(str::as_bytes)
            .copied()
            .collect::<Vec<_>>();
        let grid = GridView::from_vec(width, 0, grid_lines);

        let movements: Vec<_> = lines_iter
            .flat_map(|line| {
                line.as_bytes()
                    .iter()
                    .copied()
                    .filter(|c| !c.is_ascii_whitespace())
                    .map(Direction::try_from)
            })
            .collect::<Result<_, _>>()?;

        Ok(Self { grid, movements })
    }

    fn solve_part_1(&self) -> anyhow::Result<Solution> {
        let mut grid = self.grid.clone();
        let mut pos = Self::find_starting_pos(&grid);
        for movement in self.movements.iter().copied() {
            let (dx, dy) = movement.delta();
            if push(&mut grid, pos, movement) {
                pos = (pos.0.wrapping_add_signed(dx), pos.1.wrapping_add_signed(dy));
            }
        }

        Ok(Solution::with_description(
            "Part 1",
            Self::sum_gps(&grid, b'O').to_string(),
        ))
    }

    fn solve_part_2(&self) -> anyhow::Result<Solution> {
        let widened: Vec<u8> = self
            .grid
            .iter()
            .flat_map(|c| match c {
                b'O' => [b'[', b']'],
                b'@' => [b'@', b'.'],
                _ => [c, c],
            })
            .collect();
        let mut grid = GridView::from_vec(self.grid.width() * 2, 0, widened);
        let mut pos = Self::find_starting_pos(&grid);
        for movement in self.movements.iter().copied() {
            let (dx, dy) = movement.delta();
            if push_wide(&mut grid, pos, movement, true) {
                push_wide(&mut grid, pos, movement, false);
                grid[(pos.0.wrapping_add_signed(dx), pos.1.wrapping_add_signed(dy))] = grid[pos];
                grid[pos] = b'.';
                pos = (pos.0.wrapping_add_signed(dx), pos.1.wrapping_add_signed(dy));
            }
        }

        Ok(Solution::with_description(
            "Part 2",
            Self::sum_gps(&grid, b'[').to_string(),
        ))
    }
}

impl SolverImpl {
    fn find_starting_pos(grid: &GridView<Vec<u8>>) -> (usize, usize) {
        grid.iter()
            .position(|c| c == b'@')
            .map(|i| grid.nth_index(i))
            .expect("no starting position")
    }

    fn sum_gps(grid: &GridView<Vec<u8>>, marker: u8) -> usize {
        grid.iter()
            .enumerate()
            .filter(|(_, c)| *c == marker)
            .map(|(i, _)| {
                let (row, col) = grid.nth_index(i);
                100 * row + col
            })
            .sum()
    }
}

#[cfg(test)]
mod test {
    use super::SolverImpl;
    use crate::solvers::Solver;

    #[test]
    fn test_example_part_1_small() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day15-1-small.example"))?;
        assert_eq!(solver.solve_part_1()?.solution, "2028");
        Ok(())
    }

    #[test]
    fn test_example_part_1_large() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day15-1-large.example"))?;
        assert_eq!(solver.solve_part_1()?.solution, "10092");
        Ok(())
    }

    #[test]
    fn test_example_part_2() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day15-1-large.example"))?;
        assert_eq!(solver.solve_part_2()?.solution, "9021");
        Ok(())
    }
}
