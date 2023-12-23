use std::collections::HashSet;

use anyhow::anyhow;

use crate::solvers::{Solution, Solver};

type Coord = usize;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Brick {
    pub x: (Coord, Coord),
    pub y: (Coord, Coord),
    pub z: (Coord, Coord),
}

impl Brick {
    fn would_stack(&self, other: &Self) -> bool {
        (self.x.1 >= other.x.0 && other.x.1 >= self.x.0)
            && (self.y.1 >= other.y.0 && other.y.1 >= self.y.0)
    }
}

impl TryFrom<&str> for Brick {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let create_err = || anyhow!("Invalid brick definition");
        let (from, to) = value.split_once('~').ok_or_else(create_err)?;
        let mut from = from.split(',').map(str::parse::<Coord>);
        let mut to = to.split(',').map(str::parse::<Coord>);
        Ok(Self {
            x: (
                from.next().ok_or_else(create_err)??,
                to.next().ok_or_else(create_err)??,
            ),
            y: (
                from.next().ok_or_else(create_err)??,
                to.next().ok_or_else(create_err)??,
            ),
            z: (
                from.next().ok_or_else(create_err)??,
                to.next().ok_or_else(create_err)??,
            ),
        })
    }
}

fn let_bricks_fall(mut bricks: Vec<Brick>) -> (Vec<Brick>, usize) {
    let mut num_fallen = 0;
    bricks.sort_unstable_by(|a, b| a.z.1.cmp(&b.z.1));
    for i in 0..bricks.len() {
        let mut supported_by_other_brick = false;
        for j in (0..i).rev() {
            if bricks[i].would_stack(&bricks[j]) && bricks[i].z.0 > bricks[j].z.1 {
                let d = bricks[i].z.0 - bricks[j].z.1 - 1;
                if d > 0 {
                    num_fallen += 1;
                }
                bricks[i].z = (bricks[i].z.0 - d, bricks[i].z.1 - d);
                supported_by_other_brick = true;
                break;
            }
        }
        if !supported_by_other_brick && bricks[i].z.0 != 1 {
            bricks[i].z.1 -= bricks[i].z.0 - 1;
            bricks[i].z.0 = 1;
            num_fallen += 1;
        }
        for k in (0..i).rev() {
            if bricks[k].z.1 <= bricks[k + 1].z.1 {
                break;
            }
            bricks.swap(k, k + 1);
        }
    }
    (bricks, num_fallen)
}

pub struct SolverImpl {
    bricks: Vec<Brick>,
    required_supports: HashSet<usize>,
}

impl<'input> Solver<'input> for SolverImpl {
    fn new(input: &'input str) -> anyhow::Result<Self> {
        let mut bricks = input
            .lines()
            .map(Brick::try_from)
            .collect::<anyhow::Result<Vec<_>>>()?;
        bricks = let_bricks_fall(bricks).0;

        let mut supported_by = vec![HashSet::new(); bricks.len()];
        let mut supporting = vec![HashSet::new(); bricks.len()];
        for i in (0..bricks.len()).rev() {
            for j in (0..i).rev() {
                if bricks[j].z.1 >= bricks[i].z.0 {
                    continue;
                }
                if bricks[j].z.1 < bricks[i].z.0 - 1 {
                    break;
                }
                if bricks[i].would_stack(&bricks[j]) {
                    supporting[j].insert(i);
                    supported_by[i].insert(j);
                }
            }
        }

        let required_supports: HashSet<_> = supported_by
            .iter()
            .filter(|s| s.len() == 1)
            .flatten()
            .copied()
            .collect();

        Ok(Self {
            bricks,
            required_supports,
        })
    }

    fn solve_part_1(&self) -> anyhow::Result<Solution> {
        let disintegratable = self.bricks.len() - self.required_supports.len();
        Ok(Solution::with_description(
            "Bricks safe to disintegrate",
            disintegratable.to_string(),
        ))
    }

    fn solve_part_2(&self) -> anyhow::Result<Solution> {
        let total_falling: usize = self
            .required_supports
            .iter()
            .map(|&removed| {
                let bricks: Vec<_> = self
                    .bricks
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| *i != removed)
                    .map(|(_, b)| b.clone())
                    .collect();
                let_bricks_fall(bricks).1
            })
            .sum();

        Ok(Solution::with_description(
            "Bricks that could fall",
            total_falling.to_string(),
        ))
    }
}

#[cfg(test)]
mod test {
    use super::SolverImpl;
    use crate::solvers::Solver;

    #[test]
    fn test_example_part_1() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day22-1.example"))?;
        assert_eq!(solver.solve_part_1()?.solution, "5");
        Ok(())
    }

    #[test]
    fn test_example_part_2() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day22-1.example"))?;
        assert_eq!(solver.solve_part_2()?.solution, "7");
        Ok(())
    }
}
