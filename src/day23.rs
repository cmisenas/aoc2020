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
    let answer1 = solve1(cups.clone());
    let answer2 = solve2(cups.clone());
    println!("Answer 1 {:?}", answer1);
    println!("Answer 2 {}", answer2);
}

fn solve1(mut cups: Vec<usize>) -> String {
    let mut current_cup_i = 0;
    let mut pickup = current_cup_i + 1;
    let cups_amt = cups.len();
    let highest_cup = cups
        .iter()
        .fold(0, |max, cup| if cup > &max { *cup } else { max });
    for _ in 0..100 {
        let current_cup = cups[current_cup_i];
        let mut pick_up_cups: Vec<usize> = Vec::new();
        for _ in 0..3 {
            if pickup >= cups.len() {
                pickup = 0
            }
            pick_up_cups.push(cups.remove(pickup));
        }
        let mut comp = (current_cup - 1) as i16;
        for _ in 0..highest_cup {
            if let Some(position) = cups.iter().position(|&x| x == comp as usize) {
                if position < current_cup_i {
                    current_cup_i += 4;
                } else {
                    current_cup_i += 1;
                }
                while pick_up_cups.len() > 0 {
                    cups.insert(position + 1, pick_up_cups.pop().unwrap());
                }
                break;
            }
            comp -= 1;
            if comp < 0 {
                comp = highest_cup as i16;
            }
        }

        if current_cup_i >= cups_amt {
            current_cup_i = 0;
        }
        pickup = current_cup_i + 1;
        if pickup >= cups_amt {
            pickup = pickup % cups_amt;
        }
    }
    let start_pos = cups.iter().position(|&x| x == 1).unwrap();
    let split = cups.split_at(start_pos);
    let mut final_str = split.1[1..].to_vec();
    final_str.append(&mut split.0.to_vec());
    final_str.iter().map(|s| s.to_string()).collect::<String>()
}

fn solve2(cups: Vec<usize>) -> usize {
    let highest_cup = 1_000_000;
    let mut next: Vec<usize> = vec![0usize; highest_cup];
    // Need to be index 0 if we're making a linked list
    let mut cup_madness = cups
        .iter()
        .map(|c| (c - 1) as usize)
        .collect::<Vec<usize>>();
    cup_madness.append(&mut (9..highest_cup).collect::<Vec<usize>>());
    let mut last = cup_madness[cup_madness.len() - 1];
    for &i in cup_madness.iter() {
        next[last] = i;
        last = i;
    }
    let mut current_cup = cup_madness[0];

    for _ in 0..10_000_000 {
        let pickup1 = next[current_cup];
        let pickup2 = next[pickup1];
        let pickup3 = next[pickup2];

        let mut destination = match current_cup == 0 {
            true => highest_cup - 1,
            _ => current_cup - 1,
        };
        while destination == pickup1 || destination == pickup2 || destination == pickup3 {
            destination = match destination == 0 {
                true => highest_cup - 1,
                _ => destination - 1,
            };
        }
        next[current_cup] = next[pickup3];
        next[pickup3] = next[destination];
        next[destination] = pickup1;
        current_cup = next[current_cup];
    }
    (next[0] + 1) * (next[next[0]] + 1)
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
