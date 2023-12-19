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

#[derive(Debug, Clone)]
struct MachinePartRange {
    x: (u64, u64),
    m: (u64, u64),
    a: (u64, u64),
    s: (u64, u64),
}

impl From<&MachinePart> for MachinePartRange {
    fn from(value: &MachinePart) -> Self {
        Self {
            x: (value.x, value.x + 1),
            m: (value.m, value.m + 1),
            a: (value.a, value.a + 1),
            s: (value.s, value.s + 1),
        }
    }
}

impl MachinePartRange {
    fn num_combinations(&self) -> u64 {
        (self.x.1 - self.x.0)
            * (self.m.1 - self.m.0)
            * (self.a.1 - self.a.0)
            * (self.s.1 - self.s.0)
    }

    fn is_valid(&self) -> bool {
        self.x.0 < self.x.1 && self.m.0 < self.m.1 && self.a.0 < self.a.1 && self.s.0 < self.s.1
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

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
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

struct SplitConditionRange {
    truthy: (u64, u64),
    falsy: (u64, u64),
}

impl Comparison {
    fn compare(&self, lhs: u64, rhs: u64) -> bool {
        match self {
            Self::Lower => lhs < rhs,
            Self::Greater => lhs > rhs,
        }
    }

    fn split_range(&self, range: (u64, u64), at: u64) -> SplitConditionRange {
        match self {
            Self::Lower if range.1 < at => SplitConditionRange {
                truthy: range,
                falsy: (0, 0),
            },
            Self::Lower if at <= range.0 => SplitConditionRange {
                truthy: (0, 0),
                falsy: range,
            },
            Self::Greater if range.0 > at => SplitConditionRange {
                truthy: range,
                falsy: (0, 0),
            },
            Self::Greater if at >= range.1 => SplitConditionRange {
                truthy: (0, 0),
                falsy: range,
            },
            Self::Lower => SplitConditionRange {
                truthy: (range.0, at),
                falsy: (at, range.1),
            },
            Self::Greater => SplitConditionRange {
                truthy: (at + 1, range.1),
                falsy: (range.0, at + 1),
            },
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

    fn evaluate_range(
        &self,
        part: &MachinePartRange,
    ) -> (MachinePartRange, Operation<'a>, MachinePartRange) {
        let split_var = match self.0.var {
            Category::ExtremelyCoolLooking => part.x,
            Category::Musical => part.m,
            Category::Aerodynamic => part.a,
            Category::Shiny => part.s,
        };
        let split_range = self.0.comparison.split_range(split_var, self.0.threshold);
        let mut truthy = part.clone();
        let mut falsy = part.clone();
        match self.0.var {
            Category::ExtremelyCoolLooking => {
                truthy.x = split_range.truthy;
                falsy.x = split_range.falsy;
            }
            Category::Musical => {
                truthy.m = split_range.truthy;
                falsy.m = split_range.falsy;
            }
            Category::Aerodynamic => {
                truthy.a = split_range.truthy;
                falsy.a = split_range.falsy;
            }
            Category::Shiny => {
                truthy.s = split_range.truthy;
                falsy.s = split_range.falsy;
            }
        }
        (truthy, self.1, falsy)
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

    fn evaluate_range(&self, part: &MachinePartRange) -> Vec<(MachinePartRange, Operation<'a>)> {
        let mut result = vec![];
        let default_range = self.rules.iter().fold(part.clone(), |current_part, rule| {
            let (truthy, op, falsy) = rule.evaluate_range(&current_part);
            if op != Operation::Reject && truthy.is_valid() {
                result.push((truthy, op));
            }
            falsy
        });
        result.push((default_range, self.default));
        result
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

pub struct SolverImpl<'input> {
    workflows: HashMap<&'input str, Workflow<'input>>,
    parts: Vec<MachinePart>,
}

impl<'input> Solver<'input> for SolverImpl<'input> {
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
        Ok(Self { workflows, parts })
    }

    fn solve_part_1(&self) -> anyhow::Result<Solution> {
        let accepted_rating: u64 = self
            .parts
            .iter()
            .map(|part| {
                let mut current_workflow = "in";
                loop {
                    let operation = self.workflows.get(current_workflow).unwrap().evaluate(part);
                    match operation {
                        Operation::Accept => return part.rating(),
                        Operation::Reject => return 0,
                        Operation::JumpToLabel(label) => current_workflow = label,
                    }
                }
            })
            .sum();
        Ok(Solution::with_description(
            "Part 1",
            accepted_rating.to_string(),
        ))
    }

    fn solve_part_2(&self) -> anyhow::Result<Solution> {
        let mut ranges = vec![(
            MachinePartRange {
                x: (1, 4001),
                m: (1, 4001),
                a: (1, 4001),
                s: (1, 4001),
            },
            Operation::JumpToLabel("in"),
        )];
        while ranges.iter().any(|&(_, op)| op != Operation::Accept) {
            ranges = ranges
                .into_iter()
                .flat_map(|(range, op)| {
                    if !range.is_valid() {
                        return vec![];
                    }

                    match op {
                        Operation::Accept => vec![(range, op)],
                        Operation::Reject => vec![],
                        Operation::JumpToLabel(label) => {
                            let workflow = self.workflows.get(label).unwrap();
                            workflow.evaluate_range(&range)
                        }
                    }
                })
                .filter(|(range, _)| range.is_valid())
                .collect::<Vec<_>>();
        }
        let num_combinations: u64 = ranges
            .iter()
            .map(|(range, _)| range)
            .map(|range| range.num_combinations())
            .sum();
        Ok(Solution::with_description(
            "Part 2",
            num_combinations.to_string(),
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
        assert_eq!(solver.solve_part_2()?.solution, "167409079868000");
        Ok(())
    }
}
