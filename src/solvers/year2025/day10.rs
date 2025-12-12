use crate::solvers::{Solution, Solver};
use anyhow::anyhow;
use itertools::Itertools;
use std::collections::{BTreeSet, HashMap};

type Indicators = u16;

struct Machine {
    lights: Indicators,
    buttons: BTreeSet<Indicators>,
    joltages: Vec<Indicators>,
}

pub struct SolverImpl {
    machines: Vec<Machine>,
}

impl<'input> Solver<'input> for SolverImpl {
    fn new(input: &'input str) -> anyhow::Result<Self> {
        let machines = input
            .lines()
            .map(|line| {
                let mut items = line.split(' ');
                let lights = parse_lights(items.next().ok_or(anyhow!("no indicator lights"))?)?;

                let mut items = items.peekable();
                let mut buttons = BTreeSet::new();
                while items.peek().is_some_and(|item| item.starts_with('(')) {
                    let button_def = items.next().unwrap();
                    buttons.insert(parse_button(button_def)?);
                }

                let joltage_def = items.next().expect("no joltages");
                let joltages = joltage_def[1..joltage_def.len() - 1]
                    .split(',')
                    .map(|value| value.parse::<Indicators>())
                    .collect::<Result<Vec<_>, _>>()?;

                Ok(Machine {
                    lights,
                    buttons,
                    joltages,
                })
            })
            .collect::<anyhow::Result<Vec<_>>>()?;
        Ok(Self { machines })
    }

    fn solve_part_1(&self) -> anyhow::Result<Solution> {
        let result: usize = self
            .machines
            .iter()
            .map(|machine| {
                count_btn_presses(machine.lights, 0, &machine.buttons)
                    .expect("no solution for machine")
            })
            .sum();
        Ok(Solution::with_description("Part 1", result.to_string()))
    }

    fn solve_part_2(&self) -> anyhow::Result<Solution> {
        let result: usize = self
            .machines
            .iter()
            .map(|machine| {
                JoltageFinder::new()
                    .count_btn_presses_joltage(
                        &machine.joltages,
                        &JoltageFinder::parity_effects(&machine.buttons, machine.joltages.len()),
                    )
                    .expect("no solution for machine")
            })
            .sum();
        Ok(Solution::with_description("Part 2", result.to_string()))
    }
}

fn count_btn_presses(
    target: Indicators,
    current: Indicators,
    buttons: &BTreeSet<Indicators>,
) -> Option<usize> {
    if buttons.len() < 8 && target == current {
        return Some(0);
    }
    let mut next_buttons = buttons.clone();
    buttons
        .iter()
        .filter_map(|&button| {
            let next = current ^ button;
            next_buttons.remove(&button);
            count_btn_presses(target, next, &next_buttons).map(|x| x + 1)
        })
        .min()
}

struct JoltageFinder {
    cache: HashMap<Vec<Indicators>, Option<usize>>,
}

impl JoltageFinder {
    fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    fn parity_effects(
        buttons: &BTreeSet<Indicators>,
        n: usize,
    ) -> HashMap<Indicators, Vec<(Vec<u16>, usize)>> {
        let mut parity_effects = HashMap::new();
        for n_btns in 0..=buttons.len() {
            for btn_set in buttons.iter().combinations(n_btns) {
                let parity = btn_set.iter().fold(0, |acc, &&btn| acc ^ btn);
                let mut effect = vec![0; n];
                for btn in btn_set.iter() {
                    let mut btn = **btn;
                    let mut i = 0;
                    while btn != 0 {
                        effect[i] += btn & 1;
                        btn >>= 1;
                        i += 1;
                    }
                }
                parity_effects
                    .entry(parity)
                    .or_insert_with(Vec::new)
                    .push((effect, n_btns));
            }
        }
        parity_effects
    }

    fn count_btn_presses_joltage(
        &mut self,
        target_joltages: &Vec<Indicators>,
        parity_effects: &HashMap<Indicators, Vec<(Vec<u16>, usize)>>,
    ) -> Option<usize> {
        if target_joltages.iter().all(|&joltage| joltage == 0) {
            return Some(0);
        }

        if self.cache.contains_key(target_joltages) {
            return self.cache.get(target_joltages).cloned().flatten();
        }

        let target_parity: Indicators = target_joltages
            .iter()
            .rev()
            .fold(0, |acc, joltage| (acc << 1) | (joltage % 2));

        let effects = parity_effects.get(&target_parity);
        let result = effects.and_then(|effects| {
            effects
                .iter()
                .filter_map(|(effect, n_btns)| {
                    let mut adjusted_joltages = target_joltages.clone();
                    for i in 0..adjusted_joltages.len() {
                        if adjusted_joltages[i] < effect[i] {
                            return None;
                        }
                        adjusted_joltages[i] -= effect[i];
                    }
                    for joltage in &mut adjusted_joltages {
                        *joltage /= 2;
                    }
                    self.count_btn_presses_joltage(&adjusted_joltages, parity_effects)
                        .map(|x| x * 2 + n_btns)
                })
                .min()
        });
        self.cache.insert(target_joltages.clone(), result);
        result
    }
}

fn parse_lights(input: &str) -> anyhow::Result<Indicators> {
    Ok(input
        .chars()
        .rev()
        .map(|c| match c {
            '.' => Some(0),
            '#' => Some(1),
            _ => None,
        })
        .fold(0, |acc, bit| {
            if let Some(bit) = bit {
                return acc.checked_shl(1).expect("overflow") | bit;
            }
            acc
        }))
}

fn parse_button(input: &str) -> anyhow::Result<Indicators> {
    if !input.starts_with('(') || !input.ends_with(')') {
        return Err(anyhow!("invalid button format"));
    }
    let toggled_lights = input[1..input.len() - 1]
        .split(',')
        .map(|value| value.parse::<Indicators>())
        .collect::<Result<Vec<_>, _>>()?;
    Ok(toggled_lights
        .iter()
        .fold(0, |acc, &light| acc | (1 << light)))
}

#[cfg(test)]
mod test {
    use super::SolverImpl;
    use crate::solvers::Solver;

    #[test]
    fn test_example_part_1() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day10-1.example"))?;
        assert_eq!(solver.solve_part_1()?.solution, "7");
        Ok(())
    }

    #[test]
    fn test_example_part_2() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day10-1.example"))?;
        assert_eq!(solver.solve_part_2()?.solution, "33");
        Ok(())
    }
}
