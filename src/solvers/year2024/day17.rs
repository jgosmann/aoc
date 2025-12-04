use crate::solvers::{Solution, Solver};
use anyhow::anyhow;
use regex::Regex;

type Word = u128;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
enum Operand {
    Literal(Word),
    Combo(Word),
    Ignored(Word),
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
enum OpCode {
    Adv(Operand),
    Bxl(Operand),
    Bst(Operand),
    Jnz(Operand),
    Bxc(Operand),
    Out(Operand),
    Bdv(Operand),
    Cdv(Operand),
}

impl TryFrom<(&str, &str)> for OpCode {
    type Error = anyhow::Error;

    fn try_from(value: (&str, &str)) -> Result<Self, Self::Error> {
        match value.0 {
            "0" => Ok(OpCode::Adv(Operand::Combo(value.1.parse()?))),
            "1" => Ok(OpCode::Bxl(Operand::Literal(value.1.parse()?))),
            "2" => Ok(OpCode::Bst(Operand::Combo(value.1.parse()?))),
            "3" => Ok(OpCode::Jnz(Operand::Literal(value.1.parse()?))),
            "4" => Ok(OpCode::Bxc(Operand::Ignored(value.1.parse()?))),
            "5" => Ok(OpCode::Out(Operand::Combo(value.1.parse()?))),
            "6" => Ok(OpCode::Bdv(Operand::Combo(value.1.parse()?))),
            "7" => Ok(OpCode::Cdv(Operand::Combo(value.1.parse()?))),
            _ => Err(anyhow!("Invalid opcode")),
        }
    }
}

#[derive(Debug, Clone)]
struct Processor<'mem> {
    registers: [Word; 3],
    instruction_pointer: usize,
    memory: &'mem [OpCode],
    output: Vec<String>,
}

impl<'mem> Processor<'mem> {
    fn new(registers: [Word; 3], memory: &'mem [OpCode]) -> Self {
        Self {
            registers,
            instruction_pointer: 0,
            memory,
            output: Vec::new(),
        }
    }

    fn run(&mut self) -> String {
        while self.instruction_pointer < self.memory.len() {
            self.step();
        }
        self.output.join(",")
    }

    fn load(&self, operand: Operand) -> Word {
        match operand {
            Operand::Literal(value) => value,
            Operand::Combo(index) => match index {
                0..=3 => index,
                4..=6 => self.registers[(index - 4) as usize],
                _ => panic!("Invalid combo operand."),
            },
            Operand::Ignored(_) => panic!("Usage of ignored operand."),
        }
    }

    fn step(&mut self) {
        let instruction = self.memory[self.instruction_pointer];
        self.instruction_pointer += 1;
        match instruction {
            OpCode::Adv(operand) => {
                let operand_value = self.load(operand);
                self.registers[0] /= 1 << operand_value;
            }
            OpCode::Bxl(operand) => {
                let operand_value = self.load(operand);
                self.registers[1] ^= operand_value;
            }
            OpCode::Bst(operand) => {
                let operand_value = self.load(operand);
                self.registers[1] = operand_value & 0b0111;
            }
            OpCode::Jnz(operand) => {
                if self.registers[0] != 0 {
                    self.instruction_pointer = self.load(operand) as usize;
                }
            }
            OpCode::Bxc(_) => {
                self.registers[1] ^= self.registers[2];
            }
            OpCode::Out(operand) => {
                let operand_value = self.load(operand);
                self.output
                    .push(((operand_value & 0b0111) as u8).to_string());
            }
            OpCode::Bdv(operand) => {
                let operand_value = self.load(operand);
                self.registers[1] = self.registers[0] / (1 << operand_value);
            }
            OpCode::Cdv(operand) => {
                let operand_value = self.load(operand);
                self.registers[2] = self.registers[0] / (1 << operand_value);
            }
        }
    }
}

fn extract_input_value(line: &str) -> anyhow::Result<&str> {
    let regex = Regex::new(r".*:\s*(.*)$")?;
    Ok(regex
        .captures(line)
        .ok_or(anyhow::anyhow!("Invalid input"))?
        .get(1)
        .ok_or(anyhow!("Invalid input"))?
        .as_str())
}

pub struct SolverImpl {
    initial_registers: [Word; 3],
    program: Vec<OpCode>,
    targets: Vec<usize>,
}

