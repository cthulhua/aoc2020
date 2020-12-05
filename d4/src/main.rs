use std::fs::File;
use std::io::{BufRead, BufReader};

static REQUIRED_FIELDS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = File::open("input.txt")?;
    let lines = BufReader::new(input).lines();
    let mut buf = String::new();
    let mut valid_count = 0;
    for line in lines {
        match line {
            Ok(s) => {
                if s.trim().is_empty() {
                    // process buffer
                    if valid_passport(&buf) {
                        valid_count += 1;
                    }
                    buf.clear();
                } else {
                    buf.push_str(" ");
                    buf.push_str(&s);
                }
            }
            Err(_) => panic!("oh shit dog"),
        }
    }
    if valid_passport(&buf) {
        valid_count += 1;
    }
    dbg!(valid_count);
    Ok(())
}

fn valid_passport(buf: &str) -> bool {
    if !REQUIRED_FIELDS.iter().all(|field| buf.contains(*field)) {
        return false;
    }
    buf.split_whitespace().all(|token| valid_token(token))
}

fn valid_token(token: &str) -> bool {
    let mut sections = token.split(':');
    let label = match sections.next() {
        Some(s) => s,
        None => return false,
    };
    let data = match sections.next() {
        Some(s) => s,
        None => return false,
    };
    match label {
        "byr" => valid_byr(data),
        "iyr" => valid_iyr(data),
        "eyr" => valid_eyr(data),
        "hgt" => valid_hgt(data),
        "hcl" => valid_hcl(data),
        "ecl" => valid_ecl(data),
        "pid" => valid_pid(data),
        _ => true,
    }
}

fn valid_byr(byr: &str) -> bool {
    match byr.parse::<u64>() {
        Ok(year) => year >= 1920 && year <= 2002,
        Err(_) => false,
    }
}

fn valid_iyr(iyr: &str) -> bool {
    match iyr.parse::<u64>() {
        Ok(year) => year >= 2010 && year <= 2020,
        Err(_) => false,
    }
}

fn valid_eyr(eyr: &str) -> bool {
    match eyr.parse::<u64>() {
        Ok(year) => year >= 2020 && year <= 2030,
        Err(_) => false,
    }
}

fn valid_hgt(hgt: &str) -> bool {
    let unit = &hgt[hgt.len() - 2..hgt.len()];
    match unit {
        "cm" => valid_hgt_cm(&hgt[0..hgt.len() - 2]),
        "in" => valid_hgt_in(&hgt[0..hgt.len() - 2]),
        _ => false,
    }
}

fn valid_hgt_cm(hgt_cm: &str) -> bool {
    match hgt_cm.parse::<u64>() {
        Ok(cm) => cm >= 150 && cm <= 193,
        Err(_) => false,
    }
}

fn valid_hgt_in(hgt_in: &str) -> bool {
    match hgt_in.parse::<u64>() {
        Ok(inches) => inches >= 59 && inches <= 76,
        Err(_) => false,
    }
}

fn valid_hcl(hcl: &str) -> bool {
    if hcl.len() != 7 {
        return false;
    }
    hcl.as_bytes()[0] == b'#' && valid_hex(&hcl.as_bytes()[1..7])
}

fn valid_hex(bytes: &[u8]) -> bool {
    bytes
        .iter()
        .all(|&byte| (byte <= b'9' && byte >= b'0') || (byte <= b'f' && byte >= b'a'))
}

fn valid_ecl(ecl: &str) -> bool {
    if ecl.len() != 3 {
        return false;
    }
    ecl == "amb"
        || ecl == "blu"
        || ecl == "brn"
        || ecl == "gry"
        || ecl == "grn"
        || ecl == "hzl"
        || ecl == "oth"
}

fn valid_pid(pid: &str) -> bool {
    if pid.len() != 9 {
        return false;
    }
    valid_digit(&pid.as_bytes()[0..9])
}

fn valid_digit(bytes: &[u8]) -> bool {
    bytes.iter().all(|&byte| (byte <= b'9' && byte >= b'0'))
}
