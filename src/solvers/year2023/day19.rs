use crate::solvers::{Solution, Solver};
use anyhow::anyhow;
use regex::Regex;
use std::collections::HashMap;

enum Category {
    ExtremelyCoolLooking,
    Musical,
    Aerodynamic,
    Shiny,
}

impl TryFrom<u8> for Category {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'x' => Ok(Self::ExtremelyCoolLooking),
            b'm' => Ok(Self::Musical),
            b'a' => Ok(Self::Aerodynamic),
            b's' => Ok(Self::Shiny),
            _ => Err(anyhow!("Invalid category")),
        }
    }
}

struct MachinePart {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

impl MachinePart {
    fn rating(&self) -> u64 {
        self.x + self.m + self.a + self.s
    }
}

impl TryFrom<&str> for MachinePart {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"\{x=(?P<x>\d+),m=(?P<m>\d+),a=(?P<a>\d+),s=(?P<s>\d+)\}").unwrap();
        }

        let caps = RE.captures(value).ok_or(anyhow!("Invalid machine part"))?;
        let x = caps
            .name("x")
            .ok_or(anyhow!("expected x"))?
            .as_str()
            .parse::<u64>()?;
        let m = caps
            .name("m")
            .ok_or(anyhow!("expected m"))?
            .as_str()
            .parse::<u64>()?;
        let a = caps
            .name("a")
            .ok_or(anyhow!("expected a"))?
            .as_str()
            .parse::<u64>()?;
        let s = caps
            .name("s")
            .ok_or(anyhow!("expected s"))?
            .as_str()
            .parse::<u64>()?;
        Ok(Self { x, m, a, s })
    }
}

#[derive(Debug, Copy, Clone)]
enum Operation<'a> {
    Accept,
    Reject,
    JumpToLabel(&'a str),
}

impl<'a> TryFrom<&'a str> for Operation<'a> {
    type Error = anyhow::Error;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        match value {
            "A" => Ok(Self::Accept),
            "R" => Ok(Self::Reject),
            _ => Ok(Self::JumpToLabel(value)),
        }
    }
}

enum Comparison {
    Lower,
    Greater,
}

impl Comparison {
    fn compare(&self, lhs: u64, rhs: u64) -> bool {
        match self {
            Self::Lower => lhs < rhs,
            Self::Greater => lhs > rhs,
        }
    }
}

struct Condition {
    var: Category,
    threshold: u64,
    comparison: Comparison,
}

impl Condition {
    fn evaluate(&self, part: &MachinePart) -> bool {
        let lhs = match self.var {
            Category::ExtremelyCoolLooking => part.x,
            Category::Musical => part.m,
            Category::Aerodynamic => part.a,
            Category::Shiny => part.s,
        };
        self.comparison.compare(lhs, self.threshold)
    }
}

impl TryFrom<&str> for Condition {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^(?P<var>[xmas])(?P<cmp>[<>])(?P<threshold>.*)").unwrap();
        }

        let caps = RE.captures(value).ok_or(anyhow!("Invalid condition"))?;
        let var = Category::try_from(
            caps.name("var")
                .ok_or(anyhow!("expected var"))?
                .as_str()
                .as_bytes()[0],
        )?;
        let comparison = match caps
            .name("cmp")
            .ok_or(anyhow!("expected operation"))?
            .as_str()
        {
            "<" => Comparison::Lower,
            ">" => Comparison::Greater,
            _ => return Err(anyhow!("Invalid operation")),
        };
        let threshold = caps
            .name("threshold")
            .ok_or(anyhow!("expected threshold"))?
            .as_str()
            .parse::<u64>()?;
        Ok(Self {
            var,
            threshold,
            comparison,
        })
    }
}

struct Rule<'a>(Condition, Operation<'a>);

impl<'a> Rule<'a> {
    fn evaluate(&self, part: &MachinePart) -> Option<Operation<'a>> {
        if self.0.evaluate(part) {
            Some(self.1)
        } else {
            None
        }
    }
}

impl<'a> TryFrom<&'a str> for Rule<'a> {
    type Error = anyhow::Error;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let (condition, operation) = value.split_once(':').ok_or(anyhow!("Invalid rule"))?;
        Ok(Self(
            Condition::try_from(condition)?,
            Operation::try_from(operation)?,
        ))
    }
}

struct Workflow<'a> {
    rules: Vec<Rule<'a>>,
    default: Operation<'a>,
}

impl<'a> Workflow<'a> {
    fn evaluate(&self, part: &MachinePart) -> Operation<'a> {
        self.rules
            .iter()
            .find_map(|rule| rule.evaluate(part))
            .unwrap_or(self.default)
    }
}

impl<'a> TryFrom<&'a str> for Workflow<'a> {
    type Error = anyhow::Error;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let rule_declarations = value.split(',').collect::<Vec<_>>();

        let rules = rule_declarations[0..rule_declarations.len() - 1]
            .iter()
            .copied()
            .map(Rule::try_from)
            .collect::<Result<Vec<_>, _>>()?;
        let default = Operation::try_from(
            rule_declarations
                .last()
                .copied()
                .ok_or(anyhow!("Invalid workflow"))?,
        )?;
        Ok(Self { rules, default })
    }
}

pub struct SolverImpl {
    accepted_rating: u64,
}

impl<'input> Solver<'input> for SolverImpl {
    fn new(input: &'input str) -> anyhow::Result<Self> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(?P<label>\w+)\{(?P<rules>.*)\}").unwrap();
        }

        let mut lines = input.lines();
        let workflows = lines
            .by_ref()
            .take_while(|line| !line.trim().is_empty())
            .map(|line| {
                let caps = RE
                    .captures(line)
                    .ok_or(anyhow!("Invalid workflow declaration"))?;
                let label = caps
                    .name("label")
                    .ok_or(anyhow!("expected label"))?
                    .as_str();
                let rules = caps
                    .name("rules")
                    .ok_or(anyhow!("expected rules"))?
                    .as_str();
                Ok((label, Workflow::try_from(rules)?))
            })
            .collect::<anyhow::Result<HashMap<_, _>>>()?;
        let parts = lines
            .map(MachinePart::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        let accepted_rating = parts
            .iter()
            .map(|part| {
                let mut current_workflow = "in";
                loop {
                    let operation = workflows.get(current_workflow).unwrap().evaluate(part);
                    match operation {
                        Operation::Accept => return part.rating(),
                        Operation::Reject => return 0,
                        Operation::JumpToLabel(label) => current_workflow = label,
                    }
                }
            })
            .sum();
        Ok(Self { accepted_rating })
    }

    fn solve_part_1(&self) -> anyhow::Result<Solution> {
        Ok(Solution::with_description(
            "Part 1",
            self.accepted_rating.to_string(),
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
        let solver = SolverImpl::new(include_str!("./day19-1.example"))?;
        assert_eq!(solver.solve_part_1()?.solution, "19114");
        Ok(())
    }

    #[test]
    fn test_example_part_2() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day19-1.example"))?;
        assert_eq!(solver.solve_part_2()?.solution, "TODO");
        Ok(())
    }
}
