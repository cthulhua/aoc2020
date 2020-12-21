#![allow(dead_code)]
#[macro_use]
extern crate ndarray;
use ndarray::{Array1, Array2, ArrayView1};
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
fn main() -> Result<(), Box<dyn Error>> {
    let filename = std::env::args().nth(1).unwrap();
    let input = File::open(filename)?;
    let buffered = BufReader::new(input);
    let tiles: Vec<Tile> = read_tiles(buffered);
    let tiles_copy = tiles.clone();
    let top_edges: Vec<(ArrayView1<u8>, &Tile)> =
        tiles.iter().map(|tile| (tile.data.row(0), tile)).collect();
    let bottom_edges: Vec<(ArrayView1<u8>, &Tile)> = tiles
        .iter()
        .map(|tile| (tile.data.row(tile.data.nrows() - 1), tile))
        .collect();
    let left_edges: Vec<(ArrayView1<u8>, &Tile)> = tiles
        .iter()
        .map(|tile| (tile.data.column(0), tile))
        .collect();
    let right_edges: Vec<(ArrayView1<u8>, &Tile)> = tiles
        .iter()
        .map(|tile| (tile.data.column(tile.data.ncols() - 1), tile))
        .collect();
    let edge_index = EdgeIndex::new(top_edges, bottom_edges, left_edges, right_edges);
    let corner_id: u64 = edge_index
        .corners(&tiles)
        .into_iter()
        .map(|c| c.id)
        .next()
        .unwrap();
    edge_index.build_strip(tiles_copy, corner_id);

    Ok(())
}

enum Corner {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl Corner {
    fn _opposite(&self) -> Self {
        match self {
            Corner::TopLeft => Corner::BottomRight,
            Corner::TopRight => Corner::BottomLeft,
            Corner::BottomLeft => Corner::TopRight,
            Corner::BottomRight => Corner::TopLeft,
        }
    }
}
#[derive(Debug, PartialEq, Eq, Clone)]
struct Tile {
    id: u64,
    data: Array2<u8>,
}

struct EdgeIndex<'a> {
    edge_to_tile: HashMap<ArrayView1<'a, u8>, Vec<&'a Tile>>,
}
impl Tile {
    fn from_array(id: u64, data: Array2<u8>) -> Self {
        Tile { id, data }
    }
    fn get_corner(&self, corner: &Corner) -> (ArrayView1<u8>, ArrayView1<u8>) {
        match corner {
            Corner::TopLeft => (self.top(), self.left()),
            Corner::TopRight => (self.top(), self.right()),
            Corner::BottomLeft => (self.bottom(), self.left()),
            Corner::BottomRight => (self.bottom(), self.right()),
        }
    }
    fn get_corner_owned(&self, corner: &Corner) -> (Array1<u8>, Array1<u8>) {
        match corner {
            Corner::TopLeft => (self.top().to_owned(), self.left().to_owned()),
            Corner::TopRight => (self.top().to_owned(), self.right().to_owned()),
            Corner::BottomLeft => (self.bottom().to_owned(), self.left().to_owned()),
            Corner::BottomRight => (self.bottom().to_owned(), self.right().to_owned()),
        }
    }
    fn top(&self) -> ArrayView1<u8> {
        self.data.row(0)
    }
    fn bottom(&self) -> ArrayView1<u8> {
        self.data.row(self.data.nrows() - 1)
    }
    fn left(&self) -> ArrayView1<u8> {
        self.data.column(0)
    }
    fn right(&self) -> ArrayView1<u8> {
        self.data.column(self.data.ncols() - 1)
    }
    fn edges(&self) -> Vec<ArrayView1<u8>> {
        vec![self.top(), self.bottom(), self.left(), self.right()]
    }
    // 90 degrees
    fn rotate(&mut self) {
        self.data.swap_axes(0, 1);
        for i in 0..self.data.nrows() {
            for j in 0..(self.data.ncols() / 2) {
                self.data.swap([i, j], [i, self.data.ncols() - 1 - j])
            }
        }
    }
    fn flip_horizontal(&mut self) {
        for i in 0..self.data.nrows() {
            for j in 0..(self.data.ncols() / 2) {
                self.data.swap([i, j], [i, self.data.ncols() - 1 - j])
            }
        }
    }
    fn flip_vertical(&mut self) {
        for i in 0..(self.data.nrows() / 2) {
            for j in 0..self.data.ncols() {
                self.data.swap([i, j], [self.data.nrows() - 1 - i, j])
            }
        }
    }
}

