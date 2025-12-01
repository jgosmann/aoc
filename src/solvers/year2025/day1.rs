use crate::solvers::{Solution, Solver};
use anyhow::anyhow;

pub struct SolverImpl {
    instructions: Vec<i32>,
}

impl<'input> Solver<'input> for SolverImpl {
    fn new(input: &'input str) -> anyhow::Result<Self> {
        Ok(Self {
            instructions: input
                .lines()
                .filter(|line| !line.is_empty())
                .map(parse_line)
                .collect::<anyhow::Result<_>>()?,
        })
    }

    fn solve_part_1(&self) -> anyhow::Result<Solution> {
        let (_, password) = self
            .instructions
            .iter()
            .map(|value| {
                let mut value = *value;
                while value < 0 {
                    value += 100;
                }
                value
            })
            .fold((50, 0), |acc: (i32, i32), x: i32| {
                let (mut dial, mut zero_count) = acc;
                dial = (dial + x) % 100;
                if dial == 0 {
                    zero_count += 1;
                }
                (dial, zero_count)
            });
        Ok(Solution::with_description("Password", password.to_string()))
    }

    fn solve_part_2(&self) -> anyhow::Result<Solution> {
        let (dial, mut password) = self.instructions.iter().fold((50, 0), |acc, value| {
            let (mut dial, mut zero_count) = acc;
            if dial == 0 && *value < 0 {
                zero_count -= 1;
            }
            dial += value;
            while dial < 0 {
                dial += 100;
                zero_count += 1;
            }
            while dial > 99 {
                dial -= 100;
                zero_count += 1;
            }
            if dial == 0 && *value < 0 {
                zero_count += 1;
            }
            (dial, zero_count)
        });
        if dial == 0 {
            password += 1;
        }
        Ok(Solution::with_description(
            "Password with method 0x434C49434B",
            password.to_string(),
        ))
    }
}

fn parse_line(line: &str) -> anyhow::Result<i32> {
    let (direction, distance) = line.split_at(1);
    let sign = match direction {
        "L" => -1,
        "R" => 1,
        _ => Err(anyhow!("invalid direction: {}", direction))?,
    };
    let value: i32 = distance.parse()?;
    Ok(sign * value)
}

#[cfg(test)]
mod test {
    use super::SolverImpl;
    use crate::solvers::Solver;

    #[test]
    fn test_example_part_1() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day1-1.example"))?;
        assert_eq!(solver.solve_part_1()?.solution, "3");
        Ok(())
    }

    #[test]
    fn test_example_part_2() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day1-1.example"))?;
        assert_eq!(solver.solve_part_2()?.solution, "6");
        Ok(())
    }
}
