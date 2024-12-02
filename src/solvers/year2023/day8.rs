use crate::solvers::{Solution, Solver};
use num;
use regex::Regex;
use std::collections::HashMap;

pub struct SolverImpl<'input> {
    instructions: &'input [u8],
    network: HashMap<&'input str, (&'input str, &'input str)>,
}

impl SolverImpl<'_> {
    fn solve<C>(&self, start_node: &str, target_cond: C) -> anyhow::Result<u64>
    where
        C: Fn(&str) -> bool,
    {
        let mut current_node = start_node;
        let mut current_instruction = self.instructions.iter().cycle();
        let mut n_steps = 0;
        while !target_cond(current_node) {
            let (left, right) = self
                .network
                .get(current_node)
                .ok_or_else(|| anyhow::Error::msg("referenced node must exist"))?;
            let instruction = current_instruction.next().unwrap();
            match instruction {
                b'L' => current_node = left,
                b'R' => current_node = right,
                _ => return Err(anyhow::Error::msg("invalid instruction")),
            }
            n_steps += 1;
        }
        Ok(n_steps)
    }
}

impl<'input> Solver<'input> for SolverImpl<'input> {
    fn new(input: &'input str) -> anyhow::Result<Self> {
        let mut lines = input.lines();
        let instructions = lines
            .next()
            .expect("instructions must be given")
            .trim()
            .as_bytes();
        let node_regex =
            Regex::new(r"^(?P<node>\w+)\s*=\s*\((?P<left>\w+),\s*(?P<right>\w+)\)$").unwrap();
        let network: HashMap<&str, (&str, &str)> = lines
            .filter_map(|line| {
                if let Some(captures) = node_regex.captures(line) {
                    let node = captures.name("node").unwrap().as_str();
                    let left = captures.name("left").unwrap().as_str();
                    let right = captures.name("right").unwrap().as_str();
                    Some((node, (left, right)))
                } else {
                    None
                }
            })
            .collect();
        Ok(Self {
            instructions,
            network,
        })
    }

    fn solve_part_1(&self) -> anyhow::Result<Solution> {
        let n_steps = self.solve("AAA", |node| node == "ZZZ")?;
        Ok(Solution::with_description(
            "Steps to reach ZZZ",
            n_steps.to_string(),
        ))
    }

    fn solve_part_2(&self) -> anyhow::Result<Solution> {
        // Note this is not a general solution, but based on the assumption that for each
        // start node we will reach a target node every n steps with n being constant for the
        // n being constant for a start node.
        let start_nodes = self
            .network
            .keys()
            .copied()
            .filter(|node| node.ends_with('A'));
        let steps_per_start_node = start_nodes
            .map(|node| self.solve(node, |node| node.ends_with('Z')))
            .collect::<anyhow::Result<Vec<_>>>()?;
        let n_steps = steps_per_start_node
            .into_iter()
            .reduce(num::integer::lcm)
            .expect("at least one start node must exist");
        Ok(Solution::with_description(
            "Steps to be only on nodes ending with Z",
            n_steps.to_string(),
        ))
    }
}

#[cfg(test)]
mod test {
    use super::SolverImpl;
    use crate::solvers::Solver;

    #[test]
    fn test_example_part_1a() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day8-1a.example"))?;
        assert_eq!(solver.solve_part_1()?.solution, "2");
        Ok(())
    }

    #[test]
    fn test_example_part_1b() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day8-1b.example"))?;
        assert_eq!(solver.solve_part_1()?.solution, "6");
        Ok(())
    }

    #[test]
    fn test_example_part_2() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day8-2.example"))?;
        assert_eq!(solver.solve_part_2()?.solution, "6");
        Ok(())
    }
}
