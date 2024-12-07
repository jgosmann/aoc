use crate::solvers::{Solution, Solver};

#[derive(Clone, Debug)]
struct Equation {
    test_value: u64,
    numbers: Vec<u64>,
}

impl Equation {
    pub fn can_be_fulfilled(&self) -> bool {
        self.can_be_fulfilled_impl(self.numbers[0], &self.numbers[1..])
    }

    fn can_be_fulfilled_impl(&self, accumulator: u64, remaining: &[u64]) -> bool {
        if remaining.is_empty() {
            return accumulator == self.test_value;
        }
        if accumulator > self.test_value {
            return false;
        }
        self.can_be_fulfilled_impl(accumulator * remaining[0], &remaining[1..])
            || self.can_be_fulfilled_impl(accumulator + remaining[0], &remaining[1..])
    }

    pub fn can_be_fulfilled_with_concat(&self) -> bool {
        self.can_be_fulfilled_with_concat_impl(self.numbers[0], &self.numbers[1..])
    }

    pub fn can_be_fulfilled_with_concat_impl(&self, accumulator: u64, remaining: &[u64]) -> bool {
        if remaining.is_empty() {
            return accumulator == self.test_value;
        }
        if accumulator > self.test_value {
            return false;
        }
        self.can_be_fulfilled_with_concat_impl(accumulator * remaining[0], &remaining[1..])
            || self.can_be_fulfilled_with_concat_impl(
                num_concat(accumulator, remaining[0]),
                &remaining[1..],
            )
            || self.can_be_fulfilled_with_concat_impl(accumulator + remaining[0], &remaining[1..])
    }
}

fn num_concat(prefix: u64, suffix: u64) -> u64 {
    format!("{}{}", prefix, suffix).parse().unwrap()
}

pub struct SolverImpl {
    equations: Vec<Equation>,
}

impl<'input> Solver<'input> for SolverImpl {
    fn new(input: &'input str) -> anyhow::Result<Self> {
        let equations = input
            .lines()
            .map(|line| {
                let mut parts_iter = line.split(':').map(str::trim);
                let test_value = parts_iter
                    .next()
                    .expect("no test value")
                    .parse::<u64>()
                    .expect("invalid test value");
                let numbers = parts_iter
                    .next()
                    .expect("no numbers")
                    .split(' ')
                    .map(|num| num.parse::<u64>().expect("invalid number"))
                    .collect();
                Equation {
                    test_value,
                    numbers,
                }
            })
            .collect();
        Ok(Self { equations })
    }

    fn solve_part_1(&self) -> anyhow::Result<Solution> {
        // 303726785232 too low
        // 303766880536
        let result: u64 = self
            .equations
            .iter()
            .filter(|eq| eq.can_be_fulfilled())
            .map(|eq| eq.test_value)
            .sum();
        Ok(Solution::with_description("Part 1", result.to_string()))
    }

    fn solve_part_2(&self) -> anyhow::Result<Solution> {
        let result: u64 = self
            .equations
            .iter()
            .filter(|eq| eq.can_be_fulfilled_with_concat())
            .map(|eq| eq.test_value)
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
        let solver = SolverImpl::new(include_str!("./day7-1.example"))?;
        assert_eq!(solver.solve_part_1()?.solution, "3749");
        Ok(())
    }

    #[test]
    fn test_example_part_2() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day7-1.example"))?;
        assert_eq!(solver.solve_part_2()?.solution, "11387");
        Ok(())
    }
}
