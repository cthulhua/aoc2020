use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = File::open("input.txt")?;
    let lines = BufReader::new(input).lines();
    let mut buf = String::new();
    let mut sum = 0;
    for line in lines {
        match line {
            Ok(s) => {
                if s.trim().is_empty() {
                    sum += count_questions(&buf);
                    buf.clear();
                } else {
                    buf.push(' ');
                    buf.push_str(&s);
                }
            }
            Err(_) => panic!("oh shit dog"),
        }
    }
    sum += count_questions(&buf);
    dbg!(sum);
    Ok(())
}

fn count_questions(buf: &str) -> usize {
    let sets: Vec<HashSet<char>> = buf
        .split_whitespace()
        .map(|response| response.chars().collect())
        .collect();
    let intersection = sets[0].clone();
    sets.into_iter()
        .fold(intersection, |acc, element| {
            acc.intersection(&element).cloned().collect()
        })
        .len()
}
