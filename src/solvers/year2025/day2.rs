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
        let invalid_id_sum: u64 = self
            .ranges
            .iter()
            .map(|range| {
                (range.0..=range.1)
                    .filter(|&x| is_repeating(x))
                    .sum::<u64>()
            })
            .sum();
        Ok(Solution::with_description(
            "Part 2",
            invalid_id_sum.to_string(),
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

fn is_repeating(value: u64) -> bool {
    let base10_str = value.to_string();
    let base10_bytes = base10_str.as_bytes();
    for i in 1..base10_bytes.len() {
        if base10_bytes.len() % i != 0 {
            continue;
        }
        if (i..base10_bytes.len())
            .step_by(i)
            .all(|j| base10_bytes[..i].starts_with(&base10_bytes[j..(j + i)]))
        {
            return true;
        }
    }
    false
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
        assert_eq!(solver.solve_part_2()?.solution, "4174379265");
        Ok(())
    }
}
