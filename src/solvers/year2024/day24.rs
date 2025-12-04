use crate::solvers::{Solution, Solver};
use anyhow::anyhow;
use regex::Regex;
use std::collections::{HashMap, HashSet};

struct SwapsIter<'a> {
    elements: &'a Vec<&'a String>,
    state: Vec<(usize, usize)>,
}

impl<'a> SwapsIter<'a> {
    fn new(elements: &'a Vec<&'a String>) -> Self {
        Self {
            elements,
            state: vec![(0, 1)],
        }
    }
}

impl<'a> Iterator for SwapsIter<'a> {
    type Item = Vec<(&'a String, &'a String)>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.state.len() > 4 || self.elements.len() < 2 {
            return None;
        }
        let result: Vec<_> = self
            .state
            .iter()
            .copied()
            .map(|(a, b)| (self.elements[a], self.elements[b]))
            .collect();
        self.increment();
        while !self.all_unique() && self.state.len() <= 4 {
            self.increment();
        }
        Some(result)
    }
}

impl SwapsIter<'_> {
    fn increment_digit(&mut self, i: usize) -> bool {
        if i == self.state.len() {
            self.state.push((0, 1));
        }
        self.state[i].1 += 1;
        if self.state[i].1 >= self.elements.len() {
            self.state[i].0 += 1;
            self.state[i].1 = self.state[0].0 + 1;
            if self.state[i].0 >= self.elements.len() - 1 {
                self.state[i] = (0, 1);
                return false;
            }
        }
        true
    }

    fn increment(&mut self) {
        let mut i = 0;
        while !self.increment_digit(i) {
            i += 1;
        }
    }

    fn all_unique(&self) -> bool {
        let mut set = HashSet::new();
        for (a, b) in &self.state {
            if !set.insert(a) || !set.insert(b) {
                return false;
            }
        }
        true
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Gate {
    And,
    Or,
    Xor,
}

impl Gate {
    pub fn evaluate(&self, a: bool, b: bool) -> bool {
        match self {
            Gate::And => a & b,
            Gate::Or => a | b,
            Gate::Xor => a ^ b,
        }
    }
}

#[derive(Clone, Debug)]
enum Signal {
    Value(bool),
    Deferred(Gate, String, String),
}

fn evaluate(
    circuit: &HashMap<String, Signal>,
    wire: &str,
    trace: &mut Option<HashSet<String>>,
) -> bool {
    match circuit.get(wire) {
        Some(Signal::Value(value)) => *value,
        Some(Signal::Deferred(gate, op0, op1)) => {
            if let Some(trace) = trace.as_mut() {
                trace.insert(op0.into());
                trace.insert(op1.into());
            };
            let a = evaluate(circuit, op0, trace);
            let b = evaluate(circuit, op1, trace);
            gate.evaluate(a, b)
        }
        _ => false,
    }
}

fn set_values(circuit: &mut HashMap<String, Signal>, register: char, mut value: u64) {
    for i in 0..64 {
        let wire = format!("{register}{i:02}");
        circuit.insert(wire, Signal::Value(value & 1 == 1));
        value >>= 1;
    }
}

fn check(
    circuit: &mut HashMap<String, Signal>,
    x: bool,
    y: bool,
    carry: bool,
    index: usize,
) -> bool {
    let carry_value = ((carry as u64) << index) >> 1;
    let x_value = ((x as u64) << index) | carry_value;
    let y_value = ((y as u64) << index) | carry_value;
    set_values(circuit, 'x', x_value);
    set_values(circuit, 'y', y_value);
    let result = evaluate(circuit, &format!("z{index:02}"), &mut None);
    let expected = ((x as u8) + (y as u8) + (carry as u8)) % 2 == 1;
    result == expected
}

fn check_all(circuit: &mut HashMap<String, Signal>, index: usize) -> bool {
    check(circuit, false, false, false, index)
        && check(circuit, false, true, false, index)
        && check(circuit, true, false, false, index)
        && check(circuit, true, true, false, index)
        && (index == 0
            || check(circuit, false, false, true, index)
                && check(circuit, false, true, true, index)
                && check(circuit, true, false, true, index)
                && check(circuit, true, true, true, index))
}

fn can_swap(circuit: &HashMap<String, Signal>, a_ref: &str, b_ref: &str) -> bool {
    let mut dependencies = Some(HashSet::new());
    evaluate(circuit, b_ref, &mut dependencies);
    let dependencies = dependencies.unwrap();
    if dependencies.contains(a_ref) {
        return false;
    }
    let mut dependencies = Some(HashSet::new());
    evaluate(circuit, a_ref, &mut dependencies);
    let dependencies = dependencies.unwrap();
    if dependencies.contains(b_ref) {
        return false;
    }
    true
}

fn swap(circuit: &mut HashMap<String, Signal>, a_ref: &str, b_ref: &str) {
    let a = circuit.get(a_ref).cloned().unwrap();
    let b = circuit.get(b_ref).cloned().unwrap();
    circuit.insert(b_ref.into(), a);
    circuit.insert(a_ref.into(), b);
}

pub struct SolverImpl {
    circuit: HashMap<String, Signal>,
}

impl<'input> Solver<'input> for SolverImpl {
    fn new(input: &'input str) -> anyhow::Result<Self> {
        let mut circuit = HashMap::new();
        let mut lines = input.lines();
        for value_wire in lines.by_ref().take_while(|line| !line.trim().is_empty()) {
            if let Some((name, value)) = value_wire.split_once(": ") {
                let value = value == "1";
                circuit.insert(name.into(), Signal::Value(value));
            }
        }
        let gate_pattern = Regex::new(r"^(.*) (AND|OR|XOR) (.*) -> (.*)$").unwrap();
        for gate_wire in lines {
            if let Some(captures) = gate_pattern.captures(gate_wire) {
                let gate = match &captures[2] {
                    "AND" => Ok(Gate::And),
                    "OR" => Ok(Gate::Or),
                    "XOR" => Ok(Gate::Xor),
                    _ => Err(anyhow!("Invalid gate")),
                }?;
                let signal = Signal::Deferred(gate, captures[1].into(), captures[3].into());
                circuit.insert(captures[4].into(), signal);
            }
        }

        Ok(Self { circuit })
    }

    fn solve_part_1(&self) -> anyhow::Result<Solution> {
        let mut acc: u64 = 0;
        for i in (0..64).rev() {
            let wire = format!("z{i:02}");
            acc <<= 1;
            if evaluate(&self.circuit, &wire, &mut None) {
                acc |= 1;
            }
        }

        Ok(Solution::with_description("Part 1", acc.to_string()))
    }

    fn solve_part_2(&self) -> anyhow::Result<Solution> {
        let mut circuit = self.circuit.clone();
        let nodes: Vec<_> = circuit
            .keys()
            .filter(|k| !k.starts_with("x") && !k.starts_with("y") && !k.starts_with("z"))
            .cloned()
            .collect();
        let mut correct_outputs: HashSet<String> = HashSet::with_capacity(circuit.len());
        let mut swapped: Vec<String> = Vec::with_capacity(8);
        for i in 0..45 {
            let mut trace = Some(HashSet::new());
            evaluate(&circuit, &format!("z{i:02}"), &mut trace);
            let mut trace = trace.unwrap();
            if check_all(&mut circuit, i) {
                correct_outputs.extend(trace);
            } else {
                for wire in &correct_outputs {
                    trace.remove(wire);
                }
                trace.insert(format!("z{i:02}"));

                let mut candidates: Vec<_> = nodes
                    .iter()
                    .filter(|&n| {
                        if n.starts_with("x") || n.starts_with("y") || n.starts_with("z") {
                            return false;
                        }
                        let mut dependencies = Some(HashSet::new());
                        evaluate(&circuit, n, &mut dependencies);
                        let dependencies = dependencies.unwrap();
                        if correct_outputs.contains(n) {
                            return false;
                        }
                        for j in i + 1..64 {
                            if dependencies.contains(&format!("x{j:02}"))
                                || dependencies.contains(&format!("y{j:02}"))
                            {
                                return false;
                            }
                        }
                        true
                    })
                    .collect();
                let z = format!("z{i:02}");
                candidates.push(&z);

                for swaps in SwapsIter::new(&candidates) {
                    if !swaps.iter().all(|s| can_swap(&circuit, s.0, s.1)) {
                        continue;
                    }
                    for (a, b) in &swaps {
                        swap(&mut circuit, a, b);
                    }

                    if check_all(&mut circuit, i) {
                        for (a, b) in swaps.iter() {
                            swapped.push(a.to_string());
                            swapped.push(b.to_string());
                        }
                        break;
                    }

                    for (a, b) in swaps {
                        swap(&mut circuit, a, b);
                    }
                }
            }
        }

        swapped.sort_unstable();

        Ok(Solution::with_description(
            "Part 2",
            swapped.join(",").to_string(),
        ))
    }
}

#[cfg(test)]
mod test {
    use super::SolverImpl;
    use crate::solvers::Solver;

    #[test]
    fn test_example_part_1a() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day24-1a.example"))?;
        assert_eq!(solver.solve_part_1()?.solution, "4");
        Ok(())
    }

    #[test]
    fn test_example_part_1() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day24-1b.example"))?;
        assert_eq!(solver.solve_part_1()?.solution, "2024");
        Ok(())
    }
}
