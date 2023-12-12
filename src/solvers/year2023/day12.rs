use crate::solvers::{Solution, Solver};
use anyhow::anyhow;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum StateClass {
    Start,
    OutsideGroup,
    InsideGroup,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    class: StateClass,
    group_idx: usize,
    group_size: usize,
}

impl State {
    pub fn next(&self, input: u8, groups: &[usize]) -> Vec<Self> {
        match (self.class, input) {
            (StateClass::Start, b'.') => {
                vec![Self {
                    class: StateClass::OutsideGroup,
                    ..*self
                }]
            }
            (StateClass::Start, b'#') => {
                vec![Self {
                    class: StateClass::InsideGroup,
                    group_idx: self.group_idx,
                    group_size: 1,
                }]
            }
            (StateClass::Start, b'?') => {
                vec![
                    Self {
                        class: StateClass::OutsideGroup,
                        ..*self
                    },
                    Self {
                        class: StateClass::InsideGroup,
                        group_idx: self.group_idx,
                        group_size: 1,
                    },
                ]
            }
            (StateClass::OutsideGroup, b'.') => {
                vec![*self]
            }
            (StateClass::OutsideGroup, b'#') => {
                vec![Self {
                    class: StateClass::InsideGroup,
                    group_idx: self.group_idx,
                    group_size: 1,
                }]
            }
            (StateClass::OutsideGroup, b'?') => {
                vec![
                    *self,
                    Self {
                        class: StateClass::InsideGroup,
                        group_idx: self.group_idx,
                        group_size: 1,
                    },
                ]
            }
            (StateClass::InsideGroup, b'.') => {
                if self.group_idx < groups.len() && self.group_size == groups[self.group_idx] {
                    vec![Self {
                        class: StateClass::OutsideGroup,
                        group_idx: self.group_idx + 1,
                        group_size: 0,
                    }]
                } else {
                    vec![]
                }
            }
            (StateClass::InsideGroup, b'#') => {
                if self.group_idx < groups.len() && self.group_size < groups[self.group_idx] {
                    vec![Self {
                        class: StateClass::InsideGroup,
                        group_idx: self.group_idx,
                        group_size: self.group_size + 1,
                    }]
                } else {
                    vec![]
                }
            }
            (StateClass::InsideGroup, b'?') => {
                let mut next_states = Vec::with_capacity(2);
                if self.group_idx < groups.len() {
                    if self.group_size == groups[self.group_idx] {
                        next_states.push(Self {
                            class: StateClass::OutsideGroup,
                            group_idx: self.group_idx + 1,
                            group_size: 0,
                        });
                    }
                    if self.group_size < groups[self.group_idx] {
                        next_states.push(Self {
                            class: StateClass::InsideGroup,
                            group_idx: self.group_idx,
                            group_size: self.group_size + 1,
                        });
                    }
                }
                next_states
            }
            _ => vec![],
        }
    }

    pub fn is_terminating(&self, groups: &[usize]) -> bool {
        self.group_idx == groups.len() - 1 && self.group_size == groups[self.group_idx]
            || self.group_idx == groups.len() && self.group_size == 0
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            class: StateClass::Start,
            group_idx: 0,
            group_size: 0,
        }
    }
}

fn process(input: &[u8], groups: &[usize]) -> usize {
    let mut states = vec![State::default()];
    for &byte in input {
        states = states
            .into_iter()
            .flat_map(|state| state.next(byte, groups))
            .collect();
    }
    states
        .into_iter()
        .filter(|state| state.is_terminating(groups))
        .count()
}

pub struct SolverImpl {
    num_arrangements: usize,
}

impl<'input> Solver<'input> for SolverImpl {
    fn new(input: &'input str) -> anyhow::Result<Self> {
        let num_arrangements = input
            .lines()
            .map(|line| {
                let (springs, group_def) = line
                    .split_once(' ')
                    .ok_or_else(|| anyhow!("invalid input line"))?;
                let groups = group_def
                    .split(',')
                    .map(|group| group.parse::<usize>())
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(process(springs.as_bytes(), &groups))
            })
            .collect::<anyhow::Result<Vec<usize>>>()?
            .iter()
            .sum();
        Ok(Self { num_arrangements })
    }

    fn solve_part_1(&self) -> anyhow::Result<Solution> {
        Ok(Solution::with_description(
            "Part 1",
            self.num_arrangements.to_string(),
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
        let solver = SolverImpl::new(include_str!("./day12-1.example"))?;
        assert_eq!(solver.solve_part_1()?.solution, "21");
        Ok(())
    }

    #[test]
    fn test_example_part_2() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day12-1.example"))?;
        assert_eq!(solver.solve_part_2()?.solution, "TODO");
        Ok(())
    }
}
