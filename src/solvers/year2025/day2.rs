use crate::solvers::{Solution, Solver};

pub struct SolverImpl {
    ranges: Vec<(u64, u64)>,
}

impl<'input> Solver<'input> for SolverImpl {
    fn new(input: &'input str) -> anyhow::Result<Self> {
        let ranges = input
            .trim()
            .split(',')
            .map(|range_def| {
                let range = range_def.split_once('-').expect("invalid range");
                Ok((range.0.parse()?, range.1.parse()?))
            })
            .collect::<anyhow::Result<Vec<_>>>()?;
        Ok(Self { ranges })
    }

    fn solve_part_1(&self) -> anyhow::Result<Solution> {
        let invalid_id_sum: u64 = self
            .ranges
            .iter()
            .copied()
            .map(sum_invalid_ids_in_range)
            .sum();
        Ok(Solution::with_description(
            "Part 1",
            invalid_id_sum.to_string(),
        ))
    }

    fn solve_part_2(&self) -> anyhow::Result<Solution> {
        Ok(Solution::with_description(
            "Part 2",
            "not implemented".to_string(),
        ))
    }
}

fn sum_invalid_ids_in_range(range: (u64, u64)) -> u64 {
    let lower_bound_str = range.0.to_string();
    let lower_prefix = if lower_bound_str.len() % 2 == 1 {
        10u64.pow((lower_bound_str.len() / 2) as u32)
    } else {
        lower_bound_str
            .split_at(lower_bound_str.len() / 2)
            .0
            .parse()
            .unwrap()
    };
    (lower_prefix..)
        .map(prefix_to_id)
        .skip_while(|&id| id < range.0)
        .take_while(|&id| id <= range.1)
        .sum()
}

fn prefix_to_id(prefix: u64) -> u64 {
    10u64.pow(prefix.ilog10() + 1) * prefix + prefix
}

#[cfg(test)]
mod test {
    use super::SolverImpl;
    use crate::solvers::Solver;

    #[test]
    fn test_example_part_1() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day2-1.example"))?;
        assert_eq!(solver.solve_part_1()?.solution, "1227775554");
        Ok(())
    }

    #[test]
    fn test_example_part_2() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day2-1.example"))?;
        assert_eq!(solver.solve_part_2()?.solution, "TODO");
        Ok(())
    }
}
