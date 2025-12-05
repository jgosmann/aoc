pub mod year2023 {
    pub mod day1;
    pub mod day10;
    pub mod day11;
    pub mod day12;
    pub mod day13;
    pub mod day14;
    pub mod day15;
    pub mod day16;
    pub mod day17;
    pub mod day18;
    pub mod day19;
    pub mod day2;
    pub mod day20;
    pub mod day21;
    pub mod day22;
    pub mod day23;
    pub mod day24;
    pub mod day25;
    pub mod day3;
    pub mod day4;
    pub mod day5;
    pub mod day6;
    pub mod day7;
    pub mod day8;
    pub mod day9;
}
pub mod year2024 {
    pub mod day1;
    pub mod day10;
    pub mod day11;
    pub mod day12;
    pub mod day13;
    pub mod day14;
    pub mod day15;
    pub mod day16;
    pub mod day17;
    pub mod day18;
    pub mod day19;
    pub mod day2;
    pub mod day20;
    pub mod day21;
    pub mod day22;
    pub mod day23;
    pub mod day24;
    pub mod day25;
    pub mod day3;
    pub mod day4;
    pub mod day5;
    pub mod day6;
    pub mod day7;
    pub mod day8;
    pub mod day9;
}

pub mod year2025 {
    pub mod day1;
    pub mod day2;
    pub mod day3;
    pub mod day4;
    pub mod day5;
    // <<INSERT MARKER>>
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