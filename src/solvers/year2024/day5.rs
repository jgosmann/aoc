use crate::solvers::{Solution, Solver};

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct PageSet(u128);

impl PageSet {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn add(&self, page: u8) -> Self {
        debug_assert!(page < 128);
        Self(self.0 | (1 << page))
    }

    pub fn contains(&self, page: u8) -> bool {
        self.0 & (1 << page) != 0
    }

    pub fn union(&self, other: &Self) -> Self {
        Self(self.0 | other.0)
    }
}

pub struct PageOrder {
    not_before: [PageSet; 100],
}

impl PageOrder {
    pub fn new() -> Self {
        Self {
            not_before: [PageSet::new(); 100],
        }
    }

    pub fn add_ordering(&mut self, before: u8, after: u8) {
        assert!(after < 100);
        self.not_before[after as usize] = self.not_before[after as usize].add(before);
    }

    pub fn disallowed_before(&self, page: u8) -> PageSet {
        self.not_before[page as usize]
    }
}

pub struct SolverImpl {
    page_order: PageOrder,
    page_updates: Vec<Vec<u8>>,
}

impl<'input> Solver<'input> for SolverImpl {
    fn new(input: &'input str) -> anyhow::Result<Self> {
        let mut lines = input.lines();
        let mut page_order = PageOrder::new();
        for page_order_def in lines
            .by_ref()
            .map(str::trim)
            .take_while(|line| !line.is_empty())
        {
            let mut page_num_iter = page_order_def
                .split('|')
                .map(|page_num| page_num.parse::<u8>().expect("invalid int"));
            page_order.add_ordering(
                page_num_iter.next().expect("no before page number"),
                page_num_iter.next().expect("no after page number"),
            );
        }

        let page_updates: Vec<Vec<u8>> = lines
            .map(|line| {
                line.split(',')
                    .map(|page_num| page_num.parse::<u8>().expect("invalid int"))
                    .collect()
            })
            .collect();

        Ok(Self {
            page_order,
            page_updates,
        })
    }

    fn solve_part_1(&self) -> anyhow::Result<Solution> {
        let result: u64 = self
            .page_updates
            .iter()
            .map(|update_order| {
                let mut disallowed = PageSet::new();
                for page in update_order {
                    if disallowed.contains(*page) {
                        return 0;
                    }
                    disallowed = disallowed.union(&self.page_order.disallowed_before(*page));
                }
                update_order[update_order.len() / 2] as u64
            })
            .sum();
        Ok(Solution::with_description("Part 1", result.to_string()))
    }

    fn solve_part_2(&self) -> anyhow::Result<Solution> {
        let result: u64 = self
            .page_updates
            .iter()
            .map(|update_order| {
                let mut update_order = update_order.clone();
                let mut order_corrected = false;
                let mut order_ok = false;
                while !order_ok {
                    order_ok = true;
                    let mut disallowed = PageSet::new();
                    for i in 0..update_order.len() {
                        let page = update_order[i];
                        if disallowed.contains(page) {
                            order_corrected = true;
                            order_ok = false;
                            update_order.swap(i, i - 1);
                            break;
                        }
                        disallowed = disallowed.union(&self.page_order.disallowed_before(page));
                    }
                }
                if order_corrected {
                    return update_order[update_order.len() / 2] as u64;
                }
                0
            })
            .sum();
        Ok(Solution::with_description("Part 2", result.to_string()))
    }
}

#[cfg(test)]
mod test {
    use super::SolverImpl;
    use crate::solvers::Solver;

    #[test]
    fn test_example_part_1() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day5-1.example"))?;
        assert_eq!(solver.solve_part_1()?.solution, "143");
        Ok(())
    }

    #[test]
    fn test_example_part_2() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day5-1.example"))?;
        assert_eq!(solver.solve_part_2()?.solution, "123");
        Ok(())
    }
}
