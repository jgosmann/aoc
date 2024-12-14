use crate::solvers::{Solution, Solver};
use anyhow::anyhow;
use regex::Regex;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Prize {
    x: i64,
    y: i64,
}

impl TryFrom<&str> for Prize {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let pattern = Regex::new(r"^Prize: X=(\d+), Y=(\d+)$")?;
        let captures = pattern.captures(value).ok_or(anyhow!("Invalid input"))?;
        Ok(Self {
            x: captures[1].parse()?,
            y: captures[2].parse()?,
        })
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Button {
    dx: i64,
    dy: i64,
}

impl TryFrom<&str> for Button {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let pattern = Regex::new(r"^Button [AB]: X\+(\d+), Y\+(\d+)$")?;
        let captures = pattern.captures(value).ok_or(anyhow!("Invalid input"))?;
        Ok(Self {
            dx: captures[1].parse()?,
            dy: captures[2].parse()?,
        })
    }
}

#[derive(Debug, Clone)]
struct ClawMachine {
    buttons: [Button; 2],
    prize: Prize,
}

impl ClawMachine {
    fn fewest_tokens_to_win(&self) -> Option<u64> {
        let Prize { x, y } = self.prize;
        let Button { dx: xa, dy: ya } = self.buttons[0];
        let Button { dx: xb, dy: yb } = self.buttons[1];
        let b_denominator = xb * ya - xa * yb;
        if b_denominator == 0 {
            unimplemented!("not needed for the specific input")
        }
        let b_numerator = x * ya - xa * y;
        if b_numerator % b_denominator != 0 {
            return None;
        }
        let b = b_numerator / b_denominator;
        let a_denominator = ya;
        let a_numerator = y - b * yb;
        if a_numerator % a_denominator != 0 {
            return None;
        }
        let a = a_numerator / a_denominator;
        if a < 0 || b < 0 {
            return None;
        }
        Some((3 * a + b) as u64)
    }
}

pub struct SolverImpl {
    claw_machines: Vec<ClawMachine>,
}

impl<'input> Solver<'input> for SolverImpl {
    fn new(input: &'input str) -> anyhow::Result<Self> {
        let mut input = input.lines().filter(|line| !line.trim().is_empty());
        let mut claw_machines = Vec::new();
        while let Some(line) = input.next() {
            let button_a = Button::try_from(line)?;
            let button_b =
                Button::try_from(input.next().expect("partial claw machine definition"))?;
            let prize = Prize::try_from(input.next().expect("partial claw machine definition"))?;
            claw_machines.push(ClawMachine {
                buttons: [button_a, button_b],
                prize,
            })
        }
        Ok(Self { claw_machines })
    }

    fn solve_part_1(&self) -> anyhow::Result<Solution> {
        let result: u64 = self
            .claw_machines
            .iter()
            .filter_map(ClawMachine::fewest_tokens_to_win)
            .sum();
        Ok(Solution::with_description("Part 1", result.to_string()))
    }

    fn solve_part_2(&self) -> anyhow::Result<Solution> {
        const CONVERSION: i64 = 10_000_000_000_000;
        let result: u64 = self
            .claw_machines
            .iter()
            .map(|cm| ClawMachine {
                buttons: cm.buttons,
                prize: Prize {
                    x: cm.prize.x + CONVERSION,
                    y: cm.prize.y + CONVERSION,
                },
            })
            .filter_map(|cm| cm.fewest_tokens_to_win())
            .sum();
        Ok(Solution::with_description("Part 2", result.to_string()))
    }
}

#[cfg(test)]
mod test {
    use super::SolverImpl;
    use crate::solvers::Solver;

    #[test]
    fn test_example_part_1() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day13-1.example"))?;
        assert_eq!(solver.solve_part_1()?.solution, "480");
        Ok(())
    }
}
