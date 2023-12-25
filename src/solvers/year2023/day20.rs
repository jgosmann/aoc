use std::{
    collections::{BTreeMap, VecDeque},
    convert::identity,
    fmt::Debug,
};

use anyhow::anyhow;

use crate::solvers::{Solution, Solver};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pulse {
    Low,
    High,
}

impl From<Pulse> for bool {
    fn from(value: Pulse) -> Self {
        match value {
            Pulse::Low => false,
            Pulse::High => true,
        }
    }
}

impl From<bool> for Pulse {
    fn from(value: bool) -> Self {
        if value {
            Pulse::High
        } else {
            Pulse::Low
        }
    }
}

struct InputPulse<'a> {
    name: &'a str,
    pulse: Pulse,
}

trait Module: Debug {
    fn feed_pulse(&mut self, input: InputPulse) -> Option<Pulse>;
}

#[derive(Debug, Clone)]
struct Broadcaster;

impl Module for Broadcaster {
    fn feed_pulse(&mut self, input: InputPulse) -> Option<Pulse> {
        Some(input.pulse)
    }
}

#[derive(Debug, Clone, Default)]
struct FlipFlop {
    is_on: bool,
}

impl Module for FlipFlop {
    fn feed_pulse(&mut self, input: InputPulse) -> Option<Pulse> {
        match input.pulse {
            Pulse::Low => {
                self.is_on = !self.is_on;
                Some(self.is_on.into())
            }
            Pulse::High => None,
        }
    }
}

#[derive(Debug, Clone)]
struct Conjunction<'a> {
    input_states: BTreeMap<&'a str, bool>,
}

impl<'a> Conjunction<'a> {
    pub fn new(inputs: &[&'a str]) -> Self {
        Self {
            input_states: inputs.iter().map(|&name| (name, false)).collect(),
        }
    }
}

impl<'a> Module for Conjunction<'a> {
    fn feed_pulse(&mut self, input: InputPulse) -> Option<Pulse> {
        *self
            .input_states
            .get_mut(input.name)
            .ok_or_else(|| anyhow!("undeclared input {}", input.name))
            .unwrap() = input.pulse.into();
        Some((!self.input_states.values().copied().all(identity)).into())
    }
}

pub struct SolverImpl<'input> {
    wiring: BTreeMap<&'input str, (&'input str, Vec<&'input str>)>,
}

type InstantiatedWiring<'solver, 'input> =
    BTreeMap<&'input str, (Box<dyn Module + 'input>, &'solver Vec<&'input str>)>;

impl<'input> SolverImpl<'input> {
    fn instantiate_modules(&self) -> anyhow::Result<InstantiatedWiring<'_, 'input>> {
        self.wiring
            .iter()
            .map(|(&source, (module_type, destinations))| {
                let module_type: Box<dyn Module> = match *module_type {
                    "%" => Box::<FlipFlop>::default(),
                    "&" => Box::new(Conjunction::new(
                        &self
                            .wiring
                            .iter()
                            .filter(|(_, (_, destinations))| destinations.contains(&source))
                            .map(|(&name, _)| name)
                            .collect::<Vec<_>>(),
                    )),
                    "broadcaster" => Box::new(Broadcaster),
                    _ => anyhow::bail!("unknown module type {}", module_type),
                };
                Ok((source, (module_type, destinations)))
            })
            .collect()
    }
}

impl<'input> Solver<'input> for SolverImpl<'input> {
    fn new(input: &'input str) -> anyhow::Result<Self> {
        let wiring: BTreeMap<_, _> = input
            .lines()
            .filter_map(|line| {
                line.split_once(" -> ").map(|(source, destinations)| {
                    let destinations: Vec<_> = destinations.split(',').map(str::trim).collect();
                    let module_type = &source[0..1];
                    let (source_name, module_type) = match module_type {
                        "%" | "&" => (&source[1..], module_type),
                        _ => (source, source),
                    };
                    (source_name, (module_type, destinations))
                })
            })
            .collect();

        Ok(Self { wiring })
    }

    fn solve_part_1(&self) -> anyhow::Result<Solution> {
        let mut wiring = self.instantiate_modules()?;
        let mut num_low_pulses = 0;
        let mut num_high_pulses = 0;
        for _ in 0..1000 {
            let mut unprocessed = VecDeque::from([("button", Pulse::Low, "broadcaster")]);
            while let Some((src_name, pulse, dst_name)) = unprocessed.pop_front() {
                match pulse {
                    Pulse::Low => num_low_pulses += 1,
                    Pulse::High => num_high_pulses += 1,
                }
                if let Some((module, destinations)) = wiring.get_mut(&dst_name) {
                    if let Some(output) = module.feed_pulse(InputPulse {
                        name: src_name,
                        pulse,
                    }) {
                        for destination in destinations.iter() {
                            unprocessed.push_back((dst_name, output, destination));
                        }
                    }
                }
            }
        }

        let result = num_low_pulses * num_high_pulses;
        Ok(Solution::with_description("Part 1", result.to_string()))
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
    fn test_example_part_1a() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day20-1a.example"))?;
        assert_eq!(solver.solve_part_1()?.solution, "32000000");
        Ok(())
    }

    #[test]
    fn test_example_part_1b() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day20-1b.example"))?;
        assert_eq!(solver.solve_part_1()?.solution, "11687500");
        Ok(())
    }
}
