use crate::solvers::{Solution, Solver};
use regex::Regex;

#[derive(Debug)]
pub struct SolverImpl<'a> {
    input: &'a str,
}

impl<'input> Solver<'input> for SolverImpl<'input> {
    fn new(input: &'input str) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Ok(Self { input })
    }

    fn solve_part_1(&self) -> anyhow::Result<Solution> {
        let solution: u32 = self
            .input
            .lines()
            .map(|line| {
                let first = line
                    .bytes()
                    .find(u8::is_ascii_digit)
                    .map(parse_digit_unchecked)
                    .unwrap_or(0);
                let last = line
                    .bytes()
                    .rfind(u8::is_ascii_digit)
                    .map(parse_digit_unchecked)
                    .unwrap_or(0);
                (10 * first + last) as u32
            })
            .sum();
        Ok(Solution::with_description(
            "Calibration sum (part 1)",
            solution.to_string(),
        ))
    }

    fn solve_part_2(&self) -> anyhow::Result<Solution> {
        lazy_static! {
            static ref DIGITS: Regex =
                Regex::new("[1-9]|one|two|three|four|five|six|seven|eight|nine").unwrap();
            static ref REVERSE_DIGITS: Regex =
                Regex::new("[1-9]|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin").unwrap();
        }

        let solution: u32 = self
            .input
            .lines()
            .map(|line| {
                let first = DIGITS
                    .find(line)
                    .map(|m| parse_spelled_digit(m.as_str()))
                    .unwrap_or(0);
                let last = REVERSE_DIGITS
                    .find(&line.chars().rev().collect::<String>())
                    .map(|m| parse_spelled_digit(m.as_str()))
                    .unwrap_or(0);
                (10 * first + last) as u32
            })
            .sum();

        Ok(Solution::with_description("Calibration sum (part 2)", solution.to_string()))
    }
}

fn parse_digit_unchecked(c: u8) -> u8 {
    c - b'0'
}

fn parse_spelled_digit(digit: &str) -> u8 {
    match digit {
        "1" | "one" | "eno" => 1,
        "2" | "two" | "owt" => 2,
        "3" | "three" | "eerht" => 3,
        "4" | "four" | "ruof" => 4,
        "5" | "five" | "evif" => 5,
        "6" | "six" | "xis" => 6,
        "7" | "seven" | "neves" => 7,
        "8" | "eight" | "thgie" => 8,
        "9" | "nine" | "enin" => 9,
        _ => panic!("not a digit"),
    }
}

#[cfg(test)]
mod test {
    use crate::solvers::Solver;

    use super::SolverImpl;

    #[test]
    fn test_example_part_1() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day1-1.example"))?;
        assert_eq!(solver.solve_part_1()?.solution, "142");
        Ok(())
    }

    #[test]
    fn test_exapmle_part_2() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day1-2.example"))?;
        assert_eq!(solver.solve_part_2()?.solution, "281");
        Ok(())
    }
}
