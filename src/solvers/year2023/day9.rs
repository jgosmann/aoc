use std::num::ParseIntError;

use crate::solvers::{Solution, Solver};

pub struct SolverImpl {
    histories: Vec<Vec<i64>>,
}

fn extrapolate_left(input: &[i64]) -> i64 {
    if input.len() < 2 || input.iter().all(|&v| v == 0) {
        return input.first().copied().unwrap_or_default();
    }
    input.first().unwrap()
        - extrapolate_left(
            &input
                .windows(2)
                .map(|window| window[1] - window[0])
                .collect::<Vec<_>>(),
        )
}

fn extrapolate_right(input: &[i64]) -> i64 {
    if input.len() < 2 || input.iter().all(|&v| v == 0) {
        return input.last().copied().unwrap_or_default();
    }
    input.last().unwrap()
        + extrapolate_right(
            &input
                .windows(2)
                .map(|window| window[1] - window[0])
                .collect::<Vec<_>>(),
        )
}

impl<'input> Solver<'input> for SolverImpl {
    fn new(input: &'input str) -> anyhow::Result<Self> {
        let histories = input
            .lines()
            .map(|line| {
                line.split(' ')
                    .map(|num| num.trim().parse())
                    .collect::<Result<Vec<i64>, ParseIntError>>()
            })
            .collect::<Result<Vec<Vec<_>>, ParseIntError>>()?;
        Ok(Self { histories })
    }

    fn solve_part_1(&self) -> anyhow::Result<Solution> {
        let extrapolation: i64 = self
            .histories
            .iter()
            .map(|history| extrapolate_right(history))
            .sum();
        Ok(Solution::with_description(
            "Sum of extrapolated values",
            extrapolation.to_string(),
        ))
    }

    fn solve_part_2(&self) -> anyhow::Result<Solution> {
        let extrapolation: i64 = self
            .histories
            .iter()
            .map(|history| extrapolate_left(history))
            .sum();
        Ok(Solution::with_description(
            "Part 2",
            extrapolation.to_string(),
        ))
    }
}

#[cfg(test)]
mod test {
    use super::SolverImpl;
    use crate::solvers::Solver;

    #[test]
    fn test_example_part_1() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day9-1.example"))?;
        assert_eq!(solver.solve_part_1()?.solution, "114");
        Ok(())
    }

    #[test]
    fn test_example_part_2() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day9-1.example"))?;
        assert_eq!(solver.solve_part_2()?.solution, "2");
        Ok(())
    }
}
