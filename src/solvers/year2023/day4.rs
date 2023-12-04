use crate::solvers::{MaybeSolution, Solution, Solver};
use regex::Regex;
use std::collections::BTreeSet;
use std::num::ParseIntError;

fn parse_number_list(input: &str) -> Result<BTreeSet<u32>, ParseIntError> {
    input
        .split(' ')
        .map(|item| item.trim())
        .filter(|item| !item.is_empty())
        .map(|item| item.parse())
        .collect()
}

pub struct SolverImpl {
    num_winning: Vec<usize>,
}

impl<'input> Solver<'input> for SolverImpl {
    fn new(input: &'input str) -> anyhow::Result<Self> {
        let line_pattern = Regex::new(r"^Card\s+(\d+): ([0-9 ]*) \| ([0-9 ]*)$").unwrap();
        let num_winning = input
            .split('\n')
            .filter_map(|line| line_pattern.captures(line))
            .map(|captures| {
                let winning_numbers = parse_number_list(captures.get(2).unwrap().as_str())?;
                let our_numbers = parse_number_list(captures.get(3).unwrap().as_str())?;
                Ok(winning_numbers.intersection(&our_numbers).count())
            })
            .collect::<Result<Vec<usize>, ParseIntError>>()?;

        Ok(Self { num_winning })
    }

    fn solve_part_1(&self) -> anyhow::Result<Solution> {
        let points: u32 = self
            .num_winning
            .iter()
            .copied()
            .filter(|&value| value > 0)
            .map(|num_winning| 1u32 << (num_winning - 1))
            .sum();
        Ok(Solution::with_description("Points", points.to_string()))
    }

    fn solve_part_2(&self) -> anyhow::Result<MaybeSolution> {
        let mut n_copies = vec![1; self.num_winning.len()];
        let mut total_cards = 0;
        for i in 0..self.num_winning.len() {
            total_cards += n_copies[i];
            for j in i + 1..self.num_winning.len().min(i + 1 + self.num_winning[i]) {
                n_copies[j] += n_copies[i];
            }
        }

        Ok(MaybeSolution::Present(Solution::with_description(
            "Number of scratch cards",
            total_cards.to_string(),
        )))
    }
}

#[cfg(test)]
mod test {
    use super::SolverImpl;
    use crate::solvers::Solver;

    #[test]
    fn test_example_part_1() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day4-1.example"))?;
        assert_eq!(solver.solve_part_1()?.solution, "13");
        Ok(())
    }

    #[test]
    fn test_example_part_2() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day4-1.example"))?;
        assert_eq!(solver.solve_part_2()?.unwrap().solution, "30");
        Ok(())
    }
}
