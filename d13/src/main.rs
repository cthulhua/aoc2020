use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = std::env::args().nth(1).unwrap();
    let input = File::open(file)?;
    let mut buffered = BufReader::new(input);
    let mut buf = String::new();
    buffered.read_line(&mut buf)?;
    let ts: u64 = buf.trim().parse()?;
    buf.clear();
    buffered.read_line(&mut buf)?;
    let routes: Vec<u64> = buf
        .split(",")
        .filter_map(|x| {
            if x == "x" {
                None
            } else {
                let route: u64 = x.trim().parse().unwrap();
                Some(route)
            }
        })
        .collect();
    for t in ts.. {
        if let Some(route) = routes.iter().filter(|x| t % *x == 0).next() {
            dbg!((t - ts) * route);
            break;
        }
    }
    Ok(())
}
