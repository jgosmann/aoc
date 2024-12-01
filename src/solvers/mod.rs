pub mod year2023 {
    automod::dir!(pub "src/solvers/year2023");
}
pub mod year2024 {
    automod::dir!(pub "src/solvers/year2024");
}

use ansi_term::Style;
use std::fmt::Display;

pub trait Solver<'input> {
    fn new(input: &'input str) -> anyhow::Result<Self>
    where
        Self: Sized;
    fn solve_part_1(&self) -> anyhow::Result<Solution>;
    fn solve_part_2(&self) -> anyhow::Result<Solution>;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Solution {
    description: &'static str,
    solution: String,
}

impl Solution {
    pub fn with_description(description: &'static str, solution: String) -> Self {
        Self {
            description,
            solution,
        }
    }
}

impl Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}: {}",
            self.description,
            Style::new().bold().paint(&self.solution)
        ))
    }
}
