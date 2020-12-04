use std::fs::File;
use std::io::{BufRead, BufReader};
const TREE: u8 = 35u8;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = File::open("input.txt")?;
    let buffered = BufReader::new(input);
    let map: Vec<Vec<u8>> = buffered
        .lines()
        .map(|s| Vec::from(s.unwrap().as_bytes()))
        .collect();
    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let product = slopes
        .into_iter()
        .map(|(x, y)| {
            let s = Slope::new(&map, x, y);
            s.filter(|t| *t == TREE).count()
        })
        .fold(1, |acc, element| acc * element);
    dbg!(product);
    Ok(())
}

fn get_terrain(map: &[Vec<u8>], x: usize, y: usize) -> Option<u8> {
    if y >= map.len() {
        None
    } else {
        let wrapped_x = x % map[0].len();
        Some(map[y][wrapped_x])
    }
}

struct Slope<'a> {
    map: &'a [Vec<u8>],
    run: usize,
    fall: usize,
    x: usize,
    y: usize,
}

impl<'a> Slope<'a> {
    fn new(map: &'a [Vec<u8>], run: usize, fall: usize) -> Self {
        Self {
            map,
            run,
            fall,
            x: 0,
            y: 0,
        }
    }
}

impl<'a> Iterator for Slope<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let cur = get_terrain(self.map, self.x, self.y);
        self.x += self.run;
        self.y += self.fall;
        return cur;
    }
}
