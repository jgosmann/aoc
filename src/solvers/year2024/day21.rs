use crate::solvers::{Solution, Solver};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum DirectionalKeypadButton {
    Up,
    Down,
    Left,
    Right,
    Action,
}

impl From<&DirectionalKeypadButton> for u8 {
    fn from(value: &DirectionalKeypadButton) -> Self {
        match value {
            DirectionalKeypadButton::Up => b'^',
            DirectionalKeypadButton::Down => b'v',
            DirectionalKeypadButton::Left => b'<',
            DirectionalKeypadButton::Right => b'>',
            DirectionalKeypadButton::Action => b'A',
        }
    }
}

impl Display for DirectionalKeypadButton {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let symbol: u8 = self.into();
        write!(f, "{}", symbol as char)
    }
}

fn dir2code(directions: &[DirectionalKeypadButton]) -> Vec<u8> {
    directions.iter().map(|dir| dir.into()).collect()
}

trait Keypad {
    fn path_candidates(&self, start: u8, end: u8) -> Vec<Vec<DirectionalKeypadButton>>;
    fn button_position(&self, button: u8) -> (i8, i8);

    fn path_components(
        &self,
        start_pos: (i8, i8),
        end_pos: (i8, i8),
    ) -> (Vec<DirectionalKeypadButton>, Vec<DirectionalKeypadButton>) {
        let (drow, dcol) = (end_pos.0 - start_pos.0, end_pos.1 - start_pos.1);
        let vertical_component = vec![
            if drow > 0 {
                DirectionalKeypadButton::Down
            } else {
                DirectionalKeypadButton::Up
            };
            drow.unsigned_abs() as usize
        ];
        let horizontal_component = vec![
            if dcol > 0 {
                DirectionalKeypadButton::Right
            } else {
                DirectionalKeypadButton::Left
            };
            dcol.unsigned_abs() as usize
        ];
        (vertical_component, horizontal_component)
    }
}

struct NumericKeypad {}

impl Keypad for NumericKeypad {
    fn path_candidates(&self, start: u8, end: u8) -> Vec<Vec<DirectionalKeypadButton>> {
        let start_pos = self.button_position(start);
        let end_pos = self.button_position(end);
        let (vertical_component, horizontal_component) = self.path_components(start_pos, end_pos);
        let mut result = vec![];
        if vertical_component.is_empty() {
            result.push(horizontal_component);
        } else if horizontal_component.is_empty() {
            result.push(vertical_component);
        } else {
            if !(start_pos.1 == 0 && end_pos.0 == 3) {
                result.push(
                    vertical_component
                        .iter()
                        .chain(horizontal_component.iter())
                        .copied()
                        .collect(),
                );
            }
            if !(start_pos.0 == 3 && end_pos.1 == 0) {
                result.push(
                    horizontal_component
                        .iter()
                        .chain(vertical_component.iter())
                        .copied()
                        .collect(),
                );
            }
        }
        for path in result.iter_mut() {
            path.push(DirectionalKeypadButton::Action);
        }
        result
    }

    fn button_position(&self, button: u8) -> (i8, i8) {
        let row = match button {
            b'7'..=b'9' => 0,
            b'4'..=b'6' => 1,
            b'1'..=b'3' => 2,
            b'0' | b'A' => 3,
            _ => panic!("Invalid button"),
        };
        let col = match button {
            b'7' | b'4' | b'1' => 0,
            b'8' | b'5' | b'2' | b'0' => 1,
            b'9' | b'6' | b'3' | b'A' => 2,
            _ => panic!("Invalid button"),
        };
        (row, col)
    }
}

impl NumericKeypad {
    pub fn new() -> Self {
        Self {}
    }
}

struct DirectionalKeypad {}

impl Keypad for DirectionalKeypad {
    fn path_candidates(&self, start: u8, end: u8) -> Vec<Vec<DirectionalKeypadButton>> {
        let start_pos = self.button_position(start);
        let end_pos = self.button_position(end);
        let (vertical_component, horizontal_component) = self.path_components(start_pos, end_pos);
        let mut result = vec![];
        if vertical_component.is_empty() {
            result.push(horizontal_component);
        } else if horizontal_component.is_empty() {
            result.push(vertical_component);
        } else {
            if !(start_pos.1 == 0 && end_pos.0 == 0) {
                result.push(
                    vertical_component
                        .iter()
                        .chain(horizontal_component.iter())
                        .copied()
                        .collect(),
                );
            }
            if !(start_pos.0 == 0 && end_pos.1 == 0) {
                result.push(
                    horizontal_component
                        .iter()
                        .chain(vertical_component.iter())
                        .copied()
                        .collect(),
                );
            }
        }
        for path in result.iter_mut() {
            path.push(DirectionalKeypadButton::Action);
        }
        result
    }

