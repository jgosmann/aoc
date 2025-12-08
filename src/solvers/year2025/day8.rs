use crate::solvers::{Solution, Solver};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::num::NonZeroUsize;

type Pos = (i64, i64, i64);

pub struct SolverImpl {
    junction_boxes: Vec<Pos>,
}

impl<'input> Solver<'input> for SolverImpl {
    fn new(input: &'input str) -> anyhow::Result<Self> {
        let junction_boxes = input
            .lines()
            .map(|line| {
                let coordinates = line
                    .split(',')
                    .map(|coordinate| coordinate.parse::<i64>())
                    .collect::<Result<Vec<_>, _>>()?;
                if coordinates.len() != 3 {
                    return Err(anyhow::anyhow!("invalid coordinate"));
                }
                Ok((coordinates[0], coordinates[1], coordinates[2]))
            })
            .collect::<Result<_, _>>()?;
        Ok(Self { junction_boxes })
    }

    fn solve_part_1(&self) -> anyhow::Result<Solution> {
        self.make_connections(1000)
    }

    fn solve_part_2(&self) -> anyhow::Result<Solution> {
        self.make_single_circuit()
    }
}

impl SolverImpl {
    pub fn make_connections(&self, n: usize) -> anyhow::Result<Solution> {
        let mut distance_heap = BinaryHeap::from_iter(
            self.junction_boxes
                .iter()
                .enumerate()
                .flat_map(|(i, pos_a)| {
                    self.junction_boxes[i + 1..]
                        .iter()
                        .enumerate()
                        .map(move |(j, pos_b)| (Reverse(dist_sq(pos_a, pos_b)), (i, j + i + 1)))
                }),
        );
        let mut group_assignments: Vec<Option<NonZeroUsize>> =
            vec![None; self.junction_boxes.len()];
        let mut next_group_id = NonZeroUsize::new(1).unwrap();

        for _ in 0..n {
            let (Reverse(_distance_sq), (idx_a, idx_b)) = distance_heap.pop().unwrap();
            let group_a = group_assignments[idx_a];
            let group_b = group_assignments[idx_b];
            match (group_a, group_b) {
                (None, None) => {
                    let group = Some(next_group_id);
                    group_assignments[idx_a] = group;
                    group_assignments[idx_b] = group;
                    next_group_id = next_group_id.checked_add(1).unwrap();
                }
                (Some(group_id), None) => {
                    group_assignments[idx_b] = Some(group_id);
                }
                (None, Some(group_id)) => {
                    group_assignments[idx_a] = Some(group_id);
                }
                (Some(group_a_id), Some(group_b_id)) => {
                    group_assignments
                        .iter_mut()
                        .filter(|assignment| **assignment == Some(group_b_id))
                        .for_each(|assignment| *assignment = Some(group_a_id));
                }
            }
        }

        let mut circuit_sizes = vec![0usize; next_group_id.get()];
        group_assignments.iter().for_each(|assignment| {
            if let Some(group_id) = assignment {
                circuit_sizes[group_id.get()] += 1;
            }
        });
        circuit_sizes.sort();
        let result: usize = circuit_sizes.iter().rev().take(3).product();

        Ok(Solution::with_description("Part 1", result.to_string()))
    }

    fn make_single_circuit(&self) -> anyhow::Result<Solution> {
        let mut distance_heap = BinaryHeap::from_iter(
            self.junction_boxes
                .iter()
                .enumerate()
                .flat_map(|(i, pos_a)| {
                    self.junction_boxes[i + 1..]
                        .iter()
                        .enumerate()
                        .map(move |(j, pos_b)| (Reverse(dist_sq(pos_a, pos_b)), (i, j + i + 1)))
                }),
        );
        let mut group_assignments: Vec<Option<NonZeroUsize>> =
            vec![None; self.junction_boxes.len()];
        let mut next_group_id = NonZeroUsize::new(1).unwrap();

        while !distance_heap.is_empty() {
            let (Reverse(_distance_sq), (idx_a, idx_b)) = distance_heap.pop().unwrap();
            let group_a = group_assignments[idx_a];
            let group_b = group_assignments[idx_b];
            match (group_a, group_b) {
                (None, None) => {
                    let group = Some(next_group_id);
                    group_assignments[idx_a] = group;
                    group_assignments[idx_b] = group;
                    next_group_id = next_group_id.checked_add(1).unwrap();
                }
                (Some(group_id), None) => {
                    group_assignments[idx_b] = Some(group_id);
                }
                (None, Some(group_id)) => {
                    group_assignments[idx_a] = Some(group_id);
                }
                (Some(group_a_id), Some(group_b_id)) => {
                    group_assignments
                        .iter_mut()
                        .filter(|assignment| **assignment == Some(group_b_id))
                        .for_each(|assignment| *assignment = Some(group_a_id));
                }
            }
            if group_assignments
                .iter()
                .all(|assignment| group_assignments[0] == *assignment)
            {
                let pos_a = self.junction_boxes[idx_a];
                let pos_b = self.junction_boxes[idx_b];
                let result = pos_a.0 * pos_b.0;
                return Ok(Solution::with_description("Part 2", result.to_string()));
            }
        }

        Err(anyhow::anyhow!("No solution found"))
    }
}

fn dist_sq(a: &Pos, b: &Pos) -> i64 {
    (a.0 - b.0).pow(2) + (a.1 - b.1).pow(2) + (a.2 - b.2).pow(2)
}

#[cfg(test)]
mod test {
    use super::SolverImpl;
    use crate::solvers::Solver;

    #[test]
    fn test_example_part_1() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day8-1.example"))?;
        assert_eq!(solver.make_connections(10)?.solution, "40");
        Ok(())
    }

    #[test]
    fn test_example_part_2() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day8-1.example"))?;
        assert_eq!(solver.solve_part_2()?.solution, "25272");
        Ok(())
    }
}
