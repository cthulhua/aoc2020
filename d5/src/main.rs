use std::fs::File;
use std::io::{BufRead, BufReader};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = File::open("input.txt")?;
    let buffered = BufReader::new(input);
    let mut ids: Vec<u64> = buffered
        .lines()
        .map(|pass| get_row_and_column(&pass.unwrap()))
        .map(|(r, c)| r as u64 * 8u64 + c as u64)
        .collect();
    ids.sort();
    let mut prev = 39u64;
    for id in ids {
        if id - prev != 1 {
            dbg!(id - 1);
        }
        prev = id;
    }
    Ok(())
}

fn get_row_and_column(pass: &str) -> (u8, u8) {
    let row = decode_row(&pass[0..7]);
    let column = decode_column(&pass[7..10]);
    (row, column)
}

fn decode_row(row_spec: &str) -> u8 {
    let mut row = 0u8;
    for c in row_spec.chars() {
        match c {
            'B' => row |= 1u8,
            'F' => (), // do nothing
            _ => panic!(),
        };
        row <<= 1;
    }
    row >> 1
}
fn decode_column(column_spec: &str) -> u8 {
    let mut column = 0u8;
    for c in column_spec.chars() {
        match c {
            'R' => column |= 1u8,
            'L' => (), // do nothing
            _ => panic!(),
        };
        column <<= 1;
    }
    column >> 1
}
