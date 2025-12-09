use crate::solvers::{Solution, Solver};
use anyhow::anyhow;

type Pos = (u64, u64);

pub struct SolverImpl {
    red_tiles: Vec<Pos>,
}

impl<'input> Solver<'input> for SolverImpl {
    fn new(input: &'input str) -> anyhow::Result<Self> {
        let red_tiles = input
            .lines()
            .map(|line| {
                let (x, y) = line.split_once(',').ok_or(anyhow!("invalid input"))?;
                Ok((x.parse::<u64>()?, y.parse::<u64>()?))
            })
            .collect::<anyhow::Result<Vec<_>>>()?;

        Ok(Self { red_tiles })
    }

    fn solve_part_1(&self) -> anyhow::Result<Solution> {
        let largest_area = self
            .red_tiles
            .iter()
            .enumerate()
            .flat_map(|(i, tile_a)| {
                self.red_tiles[i + 1..].iter().map(move |tile_b| {
                    (tile_a.0.abs_diff(tile_b.0) + 1) * (tile_a.1.abs_diff(tile_b.1) + 1)
                })
            })
            .max()
            .ok_or(anyhow!("no solution found"))?;
        Ok(Solution::with_description(
            "Largest area",
            largest_area.to_string(),
        ))
    }

    fn solve_part_2(&self) -> anyhow::Result<Solution> {
        let largest_area = self
            .red_tiles
            .iter()
            .enumerate()
            .flat_map(|(i, tile_a)| {
                self.red_tiles[i + 1..]
                    .iter()
                    .filter(|tile_b| {
                        (0..self.red_tiles.len()).all(|k| {
                            !intersects(
                                (*tile_a, **tile_b),
                                (
                                    self.red_tiles[k],
                                    self.red_tiles[(k + 1) % self.red_tiles.len()],
                                ),
                            )
                        })
                    })
                    .map(move |tile_b| {
                        (tile_a.0.abs_diff(tile_b.0) + 1) * (tile_a.1.abs_diff(tile_b.1) + 1)
                    })
            })
            .max()
            .ok_or(anyhow!("no solution found"))?;
        Ok(Solution::with_description(
            "Largest area with only red and green tiles",
            largest_area.to_string(),
        ))
    }
}

fn intersects(rect: (Pos, Pos), line: (Pos, Pos)) -> bool {
    if line.0 .0 == line.1 .0 {
        let lb = rect.0 .1.min(rect.1 .1);
        let ub = rect.0 .1.max(rect.1 .1);
        !((line.0 .1 <= lb && line.1 .1 <= lb) || (line.0 .1 >= ub && line.1 .1 >= ub))
            && (rect.0 .0.min(rect.1 .0) < line.0 .0 && line.0 .0 < rect.0 .0.max(rect.1 .0))
    } else if line.0 .1 == line.1 .1 {
        let lb = rect.0 .0.min(rect.1 .0);
        let ub = rect.0 .0.max(rect.1 .0);
        !((line.0 .0 <= lb && line.1 .0 <= lb) || (line.0 .0 >= ub && line.1 .0 >= ub))
            && (rect.0 .1.min(rect.1 .1) < line.0 .1 && line.0 .1 < rect.0 .1.max(rect.1 .1))
    } else {
        panic!("invalid tile shape");
    }
}

#[cfg(test)]
mod test {
    use super::SolverImpl;
    use crate::solvers::Solver;

    #[test]
    fn test_example_part_1() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day9-1.example"))?;
        assert_eq!(solver.solve_part_1()?.solution, "50");
        Ok(())
    }

    #[test]
    fn test_example_part_2() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day9-1.example"))?;
        assert_eq!(solver.solve_part_2()?.solution, "24");
        Ok(())
    }
}
