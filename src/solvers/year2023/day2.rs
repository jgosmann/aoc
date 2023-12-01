use anyhow::anyhow;
use regex::Regex;

use crate::solvers::{MaybeSolution, Solution, Solver};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
struct Reveal {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

#[derive(Debug, Clone)]
struct Game {
    pub id: u32,
    pub reveals: Vec<Reveal>,
}

#[derive(Debug)]
pub struct SolverImpl {
    part1: u32,
    part2: u32,
}

impl Reveal {
    fn parse(expr: &str) -> anyhow::Result<Self> {
        let mut reveal = Reveal::default();
        for color_count_expr in expr.split(',').map(|e| e.trim()) {
            if let Some((count, color)) = color_count_expr.split_once(' ') {
                let count: u32 = count.parse()?;
                match color {
                    "red" => reveal.red += count,
                    "green" => reveal.green += count,
                    "blue" => reveal.blue += count,
                    _ => Err(anyhow!("invalid color {}", color))?,
                }
            } else {
                return Err(anyhow!("expected expression of the form '<count> <color>'"));
            }
        }
        Ok(reveal)
    }

    fn is_possible_to_draw_from(&self, other: &Self) -> bool {
        self.red <= other.red && self.blue <= other.blue && self.green <= other.green
    }

    fn max_over_colors(&self, other: &Self) -> Self {
        Self {
            red: self.red.max(other.red),
            green: self.green.max(other.green),
            blue: self.blue.max(other.blue),
        }
    }

    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

impl Game {
    fn parse(expr: &str) -> anyhow::Result<Self> {
        lazy_static! {
            static ref GAME_PATTERN: Regex = Regex::new(r"^Game\s+(\d+):(.*)\s*$").unwrap();
        }

        if let Some(captures) = GAME_PATTERN.captures(expr) {
            // safety: capture group 1 and 2 must exist if pattern matches
            Ok(Self {
                id: captures.get(1).unwrap().as_str().parse()?,
                reveals: captures
                    .get(2)
                    .unwrap()
                    .as_str()
                    .split(';')
                    .map(Reveal::parse)
                    .collect::<anyhow::Result<Vec<Reveal>>>()?,
            })
        } else {
            Err(anyhow!("invalid game syntax: '{}'", expr))
        }
    }
}

impl<'input> Solver<'input> for SolverImpl {
    fn new(input: &'input str) -> anyhow::Result<Self> {
        static REFERENCE_BAG: Reveal = Reveal {
            red: 12,
            green: 13,
            blue: 14,
        };

        let mut part1 = 0;
        let mut part2 = 0;

        for line in input.lines() {
            let game = Game::parse(line)?;
            if game
                .reveals
                .iter()
                .all(|reveal| reveal.is_possible_to_draw_from(&REFERENCE_BAG))
            {
                part1 += game.id;
            }
            part2 += game
                .reveals
                .iter()
                .fold(Reveal::default(), |a, b| a.max_over_colors(b))
                .power();
        }

        Ok(Self { part1, part2 })
    }

    fn solve_part_1(&self) -> anyhow::Result<Solution> {
        Ok(Solution::with_description(
            "Sum of IDs of possible games",
            self.part1.to_string(),
        ))
    }

    fn solve_part_2(&self) -> anyhow::Result<MaybeSolution> {
        Ok(MaybeSolution::Present(Solution::with_description(
            "Sum of the power",
            self.part2.to_string(),
        )))
    }
}

#[cfg(test)]
mod test {
    use crate::solvers::Solver;

    use super::SolverImpl;

    #[test]
    fn test_example_part_1() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day2-1.example"))?;
        assert_eq!(solver.solve_part_1()?.solution, "8");
        Ok(())
    }

    #[test]
    fn test_example_part_2() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day2-1.example"))?;
        assert_eq!(solver.solve_part_2()?.unwrap().solution, "2286");
        Ok(())
    }
}
