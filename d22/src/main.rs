use im_rc::Vector;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
fn main() -> Result<(), Box<dyn Error>> {
    let filename = std::env::args().nth(1).unwrap();
    let input = File::open(filename)?;
    let buffered = BufReader::new(input);
    let players: Vec<Player> = read_players(buffered);
    dbg!(recursive_play(&players[0], &players[1]));
    Ok(())
}

#[derive(Debug)]
struct Player {
    id: u64,
    deck: Vector<u64>,
}
impl Player {}

fn read_players(mut buffered: BufReader<File>) -> Vec<Player> {
    let mut players = vec![];
    let mut buf = String::new();
    let mut id: u64 = 0;
    let mut deck: Vector<u64> = Vector::new();
    while buffered.read_line(&mut buf).unwrap() != 0 {
        if buf.trim().is_empty() {
            let player = Player {
                id,
                deck: deck.clone(),
            };
            players.push(player);
            id = 0;
            deck.clear();
        } else {
            if buf.starts_with("Player") {
                let s = buf.split_whitespace().nth(1).unwrap();
                id = s[..s.len() - 1].parse().unwrap();
            } else {
                let card: u64 = buf.trim().parse().unwrap();
                deck.push_back(card);
            }
        }
        buf.clear();
    }
    players
}

fn play(player1: &Player, player2: &Player) -> u64 {
    let mut deck1 = player1.deck.clone();
    let mut deck2 = player2.deck.clone();
    while !(deck1.is_empty() || deck2.is_empty()) {
        let top1 = deck1.pop_front();
        let top2 = deck2.pop_front();
        if top1 > top2 {
            deck1.push_back(top1.unwrap());
            deck1.push_back(top2.unwrap());
        } else {
            deck2.push_back(top2.unwrap());
            deck2.push_back(top1.unwrap());
        }
    }
    let winner_deck = if deck1.len() > deck2.len() {
        deck1
    } else {
        deck2
    };
    winner_deck
        .iter()
        .enumerate()
        .map(|(i, v)| (winner_deck.len() - i) as u64 * v)
        .sum()
}

fn recursive_play(player1: &Player, player2: &Player) -> u64 {
    let mut deck1 = player1.deck.clone();
    let mut deck2 = player2.deck.clone();
    let mut previous_states: HashSet<(Vector<u64>, Vector<u64>)> = HashSet::new();
    let mut winner_cache: HashMap<(Vector<u64>, Vector<u64>), i32> = HashMap::new();
    let mut winner = 0;
    while !(deck1.is_empty() || deck2.is_empty()) {
        let state = (deck1.clone(), deck2.clone());

        if previous_states.contains(&state) {
            winner = 1;
            break;
        }
        previous_states.insert(state);
        let top1 = deck1.pop_front().unwrap();
        let top2 = deck2.pop_front().unwrap();

        if deck1.len() as u64 >= top1 && deck2.len() as u64 >= top2 {
            winner = if let Some(past_winner) = winner_cache.get(&(deck1.clone(), deck2.clone())) {
                *past_winner
            } else {
                recursive_play_helper(
                    deck1.clone().slice(..top1 as usize),
                    deck2.clone().slice(..top2 as usize),
                    &mut winner_cache,
                )
            };
            winner_cache.insert((deck1.clone(), deck2.clone()), winner);
        } else {
            if top1 > top2 {
                winner = 1;
            } else {
                winner = 2;
            }
        }
        if winner == 1 {
            deck1.push_back(top1);
            deck1.push_back(top2);
        } else {
            deck2.push_back(top2);
            deck2.push_back(top1);
        }
    }
    let winner_deck = if winner == 1 { deck1 } else { deck2 };
    winner_deck
        .iter()
        .enumerate()
        .map(|(i, v)| (winner_deck.len() - i) as u64 * v)
        .sum()
}

fn recursive_play_helper(
    mut deck1: Vector<u64>,
    mut deck2: Vector<u64>,
    mut winner_cache: &mut HashMap<(Vector<u64>, Vector<u64>), i32>,
) -> i32 {
    let mut previous_states: HashSet<(Vector<u64>, Vector<u64>)> = HashSet::new();
    let mut winner = 0;
    while !(deck1.is_empty() || deck2.is_empty()) {
        let state = (deck1.clone(), deck2.clone());
        if previous_states.contains(&state) {
            winner = 1;
            break;
        }
        previous_states.insert(state);
        let top1 = deck1.pop_front().unwrap();
        let top2 = deck2.pop_front().unwrap();

        if deck1.len() as u64 >= top1 && deck2.len() as u64 >= top2 {
            winner = if let Some(past_winner) = winner_cache.get(&(deck1.clone(), deck2.clone())) {
                *past_winner
            } else {
                recursive_play_helper(
                    deck1.clone().slice(..top1 as usize),
                    deck2.clone().slice(..top2 as usize),
                    &mut winner_cache,
                )
            };
            winner_cache.insert((deck1.clone(), deck2.clone()), winner);
        } else {
            if top1 > top2 {
                winner = 1;
            } else {
                winner = 2;
            }
        }
        if winner == 1 {
            deck1.push_back(top1);
            deck1.push_back(top2);
        } else {
            deck2.push_back(top2);
            deck2.push_back(top1);
        }
    }
    winner
}
