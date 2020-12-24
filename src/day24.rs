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
    let black_tiles = solve1(&parsed_lines);
    let answer1 = black_tiles.len();
    let answer2 = solve2(black_tiles);
    println!("Answer 1 {}", answer1);
    println!("Answer 2 {}", answer2);
}

fn parse_tile_list(list: &str) -> Vec<String> {
    let split_list: Vec<String> = list
        .split_inclusive(|c| c == 'e' || c == 'w')
        .map(|c| c.to_string())
        .collect();
    split_list
}

// Converts directions to coordinates relative to reference tile which is 0, 0
fn convert_to_coord(list: &[String]) -> (i32, i32) {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    for dir in list.iter() {
        (x, y) = get_tile_neighbor((x, y), dir.to_string());
    }
    (x, y)
}

fn get_tile_neighbor(tile: (i32, i32), dir: String) -> (i32, i32) {
    let mut x: i32 = tile.0;
    let mut y: i32 = tile.1;
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
    (x, y)
}

fn get_tile_neighbors(tile: (i32, i32)) -> Vec<(i32, i32)> {
    let mut neighbors: Vec<(i32, i32)> = Vec::new();
    neighbors.push(get_tile_neighbor(tile, String::from("e")));
    neighbors.push(get_tile_neighbor(tile, String::from("w")));
    neighbors.push(get_tile_neighbor(tile, String::from("ne")));
    neighbors.push(get_tile_neighbor(tile, String::from("se")));
    neighbors.push(get_tile_neighbor(tile, String::from("nw")));
    neighbors.push(get_tile_neighbor(tile, String::from("sw")));
    neighbors
}

fn solve1(lines: &[Vec<String>]) -> HashSet<(i32, i32)> {
    let mut black_tiles: HashSet<(i32, i32)> = HashSet::new();
    for line in lines.iter() {
        let coords = convert_to_coord(line);
        if black_tiles.contains(&coords) {
            black_tiles.remove(&coords);
        } else {
            black_tiles.insert(coords);
        }
    }
    black_tiles
}

fn solve2(mut black_tiles: HashSet<(i32, i32)>) -> usize {
    let mut new_black_tiles: HashSet<(i32, i32)> = HashSet::new();
    for _ in 1..=100 {
        for tile in black_tiles.iter() {
            let tile_neighbors = get_tile_neighbors(*tile);
            let black_tile_neighbors = tile_neighbors
                .iter()
                .filter(|t| black_tiles.contains(&t))
                .count();
            if black_tile_neighbors > 0 && black_tile_neighbors <= 2 {
                new_black_tiles.insert(*tile);
            }

            for tile_n in tile_neighbors.iter() {
                let tile_n_n = get_tile_neighbors(*tile_n);
                let black_tile_neighbors =
                    tile_n_n.iter().filter(|t| black_tiles.contains(&t)).count();
                if black_tile_neighbors == 2 && !black_tiles.contains(tile_n) {
                    new_black_tiles.insert(*tile_n);
                }
            }
        }
        black_tiles = new_black_tiles.clone();
        new_black_tiles.clear();
    }
    black_tiles.len()
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