fn read_tiles(mut buffered: BufReader<File>) -> Vec<Tile> {
    let mut tiles = vec![];
    let mut data: Vec<u8> = Vec::new();
    let mut nrows = 0;
    let mut ncols = 0;
    let mut buf = String::new();
    let mut id: u64 = 0;
    while buffered.read_line(&mut buf).unwrap() != 0 {
        if buf.trim().is_empty() {
            let tile = Tile::from_array(
                id,
                Array2::from_shape_vec((nrows, ncols), data.clone()).unwrap(),
            );
            tiles.push(tile);
            data.clear();
            nrows = 0;
            id = 0;
        } else {
            if buf.starts_with("Tile") {
                let s = buf.split_whitespace().nth(1).unwrap();
                id = s[..s.len() - 1].parse().unwrap();
            } else {
                let row = buf.trim().as_bytes().to_vec();
                ncols = row.len();
                data.extend_from_slice(&row);
                nrows += 1;
            }
        }
        buf.clear();
    }
    tiles
}
impl<'a> EdgeIndex<'a> {
    fn new(
        top: Vec<(ArrayView1<'a, u8>, &'a Tile)>,
        bottom: Vec<(ArrayView1<'a, u8>, &'a Tile)>,
        left: Vec<(ArrayView1<'a, u8>, &'a Tile)>,
        right: Vec<(ArrayView1<'a, u8>, &'a Tile)>,
    ) -> Self {
        let mut edge_to_tile: HashMap<ArrayView1<'a, u8>, Vec<&'a Tile>> = HashMap::new();
        let iter = top
            .into_iter()
            .chain(bottom.into_iter())
            .chain(left.into_iter())
            .chain(right.into_iter());
        iter.for_each(|(edge, tile)| {
            if let Some(v) = edge_to_tile.get_mut(&edge) {
                v.push(tile);
            } else {
                edge_to_tile.insert(edge, vec![tile]);
            }
        });
        Self { edge_to_tile }
    }

    fn corners(&self, tiles: &'a [Tile]) -> Vec<&'a Tile> {
        let matches: Vec<&Tile> = tiles
            .iter()
            .filter(|tile| {
                let mut unmatched_count = 0;
                for corner in &[
                    Corner::TopLeft,
                    Corner::TopRight,
                    Corner::BottomLeft,
                    Corner::BottomRight,
                ] {
                    let (vertical_edge, horizontal_edge) = tile.get_corner(corner);
                    let reversed_vertical_edge = vertical_edge.slice_move(s![..;-1]);
                    let reversed_horizontal_edge = horizontal_edge.slice_move(s![..;-1]);

                    if self
                        .edge_to_tile
                        .get(&vertical_edge)
                        .map(Vec::len)
                        .unwrap_or(0)
                        + self
                            .edge_to_tile
                            .get(&reversed_vertical_edge)
                            .map(Vec::len)
                            .unwrap_or(0)
                        == 1usize
                        && self
                            .edge_to_tile
                            .get(&horizontal_edge)
                            .map(Vec::len)
                            .unwrap_or(0)
                            + self
                                .edge_to_tile
                                .get(&reversed_horizontal_edge)
                                .map(Vec::len)
                                .unwrap_or(0)
                            == 1usize
                    {
                        unmatched_count += 1
                    }
                }
                unmatched_count == 1
            })
            .collect();
        matches
    }
    fn build_strip(self, tiles: Vec<Tile>, corner_id: u64) -> Vec<Tile> {
        let mut strip: Vec<Tile> = vec![];
        let mut used: HashSet<u64> = HashSet::new();
        let mut first = {
            tiles
                .into_iter()
                .filter(|t| t.id == corner_id)
                .next()
                .unwrap()
        };
        let owned_index: HashMap<Array1<u8>, Vec<Tile>> = self
            .edge_to_tile
            .into_iter()
            .map(|(edge, tile)| (edge.into_owned(), tile.into_iter().cloned().collect()))
            .collect();
        orient_corner(&mut first, &owned_index);
        strip.push(first.clone());
        used.insert(first.id);

        while let Some(tile) = get_pair_and_orient(
            &first.bottom().to_owned(),
            &Orientation::Top,
            &owned_index,
            &used,
        ) {
            first = tile.clone();
            used.insert(tile.id);
            strip.push(tile);
            dbg!(strip.len());
            dbg!(&used);
        }
        dbg!(strip.len());

        strip
    }
}

fn orient_corner(tile: &mut Tile, index: &HashMap<Array1<u8>, Vec<Tile>>) {
    // rotate and flip tile such that the bottom right can be adjacent to a tile
    let mut bottom = tile.bottom().to_owned();
    //find bottom first
    let mut c = 0;
    while get_pair(tile.id, &bottom, index).is_none() && c < 4 {
        tile.rotate();
        c += 1;
        bottom = tile.bottom().to_owned();
    }
    //didn't find a bottom, and 3 means we rotated back to where we were, so lets flip horizontally
    if c >= 3 {
        tile.flip_horizontal();
        c = 0;
        while get_pair(tile.id, &bottom, index).is_none() && c < 4 {
            tile.rotate();
            c += 1;
            bottom = tile.bottom().to_owned();
        }
    }
    //we want to build off the bottom right, so if the left has something, that means we need to
    //rotate counterclockwise
    if get_pair(tile.id, &tile.left().to_owned(), index).is_some() {
        tile.rotate();
        tile.rotate();
        tile.rotate();
    }
    assert!(
        get_pair(tile.id, &tile.bottom().to_owned(), index).is_some()
            && get_pair(tile.id, &tile.right().to_owned(), index).is_some()
    );
}

//an edge can be paired if there's some other tile with the same edge
fn get_pair<'a>(
    id: u64,
    edge: &Array1<u8>,
    index: &'a HashMap<Array1<u8>, Vec<Tile>>,
) -> Option<&'a Tile> {
    index
        .get(&edge)
        .filter(|v| v.iter().any(|t| t.id != id))
        .map(|s| s.first().unwrap())
}

