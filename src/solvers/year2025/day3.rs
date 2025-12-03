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
        let output_voltage: u64 = self
            .banks
            .iter()
            .copied()
            .map(max_joltage_with_override)
            .sum();
        Ok(Solution::with_description(
            "Output voltage with override",
            output_voltage.to_string(),
        ))
    }
}

fn max_joltage(bank: &[u8]) -> u64 {
    let (first_idx, first_value) = next_max(&bank[..bank.len() - 1]);
    let second_value = bank[first_idx + 1..].iter().max().unwrap_or(&0);
    ((first_value - b'0') * 10 + (second_value - b'0')) as u64
}

fn max_joltage_with_override(bank: &[u8]) -> u64 {
    (0..12)
        .fold((0usize, 0u64), |(start_idx, joltage), i| {
            let (max_idx, value) = next_max(&bank[start_idx..bank.len() - 11 + i]);
            (
                start_idx + max_idx + 1,
                joltage * 10 + ((value - b'0') as u64),
            )
        })
        .1
}

fn next_max(bank: &[u8]) -> (usize, u8) {
    bank.iter().enumerate().fold(
        (0, bank[0]),
        |acc, (idx, &value)| {
            if value > acc.1 {
                (idx, value)
            } else {
                acc
            }
        },
    )
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
        assert_eq!(solver.solve_part_2()?.solution, "3121910778619");
        Ok(())
    }
}
