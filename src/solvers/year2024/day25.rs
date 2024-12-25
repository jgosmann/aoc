use crate::solvers::{Solution, Solver};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Type {
    Lock,
    Key,
}

impl From<Type> for usize {
    fn from(value: Type) -> Self {
        match value {
            Type::Lock => 0,
            Type::Key => 1,
        }
    }
}

pub struct SolverImpl {
    keys_and_locks: [Vec<[u8; 5]>; 2],
}

impl<'input> Solver<'input> for SolverImpl {
    fn new(input: &'input str) -> anyhow::Result<Self> {
        let mut keys_and_locks = [vec![], vec![]];
        let mut input_type: Option<Type> = None;
        let mut heights = [0u8; 5];
        for line in input.lines().map(str::trim).chain(std::iter::once("")) {
            if line.is_empty() {
                {
                    let input_type = input_type.expect("invalid input");
                    if input_type == Type::Key {
                        heights.iter_mut().for_each(|value| *value -= 1);
                    }
                    keys_and_locks[usize::from(input_type)].push(heights);
                    heights = [0; 5];
                }
                input_type = None;
            } else if input_type.is_some() {
                for (index, value) in line.chars().enumerate() {
                    if value == '#' {
                        heights[index] += 1;
                    }
                }
            } else {
                match line {
                    "#####" => input_type = Some(Type::Lock),
                    "....." => input_type = Some(Type::Key),
                    _ => panic!("invalid input"),
                }
            }
        }

        Ok(Self { keys_and_locks })
    }

    fn solve_part_1(&self) -> anyhow::Result<Solution> {
        let result = self.keys_and_locks[usize::from(Type::Lock)]
            .iter()
            .flat_map(|lock| {
                self.keys_and_locks[usize::from(Type::Key)]
                    .iter()
                    .filter(|key| {
                        lock.iter()
                            .zip(key.iter())
                            .all(|(lock_height, key_height)| lock_height + key_height <= 5)
                    })
            })
            .count();
        Ok(Solution::with_description("Part 1", result.to_string()))
    }

    fn solve_part_2(&self) -> anyhow::Result<Solution> {
        Ok(Solution::with_description("Part 2", "n/a".to_string()))
    }
}

#[cfg(test)]
mod test {
    use super::SolverImpl;
    use crate::solvers::Solver;

    #[test]
    fn test_example_part_1() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day25-1.example"))?;
        assert_eq!(solver.solve_part_1()?.solution, "3");
        Ok(())
    }
}
