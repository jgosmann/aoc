use crate::datastructures::grid::GridView;
use crate::solvers::{Solution, Solver};
use std::collections::{HashMap, HashSet};

type Frequency = u8;
type Location = (isize, isize);

pub struct SolverImpl {
    antennas: HashMap<Frequency, Vec<Location>>,
    width: isize,
    height: isize,
}

impl<'input> Solver<'input> for SolverImpl {
    fn new(input: &'input str) -> anyhow::Result<Self> {
        let grid = GridView::from_separated(b'\n', input.as_bytes());
        let mut antennas: HashMap<Frequency, Vec<Location>> = HashMap::new();
        for row in 0..grid.height() {
            for col in 0..grid.width() {
                let frequency = grid[(row, col)];
                if frequency != b'.' {
                    antennas
                        .entry(frequency)
                        .or_default()
                        .push((row as isize, col as isize));
                }
            }
        }
        Ok(Self {
            antennas,
            width: grid.width() as isize,
            height: grid.height() as isize,
        })
    }

    fn solve_part_1(&self) -> anyhow::Result<Solution> {
        let antinodes: HashSet<Location> = self
            .antennas
            .iter()
            .flat_map(|(_, loc)| {
                loc.iter().flat_map(|loc_a| {
                    loc.iter()
                        .filter(|&loc_b| loc_b != loc_a)
                        .map(|loc_b| {
                            let d_row = loc_a.0 - loc_b.0;
                            let d_col = loc_a.1 - loc_b.1;
                            (loc_a.0 + d_row, loc_a.1 + d_col)
                        })
                        .collect::<Vec<_>>()
                })
            })
            .collect();
        let result = antinodes
            .iter()
            .filter(|antinode| {
                0 <= antinode.0
                    && antinode.0 < self.height
                    && 0 <= antinode.1
                    && antinode.1 < self.width
            })
            .count();
        Ok(Solution::with_description("Part 1", result.to_string()))
    }

    fn solve_part_2(&self) -> anyhow::Result<Solution> {
        // 1252 too high
        let antinodes: HashSet<Location> = self
            .antennas
            .iter()
            .flat_map(|(_, loc)| {
                loc.iter().flat_map(|loc_a| {
                    loc.iter()
                        .filter(|&loc_b| loc_b != loc_a)
                        .flat_map(|loc_b| {
                            let d_row = loc_a.0 - loc_b.0;
                            let d_col = loc_a.1 - loc_b.1;
                            let mut antinodes = Vec::new();
                            let mut antinode_candidate = *loc_a;
                            while 0 <= antinode_candidate.0
                                && antinode_candidate.0 < self.height
                                && 0 <= antinode_candidate.1
                                && antinode_candidate.1 < self.width
                            {
                                antinodes.push(antinode_candidate);
                                antinode_candidate =
                                    (antinode_candidate.0 + d_row, antinode_candidate.1 + d_col);
                            }
                            antinodes
                        })
                        .collect::<Vec<_>>()
                })
            })
            .collect();
        Ok(Solution::with_description(
            "Part 2",
            antinodes.len().to_string(),
        ))
    }
}

#[cfg(test)]
mod test {
    use super::SolverImpl;
    use crate::solvers::Solver;

    #[test]
    fn test_example_part_1() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day8-1.example"))?;
        assert_eq!(solver.solve_part_1()?.solution, "14");
        Ok(())
    }

    #[test]
    fn test_example_part_2() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day8-1.example"))?;
        assert_eq!(solver.solve_part_2()?.solution, "34");
        Ok(())
    }
}
