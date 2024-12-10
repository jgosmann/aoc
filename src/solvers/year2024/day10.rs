use crate::datastructures::grid::GridView;
use crate::datastructures::iterators::NeighborIterator2d;
use crate::solvers::{Solution, Solver};
use std::collections::BTreeSet;

fn find_summits(map: &GridView<&[u8]>, trailhead: (usize, usize)) -> Vec<(usize, usize)> {
    let mut summits = Vec::new();
    let mut to_visit = vec![trailhead];
    while let Some(pos) = to_visit.pop() {
        let elevation = map[pos];
        if elevation == b'9' {
            summits.push(pos);
            continue;
        }

        for neighbor in NeighborIterator2d::new(pos, map.size()) {
            if let Some(1) = map[neighbor].checked_sub(elevation) {
                to_visit.push(neighbor);
            }
        }
    }
    summits
}

pub struct SolverImpl {
    score_sum: usize,
    rating_sum: usize,
}

impl<'input> Solver<'input> for SolverImpl {
    fn new(input: &'input str) -> anyhow::Result<Self> {
        let map = GridView::from_separated(b'\n', input.as_bytes());
        let mut score_sum = 0;
        let mut rating_sum = 0;
        for row in 0..map.height() {
            for col in 0..map.width() {
                if map[(row, col)] == b'0' {
                    let summits = find_summits(&map, (row, col));
                    score_sum += summits.iter().collect::<BTreeSet<_>>().len();
                    rating_sum += summits.len();
                }
            }
        }
        Ok(Self {
            score_sum,
            rating_sum,
        })
    }

    fn solve_part_1(&self) -> anyhow::Result<Solution> {
        Ok(Solution::with_description(
            "Part 1",
            self.score_sum.to_string(),
        ))
    }

    fn solve_part_2(&self) -> anyhow::Result<Solution> {
        Ok(Solution::with_description(
            "Part 2",
            self.rating_sum.to_string(),
        ))
    }
}

#[cfg(test)]
mod test {
    use super::SolverImpl;
    use crate::solvers::Solver;

    #[test]
    fn test_example_part_1() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day10-1.example"))?;
        assert_eq!(solver.solve_part_1()?.solution, "36");
        Ok(())
    }

    #[test]
    fn test_example_part_2() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day10-1.example"))?;
        assert_eq!(solver.solve_part_2()?.solution, "81");
        Ok(())
    }
}
