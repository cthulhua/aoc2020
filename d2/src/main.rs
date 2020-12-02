use std::fs::File;
use std::io::{BufRead, BufReader};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = File::open("input.txt")?;
    let buffered = BufReader::new(input);
    let count = buffered
        .lines()
        .map(|line| params(&line.unwrap()))
        .filter(Params::valid)
        .count();
    dbg!(count);
    Ok(())
}

fn params(line: &str) -> Params {
    let mut tokens = line.split_whitespace();
    let mut range = tokens.next().unwrap().split('-');
    let min = range.next().unwrap().parse::<u64>().unwrap();
    let max = range.next().unwrap().parse::<u64>().unwrap();
    let c = tokens.next().unwrap().chars().next().unwrap();
    let password = String::from(tokens.next().unwrap());
    Params {
        min,
        max,
        c,
        password,
    }
}

#[derive(Debug)]
struct Params {
    min: u64,
    max: u64,
    c: char,
    password: String,
}
impl Params {
    fn valid(&self) -> bool {
        let matches = self
            .password
            .match_indices(self.c)
            .filter(|(idx, _)| *idx + 1 == self.min as usize || *idx + 1 == self.max as usize);
        matches.count() == 1
    }
}
