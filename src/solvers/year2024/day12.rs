use crate::datastructures::grid::GridView;
use crate::datastructures::iterators::NeighborIterator2d;
use crate::solvers::{Solution, Solver};

pub struct SolverImpl<'input> {
    grid: GridView<&'input [u8]>,
}

impl<'input> Solver<'input> for SolverImpl<'input> {
    fn new(input: &'input str) -> anyhow::Result<Self> {
        let grid = GridView::from_separated(b'\n', input.as_bytes());
        Ok(Self { grid })
    }

    fn solve_part_1(&self) -> anyhow::Result<Solution> {
        let mut price = 0;
        let mut visited = GridView::from_vec(
            self.grid.width(),
            0,
            vec![false; self.grid.width() * self.grid.height()],
        );
        for start_row in 0..self.grid.height() {
            for start_col in 0..self.grid.width() {
                let mut area = 0;
                let mut perimeter = 0;
                let mut to_visit = vec![(start_row, start_col)];
                while let Some((row, col)) = to_visit.pop() {
                    if visited[(row, col)] {
                        continue;
                    }
                    visited[(row, col)] = true;
                    area += 1;

                    perimeter += 4;
                    for neighbor in NeighborIterator2d::new((row, col), self.grid.size()) {
                        if self.grid[neighbor] == self.grid[(row, col)] {
                            to_visit.push(neighbor);
                            perimeter -= 1;
                        }
                    }
                }
                price += area * perimeter;
            }
        }

        Ok(Solution::with_description("Part 1", price.to_string()))
    }

    fn solve_part_2(&self) -> anyhow::Result<Solution> {
        let mut price = 0;
        let mut visited = GridView::from_vec(
            self.grid.width(),
            0,
            vec![false; self.grid.width() * self.grid.height()],
        );
        for start_row in 0..self.grid.height() {
            for start_col in 0..self.grid.width() {
                let mut area = 0;
                let mut perimeter = 0;
                let mut to_visit = vec![(start_row, start_col)];
                let mut counted_perimeters = GridView::from_vec(
                    self.grid.width(),
                    0,
                    vec![0; self.grid.width() * self.grid.height()],
                );
                while let Some((row, col)) = to_visit.pop() {
                    if visited[(row, col)] {
                        continue;
                    }
                    visited[(row, col)] = true;
                    area += 1;

                    let mut perimeter_sides = 0b1111 & !counted_perimeters[(row, col)];
                    for neighbor in NeighborIterator2d::new((row, col), self.grid.size()) {
                        if self.grid[neighbor] == self.grid[(row, col)] {
                            let side = side_from_neighbor((row, col), neighbor);
                            perimeter_sides &= !side;
                        }
                    }

                    let to_trace = [(0b0001, (1, 0)), (0b0010, (-1, 0))];
                    for (perimeter_side_to_trace, delta) in to_trace {
                        if perimeter_sides & perimeter_side_to_trace == 0 {
                            continue;
                        }

                        let column_iterators: [Box<dyn Iterator<Item = usize>>; 2] = [
                            Box::new(col + 1..self.grid.width()),
                            Box::new((0..col).rev()),
                        ];
                        for columns_iter in column_iterators {
                            for c in columns_iter {
                                if self.grid[(row, c)] == self.grid[(row, col)]
                                    && self.is_fence((row, c), delta)
                                {
                                    counted_perimeters[(row, c)] |= perimeter_side_to_trace;
                                } else {
                                    break;
                                }
                            }
                        }
                    }

                    let to_trace = [(0b0100, (0, 1)), (0b1000, (0, -1))];
                    for (perimeter_side_to_trace, delta) in to_trace {
                        if perimeter_sides & perimeter_side_to_trace == 0 {
                            continue;
                        }

                        let row_iterators: [Box<dyn Iterator<Item = usize>>; 2] = [
                            Box::new(row + 1..self.grid.height()),
                            Box::new((0..row).rev()),
                        ];
                        for row_iter in row_iterators {
                            for r in row_iter {
                                if self.grid[(r, col)] == self.grid[(row, col)]
                                    && self.is_fence((r, col), delta)
                                {
                                    counted_perimeters[(r, col)] |= perimeter_side_to_trace;
                                } else {
                                    break;
                                }
                            }
                        }
                    }

                    perimeter += perimeter_sides.count_ones();
                    for neighbor in NeighborIterator2d::new((row, col), self.grid.size()) {
                        if self.grid[neighbor] == self.grid[(row, col)] {
                            to_visit.push(neighbor);
                        }
                    }
                }
                price += area * perimeter;
            }
        }
        Ok(Solution::with_description("Part 2", price.to_string()))
    }
}

impl SolverImpl<'_> {
    fn is_fence(&self, pos: (usize, usize), delta: (isize, isize)) -> bool {
        let neighbor = (
            pos.0.checked_add_signed(delta.0),
            pos.1.checked_add_signed(delta.1),
        );
        if let (Some(neighbor_row), Some(neighbor_col)) = neighbor {
            if neighbor_row < self.grid.height() && neighbor_col < self.grid.width() {
                self.grid[pos] != self.grid[(neighbor_row, neighbor_col)]
            } else {
                true
            }
        } else {
            true
        }
    }
}

fn side_from_neighbor(pos: (usize, usize), neighbor: (usize, usize)) -> u8 {
    if pos.0 < neighbor.0 && pos.1 == neighbor.1 {
        0b0001
    } else if pos.0 > neighbor.0 && pos.1 == neighbor.1 {
        0b0010
    } else if pos.0 == neighbor.0 && pos.1 < neighbor.1 {
        0b0100
    } else if pos.0 == neighbor.0 && pos.1 > neighbor.1 {
        0b1000
    } else {
        unreachable!()
    }
}

#[cfg(test)]
mod test {
    use super::SolverImpl;
    use crate::solvers::Solver;

    #[test]
    fn test_example_part_1() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day12-1.example"))?;
        assert_eq!(solver.solve_part_1()?.solution, "1930");
        Ok(())
    }

    #[test]
    fn test_example_part_2() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day12-1.example"))?;
        assert_eq!(solver.solve_part_2()?.solution, "1206");
        Ok(())
    }
}
