use crate::solvers::{Solution, Solver};
use std::collections::HashMap;

#[derive(Clone, Debug)]
struct Trie {
    children: [Option<Box<Trie>>; 5],
    is_terminal: bool,
}

impl Default for Trie {
    fn default() -> Self {
        Self::new()
    }
}

impl Trie {
    pub fn new() -> Self {
        Self {
            children: [None, None, None, None, None],
            is_terminal: false,
        }
    }

    pub fn insert(&mut self, word: &[u8]) {
        if word.is_empty() {
            self.is_terminal = true;
            return;
        }
        let index = Self::color_to_index(word[0]);
        self.children[index]
            .get_or_insert_with(|| Box::new(Trie::new()))
            .insert(&word[1..]);
    }

    pub fn contains(&self, word: &[u8]) -> bool {
        if word.is_empty() {
            return self.is_terminal;
        }
        let index = Self::color_to_index(word[0]);
        self.children[index]
            .as_ref()
            .is_some_and(|child| child.contains(&word[1..]))
    }

    fn color_to_index(color: u8) -> usize {
        match color {
            b'w' => 0,
            b'u' => 1,
            b'b' => 2,
            b'r' => 3,
            b'g' => 4,
            _ => panic!("Invalid color"),
        }
    }
}

pub struct SolverImpl {
    counts: Vec<usize>,
}

impl<'input> Solver<'input> for SolverImpl {
    fn new(input: &'input str) -> anyhow::Result<Self> {
        let mut trie = Trie::new();
        let mut max_word_len = 0;
        let mut lines = input.lines();
        for pattern in lines.next().expect("Missing towel patterns").split(",") {
            trie.insert(pattern.trim().as_bytes());
            max_word_len = max_word_len.max(pattern.len());
        }

        let desired_designs: Vec<_> = lines
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .map(|line| line.as_bytes())
            .collect();

        let mut counter = ArrangementCounter::new(&trie, max_word_len);
        let counts = desired_designs
            .iter()
            .map(|design| counter.count(design))
            .collect();

        Ok(Self { counts })
    }

    fn solve_part_1(&self) -> anyhow::Result<Solution> {
        let result = self.counts.iter().filter(|&&count| count > 0).count();
        Ok(Solution::with_description("Part 1", result.to_string()))
    }

    fn solve_part_2(&self) -> anyhow::Result<Solution> {
        let result: usize = self.counts.iter().sum();
        Ok(Solution::with_description("Part 2", result.to_string()))
    }
}

struct ArrangementCounter<'a> {
    memo: HashMap<&'a [u8], usize>,
    trie: &'a Trie,
    max_word_len: usize,
}

impl<'a> ArrangementCounter<'a> {
    pub fn new(trie: &'a Trie, max_word_len: usize) -> Self {
        Self {
            memo: HashMap::new(),
            trie,
            max_word_len,
        }
    }

    pub fn count(&mut self, design: &'a [u8]) -> usize {
        if design.is_empty() {
            return 1;
        }

        if let Some(result) = self.memo.get(design) {
            return *result;
        }

        let result = (1..=self.max_word_len.min(design.len()))
            .map(|prefix_len| {
                if self.trie.contains(&design[..prefix_len]) {
                    self.count(&design[prefix_len..])
                } else {
                    0
                }
            })
            .sum();
        self.memo.insert(design, result);
        result
    }
}

#[cfg(test)]
mod test {
    use super::SolverImpl;
    use crate::solvers::Solver;

    #[test]
    fn test_example_part_1() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day19-1.example"))?;
        assert_eq!(solver.solve_part_1()?.solution, "6");
        Ok(())
    }

    #[test]
    fn test_example_part_2() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day19-1.example"))?;
        assert_eq!(solver.solve_part_2()?.solution, "16");
        Ok(())
    }
}
