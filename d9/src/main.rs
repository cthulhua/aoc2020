use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = File::open("input.txt")?;
    let buffered = BufReader::new(input);
    let nums: Vec<i64> = buffered
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();
    let dec = XmasDec { nums };
    let first = dec.first_break();
    dbg!(dec.find_weakness(first));
    Ok(())
}

struct XmasDec {
    nums: Vec<i64>,
}

static PREAMBLE_SIZE: usize = 25;
impl XmasDec {
    fn first_break(&self) -> i64 {
        for i in 0..(self.nums.len() - PREAMBLE_SIZE - 1) {
            let valid = XmasDec::validate(
                &self.nums[i..(i + PREAMBLE_SIZE)],
                self.nums[i + PREAMBLE_SIZE],
            );
            if !valid {
                return self.nums[i + PREAMBLE_SIZE];
            }
        }
        0
    }
    fn validate(preamble: &[i64], n: i64) -> bool {
        let differences: HashSet<i64> = preamble.iter().map(|x| n - x).collect();
        let original: HashSet<i64> = preamble.iter().map(Clone::clone).collect();
        differences.intersection(&original).any(|_| true)
    }
    fn find_weakness(&self, target: i64) -> i64 {
        for sequence_length in 2..self.nums.len() {
            for i in 0..=self.nums.len() - sequence_length {
                if let Ok(weakness) =
                    XmasDec::extract_weakness(&self.nums[i..(i + sequence_length)], target)
                {
                    return weakness;
                }
            }
        }
        0
    }
    fn extract_weakness(slice: &[i64], target: i64) -> Result<i64, ()> {
        let sum = slice.iter().sum::<i64>();
        if sum == target {
            let min = slice.iter().min().unwrap();
            let max = slice.iter().max().unwrap();
            return Ok(min + max);
        }
        Err(())
    }
}
