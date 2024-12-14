use crate::solvers::{Solution, Solver};
use anyhow::anyhow;
use regex::Regex;
use std::cmp::Ordering;
use std::collections::BTreeSet;

const WIDTH: i64 = 101; //11;
const HEIGHT: i64 = 103; //7;

#[derive(Debug, Clone)]
struct Robot {
    p: (i64, i64),
    v: (i64, i64),
}

impl Robot {
    fn position_after(&self, steps: i64, width: i64, height: i64) -> (i64, i64) {
        let mut x = (self.p.0 + self.v.0 * steps) % width;
        let mut y = (self.p.1 + steps * self.v.1) % height;
        if x < 0 {
            x += width;
        }
        if y < 0 {
            y += height;
        }
        (x, y)
    }
}

impl TryFrom<&str> for Robot {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let pattern = Regex::new(r"^p=(\d+),(\d+) v=(-?\d+),(-?\d+)$")?;
        let captures = pattern.captures(value).ok_or(anyhow!("Invalid input"))?;
        Ok(Self {
            p: (captures[1].parse()?, captures[2].parse()?),
            v: (captures[3].parse()?, captures[4].parse()?),
        })
    }
}

pub struct SolverImpl {
    robots: Vec<Robot>,
}

impl SolverImpl {
    fn solve_part_1_for_size(&self, width: i64, height: i64) -> u64 {
        self.robots
            .iter()
            .map(|robot| {
                let (x, y) = robot.position_after(100, width, height);
                let vert_half = match x.cmp(&(width / 2)) {
                    Ordering::Less => (1, 0),
                    Ordering::Greater => (0, 1),
                    Ordering::Equal => (0, 0),
                };
                match y.cmp(&(height / 2)) {
                    Ordering::Less => (vert_half, (0, 0)),
                    Ordering::Greater => ((0, 0), vert_half),
                    Ordering::Equal => ((0, 0), (0, 0)),
                }
            })
            .reduce(|a, b| {
                (
                    (a.0 .0 + b.0 .0, a.0 .1 + b.0 .1),
                    (a.1 .0 + b.1 .0, a.1 .1 + b.1 .1),
                )
            })
            .map(|((a, b), (c, d))| a * b * c * d)
            .expect("no solution")
    }

    fn solve_part_2_impl(&self) -> i64 {
        for i in 0.. {
            let positions: BTreeSet<_> = self
                .robots
                .iter()
                .map(|robot| robot.position_after(i, WIDTH, HEIGHT))
                .collect();
            for row in 0..HEIGHT {
                let mut streak = 0;
                for col in 0..WIDTH {
                    if positions.contains(&(col, row)) {
                        streak += 1;
                    } else {
                        streak = 0;
                    }
                    if streak > 10 {
                        return i;
                    }
                }
            }
        }
        unreachable!()
    }
}

impl<'input> Solver<'input> for SolverImpl {
    fn new(input: &'input str) -> anyhow::Result<Self> {
        let robots = input
            .lines()
            .map(Robot::try_from)
            .collect::<Result<_, _>>()?;
        Ok(Self { robots })
    }

    fn solve_part_1(&self) -> anyhow::Result<Solution> {
        Ok(Solution::with_description(
            "Part 1",
            self.solve_part_1_for_size(WIDTH, HEIGHT).to_string(),
        ))
    }

    fn solve_part_2(&self) -> anyhow::Result<Solution> {
        Ok(Solution::with_description(
            "Part 2",
            self.solve_part_2_impl().to_string(),
        ))
    }
}

#[cfg(test)]
mod test {
    use super::SolverImpl;
    use crate::solvers::Solver;

    #[test]
    fn test_example_part_1() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day14-1.example"))?;
        assert_eq!(solver.solve_part_1_for_size(11, 7), 12);
        Ok(())
    }
}
