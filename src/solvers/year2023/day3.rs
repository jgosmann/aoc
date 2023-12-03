use std::{
    collections::BTreeSet,
    ops::{Index, Range},
};

use crate::{
    datastructures::{grid::GridView, iterators::NeighborIterator2d},
    solvers::{MaybeSolution, Solution, Solver},
};

#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq, Hash)]
struct PartNumberLoc {
    row: usize,
    col_span: (usize, usize),
}

impl PartNumberLoc {
    fn col_span(&self) -> Range<usize> {
        Range {
            start: self.col_span.0,
            end: self.col_span.1,
        }
    }
}

impl<'a, T> Index<PartNumberLoc> for GridView<'a, T> {
    type Output = [T];

    fn index(&self, index: PartNumberLoc) -> &Self::Output {
        &self[(index.row, index.col_span())]
    }
}

fn get_part_number_loc(grid: &GridView<'_, u8>, seed: (usize, usize)) -> Option<PartNumberLoc> {
    if !grid[seed].is_ascii_digit() {
        return None;
    }

    let mut start = seed.1;
    while start > 0 && grid[(seed.0, start - 1)].is_ascii_digit() {
        start -= 1;
    }

    let mut end = seed.1;
    while end < grid.width() && grid[(seed.0, end)].is_ascii_digit() {
        end += 1;
    }

    Some(PartNumberLoc {
        col_span: (start, end),
        row: seed.0,
    })
}

#[derive(Debug)]
pub struct SolverImpl {
    part_number_sum: u32,
    gear_ratio_sum: u32,
}

impl<'input> Solver<'input> for SolverImpl {
    fn new(input: &'input str) -> anyhow::Result<Self> {
        let schematic = GridView::from_separated(b'\n', input.as_bytes());

        let mut part_number_sum: u32 = 0;
        let mut gear_ratio_sum: u32 = 0;

        for i in 0..schematic.height() {
            for j in 0..schematic.width() {
                let item = schematic[(i, j)];
                if item == b'.' || item.is_ascii_digit() {
                    continue;
                }

                let part_number_locs: BTreeSet<PartNumberLoc> =
                    NeighborIterator2d::new((i, j), schematic.size())
                        .filter_map(|neighbor| get_part_number_loc(&schematic, neighbor))
                        .collect();
                let part_numbers: Vec<u32> = part_number_locs
                    .into_iter()
                    .map(|part_number_loc| {
                        Ok(std::str::from_utf8(&schematic[part_number_loc])?.parse::<u32>()?)
                    })
                    .collect::<anyhow::Result<Vec<u32>>>()?;

                part_number_sum += part_numbers.iter().sum::<u32>();

                if item == b'*' && part_numbers.len() == 2 {
                    gear_ratio_sum += part_numbers.iter().product::<u32>();
                }
            }
        }
        Ok(Self {
            part_number_sum,
            gear_ratio_sum,
        })
    }

    fn solve_part_1(&self) -> anyhow::Result<Solution> {
        Ok(Solution::with_description(
            "Sum of part numbers",
            self.part_number_sum.to_string(),
        ))
    }

    fn solve_part_2(&self) -> anyhow::Result<MaybeSolution> {
        Ok(MaybeSolution::Present(Solution::with_description(
            "Sum of gear ratios",
            self.gear_ratio_sum.to_string(),
        )))
    }
}

#[cfg(test)]
mod test {
    use super::SolverImpl;
    use crate::solvers::Solver;

    #[test]
    fn test_example_part_1() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day3-1.example"))?;
        assert_eq!(solver.solve_part_1()?.solution, "4361");
        Ok(())
    }

    #[test]
    fn test_example_part_2() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day3-1.example"))?;
        assert_eq!(solver.solve_part_2()?.unwrap().solution, "467835");
        Ok(())
    }
}
