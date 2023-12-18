use crate::solvers::{Solution, Solver};
use anyhow::anyhow;
use regex::Regex;
use std::collections::HashSet;

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

struct DigInstruction<'a> {
    dir: Dir,
    count: usize,
    color: &'a str,
}

impl<'a> TryFrom<&'a str> for DigInstruction<'a> {
    type Error = anyhow::Error;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^(?P<dir>.)\s+(?P<count>\d+)\s+\((?P<color>#[0-9a-fA-F]{6})\)\s*$")
                    .unwrap();
        }
        let captures = RE.captures(value).ok_or(anyhow!("invalid instruction"))?;
        Ok(Self {
            dir: Dir::try_from(captures.name("dir").unwrap().as_str())?,
            count: captures.name("count").unwrap().as_str().parse()?,
            color: captures.name("color").unwrap().as_str(),
        })
    }
}

fn flood_fill(
    start: (isize, isize),
    dug_out: &HashSet<(isize, isize)>,
    min: (isize, isize),
    max: (isize, isize),
) -> Option<HashSet<(isize, isize)>> {
    let mut visited = HashSet::new();
    let mut queue = vec![start];
    while let Some(current) = queue.pop() {
        if !visited.insert(current) {
            continue;
        }
        if dug_out.contains(&current) {
            continue;
        }
        if current.0 < min.0 || current.0 > max.0 || current.1 < min.1 || current.1 > max.1 {
            return None;
        }

        let neighbors = [
            (current.0 - 1, current.1),
            (current.0 + 1, current.1),
            (current.0, current.1 - 1),
            (current.0, current.1 + 1),
        ];
        for neighbor in neighbors {
            queue.push(neighbor);
        }
    }

    Some(visited)
}

pub struct SolverImpl {
    num_dug_out: usize,
}

impl<'input> Solver<'input> for SolverImpl {
    fn new(input: &'input str) -> anyhow::Result<Self> {
        let instructions = input
            .lines()
            .map(DigInstruction::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        let mut current_pos = (0isize, 0isize);
        let mut dug_out = HashSet::new();
        dug_out.insert(current_pos);
        for instruction in instructions.iter() {
            for _ in 0..instruction.count {
                match instruction.dir {
                    Dir::Left => current_pos.0 -= 1,
                    Dir::Right => current_pos.0 += 1,
                    Dir::Up => current_pos.1 -= 1,
                    Dir::Down => current_pos.1 += 1,
                }
                dug_out.insert(current_pos);
            }
        }

        let min = (
            dug_out.iter().map(|x| x.0).min().unwrap(),
            dug_out.iter().map(|x| x.1).min().unwrap(),
        );
        let max = (
            dug_out.iter().map(|x| x.0).max().unwrap(),
            dug_out.iter().map(|x| x.1).max().unwrap(),
        );

        let mut current_pos = (0isize, 0isize);
        for instruction in instructions.iter() {
            for _ in 0..instruction.count {
                match instruction.dir {
                    Dir::Left => current_pos.0 -= 1,
                    Dir::Right => current_pos.0 += 1,
                    Dir::Up => current_pos.1 -= 1,
                    Dir::Down => current_pos.1 += 1,
                }

                let neighbors = [
                    (current_pos.0 - 1, current_pos.1),
                    (current_pos.0 + 1, current_pos.1),
                    (current_pos.0, current_pos.1 - 1),
                    (current_pos.0, current_pos.1 + 1),
                ];
                for neighbor in neighbors {
                    if let Some(visited) = flood_fill(neighbor, &dug_out, min, max) {
                        dug_out.extend(visited)
                    }
                }
            }
        }

        let num_dug_out = dug_out.len();

        Ok(Self { num_dug_out })
    }

    fn solve_part_1(&self) -> anyhow::Result<Solution> {
        Ok(Solution::with_description(
            "Capacity of the lagoon",
            self.num_dug_out.to_string(),
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
        let solver = SolverImpl::new(include_str!("./day18-1.example"))?;
        assert_eq!(solver.solve_part_1()?.solution, "62");
        Ok(())
    }

    #[test]
    fn test_example_part_2() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day18-1.example"))?;
        assert_eq!(solver.solve_part_2()?.solution, "TODO");
        Ok(())
    }
}
