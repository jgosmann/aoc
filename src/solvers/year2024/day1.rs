use crate::solvers::{Solution, Solver};

pub struct SolverImpl {
    lists: [Vec<usize>; 2],
}

impl<'input> Solver<'input> for SolverImpl {
    fn new(input: &'input str) -> anyhow::Result<Self> {
        let mut lists = [vec![], vec![]];
        for line in input.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            let mut parts = line.split_ascii_whitespace();
            lists[0].push(parts.next().expect("empty line").parse()?);
            lists[1].push(parts.last().expect("empty line").parse()?);
        }
        for list in &mut lists {
            list.sort_unstable();
        }
        Ok(Self { lists })
    }

    fn solve_part_1(&self) -> anyhow::Result<Solution> {
        let result: usize = self.lists[0]
            .iter()
            .zip(&self.lists[1])
            .map(|(a, b)| a.abs_diff(*b))
            .sum();
        Ok(Solution::with_description("Part 1", result.to_string()))
    }

    fn solve_part_2(&self) -> anyhow::Result<Solution> {
        let result: usize = self.lists[0]
            .iter()
            .map(|a| {
                a * (self.lists[1].partition_point(|x| x <= a)
                    - self.lists[1].partition_point(|x| x < a))
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
        let solver = SolverImpl::new(include_str!("./day1-1.example"))?;
        assert_eq!(solver.solve_part_1()?.solution, "11");
        Ok(())
    }

    #[test]
    fn test_example_part_2() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day1-1.example"))?;
        assert_eq!(solver.solve_part_2()?.solution, "31");
        Ok(())
    }
}
