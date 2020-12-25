use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn main() {
    let lines = read_lines_as_int("./day25.input");
    let card_pk = lines[0];
    let door_pk = lines[1];
    let answer1 = solve1(card_pk, door_pk);
    println!("Answer 1 {}", answer1);
}

fn solve1(card_pk: u64, door_pk: u64) -> u64 {
    let subject_val = 7;
    let remainder = 20201227;
    let mut card_loop_size = 0;
    let mut card_value = 1;
    let mut door_loop_size = 0;
    let mut door_value = 1;
    let mut encryption_key = 1;
    while door_value != door_pk || card_value != card_pk {
        if card_value != card_pk {
            card_value *= subject_val;
            card_value %= remainder;
            card_loop_size += 1;
        }
        if door_value != door_pk {
            door_value *= subject_val;
            door_value %= remainder;
            door_loop_size += 1;
        }
    }
    println!("Door loop size {}", door_loop_size);
    println!("Card loop size {}", card_loop_size);
    for _ in 0..door_loop_size {
        encryption_key *= card_pk;
        encryption_key %= remainder;
    }
    encryption_key
}

fn read_lines_as_int<P>(filename: P) -> Vec<u64>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("no such file");
    let buf = io::BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line").parse::<u64>().unwrap())
        .collect()
}
