extern crate itertools;

use self::itertools::Itertools;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn main() {
    let lines = read_lines_as_str("./day22.input");
    let cards = &lines
        .into_iter()
        .group_by(|line| line.is_empty())
        .into_iter()
        .filter_map(|(is_empty, line)| match !is_empty {
            true => Some(line.into_iter().collect::<Vec<String>>()),
            _ => None,
        })
        .collect::<Vec<Vec<String>>>();
    let player_1_cards = cards[0]
        .iter()
        .skip(1)
        .rev()
        .map(|n| n.parse::<usize>().expect("unable to parse number"))
        .collect::<Vec<usize>>();
    let player_2_cards = cards[1]
        .iter()
        .skip(1)
        .rev()
        .map(|n| n.parse::<usize>().expect("unable to parse number"))
        .collect::<Vec<usize>>();
    let answer1 = solve1(player_1_cards.clone(), player_2_cards.clone());
    println!("Answer 1 {}", answer1);
    // let answer2 = solve2(&lines);
    // println!("Answer 2 {}", answer2);
}

fn solve1(mut player_1_cards: Vec<usize>, mut player_2_cards: Vec<usize>) -> usize {
    loop {
        if player_1_cards.is_empty() || player_2_cards.is_empty() {
            break;
        }
        let p1card = player_1_cards.pop().unwrap();
        let p2card = player_2_cards.pop().unwrap();
        if p1card > p2card {
            player_1_cards.insert(0, p1card);
            player_1_cards.insert(0, p2card);
        } else {
            player_2_cards.insert(0, p2card);
            player_2_cards.insert(0, p1card);
        }
    }
    if player_1_cards.is_empty() {
        player_2_cards
            .iter()
            .enumerate()
            .fold(0, |acc, (i, card)| ((i + 1) * card) + acc)
    } else {
        player_1_cards
            .iter()
            .enumerate()
            .fold(0, |acc, (i, card)| ((i + 1) * card) + acc)
    }
}

fn solve2(lines: &[String]) -> i32 {
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

fn read_lines_as_int<P>(filename: P) -> Vec<i64>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("no such file");
    let buf = io::BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line").parse::<i64>().unwrap())
        .collect()
}
