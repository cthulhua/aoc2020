use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = File::open("input.txt")?;
    let mut buffered = BufReader::new(input);
    let mut buf = String::new();
    buffered.read_line(&mut buf)?;
    let nums: Vec<i64> = buf
        .split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect();
    let h: HashSet<i64> = nums.clone().into_iter().map(|x| 2020 - x).collect();
    let mut v: Vec<_> = h
        .iter()
        .map(|m| {
            let g: HashSet<i64> = nums.clone().into_iter().collect();
            let answer: Vec<i64> = nums
                .clone()
                .into_iter()
                .filter(|x| g.contains(&(m - x)))
                .collect();
            answer
        })
        .flatten()
        .collect();
    v.sort();
    v.dedup();
    dbg!(v.into_iter().fold(1, |acc, element| acc * element));
    Ok(())
}
