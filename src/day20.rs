extern crate itertools;
extern crate regex;

use self::itertools::Itertools;
use self::regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Clone, PartialEq, Eq)]
struct Tile {
    id: String,
    sides: HashMap<Side, String>,
    content: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Side {
    Top,
    Left,
    Right,
    Bottom,
}

impl fmt::Debug for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("\n{}", &self.content.join("\n")))
    }
}

impl Tile {
    fn is_side_strict_adj(&self, side: &Side, other_tile: &Tile) -> bool {
        let tile_side = self.sides.get(side).unwrap();
        other_tile
            .sides
            .iter()
            .any(|(_, other_tile_side)| other_tile_side == tile_side)
    }

    fn is_side_adj(&self, side: &Side, other_tile: &Tile) -> bool {
        let tile_side = self.sides.get(side).unwrap();
        other_tile.sides.iter().any(|(_, other_tile_side)| {
            other_tile_side == tile_side || other_tile_side.chars().rev().eq(tile_side.chars())
        })
    }

    fn get_new_side(&self, side: Side) -> String {
        match side {
            Side::Right => self
                .content
                .iter()
                .map(|l| l.chars().last().unwrap())
                .collect::<String>(),
            Side::Left => self
                .content
                .iter()
                .map(|l| l.chars().next().unwrap())
                .collect::<String>(),
            Side::Top => self.content[0].to_string(),
            Side::Bottom => self.content[self.content.len() - 1].to_string(),
        }
    }

    fn set_side(&mut self, side: Side, val: String) {
        self.sides.insert(side, val);
    }

    fn reset_sides(&mut self) {
        self.set_side(Side::Top, self.get_new_side(Side::Top));
        self.set_side(Side::Left, self.get_new_side(Side::Left));
        self.set_side(Side::Right, self.get_new_side(Side::Right));
        self.set_side(Side::Bottom, self.get_new_side(Side::Bottom));
    }

    fn flip(&mut self, horizontal: bool) {
        self.content = match horizontal {
            true => self
                .content
                .iter()
                .map(|l| l.chars().rev().collect::<String>())
                .collect::<Vec<String>>(),
            _ => self
                .content
                .iter()
                .rev()
                .map(|l| l.to_string())
                .collect::<Vec<String>>(),
        };
        self.reset_sides();
    }

