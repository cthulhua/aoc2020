use im_rc::{HashMap, Vector};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let max: u32 = std::env::args().nth(2).unwrap().parse().unwrap();
    let rounds: u32 = std::env::args().nth(3).unwrap().parse().unwrap();
    let additional = 10u32..=max;
    let mut cups: Vector<u32> = std::env::args()
        .nth(1)
        .unwrap()
        .bytes()
        .map(|b| (b - 48u8) as u32)
        .chain(additional)
        .collect();
    let mut cache = Cache::new();
    let mut current_cup_index = 0usize;
    for _ in 0..rounds {
        let (picked_up_cups, pickup_adjustment) = pick_up(current_cup_index, &mut cups);
        current_cup_index -= pickup_adjustment;
        let target_cup_index =
            get_target_cup(current_cup_index, &cups, &picked_up_cups, &mut cache, max);
        let tup = put_down_cups(
            current_cup_index,
            target_cup_index,
            cups.clone(),
            picked_up_cups,
            &mut cache,
        );
        cups = tup.0;
        current_cup_index = (tup.1 + 1) % cups.len();
    }
    cache.stats();
    Ok(())
}

struct Cache {
    inner: HashMap<u32, usize>,
    writes: usize,
    misses: usize,
    hits: usize,
    false_hits: usize,
}

impl Cache {
    fn new() -> Self {
        let inner: HashMap<u32, usize> = HashMap::new();
        Self {
            inner,
            writes: 0,
            misses: 0,
            hits: 0,
            false_hits: 0,
        }
    }
    fn get(&mut self, value: &u32) -> Option<&usize> {
        match self.inner.get(value) {
            None => {
                self.misses += 1;
                None
            }
            Some(index) => {
                self.hits += 1;
                Some(index)
            }
        }
    }

    fn insert(&mut self, value: u32, index: usize) {
        self.writes += 1;
        self.inner.insert(value, index);
    }
    fn stats(&self) {
        dbg!(self.writes);
        dbg!(self.hits);
        dbg!(self.false_hits);
        dbg!(self.misses);
    }
    fn false_hit(&mut self) {
        self.false_hits += 1;
    }
}

fn pick_up(current_cup_index: usize, cups: &mut Vector<u32>) -> (Vector<u32>, usize) {
    let wrapped_index = (current_cup_index + 1).min(cups.len());
    let mut picked_up_cups = cups.slice(wrapped_index..(wrapped_index + 3).min(cups.len()));
    let wrapped_index = match picked_up_cups.len() {
        3 => 0,
        2 => 1,
        1 => 2,
        0 => 3,
        _ => panic!("unreachable"),
    };
    picked_up_cups.append(cups.slice(0..wrapped_index));
    (picked_up_cups, wrapped_index)
}

fn get_target_cup(
    current_cup_index: usize,
    cups: &Vector<u32>,
    picked_up_cups: &Vector<u32>,
    cache: &mut Cache,
    max: u32,
) -> usize {
    let mut target_cup_value = cups[current_cup_index];
    if target_cup_value == 1 {
        target_cup_value = max_value(picked_up_cups, max);
    } else {
        target_cup_value -= 1;
    }
    while picked_up_cups.contains(&(target_cup_value as u32)) {
        if target_cup_value == 1 {
            target_cup_value = max_value(picked_up_cups, max);
        } else {
            target_cup_value -= 1;
        }
    }
    if let Some(cached_index) = cache.get(&target_cup_value) {
        //need to be careful here, as when we look this up in cups, in this method, we've already
        //picked some up. possible solution is to do some time tracking, and use that to inform our
        //search, rather than using index_of. because we always update the cache when we put them
        //down, we know that in the event of a cache miss in round n, if the cache was inserted in
        //prev round m, the real location must be within 3 * (n - m) of the last known location,
        //since in each round the most we can do is insert/remove 3 values
        if cups[*cached_index] == target_cup_value {
            *cached_index
        } else {
            cache.false_hit();
            let target_cup_index = cups.index_of(&target_cup_value).unwrap();
            cache.insert(target_cup_value, target_cup_index);
            target_cup_index
        }
    } else {
        let target_cup_index = cups.index_of(&target_cup_value).unwrap();
        cache.insert(target_cup_value, target_cup_index);
        target_cup_index
    }
}

fn max_value(picked_up_cups: &Vector<u32>, max: u32) -> u32 {
    (max - 4..=max)
        .filter(|m| !picked_up_cups.contains(m))
        .max()
        .unwrap()
}

fn put_down_cups(
    current_cup_index: usize,
    target_cup_index: usize,
    cups: Vector<u32>,
    picked_up_cups: Vector<u32>,
    cache: &mut Cache,
) -> (Vector<u32>, usize) {
    let wrapped_target_cup_index = (target_cup_index + 1) % (cups.len());
    let (mut lh, rh) = cups.split_at(wrapped_target_cup_index);
    // here we're caching where the cups will be when we put them down
    picked_up_cups.iter().enumerate().for_each(|(idx, val)| {
        cache.insert(*val, lh.len() + idx);
    });
    lh.append(picked_up_cups);
    lh.append(rh);
    (
        lh,
        if wrapped_target_cup_index <= current_cup_index {
            current_cup_index + 3 //TODO this might not always be 3
        } else {
            current_cup_index
        },
    )
}
