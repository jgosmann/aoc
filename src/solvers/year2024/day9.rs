use crate::solvers::{Solution, Solver};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MapEntry {
    FreeSpace,
    File,
}

impl From<usize> for MapEntry {
    fn from(index: usize) -> Self {
        if index % 2 == 1 {
            MapEntry::FreeSpace
        } else {
            MapEntry::File
        }
    }
}

fn idx2id(index: usize) -> usize {
    index / 2
}

fn gauss(n: usize) -> usize {
    (n * n + n) / 2
}

pub struct SolverImpl {
    disk_map: Vec<u8>,
}

impl<'input> Solver<'input> for SolverImpl {
    fn new(input: &'input str) -> anyhow::Result<Self> {
        let disk_map = input.trim().as_bytes().iter().map(|c| c - b'0').collect();
        Ok(Self { disk_map })
    }

    fn solve_part_1(&self) -> anyhow::Result<Solution> {
        let mut disk_map = self.disk_map.clone();
        let mut end_pointer = disk_map.len() - 1;
        if MapEntry::from(end_pointer) == MapEntry::FreeSpace {
            end_pointer -= 1;
        }
        let mut physical_index: usize = disk_map[0] as usize;
        let mut checksum = 0;
        for i in 1..disk_map.len() {
            match i.into() {
                MapEntry::File => {
                    let start = physical_index - 1;
                    physical_index += disk_map[i] as usize;
                    let end = physical_index - 1;
                    checksum += idx2id(i) * (gauss(end) - gauss(start));
                }
                MapEntry::FreeSpace => {
                    let mut available = disk_map[i];
                    while available > 0 && end_pointer > i {
                        let chunksize = disk_map[end_pointer].min(available);
                        let file_id = idx2id(end_pointer);
                        disk_map[end_pointer] -= chunksize;
                        if disk_map[end_pointer] == 0 {
                            end_pointer -= 2;
                        }
                        available -= chunksize;

                        let start = physical_index - 1;
                        physical_index += chunksize as usize;
                        let end = physical_index - 1;
                        checksum += file_id * (gauss(end) - gauss(start));
                    }
                }
            }
        }
        Ok(Solution::with_description("Part 1", checksum.to_string()))
    }

    fn solve_part_2(&self) -> anyhow::Result<Solution> {
        let disk_map = self.disk_map.clone();
        let mut disk_moved = vec![false; disk_map.len()];
        let mut end_pointer = disk_map.len() - 1;
        if MapEntry::from(end_pointer) == MapEntry::FreeSpace {
            end_pointer -= 1;
        }

        let mut physical_index: usize = disk_map[0] as usize;
        let mut checksum = 0;
        for i in 1..disk_map.len() {
            match i.into() {
                MapEntry::File => {
                    if disk_moved[i] {
                        physical_index += disk_map[i] as usize;
                    } else {
                        let start = physical_index - 1;
                        physical_index += disk_map[i] as usize;
                        let end = physical_index - 1;
                        checksum += idx2id(i) * (gauss(end) - gauss(start));
                    }
                }
                MapEntry::FreeSpace => {
                    let mut available = disk_map[i];
                    for j in ((i + 1)..end_pointer + 1).step_by(2).rev() {
                        if !disk_moved[j] && disk_map[j] <= available {
                            let file_id = idx2id(j);
                            let chunksize = disk_map[j];
                            disk_moved[j] = true;
                            let start = physical_index - 1;
                            physical_index += chunksize as usize;
                            let end = physical_index - 1;
                            checksum += file_id * (gauss(end) - gauss(start));
                            available -= chunksize;
                            if available == 0 {
                                break;
                            }
                        }
                    }
                    physical_index += available as usize;
                }
            }
        }

        Ok(Solution::with_description("Part 2", checksum.to_string()))
    }
}

#[cfg(test)]
mod test {
    use super::SolverImpl;
    use crate::solvers::Solver;

    #[test]
    fn test_example_part_1() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day9-1.example"))?;
        assert_eq!(solver.solve_part_1()?.solution, "1928");
        Ok(())
    }

    #[test]
    fn test_example_part_2() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day9-1.example"))?;
        assert_eq!(solver.solve_part_2()?.solution, "2858");
        Ok(())
    }
}
