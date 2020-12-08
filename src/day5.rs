use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn main() {
    let lines = read_lines_as_str("./day5.input");
    let mut seats = Vec::new();

    for line in lines {
        let row_part = line.get(0..7).expect("error").to_string();
        let col_part = line.get(7..).expect("error").to_string();
        let row = compute(row_part, 0, 127);
        let col = compute(col_part, 0, 7);
        let seat = compute_seat_id(row, col);
        seats.push(seat);
    }
    seats.sort();
    let mut my_seat = 0;
    for (i, seat) in seats.iter().enumerate() {
        if i == 0 {
            continue;
        }
        let adj_seat = seats[i - 1];
        let ds = (seat - adj_seat).abs();
        if ds == 2 {
            my_seat = adj_seat + 1;
        }
    }
    println!("Highest ID {}", seats[seats.len() - 1]);
    println!("My seat ID {}", my_seat);
}

fn compute(part: String, low: i32, hi: i32) -> i32 {
    let mid = (hi + low) / 2;
    let curr_r = part.chars().nth(0).unwrap().to_string();
    if part.len() == 1 {
        if curr_r == "F" || curr_r == "L" {
            low
        } else {
            hi
        }
    } else if curr_r == "F" || curr_r == "L" {
        compute(part.get(1..).expect("no str").to_string(), low, mid)
    } else {
        compute(part.get(1..).expect("no str").to_string(), mid + 1, hi)
    }
}

fn compute_seat_id(row: i32, col: i32) -> i32 {
    row * 8 + col
}

fn read_lines_as_str<P>(filename: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("no such file");
    let buf = io::BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
