use crate::solvers::{Solution, Solver};

pub struct SolverImpl<'input> {
    banks: Vec<&'input [u8]>,
}

impl<'input> Solver<'input> for SolverImpl<'input> {
    fn new(input: &'input str) -> anyhow::Result<Self> {
        Ok(Self {
            banks: input.lines().map(str::trim).map(str::as_bytes).collect(),
        })
    }

    fn solve_part_1(&self) -> anyhow::Result<Solution> {
        let output_voltage: u64 = self.banks.iter().copied().map(max_joltage).sum();
        Ok(Solution::with_description(
            "Output voltage",
            output_voltage.to_string(),
        ))
    }

    fn solve_part_2(&self) -> anyhow::Result<Solution> {
        Ok(Solution::with_description(
            "Part 2",
            "not implemented".to_string(),
        ))
    }
}

fn max_joltage(bank: &[u8]) -> u64 {
    let (first_idx, first_value) =
        bank[..bank.len() - 1]
            .iter()
            .enumerate()
            .fold(
                (0, bank[0]),
                |acc, (idx, &value)| {
                    if value > acc.1 {
                        (idx, value)
                    } else {
                        acc
                    }
                },
            );
    let second_value = bank[first_idx + 1..].iter().max().unwrap_or(&0);
    ((first_value - b'0') * 10 + (second_value - b'0')) as u64
}

#[cfg(test)]
mod test {
    use super::SolverImpl;
    use crate::solvers::Solver;

    #[test]
    fn test_example_part_1() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day3-1.example"))?;
        assert_eq!(solver.solve_part_1()?.solution, "357");
        Ok(())
    }

    #[test]
    fn test_example_part_2() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day3-1.example"))?;
        assert_eq!(solver.solve_part_2()?.solution, "TODO");
        Ok(())
    }
}
