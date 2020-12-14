use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = std::env::args().nth(1).unwrap();
    let input = File::open(file)?;
    let mut buffered = BufReader::new(input);
    let mut buf = String::new();
    buffered.read_line(&mut buf)?;
    buf.clear();
    buffered.read_line(&mut buf)?;
    let routes: Vec<(i128, i128)> = buf
        .split(",")
        .enumerate()
        .filter_map(|(i, x)| {
            if x == "x" {
                None
            } else {
                let route: i128 = x.trim().parse().unwrap();
                let remainder = if i == 0 {
                    0
                } else {
                    let mut r = route - i as i128;
                    while r < 0 {
                        r += route;
                    }
                    r
                };
                Some((remainder, route))
            }
        })
        .collect();
    dbg!(&routes);
    dbg!(solution(routes));
    Ok(())
}

fn bezout_coefficients(a: i128, b: i128) -> (i128, i128) {
    //from wiki
    let mut old_r = a;
    let mut r = b;
    let mut old_s = 1;
    let mut s = 0;
    let mut old_t = 0;
    let mut t = 1;

    while r != 0 {
        let quotient = old_r / r;
        let temp = r;
        r = old_r - quotient * temp;
        old_r = temp;
        let temp = s;
        s = old_s - quotient * temp;
        old_s = temp;
        let temp = t;
        t = old_t - quotient * temp;
        old_t = temp;
    }
    (old_s, old_t)
}

fn solution(routes: Vec<(i128, i128)>) -> i128 {
    let init = (routes[0].0, routes[0].1);
    let (x, y) = routes.into_iter().skip(1).fold(init, |(a, n1), (b, n2)| {
        let (m1, m2) = bezout_coefficients(n1, n2);
        println!("{} x {} + {} x {} = {}", m2, n2, m1, n1, m2 * n2 + m1 * n1);
        let x = (a * n2 * m2 + b * n1 * m1) % (n1 * n2);
        println!("{} x {} + {} x {} = {}", m2 * n2, a, m1 * n1, b, x);
        (x, (n1 * n2))
    });
    dbg!(x);
    dbg!(y);
    let mut sol = x;
    if sol < 0 {
        sol += y
    }
    sol
}
