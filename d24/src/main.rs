#[macro_use]
extern crate pest_derive;

use pest::iterators::Pairs;
use pest::Parser;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Parser)]
#[grammar = "hex.pest"]
struct HexParser;

#[derive(Debug)]
struct Hex {
    x: i32,
    y: i32,
    z: i32,
}

enum Direction {
    East,
    Southeast,
    Southwest,
    West,
    Northwest,
    Northeast,
}

impl Hex {
    fn new() -> Hex {
        Hex { x: 0, y: 0, z: 0 }
    }

    fn as_tuple(&self) -> (i32, i32, i32) {
        (self.x, self.y, self.z)
    }

    fn go(&mut self, direction: &Direction) {
        match direction {
            Direction::East => {
                self.x += 1;
                self.y -= 1;
            }
            Direction::Southeast => {
                self.z += 1;
                self.y -= 1;
            }
            Direction::Southwest => {
                self.x -= 1;
                self.z += 1;
            }
            Direction::West => {
                self.x -= 1;
                self.y += 1;
            }
            Direction::Northwest => {
                self.z -= 1;
                self.y += 1;
            }
            Direction::Northeast => {
                self.x += 1;
                self.z -= 1;
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let filename = std::env::args().nth(1).unwrap();
    let input = File::open(filename)?;
    let buffered = BufReader::new(input);
    let mut black_tiles: HashMap<(i32, i32, i32), bool> = HashMap::new();
    buffered
        .lines()
        .map(|r| eval(HexParser::parse(Rule::hex, &r.unwrap()).unwrap()))
        .for_each(|hex| {
            if let Some(is_black) = black_tiles.get_mut(&hex.as_tuple()) {
                *is_black = !*is_black;
            } else {
                black_tiles.insert(hex.as_tuple(), true);
            }
        });
    let count = black_tiles
        .into_iter()
        .filter(|(_, is_black)| *is_black)
        .count();
    dbg!(&count);
    Ok(())
}

fn eval(hex: Pairs<Rule>) -> Hex {
    let mut result = Hex::new();
    hex.for_each(|p| {
        (p.into_inner()).for_each(|r| match r.as_rule() {
            Rule::east => {
                result.go(&Direction::East);
            }
            Rule::southeast => {
                result.go(&Direction::Southeast);
            }
            Rule::southwest => {
                result.go(&Direction::Southwest);
            }
            Rule::west => {
                result.go(&Direction::West);
            }
            Rule::northwest => {
                result.go(&Direction::Northwest);
            }
            Rule::northeast => {
                result.go(&Direction::Northeast);
            }
            _ => (panic!("unexpected rule")),
        });
    });
    result
}
