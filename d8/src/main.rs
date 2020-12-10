use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
fn main() -> Result<(), Box<dyn Error>> {
    let program = Program::load("input.txt")?;
    let fuzzer = Fuzzer::for_program(program);
    let results: Vec<Result<i64, i64>> = fuzzer
        .map(|mut vm| vm.run())
        .filter(Result::is_ok)
        .collect();
    dbg!(results);
    Ok(())
}

#[derive(Clone, Debug)]
struct Program {
    instructions: Vec<Instruction>,
}

impl Program {
    fn load(filename: &str) -> Result<Self, Box<dyn Error>> {
        let input = File::open(filename)?;
        let buffered = BufReader::new(input);
        let instructions: Result<Vec<Instruction>, Box<dyn Error>> = buffered
            .lines()
            .map(|line| Instruction::parse(&line.unwrap()))
            .collect();
        Ok(Program {
            instructions: instructions?,
        })
    }
}

#[derive(Clone, Debug)]
enum Instruction {
    Acc(i64),
    Jmp(i64),
    Nop(i64),
}

impl Instruction {
    fn parse(s: &str) -> Result<Self, Box<dyn Error>> {
        let mut symbols = s.split_whitespace();
        let op = symbols.next().unwrap();
        let arg = symbols.next().unwrap();
        let instruction = match op {
            "acc" => Self::Acc(arg.parse()?),
            "jmp" => Self::Jmp(arg.parse()?),
            "nop" => Self::Nop(arg.parse()?),
            _ => panic!("Unknown op"),
        };
        Ok(instruction)
    }
}

struct HaltingVm {
    program: Program,
    visited_locations: HashSet<usize>,
    instruction_pointer: usize,
    acc: i64,
}

impl HaltingVm {
    fn run(&mut self) -> Result<i64, i64> {
        while !self.visited_locations.contains(&self.instruction_pointer)
            && &self.instruction_pointer < &self.program.instructions.len()
        {
            self.visited_locations.insert(self.instruction_pointer);
            let loaded_instruction = &self.program.instructions[self.instruction_pointer].clone();
            self.exec(loaded_instruction);
        }
        if self.instruction_pointer == self.program.instructions.len() {
            Ok(self.acc)
        } else {
            Err(self.acc)
        }
    }

    fn exec(&mut self, instruction: &Instruction) {
        match *instruction {
            Instruction::Acc(arg) => {
                self.acc += arg;
                self.instruction_pointer += 1;
            }
            Instruction::Jmp(arg) => {
                if arg >= 0 {
                    self.instruction_pointer += arg as usize
                } else {
                    self.instruction_pointer -= (arg * -1) as usize
                }
            }
            Instruction::Nop(_) => {
                self.instruction_pointer += 1;
            }
        }
    }
}
struct Fuzzer {
    program: Program,
    fuzz_points: Box<dyn Iterator<Item = usize>>,
}

impl Iterator for Fuzzer {
    type Item = HaltingVm;
    fn next(&mut self) -> Option<HaltingVm> {
        self.fuzz_points.next().map(|idx| {
            let mut new_program = self.program.clone();
            new_program.instructions[idx] = match new_program.instructions[idx] {
                Instruction::Acc(_) => panic!("shit shouldn't fuzz"),
                Instruction::Nop(arg) => Instruction::Jmp(arg),
                Instruction::Jmp(arg) => Instruction::Nop(arg),
            };

            HaltingVm {
                program: new_program,
                instruction_pointer: 0,
                visited_locations: HashSet::new(),
                acc: 0,
            }
        })
    }
}

impl Fuzzer {
    fn point_iter(program: &Program) -> Box<dyn Iterator<Item = usize>> {
        let i = program
            .instructions
            .clone()
            .into_iter()
            .enumerate()
            .filter_map(|(idx, instruction)| match instruction {
                Instruction::Jmp(_) | Instruction::Nop(_) => Some(idx),
                _ => None,
            });
        Box::new(i)
    }
    fn for_program(program: Program) -> Self {
        let fuzz_points = Self::point_iter(&program);
        Self {
            program,
            fuzz_points,
        }
    }
}
