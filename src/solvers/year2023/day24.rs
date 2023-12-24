use anyhow::anyhow;

use crate::solvers::{Solution, Solver};

struct V3d(f64, f64, f64);

impl TryFrom<&str> for V3d {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut values = value.split(",").map(str::trim).map(str::parse::<f64>);
        let mut next_value = || values.next().ok_or_else(|| anyhow!("too few values"));
        Ok(V3d(next_value()??, next_value()??, next_value()??))
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
            .split_once("@")
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
        Ok(Solution::with_description(
            "Part 2",
            "not implemented".to_string(),
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
        assert_eq!(solver.solve_part_2()?.solution, "TODO");
        Ok(())
    }
}
