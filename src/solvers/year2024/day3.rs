use crate::solvers::{Solution, Solver};
use regex::Regex;
use std::convert::identity;

pub struct SolverImpl<'input> {
    input: &'input str,
}

impl<'input> Solver<'input> for SolverImpl<'input> {
    fn new(input: &'input str) -> anyhow::Result<Self> {
        Ok(Self { input })
    }

    fn solve_part_1(&self) -> anyhow::Result<Solution> {
        let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
        let result: u64 = re
            .captures_iter(self.input)
            .map(|m| {
                m.iter()
                    .skip(1)
                    .filter_map(identity)
                    .map(|c| c.as_str().parse::<u64>().expect("invalid number"))
                    .product::<u64>()
            })
            .sum();
        Ok(Solution::with_description("Part 1", result.to_string()))
    }

    fn solve_part_2(&self) -> anyhow::Result<Solution> {
        let re = Regex::new(r"(do|don't|mul)\(((\d{1,3}),(\d{1,3}))?\)").unwrap();
        let mut mul_enabled = true;
        let result: u64 = re
            .captures_iter(self.input)
            .map(|m| {
                match &m[1] {
                    "do" => mul_enabled = true,
                    "don't" => mul_enabled = false,
                    "mul" => {
                        if mul_enabled {
                            return m
                                .iter()
                                .skip(1)
                                .filter_map(identity)
                                .filter_map(|c| c.as_str().parse::<u64>().ok())
                                .product::<u64>();
                        }
                    }
                    _ => {}
                }
                return 0;
            })
            .sum();
        Ok(Solution::with_description("Part 2", result.to_string()))
    }
}

#[cfg(test)]
mod test {
    use super::SolverImpl;
    use crate::solvers::Solver;

    #[test]
    fn test_example_part_1() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day3-1.example"))?;
        assert_eq!(solver.solve_part_1()?.solution, "161");
        Ok(())
    }

    #[test]
    fn test_example_part_2() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day3-2.example"))?;
        assert_eq!(solver.solve_part_2()?.solution, "48");
        Ok(())
    }
}
