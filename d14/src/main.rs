use bitvec::prelude::*;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn main() -> Result<(), Box<dyn Error>> {
    let file = std::env::args().nth(1).unwrap();
    let program = Program::load(&file)?;
    let mut vm = Vm {
        program,
        instruction_pointer: 0,
        mask: Mask::default(),
        memory: HashMap::new(),
    };
    let _ = dbg!(vm.run());
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
    Mask(Mask),
    Mem(u64, u64),
}

#[derive(Clone, Debug, Default)]
struct Mask {
    ones: u64,
    zeros: u64,
}

impl FromStr for Mask {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ones = bitvec![Lsb0, u64;0;64];
        let mut zeros = bitvec![Lsb0, u64;1;64];
        let mut chars: Vec<char> = s.chars().collect();
        chars.reverse();
        for (i, c) in chars.into_iter().enumerate() {
            match c {
                '1' => ones.as_mut_bitslice().set(i, true),
                '0' => zeros.as_mut_bitslice().set(i, false),
                _ => (),
            }
        }
        let (_, ones, _) = ones.domain().region().unwrap();
        let ones = ones[0];
        let (_, zeros, _) = zeros.domain().region().unwrap();
        let zeros = zeros[0];
        Ok(Self { ones, zeros })
    }
}
impl Mask {
    fn apply(&self, val: u64) -> u64 {
        let mut masked = val;
        masked |= self.ones;
        masked &= self.zeros;
    }
}

impl Instruction {
    fn parse(s: &str) -> Result<Self, Box<dyn Error>> {
        let mut symbols = s.split_whitespace();
        let op = symbols.next().unwrap();
        symbols.next();
        let arg = symbols.next().unwrap();
        let instruction = match op {
            "mask" => Self::Mask(Mask::from_str(arg).unwrap()),
            //if not a mask, it's a mem
            _ => {
                let addr: u64 = op[4..op.len() - 1].parse()?;
                let val: u64 = arg.parse()?;
                Self::Mem(addr, val)
            }
        };
        Ok(instruction)
    }
}

struct Vm {
    program: Program,
    instruction_pointer: usize,
    mask: Mask,
    memory: HashMap<u64, u64>,
}

impl Vm {
    fn run(&mut self) -> Result<u64, u64> {
        while &self.instruction_pointer < &self.program.instructions.len() {
            let loaded_instruction = &self.program.instructions[self.instruction_pointer].clone();
            self.exec(loaded_instruction);
            self.instruction_pointer += 1;
        }
        if self.instruction_pointer == self.program.instructions.len() {
            Ok(self.memory.values().sum())
        } else {
            Err(self.memory.values().sum())
        }
    }

    fn exec(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Mask(mask) => self.mask = mask.clone(),
            Instruction::Mem(addr, val) => {
                self.memory.insert(*addr, self.mask.apply(*val));
            }
        }
    }
}