fn get_pair_and_orient<'a>(
    edge: &Array1<u8>,
    _orientation: &Orientation,
    index: &'a HashMap<Array1<u8>, Vec<Tile>>,
    used: &HashSet<u64>,
) -> Option<Tile> {
    let mut tile = index
        .get(&edge)
        .map(|v| v.into_iter().filter(|t| !used.contains(&t.id)).next())
        .flatten()
        .cloned();
    if let Some(ref mut t) = tile {
        let mut c = 0;
        while &t.top().to_owned() != edge && c < 4 {
            c += 1;
            t.rotate();
        }
        if &t.top().to_owned() != edge {
            c = 0;
            t.flip_horizontal();
            while &t.top().to_owned() != edge && c < 4 {
                c += 1;
                t.rotate();
            }
        }
        if &t.top().to_owned() != edge {
            c = 0;
            t.flip_vertical();
            while &t.top().to_owned() != edge && c < 4 {
                c += 1;
                t.rotate();
            }
        }
        if &t.top().to_owned() != edge {
            c = 0;
            t.flip_horizontal();
            while &t.top().to_owned() != edge && c < 4 {
                c += 1;
                t.rotate();
            }
        }
        assert_eq!(&t.top().to_owned(), edge);
    }
    if tile.is_some() {
        return tile;
    }
    dbg!("couldn't find one, flipping");
    let edge = &edge.slice(s![..;-1]).to_owned();
    let mut tile = index
        .get(&edge)
        .map(|v| v.into_iter().filter(|t| !used.contains(&t.id)).next())
        .flatten()
        .cloned();
    if let Some(ref mut t) = tile {
        let mut c = 0;
        let u = t.clone();
        while &t.top().to_owned() != edge && c < 4 {
            c += 1;
            t.rotate();
        }
        if &t.top().to_owned() != edge {
            c = 0;
            assert_eq!(t, &u);
            t.flip_horizontal();
            while &t.top().to_owned() != edge && c < 4 {
                c += 1;
                t.rotate();
                dbg!(&t.top().to_owned(), edge);
            }
        }
        if &t.top().to_owned() != edge {
            c = 0;
            t.flip_vertical();
            while &t.top().to_owned() != edge && c < 4 {
                c += 1;
                t.rotate();
            }
        }
        if &t.top().to_owned() != edge {
            c = 0;
            t.flip_horizontal();
            while &t.top().to_owned() != edge && c < 4 {
                c += 1;
                t.rotate();
            }
        }
        assert_eq!(&t.top().to_owned(), edge);
    }
    tile
}

enum Orientation {
    Top,
}

#[test]
fn rotate() {
    use ndarray::arr2;

    let mat = arr2(&[
        [1, 2, 3, 4],
        [5, 6, 7, 8],
        [9, 10, 11, 12],
        [13, 14, 15, 16],
    ]);
    let mut tile = Tile { id: 123, data: mat };
    tile.rotate();
    let expected = arr2(&[
        [13, 9, 5, 1],
        [14, 10, 6, 2],
        [15, 11, 7, 3],
        [16, 12, 8, 4],
    ]);
    assert_eq!(tile.data, expected);
}
#[test]
fn flip_vertical() {
    use ndarray::arr2;

    let mat = arr2(&[[1, 2], [3, 4]]);
    let mut tile = Tile { id: 123, data: mat };
    tile.flip_vertical();
    let expected = arr2(&[[3, 4], [1, 2]]);
    assert_eq!(tile.data, expected);
}

#[test]
fn flip_horizontal() {
    use ndarray::arr2;

    let mat = arr2(&[[1, 2], [3, 4]]);
    let mut tile = Tile { id: 123, data: mat };
    tile.flip_horizontal();
    let expected = arr2(&[[2, 1], [4, 3]]);
    assert_eq!(tile.data, expected);
}
