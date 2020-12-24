use std::error::Error;
mod list;
use list::List;

fn main() -> Result<(), Box<dyn Error>> {
    let max: usize = std::env::args().nth(2).unwrap().parse().unwrap();
    let rounds: usize = std::env::args().nth(3).unwrap().parse().unwrap();
    let additional = 10usize..=max;
    let mut cups: List<usize> = std::env::args()
        .nth(1)
        .unwrap()
        .bytes()
        .map(|b| (b - 48u8) as usize)
        .chain(additional)
        .collect();
    let cup_index: Vec<usize> = build_index(&cups);
    let mut current_cup_index = cups.head;
    for _ in 0..rounds {
        let picked_up_cups = pick_up(current_cup_index, &mut cups);
        let target_cup_index =
            get_target_cup(current_cup_index, &cups, &picked_up_cups, &cup_index, max);
        // let tup = put_down_cups(current_cup_index, target_cup_index, cups, picked_up_cups);
        cups.add_fragment(target_cup_index, picked_up_cups.0, picked_up_cups.1);
        current_cup_index = cups.get_node(current_cup_index).next;
    }
    let node1 = cups.get_node(cup_index[1]);
    let node2 = cups.get_node(node1.next);
    let node3 = cups.get_node(node2.next);
    dbg!(node2.value * node3.value);
    Ok(())
}

fn pick_up(current_cup_index: usize, cups: &mut List<usize>) -> (usize, usize) {
    cups.remove_next_n(current_cup_index, 3)
}

fn get_target_cup(
    current_cup_index: usize,
    cups: &List<usize>,
    picked_up_cups: &(usize, usize),
    cup_index: &Vec<usize>,
    max: usize,
) -> usize {
    let mut target_cup_value = cups.get_node(current_cup_index).value;
    if target_cup_value == 1 {
        target_cup_value = max_value(picked_up_cups, cups, max);
    } else {
        target_cup_value -= 1;
    }
    while cups.fragment_contains(picked_up_cups.0, picked_up_cups.1, &target_cup_value) {
        if target_cup_value == 1 {
            target_cup_value = max_value(picked_up_cups, cups, max);
        } else {
            target_cup_value -= 1;
        }
    }
    cup_index[target_cup_value]
}

fn max_value(fragment: &(usize, usize), cups: &List<usize>, max: usize) -> usize {
    (max - 4..=max)
        .filter(|m| !cups.fragment_contains(fragment.0, fragment.1, m))
        .max()
        .unwrap()
}

fn build_index(cups: &List<usize>) -> Vec<usize> {
    let mut index: Vec<usize> = vec![0; 1_000_0001];
    for (arena_idx, node) in cups.arena.iter() {
        let idx = node.value;
        let dest = index.get_mut(idx).unwrap();
        *dest = arena_idx;
    }
    index
}
