use std::ops::{Add, Mul, Sub};

use anyhow::anyhow;
use nalgebra::{Matrix6, Matrix6x1};

use crate::solvers::{Solution, Solver};

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
struct V3d(f64, f64, f64);

impl TryFrom<&str> for V3d {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut values = value.split(',').map(str::trim).map(str::parse::<f64>);
        let mut next_value = || values.next().ok_or_else(|| anyhow!("too few values"));
        Ok(V3d(next_value()??, next_value()??, next_value()??))
    }
}

impl Add for V3d {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Sub for V3d {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl Mul<f64> for V3d {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

struct Hailstone {
    position: V3d,
    velocity: V3d,
}

impl TryFrom<&str> for Hailstone {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (position, velocity) = value
            .split_once('@')
            .ok_or_else(|| anyhow!("require position and velocity"))?;
        Ok(Hailstone {
            position: V3d::try_from(position)?,
            velocity: V3d::try_from(velocity)?,
        })
    }
}

impl Hailstone {
    pub fn intersect_2d(&self, other: &Self) -> Option<(f64, f64)> {
        let c_self = -self.velocity.1 * self.position.0 + self.position.1 * self.velocity.0;
        let c_other = -other.velocity.1 * other.position.0 + other.position.1 * other.velocity.0;
        let c_intersect = self.velocity.1 * -other.velocity.0 - other.velocity.1 * -self.velocity.0;
        if c_intersect == 0. {
            None
        } else {
            let x = -self.velocity.0 * c_other - -other.velocity.0 * c_self;
            let y = other.velocity.1 * c_self - self.velocity.1 * c_other;
            Some((x / c_intersect, y / c_intersect))
        }
    }

    pub fn is_forward_2d(&self, point: (f64, f64)) -> bool {
        let dir = (
            (point.0 - self.position.0).signum(),
            (point.1 - self.position.1).signum(),
        );
        dir == (self.velocity.0.signum(), self.velocity.1.signum())
    }
}

pub struct SolverImpl {
    hailstones: Vec<Hailstone>,
}

impl SolverImpl {
    pub fn count_intersections_2d(&self, bounds: (f64, f64)) -> usize {
        let mut num_intersects = 0;
        for (i, a) in self.hailstones.iter().enumerate() {
            for b in self.hailstones[i + 1..].iter() {
                if let Some(intersection) = a.intersect_2d(b) {
                    if a.is_forward_2d(intersection)
                        && b.is_forward_2d(intersection)
                        && bounds.0 <= intersection.0
                        && intersection.0 <= bounds.1
                        && bounds.0 <= intersection.1
                        && intersection.1 <= bounds.1
                    {
                        num_intersects += 1;
                    }
                }
            }
        }
        num_intersects
    }
}

impl<'input> Solver<'input> for SolverImpl {
    fn new(input: &'input str) -> anyhow::Result<Self> {
        let hailstones = input
            .lines()
            .map(Hailstone::try_from)
            .collect::<anyhow::Result<Vec<_>>>()?;
        Ok(Self { hailstones })
    }

    fn solve_part_1(&self) -> anyhow::Result<Solution> {
        Ok(Solution::with_description(
            "Intersections",
            self.count_intersections_2d((200000000000000., 400000000000000.))
                .to_string(),
        ))
    }

    fn solve_part_2(&self) -> anyhow::Result<Solution> {
        let a = &self.hailstones[0];
        let b = &self.hailstones[1];
        let c = &self.hailstones[2];

        let mat = Matrix6::new(
            0.,
            b.velocity.2 - a.velocity.2,
            a.velocity.1 - b.velocity.1,
            0.,
            a.position.2 - b.position.2,
            b.position.1 - a.position.1,
            a.velocity.2 - b.velocity.2,
            0.,
            b.velocity.0 - a.velocity.0,
            b.position.2 - a.position.2,
            0.,
            a.position.0 - b.position.0,
            b.velocity.1 - a.velocity.1,
            a.velocity.0 - b.velocity.0,
            0.,
            a.position.1 - b.position.1,
            b.position.0 - a.position.0,
            0.,
            0.,
            c.velocity.2 - a.velocity.2,
            a.velocity.1 - c.velocity.1,
            0.,
            a.position.2 - c.position.2,
            c.position.1 - a.position.1,
            a.velocity.2 - c.velocity.2,
            0.,
            c.velocity.0 - a.velocity.0,
            c.position.2 - a.position.2,
            0.,
            a.position.0 - c.position.0,
            c.velocity.1 - a.velocity.1,
            a.velocity.0 - c.velocity.0,
            0.,
            a.position.1 - c.position.1,
            c.position.0 - a.position.0,
            0.,
        );
        let inv = mat
            .try_inverse()
            .ok_or_else(|| anyhow!("cannot solve equation system"))?;
        let solved = inv
            * Matrix6x1::new(
                -a.position.1 * a.velocity.2
                    + b.position.1 * b.velocity.2
                    + a.position.2 * a.velocity.1
                    - b.position.2 * b.velocity.1,
                -a.position.2 * a.velocity.0
                    + b.position.2 * b.velocity.0
                    + a.position.0 * a.velocity.2
                    - b.position.0 * b.velocity.2,
                -a.position.0 * a.velocity.1
                    + b.position.0 * b.velocity.1
                    + a.position.1 * a.velocity.0
                    - b.position.1 * b.velocity.0,
                -a.position.1 * a.velocity.2
                    + c.position.1 * c.velocity.2
                    + a.position.2 * a.velocity.1
                    - c.position.2 * c.velocity.1,
                -a.position.2 * a.velocity.0
                    + c.position.2 * c.velocity.0
                    + a.position.0 * a.velocity.2
                    - c.position.0 * c.velocity.2,
                -a.position.0 * a.velocity.1
                    + c.position.0 * c.velocity.1
                    + a.position.1 * a.velocity.0
                    - c.position.1 * c.velocity.0,
            );
        let solution = solved.iter().copied().take(3).map(f64::round).sum::<f64>() as i64;

        Ok(Solution::with_description(
            "Sum of initial coordinates",
            solution.to_string(),
        ))
    }
}

#[cfg(test)]
mod test {
    use super::SolverImpl;
    use crate::solvers::Solver;

    #[test]
    fn test_example_part_1() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day24-1.example"))?;
        assert_eq!(solver.count_intersections_2d((7., 27.)), 2);
        Ok(())
    }

    #[test]
    fn test_example_part_2() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day24-1.example"))?;
        assert_eq!(solver.solve_part_2()?.solution, "47");
        Ok(())
    }
}
