use crate::solvers::{Solution, Solver};
use anyhow::anyhow;

pub fn hash(input: &[u8]) -> u8 {
    input
        .iter()
        .fold(0u16, |acc, &b| (acc + b as u16) * 17 % 256) as u8
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Lens<'input> {
    label: &'input str,
    focal_length: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Operation {
    Remove,
    Install(u8),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Step<'input> {
    label: &'input str,
    operation: Operation,
}

impl<'input> TryFrom<&'input str> for Step<'input> {
    type Error = anyhow::Error;

    fn try_from(value: &'input str) -> Result<Self, Self::Error> {
        if let Some(label) = value.strip_suffix('-') {
            Ok(Self {
                label,
                operation: Operation::Remove,
            })
        } else {
            let (label, focal_length) = value
                .split_once('=')
                .ok_or_else(|| anyhow!("invalid step syntax"))?;
            let focal_length = focal_length.parse::<u8>()?;
            Ok(Self {
                label,
                operation: Operation::Install(focal_length),
            })
        }
    }
}

pub struct SolverImpl<'input> {
    input: &'input str,
}

impl<'input> Solver<'input> for SolverImpl<'input> {
    fn new(input: &'input str) -> anyhow::Result<Self> {
        Ok(Self { input })
    }

    fn solve_part_1(&self) -> anyhow::Result<Solution> {
        let hashsum = self
            .input
            .trim()
            .split(',')
            .map(|step| hash(step.as_bytes()) as u64)
            .sum::<u64>();
        Ok(Solution::with_description(
            "Sum of HASHes",
            hashsum.to_string(),
        ))
    }

    fn solve_part_2(&self) -> anyhow::Result<Solution> {
        const EMPTY_VEC: Vec<Lens> = Vec::new();
        let mut hashmap = [EMPTY_VEC; 256];

        for step in self.input.trim().split(',') {
            let step = Step::try_from(step)?;
            let key = hash(step.label.as_bytes()) as usize;
            let lensbox = &mut hashmap[key];
            let position = lensbox.iter().position(|lens| lens.label == step.label);
            match step.operation {
                Operation::Remove => {
                    if let Some(i) = position {
                        lensbox.remove(i);
                    }
                }
                Operation::Install(focal_length) => {
                    if let Some(i) = position {
                        lensbox[i].focal_length = focal_length;
                    } else {
                        lensbox.push(Lens {
                            label: step.label,
                            focal_length,
                        });
                    }
                }
            }
        }

        let focusing_power = hashmap
            .iter()
            .enumerate()
            .map(|(box_number, lensbox)| {
                lensbox
                    .iter()
                    .enumerate()
                    .map(|(i, lens)| lens.focal_length as usize * (box_number + 1) * (i + 1))
                    .sum::<usize>()
            })
            .sum::<usize>();

        Ok(Solution::with_description(
            "Part 2",
            focusing_power.to_string(),
        ))
    }
}

#[cfg(test)]
mod test {
    use super::SolverImpl;
    use crate::solvers::Solver;

    #[test]
    fn test_example_part_1() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day15-1.example"))?;
        assert_eq!(solver.solve_part_1()?.solution, "1320");
        Ok(())
    }

    #[test]
    fn test_example_part_2() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day15-1.example"))?;
        assert_eq!(solver.solve_part_2()?.solution, "145");
        Ok(())
    }
}
