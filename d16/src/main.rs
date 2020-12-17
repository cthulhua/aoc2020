use std::collections::HashMap;
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
    let valid_nearby_tickets: Vec<Vec<u64>> = nearby_tickets
        .into_iter()
        .filter(|ticket| {
            ticket
                .iter()
                .all(|n| rules.iter().any(|rule| rule.valid(n)))
        })
        .collect();
    let ticket_len = my_ticket.len();
    assert!(valid_nearby_tickets
        .iter()
        .all(|ticket| ticket.len() == ticket_len));
    let mut possibilities: Vec<(usize, Vec<&Rule>)> = (0..ticket_len)
        .map(|i| {
            rules
                .iter()
                .filter(|rule| {
                    valid_nearby_tickets
                        .iter()
                        .all(|ticket| rule.valid(&ticket[i]))
                })
                .collect()
        })
        .enumerate()
        .collect();
    let mut known_fields: Vec<(usize, &Rule)> = vec![];
    while !possibilities.is_empty() {
        let mut newly_known_fields: Vec<(usize, &Rule)> = possibilities
            .iter()
            .filter_map(|(i, rules)| {
                if rules.len() == 1 {
                    Some((*i, rules[0]))
                } else {
                    None
                }
            })
            .collect();
        let rules_to_delete: Vec<Rule> = newly_known_fields
            .iter()
            .map(|(_, rule)| rule.clone())
            .cloned()
            .collect();
        possibilities = possibilities
            .into_iter()
            .filter_map({
                |(i, rules)| {
                    if rules.len() >= 1 {
                        Some((
                            i,
                            rules
                                .iter()
                                .filter(|rule| !rules_to_delete.contains(rule))
                                .cloned()
                                .collect(),
                        ))
                    } else {
                        None
                    }
                }
            })
            .collect();

        known_fields.append(&mut newly_known_fields);
    }
    let decoder: HashMap<usize, Rule> = known_fields
        .into_iter()
        .map(|(i, rule)| (i, rule.clone()))
        .collect();
    let decoded_ticket: HashMap<String, u64> = my_ticket
        .into_iter()
        .enumerate()
        .map(|(i, field)| (decoder.get(&i).unwrap().name.clone(), field))
        .collect();
    println!("{:#?}", decoded_ticket);
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
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
