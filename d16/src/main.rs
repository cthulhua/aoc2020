use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::RangeInclusive;
use std::str::FromStr;

fn main() -> Result<(), Box<dyn Error>> {
    let file = std::env::args().nth(1).unwrap();
    let input = File::open(file)?;
    let buffered = BufReader::new(input);
    let mut lines = buffered.lines().map(Result::unwrap);
    let rule_lines = lines.by_ref().take_while(|line| !line.trim().is_empty());
    let rules: Result<Vec<Rule>, Box<dyn Error>> = rule_lines.map(|s| Rule::from_str(&s)).collect();
    //line that looks like `your ticket:`
    let rules = rules?;
    lines.next();
    let my_ticket: Vec<u64> = lines
        .next()
        .unwrap()
        .split(",")
        .map(|n| n.parse().unwrap())
        .collect();
    // blank line
    lines.next();
    // line that looks like `nearby ticket:`
    lines.next();
    let nearby_tickets: Vec<Vec<u64>> = lines
        .map(|line| line.split(",").map(|n| n.parse().unwrap()).collect())
        .collect();
    let error_rate: u64 = nearby_tickets
        .into_iter()
        .map(|ticket| -> Vec<u64> {
            ticket
                .iter()
                .cloned()
                .filter(|n| rules.iter().all(|rule| !rule.valid(n)))
                .collect()
        })
        .flatten()
        .sum();
    dbg!(error_rate);
    Ok(())
}

#[derive(Debug, Clone)]
struct Rule {
    name: String,
    ranges: [RangeInclusive<u64>; 2],
}

#[derive(Debug, Clone)]
struct StrError(String);
impl Error for StrError {}
impl fmt::Display for StrError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Rule {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(":");
        let name: String = iter.next().unwrap().into();
        let mut ranges = iter.next().unwrap().split_whitespace();
        let r1 = parse_range(ranges.next().unwrap());
        ranges.next();
        let r2 = parse_range(ranges.next().unwrap());
        let ranges = [r1, r2];
        Ok(Rule { name, ranges })
    }
}

impl Rule {
    fn valid(&self, n: &u64) -> bool {
        self.ranges.iter().any(|r| n >= r.start() && n <= r.end())
    }
}

fn parse_range(s: &str) -> RangeInclusive<u64> {
    let mut iter = s.split("-");
    let low: u64 = iter.next().unwrap().parse().unwrap();
    let high: u64 = iter.next().unwrap().parse().unwrap();
    low..=high
}