    fn button_position(&self, button: u8) -> (i8, i8) {
        let row = match button {
            b'^' | b'A' => 0,
            b'<' | b'v' | b'>' => 1,
            _ => panic!("Invalid button"),
        };
        let col = match button {
            b'<' => 0,
            b'^' | b'v' => 1,
            b'A' | b'>' => 2,
            _ => panic!("Invalid button"),
        };
        (row, col)
    }
}

impl DirectionalKeypad {
    pub fn new() -> Self {
        Self {}
    }
}

pub struct SolverImpl<'input> {
    codes: Vec<&'input str>,
}

impl<'input> Solver<'input> for SolverImpl<'input> {
    fn new(input: &'input str) -> anyhow::Result<Self> {
        let codes = input
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .collect();
        Ok(Self { codes })
    }

    fn solve_part_1(&self) -> anyhow::Result<Solution> {
        Ok(Solution::with_description(
            "Part 1",
            self.solve(2).to_string(),
        ))
    }

    fn solve_part_2(&self) -> anyhow::Result<Solution> {
        Ok(Solution::with_description(
            "Part 2",
            self.solve(25).to_string(),
        ))
    }
}

impl SolverImpl<'_> {
    fn solve(&self, dir_keypad_stack_size: usize) -> usize {
        let mut min_path_length_memo = MemoizedMinPathLengthStackedDirKeypads::new();
        self.codes
            .iter()
            .map(|code| {
                let a = self.code_paths(code.as_bytes(), NumericKeypad::new());
                let b = a
                    .into_iter()
                    .map(|path| {
                        min_path_length_memo
                            .min_path_length_stacked_dir_keypads(path, dir_keypad_stack_size)
                    })
                    .min()
                    .unwrap_or_default();
                b * Self::numeric_code_part(code)
            })
            .sum()
    }

    fn numeric_code_part(code: &str) -> usize {
        code[..code.len() - 1]
            .parse()
            .expect("Invalid numeric code")
    }

    fn code_paths(&self, code: &[u8], keypad: impl Keypad) -> Vec<Vec<u8>> {
        let mut complete_paths = vec![vec![]];
        let mut start = b'A';
        for target in code.iter() {
            let paths = keypad.path_candidates(start, *target);
            complete_paths = complete_paths
                .iter()
                .flat_map(|prefix_path| {
                    paths.iter().map(|suffix_path| {
                        prefix_path
                            .iter()
                            .chain(suffix_path.iter())
                            .copied()
                            .collect()
                    })
                })
                .collect();
            start = *target;
        }
        complete_paths.iter().map(|path| dir2code(path)).collect()
    }
}

struct MemoizedMinPathLengthStackedDirKeypads {
    memo: HashMap<(Vec<u8>, usize), usize>,
}

impl MemoizedMinPathLengthStackedDirKeypads {
    pub fn new() -> Self {
        Self {
            memo: HashMap::new(),
        }
    }

    pub fn min_path_length_stacked_dir_keypads(
        &mut self,
        code: Vec<u8>,
        stack_height: usize,
    ) -> usize {
        if stack_height == 0 {
            return code.len();
        }

        if let Some(&result) = self.memo.get(&(code.clone(), stack_height)) {
            return result;
        }

        let keypad = DirectionalKeypad::new();

        let mut steps = 0;
        let mut start = b'A';
        for target in code.iter().copied() {
            steps += keypad
                .path_candidates(start, target)
                .iter()
                .map(|path| {
                    let path = dir2code(path);
                    self.min_path_length_stacked_dir_keypads(path, stack_height - 1)
                })
                .min()
                .unwrap_or_default();
            start = target;
        }
        self.memo.insert((code, stack_height), steps);
        steps
    }
}

#[cfg(test)]
mod test {
    use super::SolverImpl;
    use crate::solvers::Solver;

    #[test]
    fn test_example_part_1() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day21-1.example"))?;
        assert_eq!(solver.solve_part_1()?.solution, "126384");
        Ok(())
    }
}
