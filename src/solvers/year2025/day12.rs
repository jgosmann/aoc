use crate::solvers::{Solution, Solver};
use anyhow::anyhow;

struct Region {
    area: (u64, u64),
    counts: Vec<u64>,
}

pub struct SolverImpl {
    regions: Vec<Region>,
}

impl<'input> Solver<'input> for SolverImpl {
    fn new(input: &'input str) -> anyhow::Result<Self> {
        let regions: Vec<Region> = input
            .lines()
            .filter(|line| line.contains('x'))
            .map(|line| {
                let (area, counts) = line.split_once(':').ok_or(anyhow!("invalid input"))?;
                let area = area.split_once('x').ok_or(anyhow!("invalid input"))?;
                let area = (area.0.parse::<u64>()?, area.1.parse::<u64>()?);
                let counts: Vec<u64> = counts
                    .trim()
                    .split(' ')
                    .map(|count| count.parse::<u64>())
                    .collect::<Result<_, _>>()?;
                Ok(Region { area, counts })
            })
            .collect::<anyhow::Result<_>>()?;
        Ok(Self { regions })
    }

    fn solve_part_1(&self) -> anyhow::Result<Solution> {
        let result = self
            .regions
            .iter()
            .filter(|region| {
                region.counts.iter().map(|&c| 9 * c).sum::<u64>() <= region.area.0 * region.area.1
            })
            .count();
        Ok(Solution::with_description("Part 1", result.to_string()))
    }

    fn solve_part_2(&self) -> anyhow::Result<Solution> {
        Ok(Solution::with_description(
            "Part 2",
            "no part 2".to_string(),
        ))
    }
}
