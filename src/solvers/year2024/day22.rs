use crate::solvers::{Solution, Solver};
use std::collections::VecDeque;

struct Rng {
    seed: u64,
}

impl Rng {
    pub fn new(seed: u64) -> Self {
        Self { seed }
    }
}

impl Iterator for Rng {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.seed = ((self.seed * 64) ^ self.seed) % 16777216;
        self.seed = ((self.seed / 32) ^ self.seed) % 16777216;
        self.seed = ((self.seed * 2048) ^ self.seed) % 16777216;
        Some(self.seed)
    }
}

pub struct SolverImpl {
    seeds: Vec<u64>,
}

impl<'input> Solver<'input> for SolverImpl {
    fn new(input: &'input str) -> anyhow::Result<Self> {
        let seeds = input
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|l| l.parse().expect("Invalid seed"))
            .collect();
        Ok(Self { seeds })
    }

    fn solve_part_1(&self) -> anyhow::Result<Solution> {
        let result: u64 = self
            .seeds
            .iter()
            .map(|&seed| Rng::new(seed).nth(1999).unwrap())
            .sum();
        Ok(Solution::with_description("Part 1", result.to_string()))
    }

    fn solve_part_2(&self) -> anyhow::Result<Solution> {
        let mut bananas = [0usize; 19 * 19 * 19 * 19];
        for &seed in self.seeds.iter() {
            let mut sold = [false; 19 * 19 * 19 * 19];
            let mut rng = Rng::new(seed);
            let mut previous_price = (seed % 10) as i8;
            let mut changes: VecDeque<i8> = VecDeque::with_capacity(4);
            for num in (&mut rng).take(3) {
                let price = (num % 10) as i8;
                changes.push_back(price - previous_price);
                previous_price = price;
            }
            for num in rng.take(1997) {
                let price = (num % 10) as i8;
                changes.push_back(price - previous_price);
                previous_price = price;

                let index = changes
                    .iter()
                    .fold(0, |acc, &x| acc * 19 + (x + 9) as usize);
                if !sold[index] {
                    bananas[index] += price as usize;
                    sold[index] = true;
                }

                changes.pop_front();
            }
        }
        let result = bananas.iter().max().unwrap();
        Ok(Solution::with_description("Part 2", result.to_string()))
    }
}

#[cfg(test)]
mod test {
    use super::SolverImpl;
    use crate::solvers::Solver;

    #[test]
    fn test_example_part_1() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day22-1.example"))?;
        assert_eq!(solver.solve_part_1()?.solution, "37327623");
        Ok(())
    }

    #[test]
    fn test_example_part_2() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day22-2.example"))?;
        assert_eq!(solver.solve_part_2()?.solution, "23");
        Ok(())
    }
}
