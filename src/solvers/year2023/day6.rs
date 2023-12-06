use crate::solvers::{Solution, Solver};
use std::num::ParseIntError;

fn parse_line(line: &str, prefix: &str) -> anyhow::Result<Vec<u64>> {
    anyhow::ensure!(line.starts_with(prefix), "invalid line prefix");
    let line = &line[prefix.len()..];
    line.split(' ')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|s| Ok(s.parse()?))
        .collect()
}

fn solve_quadratic(p: f64, q: f64) -> (f64, f64) {
    let p_half = p / 2.0;
    let discriminant = (p_half * p_half - q).sqrt();
    (-p_half - discriminant, -p_half + discriminant)
}

fn calc_ways_to_win(time: u64, distance: u64) -> u64 {
    let (min, max) = solve_quadratic(-(time as f64), distance as f64);
    (max.ceil() - min.floor()) as u64 - 1
}

fn join_numbers(numbers: &[u64]) -> Result<u64, ParseIntError> {
    numbers
        .iter()
        .map(u64::to_string)
        .collect::<Vec<_>>()
        .join("")
        .parse::<u64>()
}

pub struct SolverImpl {
    times: Vec<u64>,
    distances: Vec<u64>,
}

impl<'input> Solver<'input> for SolverImpl {
    fn new(input: &'input str) -> anyhow::Result<Self> {
        let mut lines = input.lines();
        let times = parse_line(lines.next().expect("times line"), "Time:")?;
        let distances = parse_line(lines.next().expect("distances line"), "Distance:")?;
        anyhow::ensure!(
            times.len() == distances.len(),
            "times and distances must have the same length"
        );

        Ok(Self { times, distances })
    }

    fn solve_part_1(&self) -> anyhow::Result<Solution> {
        let ways_to_win: u64 = self
            .times
            .iter()
            .zip(self.distances.iter())
            .map(|(&t, &d)| calc_ways_to_win(t, d))
            .product();
        Ok(Solution::with_description(
            "Product of ways to win (part 1)",
            ways_to_win.to_string(),
        ))
    }

    fn solve_part_2(&self) -> anyhow::Result<Solution> {
        let time = join_numbers(&self.times)?;
        let distance = join_numbers(&self.distances)?;
        let ways_to_win = calc_ways_to_win(time, distance);
        Ok(Solution::with_description(
            "Part 2",
            ways_to_win.to_string(),
        ))
    }
}

#[cfg(test)]
mod test {
    use super::SolverImpl;
    use crate::solvers::Solver;

    #[test]
    fn test_example_part_1() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day6-1.example"))?;
        assert_eq!(solver.solve_part_1()?.solution, "288");
        Ok(())
    }

    #[test]
    fn test_example_part_2() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day6-1.example"))?;
        assert_eq!(solver.solve_part_2()?.solution, "71503");
        Ok(())
    }
}
