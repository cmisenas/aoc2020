extern crate itertools;

use self::itertools::Itertools;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn main() {
    let lines = read_lines_as_str("./day16.input");
    let grouped_lines: &Vec<Vec<String>> = &lines
        .into_iter()
        .group_by(|line| line == "")
        .into_iter()
        .filter_map(|(is_empty, line)| {
            if is_empty {
                None
            } else {
                Some(line.into_iter().collect::<Vec<String>>())
            }
        })
        .collect();
    let rules: Vec<Vec<(i32, i32)>> = grouped_lines[0]
        .clone()
        .into_iter()
        .map(|rule| {
            rule.split(": ")
                .nth(1)
                .unwrap()
                .split(" or ")
                .map(|s| {
                    let parse: Vec<i32> = s
                        .split("-")
                        .map(|i| i.parse::<i32>().expect("unable to parse int"))
                        .collect();
                    (parse[0], parse[1])
                })
                .collect::<Vec<(i32, i32)>>()
        })
        .collect();
    let my_ticket: String = grouped_lines[1][1].to_string();
    let nearby_tickets: Vec<String> = grouped_lines[2][1..].to_vec();
    println!("Rules {:?}", rules);
    println!("My ticket {:?}", my_ticket);
    println!("Nearby ticket {:?}", nearby_tickets);

    let answer1 = solve1(&rules, &nearby_tickets);
    println!("Answer 1 {}", answer1);

    // let mut answer2 = 0;

    // println!("Answer 2 {}", answer2);
}

fn solve1(rules: &Vec<Vec<(i32, i32)>>, tickets: &Vec<String>) -> i32 {
    let mut invalid = 0;
    for ticket in tickets {
        let vals = ticket
            .split(",")
            .map(|v| v.parse::<i32>().expect("invalid int"))
            .collect::<Vec<i32>>();
        for v in vals {
            if !rules.iter().any(|rule| {
                (v >= rule[0].0 && v <= rule[0].1) || (v >= rule[1].0 && v <= rule[1].1)
            }) {
                invalid += v;
            }
        }
    }
    invalid
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
