use std::collections::{BTreeSet, HashMap, HashSet};

use anyhow::anyhow;
use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};

use crate::solvers::{Solution, Solver};

pub struct SolverImpl {
    solution: usize,
}

fn determine_canidates_with_clustering_heuristic<'a>(
    graph: &HashMap<&'a str, BTreeSet<&'a str>>,
) -> Vec<(&'a str, &'a str)> {
    let mut groups = [HashSet::<&str>::new(), HashSet::<&str>::new()];
    {
        let keys_iter = &mut graph.keys();
        groups[0].extend(keys_iter.take(graph.len() / 2));
        groups[1].extend(keys_iter);
    }

    let mut threshold = 1;
    let max_edges = graph.values().map(BTreeSet::len).max().unwrap_or_default();
    loop {
        let mut settled = true;
        for (n, edges) in graph.iter() {
            if groups[0].contains(n) {
                if edges.len() - edges.iter().filter(|&e| groups[1].contains(e)).count()
                    < threshold.min(edges.len() - 1)
                    && groups[0].len() > groups[1].len()
                {
                    groups[0].remove(n);
                    groups[1].insert(n);
                    settled = false;
                }
            } else if groups[1].contains(n)
                && edges.len() - edges.iter().filter(|&e| groups[0].contains(e)).count()
                    < threshold.min(edges.len() - 1)
                && groups[0].len() < groups[1].len()
            {
                groups[1].remove(n);
                groups[0].insert(n);
                settled = false;
            }
        }
        if settled {
            if threshold > max_edges {
                break;
            }
            threshold += 1;
        }
    }

    groups[0]
        .iter()
        .filter(|&n| graph.get(n).unwrap().iter().any(|e| groups[1].contains(e)))
        .map(|&n| {
            (
                n,
                graph
                    .get(n)
                    .unwrap()
                    .iter()
                    .copied()
                    .find(|&e| groups[1].contains(e))
                    .unwrap(),
            )
        })
        .collect()
}

impl<'input> Solver<'input> for SolverImpl {
    fn new(input: &'input str) -> anyhow::Result<Self> {
        let mut graph: HashMap<_, _> = input
            .lines()
            .map(|line| {
                let (src, destinations) =
                    line.split_once(':').ok_or(anyhow!("invalid input line"))?;
                let destinations: BTreeSet<_> = destinations.trim().split(' ').collect();
                Ok((src, destinations))
            })
            .collect::<anyhow::Result<HashMap<_, _>>>()?;
        for (src, destinations) in graph.clone().iter() {
            for dst in destinations.iter() {
                let entry = graph.entry(dst).or_insert_with(BTreeSet::new);
                entry.insert(src);
            }
        }

        let candidates = if graph.len() > 100 {
            determine_canidates_with_clustering_heuristic(&graph)
        } else {
            graph
                .iter()
                .flat_map(|(&n, edges)| edges.iter().map(move |&e| (n, e)))
                .collect()
        };

        let solution = candidates
            .par_iter()
            .enumerate()
            .find_map_any(|(i, &a)| {
                candidates
                    .iter()
                    .enumerate()
                    .skip(i + 1)
                    .find_map(|(j, &b)| {
                        candidates.iter().skip(j + 1).find_map(|&c| {
                            let removals = [a, b, c];

                            let mut visited = HashSet::new();
                            let mut queue =
                                graph.keys().next().map(|x| vec![x]).unwrap_or_default();
                            let empty_set = BTreeSet::new();
                            while let Some(node) = queue.pop() {
                                if !visited.contains(node) {
                                    queue.extend(
                                        graph.get(node).unwrap_or(&empty_set).iter().filter(|&e| {
                                            !removals.contains(&(node, e))
                                                && !removals.contains(&(e, node))
                                        }),
                                    );
                                }
                                visited.insert(node);
                            }

                            if visited.len() != graph.len() {
                                Some((visited.len(), graph.len() - visited.len()))
                            } else {
                                None
                            }
                        })
                    })
            })
            .unwrap_or_default();

        let solution = solution.0 * solution.1;

        Ok(Self { solution })
    }

    fn solve_part_1(&self) -> anyhow::Result<Solution> {
        Ok(Solution::with_description(
            "Group size product",
            self.solution.to_string(),
        ))
    }

    fn solve_part_2(&self) -> anyhow::Result<Solution> {
        Ok(Solution::with_description("Part 2", "n/a".to_string()))
    }
}

#[cfg(test)]
mod test {
    use super::SolverImpl;
    use crate::solvers::Solver;

    #[test]
    fn test_example_part_1() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day25-1.example"))?;
        assert_eq!(solver.solve_part_1()?.solution, "54");
        Ok(())
    }
}
