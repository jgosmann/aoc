use crate::solvers::{Solution, Solver};

pub struct SolverImpl {
    reports: Vec<Vec<i64>>,
}

fn is_safe<'a>(levels: impl IntoIterator<Item = &'a i64>) -> bool {
    let mut iter = levels.into_iter();
    let mut prev = iter.next().expect("empty input");
    let differences: Vec<i64> = iter
        .map(|level| {
            let diff = level - prev;
            prev = level;
            diff
        })
        .collect();
    differences
        .iter()
        .copied()
        .all(|diff| 1 <= diff.abs() && diff.abs() <= 3 && differences[0].signum() == diff.signum())
}

impl<'input> Solver<'input> for SolverImpl {
    fn new(input: &'input str) -> anyhow::Result<Self> {
        let reports = input
            .lines()
            .map(|line| {
                line.split_ascii_whitespace()
                    .map(|level| level.parse().expect("invalid input"))
                    .collect()
            })
            .collect();

        Ok(Self { reports })
    }

    fn solve_part_1(&self) -> anyhow::Result<Solution> {
        let num_safe = self
            .reports
            .iter()
            .filter(|&report| is_safe(report))
            .count();
        Ok(Solution::with_description("Part 1", num_safe.to_string()))
    }

    fn solve_part_2(&self) -> anyhow::Result<Solution> {
        let num_safe = self
            .reports
            .iter()
            .filter(|report| {
                (0..report.len()).into_iter().any(|i| {
                    let (head, tail) = report.split_at(i);
                    is_safe(head.iter().chain(tail[1..].iter()))
                })
            })
            .count();

        Ok(Solution::with_description("Part 2", num_safe.to_string()))
    }
}

#[cfg(test)]
mod test {
    use super::SolverImpl;
    use crate::solvers::Solver;

    #[test]
    fn test_example_part_1() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day2-1.example"))?;
        assert_eq!(solver.solve_part_1()?.solution, "2");
        Ok(())
    }

    #[test]
    fn test_example_part_2() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day2-1.example"))?;
        assert_eq!(solver.solve_part_2()?.solution, "4");
        Ok(())
    }
}
