#[macro_use]
extern crate ndarray;
use ndarray::{Array2, ArrayView1};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
fn main() -> Result<(), Box<dyn Error>> {
    let filename = std::env::args().nth(1).unwrap();
    let input = File::open(filename)?;
    let buffered = BufReader::new(input);
    let tiles: Vec<Tile> = read_tiles(buffered);
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
    let edge_index = EdgeIndex::new(&top_edges, &bottom_edges, &left_edges, &right_edges);
    let corner_product: u64 = edge_index
        .corners(&tiles)
        .into_iter()
        .map(|c| c.id)
        .product();
    dbg!(corner_product);

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
#[derive(Debug)]
struct Tile {
    id: u64,
    data: Array2<u8>,
}

struct EdgeIndex<'a> {
    edge_to_tile: HashMap<&'a ArrayView1<'a, u8>, Vec<&'a Tile>>,
}
impl Tile {
    fn from_array(id: u64, data: Array2<u8>) -> Self {
        Tile { id, data }
    }
    fn get_corner(&self, corner: &Corner) -> (ArrayView1<u8>, ArrayView1<u8>) {
        match corner {
            Corner::TopLeft => (self.data.row(0), self.data.column(0)),
            Corner::TopRight => (self.data.row(0), self.data.column(self.data.ncols() - 1)),
            Corner::BottomLeft => (self.data.row(self.data.nrows() - 1), self.data.column(0)),
            Corner::BottomRight => (
                self.data.row(self.data.nrows() - 1),
                self.data.column(self.data.ncols() - 1),
            ),
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
        top: &'a Vec<(ArrayView1<'a, u8>, &'a Tile)>,
        bottom: &'a Vec<(ArrayView1<'a, u8>, &'a Tile)>,
        left: &'a Vec<(ArrayView1<'a, u8>, &'a Tile)>,
        right: &'a Vec<(ArrayView1<'a, u8>, &'a Tile)>,
    ) -> Self {
        let mut edge_to_tile: HashMap<&'a ArrayView1<'a, u8>, Vec<&'a Tile>> = HashMap::new();
        let iter = top
            .iter()
            .chain(bottom.iter())
            .chain(left.iter())
            .chain(right.iter());
        iter.for_each(|(edge, tile)| {
            if let Some(v) = edge_to_tile.get_mut(edge) {
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
}
