use crate::solvers::{Solution, Solver};
use std::collections::HashSet;

pub struct SolverImpl {
    ranges: Vec<(u64, u64)>,
    ingredient_ids: Vec<u64>,
}

impl<'input> Solver<'input> for SolverImpl {
    fn new(input: &'input str) -> anyhow::Result<Self> {
        let mut lines = input.lines();
        let ranges: Vec<_> = lines
            .by_ref()
            .take_while(|line| !line.is_empty())
            .map(|line| {
                if let Some((lower_bound, upper_bound)) = line.split_once('-') {
                    Ok((lower_bound.parse::<u64>()?, upper_bound.parse::<u64>()?))
                } else {
                    Err(anyhow::anyhow!("invalid range"))
                }
            })
            .collect::<anyhow::Result<_>>()?;
        let ingredient_ids: Vec<_> = lines.map(|line| line.parse()).collect::<Result<_, _>>()?;
        Ok(Self {
            ranges,
            ingredient_ids,
        })
    }

    fn solve_part_1(&self) -> anyhow::Result<Solution> {
        let num_fresh = self
            .ingredient_ids
            .iter()
            .filter(|&&ingredient_id| self.is_fresh(ingredient_id))
            .count();
        Ok(Solution::with_description(
            "Fresh ingredients count",
            num_fresh.to_string(),
        ))
    }

    fn solve_part_2(&self) -> anyhow::Result<Solution> {
        let mut merged_ranges: HashSet<(u64, u64)> = HashSet::with_capacity(self.ranges.len());
        let mut queue: Vec<_> = self.ranges.iter().rev().copied().collect();
        while let Some(range) = queue.pop() {
            if let Some(ovelapping_range) = merged_ranges.iter().copied().find(|merge_candidate| {
                (merge_candidate.0 <= range.0 && range.0 <= merge_candidate.1)
                    || (merge_candidate.0 <= range.1 && range.1 <= merge_candidate.1)
                    || (range.0 <= merge_candidate.0 && merge_candidate.0 <= range.1)
                    || (range.0 <= merge_candidate.1 && merge_candidate.1 <= range.1)
            }) {
                let merged_range = (
                    ovelapping_range.0.min(range.0),
                    ovelapping_range.1.max(range.1),
                );
                merged_ranges.remove(&ovelapping_range);
                queue.push(merged_range);
            } else {
                merged_ranges.insert(range);
            }
        }

        let num_fresh: u64 = merged_ranges.iter().map(|(lb, ub)| ub - lb + 1).sum();

        Ok(Solution::with_description(
            "Fresh according to ranges",
            num_fresh.to_string(),
        ))
    }
}

impl SolverImpl {
    fn is_fresh(&self, ingredient_id: u64) -> bool {
        self.ranges.iter().any(|&(lower_bound, upper_bound)| {
            ingredient_id >= lower_bound && ingredient_id <= upper_bound
        })
    }
}

#[cfg(test)]
mod test {
    use super::SolverImpl;
    use crate::solvers::Solver;

    #[test]
    fn test_example_part_1() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day5-1.example"))?;
        assert_eq!(solver.solve_part_1()?.solution, "3");
        Ok(())
    }

    #[test]
    fn test_example_part_2() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day5-1.example"))?;
        assert_eq!(solver.solve_part_2()?.solution, "14");
        Ok(())
    }
}
