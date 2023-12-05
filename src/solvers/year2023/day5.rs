use crate::solvers::{Solution, Solver};
use regex::Regex;
use std::cmp::min;
use std::collections::BTreeMap;
use std::num::ParseIntError;
use std::ops::Range;

#[derive(Debug, Clone)]
struct RangeKey(Range<u64>);

impl PartialEq<Self> for RangeKey {
    fn eq(&self, other: &Self) -> bool {
        !(self.0.end <= other.0.start || other.0.end <= self.0.start)
    }
}

impl Eq for RangeKey {}

impl PartialOrd for RangeKey {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self == other {
            return Some(std::cmp::Ordering::Equal);
        }
        Some(self.0.start.cmp(&other.0.start))
    }
}

impl Ord for RangeKey {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self == other {
            return std::cmp::Ordering::Equal;
        }
        self.0.start.cmp(&other.0.start)
    }
}

#[derive(Debug, Clone)]
struct RangeMap(BTreeMap<RangeKey, u64>);

impl RangeMap {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    pub fn insert(&mut self, range: Range<u64>, value: u64) {
        let key = RangeKey(range);
        self.0.insert(key, value);
    }

    pub fn get(&self, key: u64) -> u64 {
        if let Some((range_key, value)) = self.0.get_key_value(&RangeKey(key..key + 1)) {
            value + (key - range_key.0.start)
        } else {
            key
        }
    }

    pub fn get_range(&self, key: &Range<u64>) -> Vec<Range<u64>> {
        self.0
            .iter()
            .filter_map(|(range_key, dest_start)| {
                // obtain all sub-ranges of the key that need to be mapped and map them
                Self::intersect(key, &range_key.0).map(|intersection| {
                    dest_start + (intersection.start - range_key.0.start)
                        ..dest_start + (intersection.end - range_key.0.start)
                })
            })
            .chain(self.0.keys().fold(vec![key.clone()], |acc, range_key| {
                // obtain all sub-ranges of the key that are not mapped
                acc.into_iter()
                    .flat_map(|a| Self::subtract(&a, &range_key.0))
                    .collect()
            }))
            .collect()
    }

    fn subtract(minuend: &Range<u64>, subtrahend: &Range<u64>) -> Vec<Range<u64>> {
        let mut difference = Vec::with_capacity(2);
        if minuend.start < subtrahend.start {
            difference.push(minuend.start..min(minuend.end, subtrahend.start));
        }
        if minuend.end > subtrahend.end {
            difference.push(minuend.start.max(subtrahend.end)..minuend.end);
        }
        difference
    }

    fn intersect(a: &Range<u64>, b: &Range<u64>) -> Option<Range<u64>> {
        if a.start > b.start {
            return Self::intersect(b, a);
        }
        if a.end <= b.start {
            return None;
        }
        Some(b.start..a.end.min(b.end))
    }
}

pub struct SolverImpl {
    seeds: Vec<u64>,
    range_maps: Vec<RangeMap>,
}

impl<'input> Solver<'input> for SolverImpl {
    fn new(input: &'input str) -> anyhow::Result<Self> {
        let mut lines = input.lines();
        let seeds_line = lines.next().expect("must define seeds");
        let seeds = seeds_line
            .split_once(':')
            .expect("must define seeds")
            .1
            .split(' ')
            .map(|part| part.trim())
            .filter(|part| !part.is_empty())
            .map(|part| part.parse::<u64>())
            .collect::<Result<Vec<u64>, ParseIntError>>()?;

        let map_declaration_regex = Regex::new(r"^\w+-to-\w+ map:$").unwrap();
        let mut range_maps: Vec<RangeMap> = vec![RangeMap::new()];
        for line in lines {
            if line.trim().is_empty() {
                continue;
            }
            if map_declaration_regex.is_match(line) {
                range_maps.push(RangeMap::new());
                continue;
            }

            let values = line
                .split(' ')
                .map(|part| part.trim().parse::<u64>())
                .collect::<Result<Vec<u64>, ParseIntError>>()?;
            if values.len() != 3 {
                continue;
            }

            let source_range_start = values[1];
            let dest_range_start = values[0];
            let range_length = values[2];
            range_maps.last_mut().unwrap().insert(
                source_range_start..source_range_start + range_length,
                dest_range_start,
            );
        }

        Ok(Self { seeds, range_maps })
    }

    fn solve_part_1(&self) -> anyhow::Result<Solution> {
        let min_location = self
            .seeds
            .iter()
            .copied()
            .map(|seed| {
                self.range_maps
                    .iter()
                    .fold(seed, |value, mapping| mapping.get(value))
            })
            .min()
            .unwrap();
        Ok(Solution::with_description(
            "Lowest location (part 1)",
            min_location.to_string(),
        ))
    }

    fn solve_part_2(&self) -> anyhow::Result<Solution> {
        let ranges: Vec<_> = self
            .seeds
            .windows(2)
            .step_by(2)
            .map(|seed_range| seed_range[0]..seed_range[0] + seed_range[1])
            .collect();
        let min_location = self
            .range_maps
            .iter()
            .fold(ranges, |ranges, mapping| {
                ranges
                    .iter()
                    .flat_map(|seed_range| mapping.get_range(seed_range))
                    .collect()
            })
            .iter()
            .map(|range| range.start)
            .min()
            .unwrap();

        Ok(Solution::with_description(
            "Lowest location (part 2)",
            min_location.to_string(),
        ))
    }
}

#[cfg(test)]
mod test {
    use super::SolverImpl;
    use crate::solvers::Solver;

    #[test]
    fn test_example_part_1() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day5-1.example"))?;
        assert_eq!(solver.solve_part_1()?.solution, "35");
        Ok(())
    }

    #[test]
    fn test_example_part_2() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day5-1.example"))?;
        assert_eq!(solver.solve_part_2()?.solution, "46");
        Ok(())
    }
}
