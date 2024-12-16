use crate::datastructures::grid::GridView;
use crate::solvers::{Solution, Solver};
use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet};

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, Ord, PartialOrd)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn delta(&self) -> (isize, isize) {
        match self {
            Direction::North => (-1, 0),
            Direction::South => (1, 0),
            Direction::East => (0, 1),
            Direction::West => (0, -1),
        }
    }

    fn lturn(&self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
            Direction::West => Direction::South,
        }
    }

    fn rturn(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::East => Direction::South,
            Direction::West => Direction::North,
        }
    }
}

struct MazeResult {
    score: usize,
    tiles_part_of_path: usize,
}

pub struct SolverImpl {
    result: MazeResult,
}

impl<'input> Solver<'input> for SolverImpl {
    fn new(input: &'input str) -> anyhow::Result<Self> {
        let grid = GridView::from_separated(b'\n', input.as_bytes());
        let start_pos = grid
            .iter()
            .position(|c| c == b'S')
            .map(|p| grid.nth_index(p))
            .expect("no starting position");
        let result = Self::find_lowest_score(&grid, start_pos);

        Ok(Self { result })
    }

    fn solve_part_1(&self) -> anyhow::Result<Solution> {
        Ok(Solution::with_description(
            "Part 1",
            self.result.score.to_string(),
        ))
    }

    fn solve_part_2(&self) -> anyhow::Result<Solution> {
        Ok(Solution::with_description(
            "Part 2",
            self.result.tiles_part_of_path.to_string(),
        ))
    }
}

impl SolverImpl {
    fn find_lowest_score(grid: &GridView<&[u8]>, start_pos: (usize, usize)) -> MazeResult {
        let mut to_visit = BinaryHeap::new();
        to_visit.push((
            Reverse(0),
            start_pos,
            Direction::East,
            (start_pos, Direction::East),
        ));
        type DirectionalPos = ((usize, usize), Direction);
        let mut reachable_from: HashMap<DirectionalPos, (usize, Vec<DirectionalPos>)> =
            HashMap::new();

        while let Some((Reverse(score), pos, dir, prev)) = to_visit.pop() {
            if grid[pos] == b'#' {
                continue;
            }

            let (best_score, prev_positions) = reachable_from
                .entry((pos, dir))
                .or_insert((usize::MAX, vec![]));
            match score.cmp(best_score) {
                Ordering::Equal => {
                    prev_positions.push(prev);
                }
                Ordering::Less => {
                    reachable_from.insert((pos, dir), (score, vec![prev]));
                }
                Ordering::Greater => {
                    continue;
                }
            }

            if grid[pos] == b'E' {
                let mut paths = HashSet::new();
                let mut to_backtrack = vec![(pos, dir)];
                while let Some((bpos, bdir)) = to_backtrack.pop() {
                    paths.insert((bpos, bdir));
                    to_backtrack.extend(
                        reachable_from[&(bpos, bdir)]
                            .1
                            .iter()
                            .filter(|&p| !paths.contains(p)),
                    );
                }
                let paths = paths.iter().map(|(pos, _)| pos).collect::<HashSet<_>>();
                return MazeResult {
                    score,
                    tiles_part_of_path: paths.len(),
                };
            }

            if Self::next_pos(grid, pos, dir.lturn()).is_some() {
                to_visit.push((Reverse(score + 1000), pos, dir.lturn(), (pos, dir)));
            }
            if Self::next_pos(grid, pos, dir.rturn()).is_some() {
                to_visit.push((Reverse(score + 1000), pos, dir.rturn(), (pos, dir)));
            }
            if let Some(forward_pos) = Self::next_pos(grid, pos, dir) {
                to_visit.push((Reverse(score + 1), forward_pos, dir, (pos, dir)));
            }
        }

        panic!("no path to exit");
    }

    fn next_pos(
        grid: &GridView<&[u8]>,
        pos: (usize, usize),
        dir: Direction,
    ) -> Option<(usize, usize)> {
        let forward_pos = (
            pos.0.checked_add_signed(dir.delta().0),
            pos.1.checked_add_signed(dir.delta().1),
        );
        let forward_pos = forward_pos.0.and_then(|x| forward_pos.1.map(|y| (x, y)));
        if let Some(forward_pos) = forward_pos {
            if forward_pos.0 < grid.height() && forward_pos.1 < grid.width() {
                return Some(forward_pos);
            }
        }
        None
    }
}

#[cfg(test)]
mod test {
    use super::SolverImpl;
    use crate::solvers::Solver;

    #[test]
    fn test_example_part_1_1() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day16-1.example"))?;
        assert_eq!(solver.solve_part_1()?.solution, "7036");
        Ok(())
    }

    #[test]
    fn test_example_part_1_2() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day16-2.example"))?;
        assert_eq!(solver.solve_part_1()?.solution, "11048");
        Ok(())
    }

    #[test]
    fn test_example_part_2_1() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day16-1.example"))?;
        assert_eq!(solver.solve_part_2()?.solution, "45");
        Ok(())
    }

    #[test]
    fn test_example_part_2_2() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day16-2.example"))?;
        assert_eq!(solver.solve_part_2()?.solution, "64");
        Ok(())
    }
}
