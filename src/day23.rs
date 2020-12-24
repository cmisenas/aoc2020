use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn main() {
    let lines = read_lines_as_str("./day23.input");
    let cups = lines[0]
        .split("")
        .filter_map(|l| match l.is_empty() {
            true => None,
            _ => Some(l.parse::<usize>().unwrap()),
        })
        .collect::<Vec<usize>>();
    let answer1 = solve1(&cups);
    let answer2 = solve2(&cups);
    println!("Answer 1 {}", answer1);
    println!("Answer 2 {}", answer2);
}

fn solve1(cups: &[usize]) -> String {
    // Need to be index 0 if we're making a linked list
    let cup_madness = cups
        .iter()
        .map(|c| (c - 1) as usize)
        .collect::<Vec<usize>>();
    let highest_cup = cups
        .iter()
        .fold(0, |max, cup| if cup > &max { *cup } else { max });
    let cups_circle = move_cups(&cup_madness, 100, highest_cup);
    let mut final_str = String::from("");
    let mut curr_cup = cups_circle[0];
    while curr_cup != 0 {
        final_str.push_str(&(curr_cup + 1).to_string());
        curr_cup = cups_circle[curr_cup];
    }
    final_str
}

fn solve2(cups: &[usize]) -> usize {
    let highest_cup = 1_000_000;
    let rounds = 10_000_000;
    // Need to be index 0 if we're making a linked list
    let mut cup_madness = cups
        .iter()
        .map(|c| (c - 1) as usize)
        .collect::<Vec<usize>>();
    cup_madness.append(&mut (9..highest_cup).collect::<Vec<usize>>());
    let cups_circle = move_cups(&cup_madness, rounds, highest_cup);
    (cups_circle[0] + 1) * (cups_circle[cups_circle[0]] + 1)
}

fn move_cups(cups: &[usize], rounds: usize, max: usize) -> Vec<usize> {
    let mut next: Vec<usize> = vec![0usize; max];
    let mut last = cups[cups.len() - 1];
    for &i in cups.iter() {
        next[last] = i;
        last = i;
    }
    let mut current_cup = cups[0];

    for _ in 0..rounds {
        let pickup1 = next[current_cup];
        let pickup2 = next[pickup1];
        let pickup3 = next[pickup2];

        let mut destination = match current_cup == 0 {
            true => max - 1,
            _ => current_cup - 1,
        };
        while destination == pickup1 || destination == pickup2 || destination == pickup3 {
            destination = match destination == 0 {
                true => max - 1,
                _ => destination - 1,
            };
        }
        next[current_cup] = next[pickup3];
        next[pickup3] = next[destination];
        next[destination] = pickup1;
        current_cup = next[current_cup];
    }
    next
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
