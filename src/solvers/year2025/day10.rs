use crate::solvers::{Solution, Solver};
use anyhow::anyhow;
use std::collections::BTreeSet;

type Indicators = u16;

struct Machine {
    lights: Indicators,
    buttons: BTreeSet<Indicators>,
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
                    buttons.insert(parse_button(items.next().unwrap())?);
                }

                Ok(Machine { lights, buttons })
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
        Ok(Solution::with_description(
            "Part 2",
            "not implemented".to_string(),
        ))
    }
}

fn count_btn_presses(
    target: Indicators,
    current: Indicators,
    buttons: &BTreeSet<Indicators>,
) -> Option<usize> {
    if target == current {
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
            return acc;
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
