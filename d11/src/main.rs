use ndarray::{s, Array2, ArrayView2};
use std::collections::VecDeque;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
fn main() -> Result<(), Box<dyn Error>> {
    let mut seat_ca = SeatCA::load("input.txt")?;
    seat_ca.run_until_stable(SeatCA::get_visible_next);
    dbg!(seat_ca.occupied_count());
    Ok(())
}

#[derive(Clone, Debug)]
struct SeatCA {
    mat: Array2<u8>,
}

static FLOOR: u8 = b'.';
static EMPTY_SEAT: u8 = b'L';
static OCCUPIED_SEAT: u8 = b'#';

impl SeatCA {
    fn load(filename: &str) -> Result<Self, Box<dyn Error>> {
        let input = File::open(filename)?;
        let buffered = BufReader::new(input);
        let mut v: VecDeque<VecDeque<u8>> = buffered
            .lines()
            .map(|r| {
                let mut v: VecDeque<u8> = r.unwrap().into_bytes().into();
                v.push_back(FLOOR);
                v.push_front(FLOOR);
                v
            })
            .collect();
        let dummy_row: VecDeque<u8> = std::iter::repeat(FLOOR).take(v[0].len()).collect();
        v.push_front(dummy_row.clone());
        v.push_back(dummy_row);
        let mut mat = Array2::zeros((v.len(), v[0].len()));
        for (i, row) in v.into_iter().enumerate() {
            for (j, e) in row.iter().enumerate() {
                mat[[i, j]] = *e;
            }
        }
        Ok(SeatCA { mat })
    }

    fn get_next(&self) -> Array2<u8> {
        let mut next = self.mat.clone();
        for ((r, c), x) in next.indexed_iter_mut() {
            if *x == FLOOR {
                continue;
            }
            let neighbors = self
                .get_neighbors(r, c)
                .iter()
                .filter(|n| **n == OCCUPIED_SEAT)
                .count();
            if *x == EMPTY_SEAT && neighbors == 0 {
                *x = OCCUPIED_SEAT;
            } else if *x == OCCUPIED_SEAT && neighbors >= 5 {
                *x = EMPTY_SEAT;
            }
        }
        next
    }

    fn get_visible_next(&self) -> Array2<u8> {
        let mut next = self.mat.clone();
        for ((r, c), x) in next.indexed_iter_mut() {
            if *x == FLOOR {
                continue;
            }
            let visible = self.get_visible(r, c);
            if *x == EMPTY_SEAT && visible == 0 {
                *x = OCCUPIED_SEAT;
            } else if *x == OCCUPIED_SEAT && visible >= 5 {
                *x = EMPTY_SEAT;
            }
        }
        next
    }
    fn get_neighbors(&self, r: usize, c: usize) -> ArrayView2<u8> {
        if (r > 0 && r < self.mat.nrows()) && (c > 0 && c < self.mat.ncols()) {
            self.mat.slice(s![(r - 1)..=(r + 1), (c - 1)..=(c + 1)])
        } else {
            self.mat.slice(s![0..0, 0..0])
        }
    }
    fn get_visible(&self, r: usize, c: usize) -> usize {
        let e_visible = self
            .mat
            .row(r)
            .slice(s![c..])
            .iter()
            .skip(1)
            .skip_while(|n| **n == FLOOR)
            .next()
            .map(|n| if *n == OCCUPIED_SEAT { 1 } else { 0 })
            .unwrap_or(0);
        let w_visible = self
            .mat
            .row(r)
            .slice(s![0..c;-1])
            .iter()
            .skip_while(|n| **n == FLOOR)
            .next()
            .map(|n| if *n == OCCUPIED_SEAT { 1 } else { 0 })
            .unwrap_or(0);
        let s_visible = self
            .mat
            .column(c)
            .slice(s![r..])
            .iter()
            .skip(1)
            .skip_while(|n| **n == FLOOR)
            .next()
            .map(|n| if *n == OCCUPIED_SEAT { 1 } else { 0 })
            .unwrap_or(0);
        let n_visible = self
            .mat
            .column(c)
            .slice(s![0..r;-1])
            .iter()
            .skip_while(|n| **n == FLOOR)
            .next()
            .map(|n| if *n == OCCUPIED_SEAT { 1 } else { 0 })
            .unwrap_or(0);
        let se_visible = self
            .mat
            .slice(s![r.., c..])
            .diag()
            .iter()
            .skip(1)
            .skip_while(|n| **n == FLOOR)
            .next()
            .map(|n| if *n == OCCUPIED_SEAT { 1 } else { 0 })
            .unwrap_or(0);
        let sw_visible = self
            .mat
            .slice(s![r.., 0..=c;-1])
            .diag()
            .iter()
            .skip(1)
            .skip_while(|n| **n == FLOOR)
            .next()
            .map(|n| if *n == OCCUPIED_SEAT { 1 } else { 0 })
            .unwrap_or(0);
        let ne_visible = self
            .mat
            .slice(s![0..=r;-1, c..])
            .diag()
            .iter()
            .skip(1)
            .skip_while(|n| **n == FLOOR)
            .next()
            .map(|n| if *n == OCCUPIED_SEAT { 1 } else { 0 })
            .unwrap_or(0);
        let nw_visible = self
            .mat
            .slice(s![0..r;-1, 0..c;-1])
            .diag()
            .iter()
            .skip_while(|n| **n == FLOOR)
            .next()
            .map(|n| if *n == OCCUPIED_SEAT { 1 } else { 0 })
            .unwrap_or(0);
        e_visible
            + w_visible
            + s_visible
            + n_visible
            + se_visible
            + sw_visible
            + ne_visible
            + nw_visible
    }
    fn run_until_stable(&mut self, step_function: fn(&SeatCA) -> Array2<u8>) {
        let mut next = step_function(self);
        while next != self.mat {
            self.mat = next;
            next = step_function(self);
        }
    }
    fn occupied_count(&self) -> usize {
        self.mat.iter().filter(|x| **x == OCCUPIED_SEAT).count()
    }
}
