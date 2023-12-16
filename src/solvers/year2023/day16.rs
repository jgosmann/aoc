use crate::datastructures::grid::GridView;
use crate::solvers::{Solution, Solver};
use rayon::prelude::*;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
    Left,
    Right,
    Up,
    Down,
}

fn push_dir(
    queue: &mut Vec<(Dir, (usize, usize))>,
    grid: &GridView<&[u8]>,
    dir: Dir,
    tile_idx: (usize, usize),
) {
    match dir {
        Dir::Left => {
            if tile_idx.1 > 0 {
                queue.push((Dir::Left, (tile_idx.0, tile_idx.1 - 1)));
            }
        }
        Dir::Right => {
            if tile_idx.1 < grid.width() - 1 {
                queue.push((Dir::Right, (tile_idx.0, tile_idx.1 + 1)));
            }
        }
        Dir::Up => {
            if tile_idx.0 > 0 {
                queue.push((Dir::Up, (tile_idx.0 - 1, tile_idx.1)));
            }
        }
        Dir::Down => {
            if tile_idx.0 < grid.height() - 1 {
                queue.push((Dir::Down, (tile_idx.0 + 1, tile_idx.1)));
            }
        }
    }
}

fn count_energized_tiles(grid: &GridView<&[u8]>, start: (Dir, (usize, usize))) -> usize {
    let mut energized = HashSet::new();
    let mut seen = HashSet::new();
    let mut queue = vec![start];
    while let Some((dir, tile_idx)) = queue.pop() {
        if seen.contains(&(dir, tile_idx)) {
            continue;
        }
        energized.insert(tile_idx);
        seen.insert((dir, tile_idx));
        let tile = grid[tile_idx];
        match (dir, tile) {
            (Dir::Left, b'\\') | (Dir::Right, b'/') => {
                push_dir(&mut queue, grid, Dir::Up, tile_idx);
            }
            (Dir::Right, b'\\') | (Dir::Left, b'/') => {
                push_dir(&mut queue, grid, Dir::Down, tile_idx);
            }
            (Dir::Up, b'\\') | (Dir::Down, b'/') => {
                push_dir(&mut queue, grid, Dir::Left, tile_idx);
            }
            (Dir::Down, b'\\') | (Dir::Up, b'/') => {
                push_dir(&mut queue, grid, Dir::Right, tile_idx);
            }
            (Dir::Left, b'|') | (Dir::Right, b'|') => {
                push_dir(&mut queue, grid, Dir::Up, tile_idx);
                push_dir(&mut queue, grid, Dir::Down, tile_idx);
            }
            (Dir::Up, b'-') | (Dir::Down, b'-') => {
                push_dir(&mut queue, grid, Dir::Left, tile_idx);
                push_dir(&mut queue, grid, Dir::Right, tile_idx);
            }
            (dir, _) => {
                push_dir(&mut queue, grid, dir, tile_idx);
            }
        }
    }
    energized.len()
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
        Ok(Solution::with_description(
            "Energized tiles",
            count_energized_tiles(&self.grid, (Dir::Right, (0, 0))).to_string(),
        ))
    }

    fn solve_part_2(&self) -> anyhow::Result<Solution> {
        let possible_starts: Vec<_> = (0..self.grid.width())
            .flat_map(|i| [(Dir::Up, (self.grid.height() - 1, i)), (Dir::Down, (0, i))])
            .chain((0..self.grid.height()).flat_map(|i| {
                [
                    (Dir::Left, (i, self.grid.width() - 1)),
                    (Dir::Right, (i, 0)),
                ]
            }))
            .collect();
        let max_energization = possible_starts
            .into_par_iter()
            .map(|start| count_energized_tiles(&self.grid, start))
            .max()
            .unwrap_or_default();
        Ok(Solution::with_description(
            "Part 2",
            max_energization.to_string(),
        ))
    }
}

#[cfg(test)]
mod test {
    use super::SolverImpl;
    use crate::solvers::Solver;

    #[test]
    fn test_example_part_1() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day16-1.example"))?;
        assert_eq!(solver.solve_part_1()?.solution, "46");
        Ok(())
    }

    #[test]
    fn test_example_part_2() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day16-1.example"))?;
        assert_eq!(solver.solve_part_2()?.solution, "51");
        Ok(())
    }
}
