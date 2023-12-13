use crate::datastructures::grid::GridView;
use crate::solvers::{Solution, Solver};

fn is_horizontal_reflection(grid: &GridView<&[u8]>, index: usize) -> bool {
    let mut top = index;
    let mut bottom = index + 1;
    loop {
        if !grid
            .row(top)
            .iter()
            .zip(grid.row(bottom).iter())
            .all(|(a, b)| a == b)
        {
            return false;
        }

        if top == 0 || bottom >= grid.height() - 1 {
            return true;
        }

        top -= 1;
        bottom += 1;
    }
}

fn is_vertical_reflection(grid: &GridView<&[u8]>, index: usize) -> bool {
    let mut left = index;
    let mut right = index + 1;
    loop {
        if !grid
            .col(left)
            .iter()
            .zip(grid.col(right).iter())
            .all(|(a, b)| a == b)
        {
            return false;
        }

        if left == 0 || right >= grid.width() - 1 {
            return true;
        }

        left -= 1;
        right += 1;
    }
}

fn find_grid_reflection(grid: &GridView<&[u8]>) -> Option<usize> {
    for i in 0..grid.height() - 1 {
        if is_horizontal_reflection(grid, i) {
            return Some(100 * (i + 1));
        }
    }
    for i in 0..grid.width() - 1 {
        if is_vertical_reflection(grid, i) {
            return Some(i + 1);
        }
    }
    None
}

pub struct SolverImpl<'input> {
    grids: Vec<GridView<&'input [u8]>>,
}

impl<'input> Solver<'input> for SolverImpl<'input> {
    fn new(input: &'input str) -> anyhow::Result<Self> {
        let grids = input
            .split("\n\n")
            .map(|grid| GridView::from_separated(b'\n', grid.as_bytes()))
            .collect::<Vec<_>>();

        Ok(Self { grids })
    }

    fn solve_part_1(&self) -> anyhow::Result<Solution> {
        let result: usize = self.grids.iter().filter_map(find_grid_reflection).sum();
        Ok(Solution::with_description("Part 1", result.to_string()))
    }

    fn solve_part_2(&self) -> anyhow::Result<Solution> {
        Ok(Solution::with_description(
            "Part 2",
            "not implemented".to_string(),
        ))
    }
}

#[cfg(test)]
mod test {
    use super::SolverImpl;
    use crate::solvers::Solver;

    #[test]
    fn test_example_part_1() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day13-1.example"))?;
        assert_eq!(solver.solve_part_1()?.solution, "405");
        Ok(())
    }

    #[test]
    fn test_example_part_2() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day13-1.example"))?;
        assert_eq!(solver.solve_part_2()?.solution, "TODO");
        Ok(())
    }
}
