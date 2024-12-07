use crate::datastructures::grid::GridView;
use crate::solvers::{Solution, Solver};
use std::collections::HashMap;

fn roll_north(mut input: GridView<Vec<u8>>) -> GridView<Vec<u8>> {
    for col_idx in 0..input.width() {
        let mut min_free_idx = 0;
        for row_idx in 0..input.height() {
            match input[(row_idx, col_idx)] {
                b'O' => {
                    if min_free_idx < row_idx {
                        input[(min_free_idx, col_idx)] = b'O';
                        input[(row_idx, col_idx)] = b'.';
                    }
                    min_free_idx += 1;
                }
                b'#' => min_free_idx = row_idx + 1,
                _ => (),
            }
        }
    }
    input
}

fn roll_south(mut input: GridView<Vec<u8>>) -> GridView<Vec<u8>> {
    for col_idx in 0..input.width() {
        let mut max_free_idx = input.height() - 1;
        for row_idx in (0..input.height()).rev() {
            match input[(row_idx, col_idx)] {
                b'O' => {
                    if max_free_idx > row_idx {
                        input[(max_free_idx, col_idx)] = b'O';
                        input[(row_idx, col_idx)] = b'.';
                    }
                    max_free_idx = max_free_idx.saturating_sub(1)
                }
                b'#' => max_free_idx = if row_idx > 0 { row_idx - 1 } else { 0 },
                _ => (),
            }
        }
    }
    input
}

fn roll_west(mut input: GridView<Vec<u8>>) -> GridView<Vec<u8>> {
    for row_idx in 0..input.height() {
        let mut min_free_idx = 0;
        for col_idx in 0..input.width() {
            match input[(row_idx, col_idx)] {
                b'O' => {
                    if min_free_idx < col_idx {
                        input[(row_idx, min_free_idx)] = b'O';
                        input[(row_idx, col_idx)] = b'.';
                    }
                    min_free_idx += 1;
                }
                b'#' => min_free_idx = col_idx + 1,
                _ => (),
            }
        }
    }
    input
}

fn roll_east(mut input: GridView<Vec<u8>>) -> GridView<Vec<u8>> {
    for row_idx in 0..input.height() {
        let mut max_free_idx = input.width() - 1;
        for col_idx in (0..input.width()).rev() {
            match input[(row_idx, col_idx)] {
                b'O' => {
                    if max_free_idx > col_idx {
                        input[(row_idx, max_free_idx)] = b'O';
                        input[(row_idx, col_idx)] = b'.';
                    }
                    max_free_idx = max_free_idx.saturating_sub(1)
                }
                b'#' => max_free_idx = if col_idx > 0 { col_idx - 1 } else { 0 },
                _ => (),
            }
        }
    }
    input
}

fn spin_one_cycle(input: GridView<Vec<u8>>) -> GridView<Vec<u8>> {
    let input = roll_north(input);
    let input = roll_west(input);
    let input = roll_south(input);
    roll_east(input)
}

fn determine_load_rolled_north(grid: &GridView<Vec<u8>>) -> usize {
    let mut load: usize = 0;
    for col_idx in 0..grid.width() {
        let mut round_rock_count = 0;
        let mut cube_offset = 0;
        for row_idx in 0..=grid.height() {
            let tile = if row_idx < grid.height() {
                grid[(row_idx, col_idx)]
            } else {
                b'#'
            };
            match tile {
                b'O' => round_rock_count += 1,
                b'#' => {
                    let max_leverage = grid.height() - cube_offset;
                    let min_leverage = max_leverage - round_rock_count;
                    load += (max_leverage * max_leverage + max_leverage
                        - min_leverage * min_leverage
                        - min_leverage)
                        / 2;
                    round_rock_count = 0;
                    cube_offset = row_idx + 1;
                }
                _ => (),
            }
        }
    }
    load
}

fn determine_load(grid: &GridView<Vec<u8>>) -> usize {
    let mut load: usize = 0;
    for col_idx in 0..grid.width() {
        for row_idx in 0..grid.height() {
            if grid[(row_idx, col_idx)] == b'O' {
                load += grid.height() - row_idx;
            }
        }
    }
    load
}
pub struct SolverImpl {
    grid: GridView<Vec<u8>>,
}

impl<'input> Solver<'input> for SolverImpl {
    fn new(input: &'input str) -> anyhow::Result<Self> {
        let buf = Vec::from(input.as_bytes());
        let grid = GridView::from_separated_vec(b'\n', buf);
        Ok(Self { grid })
    }

    fn solve_part_1(&self) -> anyhow::Result<Solution> {
        let load = determine_load_rolled_north(&self.grid);
        Ok(Solution::with_description(
            "Total load (part 1)",
            load.to_string(),
        ))
    }

    fn solve_part_2(&self) -> anyhow::Result<Solution> {
        let mut grid = self.grid.clone();

        let mut seen = HashMap::new();
        seen.insert(grid.clone(), 0);

        const MAX_CYCLES: usize = 1_000_000_000;
        for i in 1..=MAX_CYCLES {
            grid = spin_one_cycle(grid);
            if let Some(x) = seen.get(&grid) {
                let remaining_cycles = (MAX_CYCLES - i) % (i - x);
                for _ in 0..remaining_cycles {
                    grid = spin_one_cycle(grid);
                }
                break;
            }
            seen.insert(grid.clone(), i);
        }

        let load = determine_load(&grid);

        Ok(Solution::with_description(
            "Total load (part 2)",
            load.to_string(),
        ))
    }
}

#[cfg(test)]
mod test {
    use super::SolverImpl;
    use crate::solvers::Solver;

    #[test]
    fn test_example_part_1() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day14-1.example"))?;
        assert_eq!(solver.solve_part_1()?.solution, "136");
        Ok(())
    }

    #[test]
    fn test_example_part_2() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day14-1.example"))?;
        assert_eq!(solver.solve_part_2()?.solution, "64");
        Ok(())
    }
}
