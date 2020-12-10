use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = std::env::args().nth(1).unwrap();
    let input = File::open(file)?;
    let buffered = BufReader::new(input);
    let mut nums: Vec<u8> = buffered
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();
    nums.sort();
    let mut threes = 1;
    let mut ones = 0;
    for i in 1..nums.len() {
        if nums[i] - nums[i - 1] == 3 {
            threes += 1;
        } else if nums[i] - nums[i - 1] == 1 {
            ones += 1;
        }
    }
    if nums[0] == 3 {
        threes += 1;
    } else if nums[0] == 1 {
        ones += 1;
    }
    dbg!(threes * ones);

    dbg!(trib_path(&nums));
    let initial = 0;
    let target = nums.last().unwrap() + 3;
    dbg!(target);
    let count = find_path(&nums, initial, target);
    dbg!(count);
    Ok(())
}

fn find_path(adapters: &[u8], initial: u8, target: u8) -> usize {
    // dbg!(initial);
    if target - initial <= 3 {
        // dbg!("found one");
        1
    } else {
        // dbg!(adapters);
        let num_candidates: usize = adapters
            .iter()
            .take_while(|candidate| **candidate - initial <= 3)
            .count();
        // dbg!(num_candidates);
        let mut total = 0;
        adapters[0..num_candidates]
            .into_iter()
            .enumerate()
            .for_each(|(idx, candidate)| {
                // dbg!(initial);
                // dbg!(candidate);
                // dbg!(&adapters[1 + idx..]);
                let count = find_path(&adapters[1 + idx..], *candidate, target);
                total += count;
            });
        total
    }
}
fn trib(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        2 => 1,
        n => trib(n - 3) + trib(n - 2) + trib(n - 1),
    }
}

fn trib_path(adapters: &[u8]) -> u64 {
    let diffs = adapters.iter().scan(0u8, |prev, &cur| {
        let diff = Some(cur - *prev);
        *prev = cur;
        diff
    });
    let mut lengths = VecDeque::new();
    lengths.push_back(1);
    diffs.fold(&mut lengths, |lengths, cur| {
        if cur == 1 {
            let back = lengths.back_mut().unwrap();
            *back += 1;
            lengths
        } else {
            lengths.push_back(1);
            lengths
        }
    });
    lengths
        .into_iter()
        .fold(1, |product, cur| product * trib(cur))
}
