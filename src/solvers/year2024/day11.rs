use crate::solvers::{Solution, Solver};
use std::collections::HashMap;

struct StoneOracle {
    cache: HashMap<(usize, usize), usize>,
}

impl StoneOracle {
    fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    fn blink(&mut self, stone: usize, steps: usize) -> usize {
        if steps == 0 {
            return 1;
        }

        if let Some(&result) = self.cache.get(&(stone, steps)) {
            return result;
        }

        if stone == 0 {
            let result = self.blink(1, steps - 1);
            self.cache.insert((stone, steps), result);
            return result;
        }
        let digits = stone.to_string();
        if digits.len() % 2 == 0 {
            let left = digits[..digits.len() / 2].parse().unwrap();
            let right = digits[digits.len() / 2..].parse().unwrap();
            let result = self.blink(left, steps - 1) + self.blink(right, steps - 1);
            self.cache.insert((stone, steps), result);
            return result;
        }

        let result = self.blink(stone * 2024, steps - 1);
        self.cache.insert((stone, steps), result);
        result
    }
}

pub struct SolverImpl {
    stones: Vec<usize>,
}

impl<'input> Solver<'input> for SolverImpl {
    fn new(input: &'input str) -> anyhow::Result<Self> {
        let stones = input
            .split(' ')
            .map(|s| s.trim().parse().expect("not a valid number"))
            .collect();
        Ok(Self { stones })
    }

    fn solve_part_1(&self) -> anyhow::Result<Solution> {
        let mut stone_oracle = StoneOracle::new();
        let mut result = 0;
        for &stone in &self.stones {
            result += stone_oracle.blink(stone, 25);
        }
        Ok(Solution::with_description("Part 1", result.to_string()))
    }

    fn solve_part_2(&self) -> anyhow::Result<Solution> {
        let mut stone_oracle = StoneOracle::new();
        let mut result = 0;
        for &stone in &self.stones {
            result += stone_oracle.blink(stone, 75);
        }
        Ok(Solution::with_description("Part 2", result.to_string()))
    }
}

#[cfg(test)]
mod test {
    use super::SolverImpl;
    use crate::solvers::Solver;

    #[test]
    fn test_example_part_1() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day11-1.example"))?;
        assert_eq!(solver.solve_part_1()?.solution, "55312");
        Ok(())
    }
}
