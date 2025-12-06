use crate::datastructures::grid::GridView;
use crate::solvers::{Solution, Solver};
use anyhow::anyhow;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Operator {
    Add,
    Multiply,
}

impl TryFrom<&str> for Operator {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "+" => Ok(Operator::Add),
            "*" => Ok(Operator::Multiply),
            _ => Err(anyhow!("Invalid operator: {}", value)),
        }
    }
}

impl TryFrom<u8> for Operator {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'+' => Ok(Operator::Add),
            b'*' => Ok(Operator::Multiply),
            _ => Err(anyhow!("Invalid operator: {}", value)),
        }
    }
}

pub struct SolverImpl<'input> {
    input: &'input str,
}

impl<'input> Solver<'input> for SolverImpl<'input> {
    fn new(input: &'input str) -> anyhow::Result<Self> {
        Ok(Self { input })
    }

    fn solve_part_1(&self) -> anyhow::Result<Solution> {
        let mut lines = self.input.lines().peekable();
        let mut operands: Vec<Vec<u64>> = Vec::new();
        while lines
            .peek()
            .map(|next_line| !next_line.contains(|c| c == '*' || c == '+'))
            .unwrap_or(false)
        {
            operands.push(
                lines
                    .next()
                    .unwrap()
                    .split(' ')
                    .filter(|line| !line.is_empty())
                    .map(str::parse)
                    .collect::<Result<Vec<_>, _>>()?,
            );
        }
        let operators = lines
            .next()
            .ok_or(anyhow!("No operator line found"))?
            .split(' ')
            .filter(|line| !line.is_empty())
            .map(Operator::try_from)
            .collect::<Result<Vec<_>, _>>()?;
        let result: u64 = operators
            .iter()
            .enumerate()
            .map(|(i, operator)| {
                let problem_operands = operands.iter().map(|op_list| op_list[i]);
                match operator {
                    Operator::Add => problem_operands.sum::<u64>(),
                    Operator::Multiply => problem_operands.product::<u64>(),
                }
            })
            .sum();
        Ok(Solution::with_description(
            "Grand total",
            result.to_string(),
        ))
    }

    fn solve_part_2(&self) -> anyhow::Result<Solution> {
        let grid = GridView::from_separated(b'\n', self.input.as_bytes());
        let mut operand_stack = Vec::with_capacity(4);
        let mut result: u64 = 0;
        for col in (0..grid.width()).rev() {
            let num = (0..grid.height() - 1)
                .map(|row| grid[(row, col)])
                .filter(|digit| digit.is_ascii_digit())
                .map(|digit| (digit - b'0') as u64)
                .fold(0, |acc, x| acc * 10 + x);
            if num > 0 {
                operand_stack.push(num);
            }
            if let Ok(operator) = Operator::try_from(grid[(grid.height() - 1, col)]) {
                result += match operator {
                    Operator::Add => operand_stack.iter().sum::<u64>(),
                    Operator::Multiply => operand_stack.iter().product::<u64>(),
                };
                operand_stack.clear();
            }
        }
        Ok(Solution::with_description(
            "Grand total, part 2",
            result.to_string(),
        ))
    }
}

#[cfg(test)]
mod test {
    use super::SolverImpl;
    use crate::solvers::Solver;

    #[test]
    fn test_example_part_1() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day6-1.example"))?;
        assert_eq!(solver.solve_part_1()?.solution, "4277556");
        Ok(())
    }

    #[test]
    fn test_example_part_2() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day6-1.example"))?;
        assert_eq!(solver.solve_part_2()?.solution, "3263827");
        Ok(())
    }
}
