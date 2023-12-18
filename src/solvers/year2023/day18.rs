use crate::solvers::{Solution, Solver};
use anyhow::anyhow;
use regex::Regex;
use std::collections::BTreeSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
    Left,
    Right,
    Up,
    Down,
}

impl TryFrom<&str> for Dir {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "L" => Ok(Self::Left),
            "R" => Ok(Self::Right),
            "U" => Ok(Self::Up),
            "D" => Ok(Self::Down),
            _ => Err(anyhow!("invalid direction")),
        }
    }
}

impl TryFrom<u8> for Dir {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'0' => Ok(Self::Right),
            b'1' => Ok(Self::Down),
            b'2' => Ok(Self::Left),
            b'3' => Ok(Self::Up),
            _ => Err(anyhow!("invalid direction")),
        }
    }
}

struct DigInstruction {
    dir: Dir,
    count: usize,
}

impl TryFrom<&str> for DigInstruction {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^(?P<dir>.)\s+(?P<count>\d+)\s+\((?P<color>#[0-9a-fA-F]{6})\)\s*$")
                    .unwrap();
        }
        let captures = RE.captures(value).ok_or(anyhow!("invalid instruction"))?;
        Ok(Self {
            dir: Dir::try_from(captures.name("dir").unwrap().as_str())?,
            count: captures.name("count").unwrap().as_str().parse()?,
        })
    }
}

impl DigInstruction {
    fn from_color(value: &str) -> anyhow::Result<Self> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^.\s+\d+\s+\(#(?P<count>[0-9a-fA-F]{5})(?P<dir>[0-3])\)\s*$").unwrap();
        }
        let captures = RE.captures(value).ok_or(anyhow!("invalid instruction"))?;
        let count = usize::from_str_radix(captures.name("count").unwrap().as_str(), 16)?;
        let dir = Dir::try_from(captures.name("dir").unwrap().as_str().as_bytes()[0])?;
        Ok(Self { dir, count })
    }
}

#[derive(Debug, Copy, Clone)]
struct Bound {
    row_range: (isize, isize),
    col: isize,
    dir: Dir,
}

fn dig_yourself_a_hole(instructions: &[DigInstruction]) -> usize {
    let mut vertical_bounds = Vec::new();
    let mut current_pos = (0isize, 0isize);
    for instruction in instructions.iter() {
        let next_pos = match instruction.dir {
            Dir::Up => (current_pos.0 - instruction.count as isize, current_pos.1),
            Dir::Down => (current_pos.0 + instruction.count as isize, current_pos.1),
            Dir::Left => (current_pos.0, current_pos.1 - instruction.count as isize),
            Dir::Right => (current_pos.0, current_pos.1 + instruction.count as isize),
        };
        if instruction.dir == Dir::Up || instruction.dir == Dir::Down {
            vertical_bounds.push(Bound {
                row_range: if current_pos.0 < next_pos.0 {
                    (current_pos.0, next_pos.0)
                } else {
                    (next_pos.0, current_pos.0)
                },
                col: current_pos.1,
                dir: instruction.dir,
            });
        }
        current_pos = next_pos;
    }

    let cuts = vertical_bounds
        .iter()
        .flat_map(|bounds| [bounds.row_range.0, bounds.row_range.1])
        .collect::<BTreeSet<_>>();
    let extended_cuts = cuts
        .iter()
        .copied()
        .flat_map(|c| [c - 1, c, c + 1])
        .collect::<BTreeSet<_>>();

    let mut dug_out = 0;
    let mut last_row = cuts.first().expect("some boundary required") - 1;
    let mut last_diff: usize = 0;
    for &row in extended_cuts.iter() {
        dug_out += (row - last_row) as usize * last_diff;
        last_row = row;
        last_diff = 0;

        let mut intersected_bounds = vertical_bounds
            .iter()
            .copied()
            .filter(|bounds| bounds.row_range.0 <= row && bounds.row_range.1 >= row)
            .collect::<Vec<_>>();
        intersected_bounds.sort_by(|a, b| a.col.cmp(&b.col));

        let mut last_corner: Option<Bound> = None;
        let mut last_bound: Option<Bound> = None;
        for &bound in intersected_bounds.iter() {
            if row != bound.row_range.0 && row != bound.row_range.1 {
                if let Some(from_bound) = last_bound {
                    // bounds inclusive
                    last_diff += (bound.col - from_bound.col + 1) as usize;
                    last_bound = None;
                } else {
                    last_bound = Some(bound);
                }
            } else {
                // special handling for corners
                if let Some(corner) = last_corner {
                    if corner.dir == bound.dir {
                        if let Some(from_bound) = last_bound {
                            last_diff += (bound.col - from_bound.col + 1) as usize;
                            last_bound = None;
                        } else {
                            last_bound = Some(corner);
                        }
                    } else if last_bound.is_none() {
                        last_diff += (bound.col - corner.col + 1) as usize;
                    }
                    last_corner = None;
                } else {
                    // hit odd corner
                    last_corner = Some(bound);
                }
            }
        }
    }

    dug_out
}

pub struct SolverImpl<'input> {
    input: &'input str,
}

impl<'input> Solver<'input> for SolverImpl<'input> {
    fn new(input: &'input str) -> anyhow::Result<Self> {
        Ok(Self { input })
    }

    fn solve_part_1(&self) -> anyhow::Result<Solution> {
        let instructions = self
            .input
            .lines()
            .map(DigInstruction::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Solution::with_description(
            "Capacity of the lagoon (part 1)",
            dig_yourself_a_hole(&instructions).to_string(),
        ))
    }

    fn solve_part_2(&self) -> anyhow::Result<Solution> {
        let instructions = self
            .input
            .lines()
            .map(DigInstruction::from_color)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Solution::with_description(
            "Capacity of the lagoon (part 2)",
            dig_yourself_a_hole(&instructions).to_string(),
        ))
    }
}

#[cfg(test)]
mod test {
    use super::SolverImpl;
    use crate::solvers::Solver;

    #[test]
    fn test_example_part_1() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day18-1.example"))?;
        assert_eq!(solver.solve_part_1()?.solution, "62");
        Ok(())
    }

    #[test]
    fn test_example_part_2() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day18-1.example"))?;
        assert_eq!(solver.solve_part_2()?.solution, "952408144115");
        Ok(())
    }
}
