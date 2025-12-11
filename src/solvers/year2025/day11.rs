use crate::solvers::{Solution, Solver};
use anyhow::anyhow;
use std::collections::HashMap;

pub struct SolverImpl<'input> {
    outputs: HashMap<&'input str, Vec<&'input str>>,
}

impl<'input> Solver<'input> for SolverImpl<'input> {
    fn new(input: &'input str) -> anyhow::Result<Self> {
        let outputs = input
            .lines()
            .filter_map(|line| {
                let mut parts = line.split(' ');
                let key = parts.next();
                key.map(|key| {
                    let key = key.strip_suffix(':').ok_or(anyhow!("invalid key"))?;
                    let values: Vec<&'input str> = parts.collect();
                    Ok((key, values))
                })
            })
            .collect::<anyhow::Result<_>>()?;
        Ok(Self { outputs })
    }

    fn solve_part_1(&self) -> anyhow::Result<Solution> {
        let n_paths = PathCounter::new(&self.outputs, "out").count_paths("you");
        Ok(Solution::with_description(
            "Paths to `out`",
            n_paths.to_string(),
        ))
    }

    fn solve_part_2(&self) -> anyhow::Result<Solution> {
        let mut pc = PathCounter::new(&self.outputs, "out");
        let n_dac_to_out = pc.count_paths("dac");
        let n_fft_to_out = pc.count_paths("fft");
        let n_dac_to_fft = PathCounter::new(&self.outputs, "fft").count_paths("dac");
        let n_fft_to_dac = PathCounter::new(&self.outputs, "dac").count_paths("fft");
        let n_svr_to_dac = PathCounter::new(&self.outputs, "dac").count_paths("svr");
        let n_svr_to_fft = PathCounter::new(&self.outputs, "fft").count_paths("svr");
        let n_paths =
            n_svr_to_dac * n_dac_to_fft * n_fft_to_out + n_svr_to_fft * n_fft_to_dac * n_dac_to_out;
        Ok(Solution::with_description(
            "Paths with `fft` and `dac`",
            n_paths.to_string(),
        ))
    }
}

struct PathCounter<'a> {
    graph: &'a HashMap<&'a str, Vec<&'a str>>,
    path_count: HashMap<&'a str, usize>,
    target: &'a str,
}

impl<'a> PathCounter<'a> {
    pub fn new(graph: &'a HashMap<&'a str, Vec<&'a str>>, target: &'a str) -> Self {
        Self {
            graph,
            path_count: HashMap::new(),
            target,
        }
    }
}

impl<'a> PathCounter<'a> {
    pub fn count_paths(&mut self, node: &'a str) -> usize {
        if node == self.target {
            return 1;
        }
        if let Some(&count) = self.path_count.get(node) {
            return count;
        }
        let count = if let Some(neighbors) = self.graph.get(node) {
            neighbors
                .iter()
                .map(|&neighbor| self.count_paths(neighbor))
                .sum()
        } else {
            0
        };
        self.path_count.insert(node, count);
        count
    }
}

#[cfg(test)]
mod test {
    use super::SolverImpl;
    use crate::solvers::Solver;

    #[test]
    fn test_example_part_1() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day11-1.example"))?;
        assert_eq!(solver.solve_part_1()?.solution, "5");
        Ok(())
    }

    #[test]
    fn test_example_part_2() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day11-2.example"))?;
        assert_eq!(solver.solve_part_2()?.solution, "2");
        Ok(())
    }
}
