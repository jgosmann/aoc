use crate::datastructures::grid::GridView;
use crate::solvers::{Solution, Solver};

pub struct SolverImpl<'input> {
    grid: GridView<&'input [u8]>,
}

impl<'input> Solver<'input> for SolverImpl<'input> {
    fn new(input: &'input str) -> anyhow::Result<Self> {
        Ok(Self {
            grid: GridView::from_separated(b'\n', input.as_bytes()),
        })
    }

    fn solve_part_1(&self) -> anyhow::Result<Solution> {
        let mut xmas_count = 0usize;

        for row in 0..self.grid.height() {
            for col in 0..self.grid.width() {
                xmas_count += [
                    (0, -1),
                    (0, 1),
                    (-1, 0),
                    (1, 0),
                    (-1, -1),
                    (1, 1),
                    (-1, 1),
                    (1, -1),
                ]
                .iter()
                .filter(|&direction| self.check_for_xmas((row, col), *direction))
                .count();
            }
        }

        Ok(Solution::with_description("Part 1", xmas_count.to_string()))
    }

    fn solve_part_2(&self) -> anyhow::Result<Solution> {
        let mut crossmas_count = 0usize;

        for row in 1..self.grid.height() - 1 {
            for col in 1..self.grid.width() - 1 {
                if [
                    ((1, 1), (1, -1)),
                    ((-1, -1), (1, -1)),
                    ((1, 1), (-1, 1)),
                    ((-1, -1), (-1, 1)),
                ]
                .iter()
                .any(|&(d0, d1)| {
                    self.check_for_mas((row, col), d0) && self.check_for_mas((row, col), d1)
                }) {
                    crossmas_count += 1;
                }
            }
        }

        Ok(Solution::with_description(
            "Part 2",
            crossmas_count.to_string(),
        ))
    }
}

impl<'input> SolverImpl<'input> {
    fn check_for_xmas(&self, (row, col): (usize, usize), direction: (isize, isize)) -> bool {
        if self.grid[(row, col)] != b'X' {
            return false;
        }

        if direction.0 < 0 && row < 3 {
            return false;
        }
        if direction.0 > 0 && row >= self.grid.height() - 3 {
            return false;
        }
        if direction.1 < 0 && col < 3 {
            return false;
        }
        if direction.1 > 0 && col >= self.grid.width() - 3 {
            return false;
        }

        for (i, c) in [b'M', b'A', b'S'].iter().enumerate() {
            if self.grid[(
                row.checked_add_signed((i + 1) as isize * direction.0)
                    .unwrap(),
                col.checked_add_signed((i + 1) as isize * direction.1)
                    .unwrap(),
            )] != *c
            {
                return false;
            }
        }

        true
    }

    fn check_for_mas(&self, (row, col): (usize, usize), direction: (isize, isize)) -> bool {
        if self.grid[(row, col)] != b'A' {
            return false;
        }
        if self.grid[(
            row.checked_add_signed(-direction.0).expect("invalid row"),
            col.checked_add_signed(-direction.1).expect("invalid col"),
        )] == b'M'
            && self.grid[(
                row.checked_add_signed(direction.0).expect("invalid row"),
                col.checked_add_signed(direction.1).expect("invalid col"),
            )] == b'S'
        {
            return true;
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
        let solver = SolverImpl::new(include_str!("./day4-1.example"))?;
        assert_eq!(solver.solve_part_1()?.solution, "18");
        Ok(())
    }

    #[test]
    fn test_example_part_2() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day4-1.example"))?;
        assert_eq!(solver.solve_part_2()?.solution, "9");
        Ok(())
    }
}
