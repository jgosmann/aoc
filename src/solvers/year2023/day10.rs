use std::{collections::HashSet, ops::Index};

use anyhow::anyhow;

use crate::{
    datastructures::{grid::GridView, iterators::NeighborIterator2d},
    solvers::{Solution, Solver},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Dir {
    North,
    South,
    East,
    West,
}

struct PipesIterator<'a, T> {
    grid: &'a GridView<T>,
    row: usize,
    col: usize,
    next_dir: Option<Dir>,
}

impl<'a, T> TryFrom<&'a GridView<T>> for PipesIterator<'a, T>
where
    GridView<T>: Index<(usize, usize), Output = u8>,
{
    type Error = anyhow::Error;

    fn try_from(grid: &'a GridView<T>) -> Result<Self, Self::Error> {
        let start_tile = grid.nth_index(
            grid.iter()
                .enumerate()
                .find(|&(_, value)| value == b'S')
                .ok_or_else(|| anyhow!("start position is required"))?
                .0,
        );

        let start_dir =
            if start_tile.0 > 0 && b"F7|".contains(&grid[(start_tile.0 - 1, start_tile.1)]) {
                Dir::North
            } else if start_tile.0 < grid.height()
                && b"LJ|".contains(&grid[(start_tile.0 + 1, start_tile.1)])
            {
                Dir::South
            } else if start_tile.1 > 0 && b"FL-".contains(&grid[(start_tile.0, start_tile.1 - 1)]) {
                Dir::West
            } else if start_tile.1 < grid.width()
                && b"7J-".contains(&grid[(start_tile.0, start_tile.1 + 1)])
            {
                Dir::East
            } else {
                anyhow::bail!("Pipe leaving start tile required.")
            };

        Ok(Self {
            grid,
            row: start_tile.0,
            col: start_tile.1,
            next_dir: Some(start_dir),
        })
    }
}

impl<T> Iterator for PipesIterator<'_, T>
where
    GridView<T>: Index<(usize, usize), Output = u8>,
{
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let next_tile = match self.next_dir {
            Some(Dir::North) if self.row > 0 => Some((self.row - 1, self.col)),
            Some(Dir::South) if self.row < self.grid.height() => Some((self.row + 1, self.col)),
            Some(Dir::East) if self.col < self.grid.width() => Some((self.row, self.col + 1)),
            Some(Dir::West) if self.col > 0 => Some((self.row, self.col - 1)),
            _ => None,
        }?;
        let shape = self.grid[next_tile];

        self.next_dir = match (shape, &self.next_dir) {
            (b'S', _) => None,
            (b'F', Some(Dir::North)) => Some(Dir::East),
            (b'F', Some(Dir::West)) => Some(Dir::South),
            (b'L', Some(Dir::South)) => Some(Dir::East),
            (b'L', Some(Dir::West)) => Some(Dir::North),
            (b'7', Some(Dir::East)) => Some(Dir::South),
            (b'7', Some(Dir::North)) => Some(Dir::West),
            (b'J', Some(Dir::South)) => Some(Dir::West),
            (b'J', Some(Dir::East)) => Some(Dir::North),
            (b'|', Some(Dir::North)) => Some(Dir::North),
            (b'|', Some(Dir::South)) => Some(Dir::South),
            (b'-', Some(Dir::East)) => Some(Dir::East),
            (b'-', Some(Dir::West)) => Some(Dir::West),
            _ => None,
        };

        self.row = next_tile.0;
        self.col = next_tile.1;
        Some(next_tile)
    }
}

struct FloodFill {
    grid: GridView<Vec<u8>>,
    inner_tiles: Vec<(usize, usize)>,
}

impl FloodFill {
    pub fn count_inner(grid: GridView<Vec<u8>>) -> anyhow::Result<Vec<(usize, usize)>> {
        let pipeloop: HashSet<_> = PipesIterator::try_from(&grid)?.collect();
        let mut state = Self {
            grid,
            inner_tiles: vec![],
        };
        state.process(&pipeloop);
        Ok(state.inner_tiles)
    }

