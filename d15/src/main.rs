use std::collections::HashMap;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let nums: Vec<u64> = std::env::args()
        .nth(1)
        .unwrap()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();
    let mut elves = Elves::new(nums);
    // let n: Vec<u64> = elves.take(2020).collect();
    let n = elves.nth(2019).unwrap();
    dbg!(n);
    Ok(())
}

#[derive(Debug)]
struct Elves {
    n: usize,
    initial: Vec<u64>,
    history: HashMap<u64, u64>,
    prev: Option<u64>,
}

impl Elves {
    fn new(initial: Vec<u64>) -> Self {
        Self {
            n: 1,
            initial,
            history: HashMap::new(),
            prev: None,
        }
    }
}

impl Iterator for Elves {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        let res = if self.n <= self.initial.len() {
            let num = self.initial[self.n - 1];
            match self.prev {
                Some(prev) => {
                    self.history.insert(prev, (self.n as u64) - 1);
                }
                None => (),
            }
            self.prev = Some(num);

            Some(num)
        } else {
            let prev = &self.prev.unwrap();
            match self.history.get_mut(prev) {
                Some(timestamp) => {
                    //seen it before
                    let age = (self.n - 1) as u64 - *timestamp;
                    self.history.insert(*prev, (self.n as u64) - 1);
                    Some(age)
                }
                None => {
                    //new number
                    // record it
                    self.history.insert(*prev, (self.n as u64) - 1);
                    Some(0)
                }
            }
        };
        self.n += 1;
        self.prev = res;
        res
    }
}
