extern crate itertools;

use self::itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/**
 * TILE
 * Tile ID, [Side, Side, Side, Side], Matches [Tile]
 * SIDE
 * [index of all #]
 * If there are only 2 matches for a tile, they are corner pieces
 */

#[derive(Clone, Debug)]
struct Tile {
    id: String,
    sides: Vec<String>,
}

impl Tile {
    fn is_adj(&self, side_index: usize, other_tile: &Tile) -> bool {
        let side = &self.sides[side_index];
        other_tile.sides.iter().any(|other_tile_side| {
            other_tile_side == side || other_tile_side.chars().rev().eq(side.chars())
        })
    }
}

pub fn main() {
    let lines = read_lines_as_str("./day20.input");
    let tiles = &lines
        .into_iter()
        .group_by(|line| line.is_empty())
        .into_iter()
        .filter_map(|(is_empty, line)| match !is_empty {
            true => Some(parse_tile(line.into_iter().collect::<Vec<String>>())),
            _ => None,
        })
        .collect::<Vec<Tile>>();

    let mut answer1 = solve1(&tiles);
    let mut answer2 = solve2(&tiles);

    println!("Answer 1 {}", answer1);
    println!("Answer 2 {}", answer2);
}

fn parse_tile(lines: Vec<String>) -> Tile {
    let rside = lines
        .iter()
        .skip(1)
        .map(|l| l.chars().last().unwrap())
        .collect::<String>();
    let lside = lines
        .iter()
        .skip(1)
        .map(|l| l.chars().nth(0).unwrap())
        .collect::<String>();
    Tile {
        id: lines[0].replace(':', "").trim().to_string(),
        sides: vec![
            lines[1].to_string(),
            lside.to_string(),
            rside.to_string(),
            lines[lines.len() - 1].to_string(),
        ],
    }
}

fn solve1(tiles: &Vec<Tile>) -> u64 {
    let mut tile_matches: HashMap<String, HashSet<String>> = HashMap::new();
    // Find the matches for each tile
    for tile in tiles {
        for i in 0..4 {
            for other_tile in tiles {
                if other_tile.id == tile.id {
                    continue;
                }
                if tile.is_adj(i, other_tile) {
                    tile_matches
                        .entry(tile.id.to_string())
                        .or_insert(HashSet::new())
                        .insert(other_tile.id.to_string());
                    tile_matches
                        .entry(other_tile.id.to_string())
                        .or_insert(HashSet::new())
                        .insert(tile.id.to_string());
                }
            }
        }
    }
    let mut prod = 1;
    for (tile, matches) in &tile_matches {
        println!("Matches for tile {} {:?}", tile, matches);
        if matches.len() == 2 {
            prod *= tile.strip_prefix("Tile ").unwrap().parse::<u64>().unwrap();
        }
    }
    prod
}

fn solve2(tiles: &Vec<Tile>) -> i32 {
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
