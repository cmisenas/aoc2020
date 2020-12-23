extern crate itertools;

use self::itertools::Itertools;
use std::collections::HashSet;
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
    let answer2 = solve2(player_1_cards.clone(), player_2_cards.clone());
    println!("Answer 2 {}", answer2);
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
        calc_score(&player_2_cards)
    } else {
        calc_score(&player_1_cards)
    }
}

fn solve2(player_1_cards: Vec<usize>, player_2_cards: Vec<usize>) -> usize {
    let results = run_recursive_combat(player_1_cards, player_2_cards);
    match results.0.len() > 0 {
        true => calc_score(&results.0),
        _ => calc_score(&results.1),
    }
}

fn run_recursive_combat(
    mut player_1_cards: Vec<usize>,
    mut player_2_cards: Vec<usize>,
) -> (Vec<usize>, Vec<usize>) {
    let mut infinite_prevention1: HashSet<String> = HashSet::new();
    // let mut infinite_prevention2: HashSet<String> = HashSet::new();
    let mut recursion_found = false;
    loop {
        let current_cards1 = player_1_cards.iter().map(|c| c.to_string()).join(",");
        // let current_cards2 = player_2_cards.iter().map(|c| c.to_string()).join(",");
        if player_1_cards.is_empty() || player_2_cards.is_empty() {
            break;
        }
        if infinite_prevention1.contains(&current_cards1)
        // || infinite_prevention2.contains(&current_cards2)
        {
            recursion_found = true;
            break;
        }
        infinite_prevention1.insert(current_cards1.to_string());
        //infinite_prevention2.insert(current_cards2.to_string());

        let p1card = player_1_cards.pop().unwrap();
        let p2card = player_2_cards.pop().unwrap();

        if player_1_cards.len() >= p1card && player_2_cards.len() >= p2card {
            let p1cards_copy = player_1_cards[player_1_cards.len() - p1card..].to_vec();
            let p2cards_copy = player_2_cards[player_2_cards.len() - p2card..].to_vec();
            let results = run_recursive_combat(p1cards_copy, p2cards_copy);
            if results.0.len() > 0 {
                player_1_cards.insert(0, p1card);
                player_1_cards.insert(0, p2card);
            } else {
                player_2_cards.insert(0, p2card);
                player_2_cards.insert(0, p1card);
            }
        } else if p1card > p2card {
            player_1_cards.insert(0, p1card);
            player_1_cards.insert(0, p2card);
        } else {
            player_2_cards.insert(0, p2card);
            player_2_cards.insert(0, p1card);
        }
    }
    if recursion_found {
        (player_1_cards, Vec::new())
    } else {
        (player_1_cards, player_2_cards)
    }
}

fn calc_score(cards: &Vec<usize>) -> usize {
    cards
        .iter()
        .enumerate()
        .fold(0, |acc, (i, card)| ((i + 1) * card) + acc)
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
