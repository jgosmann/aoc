use crate::solvers::{Solution, Solver};
use anyhow::anyhow;
use std::collections::HashMap;

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

struct ArrangementCounter<'input> {
    input: &'input [u8],
    groups: &'input [usize],
    cache: HashMap<(usize, State), usize>,
}

impl<'input> ArrangementCounter<'input> {
    pub fn count(input: &'input [u8], groups: &'input [usize]) -> usize {
        Self {
            input,
            groups,
            cache: HashMap::new(),
        }
        .process()
    }

    fn process(&mut self) -> usize {
        self.step(0, State::default())
    }

    fn step(&mut self, idx: usize, state: State) -> usize {
        if idx >= self.input.len() {
            if state.is_terminating(self.groups) {
                1
            } else {
                0
            }
        } else if let Some(&result) = self.cache.get(&(idx, state)) {
            result
        } else {
            let next_states = state.next(self.input[idx], self.groups);
            let result = next_states
                .into_iter()
                .map(|next_state| self.step(idx + 1, next_state))
                .sum();
            self.cache.insert((idx, state), result);
            result
        }
    }
}

struct ParsedLine<'input> {
    springs: &'input str,
    groups: Vec<usize>,
}

pub struct SolverImpl<'input> {
    lines: Vec<ParsedLine<'input>>,
}

impl<'input> Solver<'input> for SolverImpl<'input> {
    fn new(input: &'input str) -> anyhow::Result<Self> {
        let lines = input
            .lines()
            .map(|line| {
                let (springs, group_def) = line
                    .split_once(' ')
                    .ok_or_else(|| anyhow!("invalid input line"))?;
                let groups = group_def
                    .split(',')
                    .map(|group| group.parse::<usize>())
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(ParsedLine { springs, groups })
            })
            .collect::<anyhow::Result<Vec<_>>>()?;
        Ok(Self { lines })
    }

    fn solve_part_1(&self) -> anyhow::Result<Solution> {
        let num_arrangements: usize = self
            .lines
            .iter()
            .map(|line| ArrangementCounter::count(line.springs.as_bytes(), &line.groups))
            .sum();
        Ok(Solution::with_description(
            "Possible arrangements sum (part 1)",
            num_arrangements.to_string(),
        ))
    }

    fn solve_part_2(&self) -> anyhow::Result<Solution> {
        let num_arrangements: usize = self
            .lines
            .iter()
            .map(|line| {
                let springs = format!("{}?", line.springs).repeat(5);
                let springs = springs.as_bytes();
                let springs = &springs[0..springs.len() - 1];
                let groups = line.groups.repeat(5);
                ArrangementCounter::count(springs, &groups)
            })
            .sum();
        Ok(Solution::with_description(
            "Possible arrangements sum (part 2)",
            num_arrangements.to_string(),
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
        assert_eq!(solver.solve_part_2()?.solution, "525152");
        Ok(())
    }
}
