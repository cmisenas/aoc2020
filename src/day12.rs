extern crate regex;

use self::regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// enum Direction {
//     North(i32),
//     East(i32),
//     West(i32),
//     South(i32),
// }
//
// impl Direction {
//     fn face(&self, deg: i32) -> Direction {
//     }
// }

pub fn main() {
    let lines = read_lines_as_str("./day12.input");
    let answer1 = solve1(lines);
    let mut answer2 = 0;

    println!("Answer 1 {}", answer1);
    println!("Answer 2 {}", answer2);
}

fn solve1(instructions: Vec<String>) -> i32 {
    let N = "N".to_string();
    let E = "E".to_string();
    let S = "S".to_string();
    let W = "W".to_string();
    let L = "L".to_string();
    let R = "R".to_string();
    let F = "F".to_string();
    let mut facing_dir = "E".to_string();
    let mut h_distance = 0;
    let mut v_distance = 0;
    for line in instructions.iter() {
        let ins = line.to_string().chars().nth(0).unwrap().to_string();
        let units = line
            .to_string()
            .chars()
            .skip(1)
            .collect::<String>()
            .parse::<i32>()
            .unwrap();
        if ins == F {
            if facing_dir == N {
                v_distance -= units;
            } else if facing_dir == E {
                h_distance += units;
            } else if facing_dir == S {
                v_distance += units;
            } else if facing_dir == W {
                h_distance -= units;
            }
        } else if ins == N {
            v_distance -= units;
        } else if ins == E {
            h_distance += units;
        } else if ins == S {
            v_distance += units;
        } else if ins == W {
            h_distance -= units;
        } else if ins == L {
            facing_dir = rotate_dir(&facing_dir, units, true);
        } else if ins == R {
            facing_dir = rotate_dir(&facing_dir, units, false);
        }
    }

    h_distance + v_distance
}

fn rotate_dir(start_dir: &String, deg: i32, counter_clockwise: bool) -> String {
    let turn_deg = deg % 360;
    let order = match counter_clockwise {
        true => vec!["W", "S", "E", "N"],
        _ => vec!["N", "E", "S", "W"],
    };
    let start_dir_index = order
        .iter()
        .position(|&x| x == start_dir)
        .expect("Direction not found");
    match turn_deg {
        90 => order
            .iter()
            .nth((start_dir_index + 1) % 4)
            .unwrap()
            .to_string(),
        180 => order
            .iter()
            .nth((start_dir_index + 2) % 4)
            .unwrap()
            .to_string(),
        270 => order
            .iter()
            .nth((start_dir_index + 3) % 4)
            .unwrap()
            .to_string(),
        _ => start_dir.to_string(),
    }
}

fn solve2() -> i32 {
    0
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
