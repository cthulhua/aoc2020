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
    let mut cache: HashMap<u32, usize> = HashMap::new();
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
    Ok(())
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
    cache: &mut HashMap<u32, usize>,
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
        if cups[*cached_index] == target_cup_value {
            *cached_index
        } else {
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
    cache: &mut HashMap<u32, usize>,
) -> (Vector<u32>, usize) {
    let wrapped_target_cup_index = (target_cup_index + 1) % (cups.len());
    let (mut lh, rh) = cups.split_at(wrapped_target_cup_index);
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