impl<'input> Solver<'input> for SolverImpl {
    fn new(input: &'input str) -> anyhow::Result<Self> {
        let mut lines = input.lines();
        let registers = [
            extract_input_value(lines.next().unwrap_or_default())?.parse()?,
            extract_input_value(lines.next().unwrap_or_default())?.parse()?,
            extract_input_value(lines.next().unwrap_or_default())?.parse()?,
        ];
        lines.next(); // skip empty line
        let program_def = extract_input_value(lines.next().unwrap_or_default())?;
        let mut program = Vec::with_capacity((program_def.len() + 1) / 4);
        let mut targets = Vec::with_capacity(program_def.len().div_ceil(2));
        let mut program_def = program_def.split(',');
        while let Some(opcode) = program_def.next() {
            let operand = program_def.next().ok_or(anyhow!("Invalid input"))?;
            targets.push(opcode.parse()?);
            targets.push(operand.parse()?);
            program.push(OpCode::try_from((opcode, operand))?);
        }

        Ok(Self {
            initial_registers: registers,
            program,
            targets,
        })
    }

    fn solve_part_1(&self) -> anyhow::Result<Solution> {
        let result = Processor::new(self.initial_registers, &self.program).run();
        Ok(Solution::with_description("Part 1", result))
    }

    fn solve_part_2(&self) -> anyhow::Result<Solution> {
        let target_string = self
            .targets
            .iter()
            .map(|t| t.to_string())
            .collect::<Vec<_>>()
            .join(",");

        let producing_bits: Vec<Vec<_>> = (0..8)
            .map(|target| {
                (0..8u128)
                    .flat_map(|b| {
                        (0..8u128).filter_map(move |c| {
                            let b1 = b ^ 0b101;
                            let c1 = c << b1;
                            let b2 = b1 ^ 0b110;
                            if b2 ^ c == target {
                                let overlap_mask = (0b111 << b1) & 0b111;
                                if (c1 & overlap_mask) == (b & overlap_mask) {
                                    let mask = 0b111u128 << b1 | 0b111;
                                    return Some((b | c1, mask));
                                }
                            }
                            None
                        })
                    })
                    .collect()
            })
            .collect();

        let mut candidates = vec![];
        let mut chosen_producers = [0usize; 16];
        while chosen_producers
            .iter()
            .enumerate()
            .all(|(i, producer)| *producer < producing_bits[self.targets[i]].len())
        {
            let mut fixed = 0u128;
            let mut value = 0u128;
            let mut failed = false;
            for (i, producer) in chosen_producers.iter_mut().enumerate() {
                let producing_bits_for_target = &producing_bits[self.targets[i]];
                let (mut bits, mut mask) = producing_bits_for_target[*producer];
                bits <<= 3 * i;
                mask <<= 3 * i;
                if (fixed & mask) & value != (fixed & mask) & bits {
                    self.inc_chosen_producers(i, &mut chosen_producers, &producing_bits);
                    failed = true;
                    break;
                }
                fixed |= mask;
                value |= bits;
            }
            if !failed {
                let mut processor = Processor::new(
                    [value, self.initial_registers[1], self.initial_registers[2]],
                    &self.program,
                );
                let result = processor.run();
                if result == target_string {
                    candidates.push(value);
                }
                self.inc_chosen_producers(15, &mut chosen_producers, &producing_bits);
            }
        }
        Ok(Solution::with_description(
            "Part 1",
            candidates
                .iter()
                .min()
                .ok_or(anyhow!("No solution"))?
                .to_string(),
        ))
    }
}

impl SolverImpl {
    fn inc_chosen_producers(
        &self,
        index: usize,
        chosen_producers: &mut [usize],
        producing_bits: &[Vec<(u128, u128)>],
    ) {
        let mut i = index;
        chosen_producers[i] += 1;
        while i > 0 && chosen_producers[i] >= producing_bits[self.targets[i]].len() {
            chosen_producers[i] = 0;
            i -= 1;
            chosen_producers[i] += 1;
        }
    }
}

#[cfg(test)]
mod test {
    use super::SolverImpl;
    use crate::solvers::Solver;

    #[test]
    fn test_example_part_1() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day17-1.example"))?;
        assert_eq!(solver.solve_part_1()?.solution, "4,6,3,5,6,3,5,2,1,0");
        Ok(())
    }
}
