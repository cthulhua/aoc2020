#[macro_use]
extern crate pest_derive;

use pest::iterators::Pairs;
use pest::Parser;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Parser)]
#[grammar = "hex.pest"]
struct HexParser;

#[derive(Debug, Clone)]
struct Hex((i32, i32, i32));

enum Direction {
    East,
    Southeast,
    Southwest,
    West,
    Northwest,
    Northeast,
}

static DIRECTIONS: [&Direction; 6] = [
    &Direction::East,
    &Direction::Southeast,
    &Direction::Southwest,
    &Direction::West,
    &Direction::Northwest,
    &Direction::Northeast,
];

impl Hex {
    fn new() -> Hex {
        Hex((0, 0, 0))
    }

    fn into_tuple(self) -> (i32, i32, i32) {
        self.0
    }

    fn from_tuple(tup: &(i32, i32, i32)) -> Self {
        Self(*tup)
    }

    fn neighbors(&self) -> Box<dyn Iterator<Item = Self> + '_> {
        Box::new(DIRECTIONS.iter().map(move |direction| {
            let mut new = self.clone();
            new.go(direction);
            new
        }))
    }

    fn go(&mut self, direction: &Direction) {
        match direction {
            Direction::East => {
                self.0 .0 += 1;
                self.0 .1 -= 1;
            }
            Direction::Southeast => {
                self.0 .2 += 1;
                self.0 .1 -= 1;
            }
            Direction::Southwest => {
                self.0 .0 -= 1;
                self.0 .2 += 1;
            }
            Direction::West => {
                self.0 .0 -= 1;
                self.0 .1 += 1;
            }
            Direction::Northwest => {
                self.0 .2 -= 1;
                self.0 .1 += 1;
            }
            Direction::Northeast => {
                self.0 .0 += 1;
                self.0 .2 -= 1;
            }
        }
    }
}

type State = HashMap<(i32, i32, i32), bool>;

fn main() -> Result<(), Box<dyn Error>> {
    let filename = std::env::args().nth(1).unwrap();
    let input = File::open(filename)?;
    let buffered = BufReader::new(input);
    let mut black_tiles: State = HashMap::new();
    buffered
        .lines()
        .map(|r| eval(HexParser::parse(Rule::hex, &r.unwrap()).unwrap()))
        .for_each(|hex| {
            let tup = hex.into_tuple();
            if let Some(is_black) = black_tiles.get_mut(&tup) {
                *is_black = !*is_black;
            } else {
                black_tiles.insert(tup, true);
            }
        });
    for _ in 0..100 {
        black_tiles = next_state(black_tiles);
    }

    let count = black_tiles
        .into_iter()
        .filter(|(_, is_black)| *is_black)
        .count();
    dbg!(&count);
    Ok(())
}

fn next_state(state: State) -> State {
    let mut new_hexes: HashSet<(i32, i32, i32)> = HashSet::new();
    let mut new_state = HashMap::new();
    for (tup, is_black) in state.iter() {
        let hex = Hex::from_tuple(&tup);
        let neighbors: HashSet<(i32, i32, i32)> = hex.neighbors().map(Hex::into_tuple).collect();
        let black_neighbors = neighbors
            .iter()
            .filter_map(|neighbor_tup| match &state.get(&neighbor_tup) {
                None => None,
                Some(true) => Some(true),
                Some(false) => None,
            })
            .count();
        if *is_black {
            if black_neighbors == 1 || black_neighbors == 2 {
                new_state.insert(*tup, true);
            }
        } else {
            if black_neighbors == 2 {
                new_state.insert(*tup, true);
            }
        }
        new_hexes = new_hexes.union(&neighbors).cloned().collect();
    }
    new_hexes = new_hexes
        .difference(&state.keys().cloned().collect())
        .cloned()
        .collect();
    assert!(
        new_hexes
            .clone()
            .intersection(&state.keys().cloned().collect())
            .count()
            == 0
    );
    for tup in new_hexes {
        let hex = Hex::from_tuple(&tup);
        let neighbors: HashSet<(i32, i32, i32)> = hex.neighbors().map(Hex::into_tuple).collect();
        let black_neighbors = neighbors
            .iter()
            .filter_map(|tup| match &state.get(&tup) {
                None => None,
                Some(true) => Some(true),
                Some(false) => None,
            })
            .count();
        if black_neighbors == 2 {
            new_state.insert(tup, true);
        }
    }
    new_state
}

fn eval(hex: Pairs<Rule>) -> Hex {
    let mut result = Hex::new();
    hex.for_each(|p| {
        (p.into_inner()).for_each(|r| match r.as_rule() {
            Rule::east => {
                result.go(&Direction::East);
            }
            Rule::southeast => {
                result.go(&Direction::Southeast);
            }
            Rule::southwest => {
                result.go(&Direction::Southwest);
            }
            Rule::west => {
                result.go(&Direction::West);
            }
            Rule::northwest => {
                result.go(&Direction::Northwest);
            }
            Rule::northeast => {
                result.go(&Direction::Northeast);
            }
            _ => (panic!("unexpected rule")),
        });
    });
    result
}