    fn process(&mut self, pipeloop: &HashSet<(usize, usize)>) {
        for tile in pipeloop {
            for neighbor in NeighborIterator2d::new(*tile, self.grid.size()) {
                if self.grid[neighbor] == b'I' || self.grid[neighbor] == b'O' {
                    continue;
                }
                self.fill_from(neighbor, pipeloop);
            }
        }
    }

    fn fill_from(&mut self, start: (usize, usize), pipeloop: &HashSet<(usize, usize)>) {
        let mut visited = HashSet::new();
        let mut visit_queue = vec![start];
        let mut is_outer_area = false;
        while let Some(tile) = visit_queue.pop() {
            if visited.contains(&tile) {
                continue;
            }
            if pipeloop.contains(&tile) {
                continue;
            }
            is_outer_area |= tile.0 == 0
                || tile.1 == 0
                || tile.0 == self.grid.height() - 1
                || tile.1 == self.grid.width() - 1;

            visited.insert(tile);

            if tile.0 > 0 {
                visit_queue.push((tile.0 - 1, tile.1))
            }
            if tile.0 < self.grid.height() - 1 {
                visit_queue.push((tile.0 + 1, tile.1))
            }
            if tile.1 > 0 {
                visit_queue.push((tile.0, tile.1 - 1))
            }
            if tile.1 < self.grid.width() - 1 {
                visit_queue.push((tile.0, tile.1 + 1))
            }
        }

        let marker = if is_outer_area {
            b'O'
        } else {
            self.inner_tiles.extend(&visited);
            b'I'
        };
        for tile in visited {
            self.grid[tile] = marker
        }
    }
}

fn enlarge(grid: &GridView<&[u8]>) -> GridView<Vec<u8>> {
    let enlarged_width = grid.width() * 2 - 1;
    let mut enlarged_grid = GridView::from_vec(
        enlarged_width,
        0,
        vec![b'.'; enlarged_width * (grid.height() * 2 - 1)],
    );

    for i in 0..enlarged_grid.height() {
        for j in 0..enlarged_grid.width() {
            enlarged_grid[(i, j)] = match (i % 2, j % 2) {
                (0, 0) => grid[(i / 2, j / 2)],
                (_, 0)
                    if b"F7|S".contains(&grid[((i - 1) / 2, j / 2)])
                        && b"LJ|S".contains(&grid[((i + 1) / 2, j / 2)]) =>
                {
                    b'|'
                }
                (0, _)
                    if b"FL-S".contains(&grid[(i / 2, (j - 1) / 2)])
                        && b"7J-S".contains(&grid[(i / 2, (j + 1) / 2)]) =>
                {
                    b'-'
                }
                _ => b'.',
            }
        }
    }

    enlarged_grid
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
        let loop_length = PipesIterator::try_from(&self.grid)?.count();
        Ok(Solution::with_description(
            "Distance of farthest point from starting position",
            (loop_length / 2).to_string(),
        ))
    }

    fn solve_part_2(&self) -> anyhow::Result<Solution> {
        let enlarged = enlarge(&self.grid);
        let inner_tiles = FloodFill::count_inner(enlarged)?;
        let num_inner_tiles = inner_tiles
            .iter()
            .filter(|(row, col)| row % 2 == 0 && col % 2 == 0)
            .count();
        Ok(Solution::with_description(
            "Tiles inside the loop",
            num_inner_tiles.to_string(),
        ))
    }
}

#[cfg(test)]
mod test {
    use super::SolverImpl;
    use crate::solvers::Solver;

    #[test]
    fn test_example_part_1() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day10-1.example"))?;
        assert_eq!(solver.solve_part_1()?.solution, "8");
        Ok(())
    }

    #[test]
    fn test_example_part_2a() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day10-2a.example"))?;
        assert_eq!(solver.solve_part_2()?.solution, "4");
        Ok(())
    }

    #[test]
    fn test_example_part_2b() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day10-2b.example"))?;
        assert_eq!(solver.solve_part_2()?.solution, "8");
        Ok(())
    }
}
