use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
fn main() -> Result<(), Box<dyn Error>> {
    let program = Program::load("input.txt")?;
    let mut vm = Vm {
        program,
        x: 0,
        y: 0,
        wx: 10,
        wy: -1,
        instruction_pointer: 0,
    };

    dbg!(vm.run());
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
    N(i64),
    E(i64),
    S(i64),
    W(i64),
    F(i64),
    L(i64),
    R(i64),
}

impl Instruction {
    fn parse(s: &str) -> Result<Self, Box<dyn Error>> {
        let (op, arg) = s.split_at(1);
        let instruction = match op {
            "N" => Self::N(arg.parse()?),
            "E" => Self::E(arg.parse()?),
            "S" => Self::S(arg.parse()?),
            "W" => Self::W(arg.parse()?),
            "F" => Self::F(arg.parse()?),
            "L" => Self::L(arg.parse()?),
            "R" => Self::R(arg.parse()?),
            _ => panic!("Unknown op"),
        };
        Ok(instruction)
    }
}

struct Vm {
    program: Program,
    instruction_pointer: usize,
    x: i64,
    y: i64,
    wx: i64,
    wy: i64,
}

impl Vm {
    fn run(&mut self) -> i64 {
        while &self.instruction_pointer < &self.program.instructions.len() {
            let loaded_instruction = &self.program.instructions[self.instruction_pointer].clone();
            self.exec(loaded_instruction);
            dbg!(self.x, self.y, self.wx, self.wy);
        }
        self.x.abs() + self.y.abs()
    }

    fn exec(&mut self, instruction: &Instruction) {
        match *instruction {
            Instruction::N(arg) => {
                self.wy -= arg;
            }
            Instruction::E(arg) => {
                self.wx += arg;
            }
            Instruction::S(arg) => {
                self.wy += arg;
            }
            Instruction::W(arg) => {
                self.wx -= arg;
            }
            Instruction::L(arg) => {
                let mut deg = arg;
                while deg > 0 {
                    let temp = self.wx;
                    self.wx = self.wy;
                    self.wy = temp;
                    self.wy *= -1;
                    deg -= 90;
                }
            }
            Instruction::R(arg) => {
                let mut deg = arg;
                while deg > 0 {
                    let temp = self.wx;
                    self.wx = self.wy;
                    self.wy = temp;
                    self.wx *= -1;
                    deg -= 90;
                }
            }
            Instruction::F(arg) => {
                self.x += arg * self.wx;
                self.y += arg * self.wy;
            }
        }
        self.instruction_pointer += 1;
    }
}
