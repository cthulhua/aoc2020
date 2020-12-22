use std::collections::VecDeque;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
fn main() -> Result<(), Box<dyn Error>> {
    let filename = std::env::args().nth(1).unwrap();
    let input = File::open(filename)?;
    let buffered = BufReader::new(input);
    let players: Vec<Player> = read_players(buffered);
    dbg!(play(&players[0], &players[1]));
    Ok(())
}

#[derive(Debug)]
struct Player {
    id: u64,
    deck: VecDeque<u64>,
}
impl Player {}

fn read_players(mut buffered: BufReader<File>) -> Vec<Player> {
    let mut players = vec![];
    let mut buf = String::new();
    let mut id: u64 = 0;
    let mut deck: VecDeque<u64> = VecDeque::new();
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
