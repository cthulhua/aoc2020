#[macro_use]
extern crate pest_derive;

use pest::Parser;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Parser)]
#[grammar = "rules.pest"]
struct RuleParser;

fn main() -> Result<(), Box<dyn Error>> {
    let filename = std::env::args().nth(1).unwrap();
    let input = File::open(filename)?;
    let buffered = BufReader::new(input);
    let v: usize = buffered
        .lines()
        .filter(|r| (RuleParser::parse(Rule::r0, &r.as_ref().unwrap())).is_ok())
        .count();
    dbg!(v);
    Ok(())
}
