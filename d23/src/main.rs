use im_rc::Vector;
use std::error::Error;
fn main() -> Result<(), Box<dyn Error>> {
    let mut cups: Vector<u8> = std::env::args()
        .nth(1)
        .unwrap()
        .bytes()
        .map(|b| b - 48u8)
        .collect();
    let mut current_cup_index = 0usize;
    for _ in 0..100 {
        let (picked_up_cups, pickup_adjustment) = pick_up(current_cup_index, &mut cups);
        current_cup_index -= pickup_adjustment;
        let target_cup_index = get_target_cup(current_cup_index, &cups);
        let tup = put_down_cups(
            current_cup_index,
            target_cup_index,
            cups.clone(),
            picked_up_cups,
        );
        cups = tup.0;
        current_cup_index = (tup.1 + 1) % cups.len();
    }
    dbg!(&cups);
    Ok(())
}

fn pick_up(current_cup_index: usize, cups: &mut Vector<u8>) -> (Vector<u8>, usize) {
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

fn get_target_cup(current_cup_index: usize, cups: &Vector<u8>) -> usize {
    let mut target_cup_value = cups[current_cup_index];
    if target_cup_value == 0 {
        target_cup_value = max_value(cups);
    } else {
        target_cup_value -= 1;
    }
    while !cups.contains(&(target_cup_value as u8)) {
        if target_cup_value == 0 {
            target_cup_value = max_value(cups);
        } else {
            target_cup_value -= 1;
        }
    }
    cups.index_of(&target_cup_value).unwrap()
}

fn max_value(cups: &Vector<u8>) -> u8 {
    *cups.iter().max().unwrap()
}

fn put_down_cups(
    current_cup_index: usize,
    target_cup_index: usize,
    cups: Vector<u8>,
    picked_up_cups: Vector<u8>,
) -> (Vector<u8>, usize) {
    let wrapped_target_cup_index = (target_cup_index + 1) % 6;
    let (mut lh, rh) = cups.split_at(wrapped_target_cup_index); //TODO might not always be 6
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
