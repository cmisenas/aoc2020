use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn main() {
    let lines = read_lines_as_str("./day5.input");
    let mut seats: Vec<i32> = lines
        .iter()
        .map(|line| {
            let row_part = line.get(0..7).expect("error").to_string();
            let col_part = line.get(7..).expect("error").to_string();
            let row = compute(row_part, 0, 127);
            let col = compute(col_part, 0, 7);
            row * 8 + col
        })
        .collect();
    seats.sort();
    let ans1 = seats[seats.len() - 1];
    let ans2 = seats
        .iter()
        .skip(1)
        .enumerate()
        .find(|(i, &seat)| (seat - seats[*i]) == 2)
        .map(|(_, seat)| seat - 1)
        .unwrap();
    println!("Answer 1 {}", ans1);
    println!("Answer 2 {}", ans2);
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
