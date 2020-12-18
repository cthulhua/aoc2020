use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn Error>> {
    let filename = std::env::args().nth(1).unwrap();
    let input = File::open(filename)?;
    let buffered = BufReader::new(input);
    let v: Vec<Vec<u8>> = buffered.lines().map(|r| r.unwrap().into_bytes()).collect();
    let space = Space::seed(v)
        .next_cycle()
        .next_cycle()
        .next_cycle()
        .next_cycle()
        .next_cycle()
        .next_cycle();
    dbg!(space.cubes.len());
    Ok(())
}

struct Space {
    cubes: HashSet<(i64, i64, i64)>,
    x_index: HashMap<i64, HashSet<(i64, i64)>>,
    y_index: HashMap<i64, HashSet<(i64, i64)>>,
    z_index: HashMap<i64, HashSet<(i64, i64)>>,
}

impl Space {
    fn seed(initial_state: Vec<Vec<u8>>) -> Self {
        let mut new_space = Space {
            cubes: HashSet::new(),
            x_index: HashMap::new(),
            y_index: HashMap::new(),
            z_index: HashMap::new(),
        };
        for i in 0..initial_state.len() {
            for j in 0..initial_state[0].len() {
                if initial_state[i][j] == b'#' {
                    new_space.add(&(i as i64, j as i64, 0));
                }
            }
        }
        new_space
    }
    fn next_cycle(self) -> Self {
        let mut new_space = Space {
            cubes: HashSet::new(),
            x_index: HashMap::new(),
            y_index: HashMap::new(),
            z_index: HashMap::new(),
        };
        let empty_neighbors: HashSet<(i64, i64, i64)> = self
            .cubes
            .iter()
            .map(|point| Self::neighbors(point).into_iter())
            .flatten()
            .filter(|point| !self.cubes.contains(point))
            .collect();
        self.cubes.iter().for_each(|point| {
            let occupied_neighbors = Self::neighbors(point)
                .iter()
                .filter(|neighbor_point| self.cubes.contains(neighbor_point))
                .count();
            if occupied_neighbors == 2 || occupied_neighbors == 3 {
                new_space.add(point);
            }
        });
        empty_neighbors.iter().for_each(|point| {
            let occupied_neighbors = Self::neighbors(point)
                .iter()
                .filter(|neighbor_point| self.cubes.contains(neighbor_point))
                .count();
            if occupied_neighbors == 3 {
                new_space.add(point);
            }
        });
        new_space
    }
    fn add(&mut self, point: &(i64, i64, i64)) {
        self.cubes.insert(*point);

        let (x, y, z) = point;
        if !self.x_index.contains_key(x) {
            self.x_index.insert(*x, HashSet::new());
        }
        let x_index = self.x_index.get_mut(x).unwrap();
        x_index.insert((*y, *z));

        if !self.y_index.contains_key(y) {
            self.y_index.insert(*y, HashSet::new());
        }
        let y_index = self.y_index.get_mut(y).unwrap();
        y_index.insert((*x, *z));

        if !self.z_index.contains_key(z) {
            self.z_index.insert(*z, HashSet::new());
        }
        let z_index = self.z_index.get_mut(z).unwrap();
        z_index.insert((*x, *y));
    }

    fn remove(&mut self, point: &(i64, i64, i64)) {
        self.cubes.remove(point);
        let (x, y, z) = point;

        let x_index = self.x_index.get_mut(x).unwrap();
        x_index.remove(&(*y, *z));

        let y_index = self.y_index.get_mut(y).unwrap();
        y_index.remove(&(*x, *z));

        let z_index = self.z_index.get_mut(z).unwrap();
        z_index.remove(&(*x, *y));
    }

    fn neighbors(point: &(i64, i64, i64)) -> HashSet<(i64, i64, i64)> {
        let mut n = HashSet::new();
        let (x, y, z) = point;
        for i in x - 1..=x + 1 {
            for j in y - 1..=y + 1 {
                for k in z - 1..=z + 1 {
                    if (i, j, k) != *point {
                        n.insert((i, j, k));
                    }
                }
            }
        }
        n
    }
}