    fn rotate(&mut self, turns: usize) {
        self.content = match turns {
            1 => (0..self.content[0].len())
                .map(|c| {
                    (0..self.content[0].len())
                        .map(|r| {
                            let row = self.content[0].len() - r - 1;
                            self.content[row].chars().nth(c).unwrap()
                        })
                        .collect::<String>()
                })
                .collect::<Vec<String>>(),
            2 => self
                .content
                .iter()
                .rev()
                .map(|l| l.chars().rev().collect::<String>())
                .collect::<Vec<String>>(),
            _ => (0..self.content[0].len())
                .map(|c| {
                    let col = self.content[0].len() - c - 1;
                    (0..self.content[0].len())
                        .map(|r| self.content[r].chars().nth(col).unwrap())
                        .collect::<String>()
                })
                .collect::<Vec<String>>(),
        };
        self.reset_sides();
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

    let answer1 = solve1(&tiles);
    let answer2 = solve2(&tiles);
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
        .map(|l| l.chars().next().unwrap())
        .collect::<String>();
    let mut sides: HashMap<Side, String> = HashMap::new();
    sides.insert(Side::Top, lines[1].to_string());
    sides.insert(Side::Bottom, lines[lines.len() - 1].to_string());
    sides.insert(Side::Right, rside);
    sides.insert(Side::Left, lside);
    Tile {
        id: lines[0].replace(':', "").trim().to_string(),
        content: lines[1..].to_vec(),
        sides,
    }
}

fn solve1(tiles: &[Tile]) -> u64 {
    let mut tile_matches: HashMap<String, HashSet<String>> = HashMap::new();
    let sides = vec![Side::Top, Side::Right, Side::Bottom, Side::Left];
    // Find the matches for each tile
    for tile in tiles {
        for side in &sides {
            for other_tile in tiles {
                if other_tile.id == tile.id {
                    continue;
                }
                if tile.is_side_adj(side, other_tile) {
                    tile_matches
                        .entry(tile.id.to_string())
                        .or_insert_with(HashSet::new)
                        .insert(other_tile.id.to_string());
                    tile_matches
                        .entry(other_tile.id.to_string())
                        .or_insert_with(HashSet::new)
                        .insert(tile.id.to_string());
                }
            }
        }
    }
    let mut prod = 1;
    for (tile, matches) in &tile_matches {
        if matches.len() == 2 {
            prod *= tile.strip_prefix("Tile ").unwrap().parse::<u64>().unwrap();
        }
    }
    prod
}

fn solve2(tiles: &[Tile]) -> usize {
    let mut tiles_by_id: HashMap<String, Tile> = HashMap::new();
    let mut tile_matches: HashMap<String, HashMap<String, Tile>> = HashMap::new();
    let sides = vec![Side::Top, Side::Right, Side::Bottom, Side::Left];
    // Find the matches for each tile
    for tile in tiles {
        tiles_by_id.insert(tile.id.to_string(), tile.clone());
        for side in &sides {
            for other_tile in tiles {
                if other_tile.id == tile.id {
                    continue;
                }
                if tile.is_side_adj(&side, other_tile) {
                    tile_matches
                        .entry(tile.id.to_string())
                        .or_insert_with(HashMap::new)
                        .insert(other_tile.id.to_string(), other_tile.clone());
                    tile_matches
                        .entry(other_tile.id.to_string())
                        .or_insert_with(HashMap::new)
                        .insert(tile.id.to_string(), tile.clone());
                }
            }
        }
    }
    // Pick a corner piece, rotate until top and left don't have a match
    // Iterate over matches and find the right,
    let mut curr_tile_id = tile_matches
        .iter()
        .find(|(_, matches)| matches.len() == 2)
        .unwrap();
    // Start with the top left corner
    let mut curr_tile = tiles_by_id.get_mut(curr_tile_id.0).unwrap();
    let mut image: Vec<Vec<String>> = Vec::new();
    let mut row = 0;
    let mut col = 0;
    let end = (tiles.len() as f64).sqrt() as u8;
    let mut prev_tile = curr_tile.clone();
    let mut prev_first_tile = curr_tile.clone();
    let mut next_first_tile = curr_tile.clone();
    while row < end {
        let mut curr_layer: Vec<Vec<String>> = Vec::new();
        let mut curr_layer_ids: Vec<String> = Vec::new();
        while col < end {
            // Top left corner
            if col == 0 && row == 0 {
                for i in 0..8 {
                    let r_has_match = curr_tile_id
                        .1
                        .iter()
                        .find(|(_, m)| curr_tile.is_side_adj(&Side::Right, m));
                    let b_has_match = curr_tile_id
                        .1
                        .iter()
                        .find(|(_, m)| curr_tile.is_side_adj(&Side::Bottom, m));
                    if r_has_match.is_some() && b_has_match.is_some() {
                        next_first_tile = b_has_match.unwrap().1.clone();
                        prev_first_tile = curr_tile.clone();
                        curr_layer.push(curr_tile.content.clone());
                        curr_layer_ids.push(curr_tile.id.to_string());
                        curr_tile_id = tile_matches
                            .iter()
                            .find(|(id, _)| *id == r_has_match.unwrap().0)
                            .unwrap();
                        prev_tile = curr_tile.clone();
                        curr_tile = tiles_by_id.get_mut(curr_tile_id.0).unwrap();
                        break;
                    }
                    if i == 4 {
                        curr_tile.flip(true);
                    } else {
                        curr_tile.rotate(1);
                    }
                }
            } else if col == 0 {
                for i in 0..16 {
                    let r_has_match = curr_tile_id
                        .1
                        .iter()
                        .find(|(_, m)| curr_tile.is_side_adj(&Side::Right, m));
                    let b_has_match = curr_tile_id
                        .1
                        .iter()
                        .find(|(_, m)| curr_tile.is_side_adj(&Side::Bottom, m));
                    let t_is_strict_match =
                        curr_tile.is_side_strict_adj(&Side::Top, &prev_first_tile);
                    if r_has_match.is_some()
                        && ((b_has_match.is_some() && row < end - 1)
                            || (b_has_match.is_none() && row == end - 1))
                        && t_is_strict_match
                    {
                        if row < end - 1 {
                            next_first_tile = b_has_match.unwrap().1.clone();
                        }
                        prev_first_tile = curr_tile.clone();
                        curr_layer.push(curr_tile.content.clone());
                        curr_layer_ids.push(curr_tile.id.to_string());
                        curr_tile_id = tile_matches
                            .iter()
                            .find(|(id, _)| *id == r_has_match.unwrap().0)
                            .unwrap();
                        prev_tile = curr_tile.clone();
                        curr_tile = tiles_by_id.get_mut(curr_tile_id.0).unwrap();
                        break;
                    }
                    if i == 8 {
                        curr_tile.flip(true);
                    } else {
                        curr_tile.rotate(1);
                    }
                }
            } else if col == end - 1 {
                for i in 0..16 {
                    let l_is_strict_match = curr_tile.is_side_strict_adj(&Side::Left, &prev_tile);
                    if l_is_strict_match {
                        curr_layer.push(curr_tile.content.clone());
                        curr_layer_ids.push(curr_tile.id.to_string());
                        break;
                    }
                    if i == 8 {
                        curr_tile.flip(true);
                    } else {
                        curr_tile.rotate(1);
                    }
                }
            } else {
                for i in 0..16 {
                    let l_is_strict_match = curr_tile.is_side_strict_adj(&Side::Left, &prev_tile);
                    let r_has_match = curr_tile_id
                        .1
                        .iter()
                        .find(|(_, m)| curr_tile.is_side_adj(&Side::Right, m));
                    if l_is_strict_match && r_has_match.is_some() {
                        curr_layer.push(curr_tile.content.clone());
                        curr_layer_ids.push(curr_tile.id.to_string());
                        prev_tile = curr_tile.clone();
                        curr_tile_id = tile_matches
                            .iter()
                            .find(|(id, _)| *id == r_has_match.unwrap().0)
                            .unwrap();
                        curr_tile = tiles_by_id.get_mut(curr_tile_id.0).unwrap();
                        break;
                    }
                    if i == 8 {
                        curr_tile.flip(true);
                    } else {
                        curr_tile.rotate(1);
                    }
                }
            }
            col += 1;
        }
        *curr_tile = next_first_tile.clone();
        curr_tile_id = tile_matches
            .iter()
            .find(|(id, _)| id.to_string() == curr_tile.id)
            .unwrap();
        let mut image_without_border: Vec<String> = Vec::new();
        let dimension = curr_layer[0].len();
        for i in 1..dimension - 1 {
            image_without_border.push(
                curr_layer
                    .iter()
                    .map(|l| l[i as usize].get(1..dimension - 1).unwrap().to_string())
                    .join(""),
            );
        }
        image.push(image_without_border);
        col = 0;
        row += 1;
    }
    let full_image = image.into_iter().flatten().collect::<Vec<String>>();
    let mut image_tile = Tile {
        id: String::from("0000"),
        sides: HashMap::new(), // Don't care about this
        content: full_image,
    };
    let monster_plaintext = vec![
        "..................#.",
        "#....##....##....###",
        ".#..#..#..#..#..#...",
    ];
    let monster_len = monster_plaintext[0].len();
    let monster_ht = monster_plaintext.len();
    let monster = monster_plaintext
        .iter()
        .map(|t| Regex::new(t).unwrap())
        .collect::<Vec<Regex>>();
    let mut monster_pixels: Vec<Vec<char>> = Vec::new();
    let mut found_match = false;
    for i in 0..9 {
        for c in image_tile.content.iter() {
            monster_pixels.push(c.clone().chars().collect::<Vec<char>>());
        }
        for row in 0..(image_tile.content.len() - monster_ht) {
            for col in 0..(image_tile.content[0].len() - monster_len) {
                let monster_found = monster.iter().enumerate().all(|(j, mon)| {
                    let image_row = row + j;
                    let chunk = image_tile.content[image_row]
                        .get(col..col + monster_len)
                        .unwrap();
                    mon.is_match(chunk)
                });
                if monster_found {
                    found_match = true;
                    for (j, mon) in monster_plaintext.iter().enumerate() {
                        let image_row = row + j;
                        let chunk = monster_pixels[image_row]
                            .get_mut(col..col + monster_len)
                            .unwrap();
                        for (y, mon_ch) in mon.chars().enumerate() {
                            if mon_ch == '#' {
                                chunk[y] = 'O';
                            }
                        }
                    }
                }
            }
        }
        if found_match {
            break;
        }
        if i == 4 {
            image_tile.flip(true);
        } else {
            image_tile.rotate(1);
        }
        monster_pixels.clear();
    }
    monster_pixels.iter().fold(0, |acc, l| {
        l.iter().filter(|&&c| c == '#').count() + acc as usize
    })
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
