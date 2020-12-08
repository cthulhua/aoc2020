use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = File::open("input.txt")?;
    let buffered = BufReader::new(input);
    let lines: Vec<String> = buffered.lines().into_iter().map(Result::unwrap).collect();
    let h = BagContents::from_lines(&lines);
    dbg!(count_bags(&h, "shiny gold") - 1);
    Ok(())
}

fn count_bags(hash: &HashMap<String, BagContents>, bag: &str) -> usize {
    match hash.get(bag.clone()) {
        Some(contents) => {
            let count: usize = contents
                .inners
                .iter()
                .map(|(number, inner)| {
                    let count = count_bags(hash, inner);
                    number * count
                })
                .sum();
            count + 1usize
        }
        None => 1,
    }
}

#[derive(Debug)]
struct BagContents {
    inners: Vec<(usize, String)>,
}

impl BagContents {
    fn from_lines(lines: &[String]) -> HashMap<String, BagContents> {
        let mut h: HashMap<String, BagContents> = HashMap::new();
        lines.iter().for_each(|line| {
            let mut iter = line.split("contain");
            let v: Vec<String> = iter
                .next()
                .unwrap()
                .split_whitespace()
                .take(2)
                .map(str::to_owned)
                .collect();
            let desc = v.join(" ");
            let contents = iter.next().map(|s| Self::from_string(s.trim()));
            h.insert(desc, contents.unwrap());
        });
        h
    }
    fn from_string(contents: &str) -> Self {
        let inners = contents
            .split("bag")
            .filter(|substring| substring.len() > 2 && substring.trim() != "no other")
            .map(|substr| {
                let mut words = substr.split_whitespace().peekable();
                while words.peek().unwrap().parse::<usize>().is_err() {
                    words.next();
                }
                let quantity = words.next().unwrap().parse::<usize>().unwrap();
                let v: Vec<String> = words.take(2).map(str::to_owned).collect();
                let bag = v.join(" ");
                (quantity, bag)
            })
            .collect();
        BagContents { inners }
    }
}
