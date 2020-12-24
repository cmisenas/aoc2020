use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn main() {
    let lines = read_lines_as_str("./day24.input");
    let mut parsed_lines: Vec<Vec<String>> = Vec::new();
    for line in lines.iter() {
        parsed_lines.push(parse_tile_list(line));
    }
    let answer1 = solve1(&parsed_lines);
    let answer2 = solve2(&lines);
    println!("Answer 1 {}", answer1);
    println!("Answer 2 {}", answer2);
}

fn parse_tile_list(list: &String) -> Vec<String> {
    let split_list: Vec<String> = list
        .split_inclusive(|c| c == 'e' || c == 'w')
        .map(|c| c.to_string())
        .collect();
    split_list
}

// Converts directions to coordinates relative to reference tile which is 0, 0
fn convert_to_coord(list: &Vec<String>) -> (i32, i32) {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    for dir in list.iter() {
        if dir == "ne" {
            y -= 1;
            if y.abs() % 2 == 0 {
                x += 1;
            }
        } else if dir == "nw" {
            y -= 1;
            if y.abs() % 2 == 1 {
                x -= 1;
            }
        } else if dir == "se" {
            y += 1;
            if y.abs() % 2 == 0 {
                x += 1;
            }
        } else if dir == "sw" {
            y += 1;
            if y.abs() % 2 == 1 {
                x -= 1;
            }
        } else if dir == "w" {
            x -= 1
        } else if dir == "e" {
            x += 1
        }
    }
    (x, y)
}

fn solve1(lines: &Vec<Vec<String>>) -> usize {
    let mut black_tiles: HashSet<(i32, i32)> = HashSet::new();
    for line in lines.iter() {
        let coords = convert_to_coord(line);
        if black_tiles.contains(&coords) {
            black_tiles.remove(&coords);
        } else {
            black_tiles.insert(coords);
        }
    }
    black_tiles.len()
}

fn solve2(lines: &Vec<String>) -> i32 {
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
