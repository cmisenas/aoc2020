extern crate itertools;

use self::itertools::Itertools;
use std::collections::HashMap;
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
    let rule_names: Vec<String> = grouped_lines[0]
        .clone()
        .into_iter()
        .map(|rule| rule.split(": ").nth(0).unwrap().to_string())
        .collect::<Vec<String>>();
    let rules: Vec<Vec<(usize, usize)>> = grouped_lines[0]
        .clone()
        .into_iter()
        .map(|rule| {
            rule.split(": ")
                .nth(1)
                .unwrap()
                .split(" or ")
                .map(|s| {
                    let parse: Vec<usize> = s
                        .split("-")
                        .map(|i| i.parse::<usize>().expect("unable to parse int"))
                        .collect();
                    (parse[0], parse[1])
                })
                .collect::<Vec<(usize, usize)>>()
        })
        .collect();
    let my_ticket: Vec<usize> = grouped_lines[1][1]
        .split(",")
        .map(|i| i.parse::<usize>().expect("unable to parse int"))
        .collect();
    let nearby_tickets: Vec<String> = grouped_lines[2][1..].to_vec();

    let answer1 = solve1(&rules, &nearby_tickets);
    println!("Answer 1 {}", answer1);

    let answer2 = solve2(&my_ticket, &rule_names, &rules, &nearby_tickets);
    println!("Answer 2 {}", answer2);
}

fn solve1(rules: &Vec<Vec<(usize, usize)>>, tickets: &Vec<String>) -> usize {
    let mut invalid = 0;
    for ticket in tickets {
        let vals = ticket
            .split(",")
            .map(|v| v.parse::<usize>().expect("invalid int"))
            .collect::<Vec<usize>>();
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

fn solve2(
    my_ticket: &Vec<usize>,
    rule_names: &Vec<String>,
    rules: &Vec<Vec<(usize, usize)>>,
    tickets: &Vec<String>,
) -> usize {
    let valid_tickets: Vec<Vec<usize>> = tickets
        .iter()
        .filter_map(|ticket| {
            let parsed_ticket = ticket
                .split(",")
                .map(|v| v.parse::<usize>().expect("invalid int"))
                .collect::<Vec<usize>>();
            if parsed_ticket.iter().all(|v| {
                rules.into_iter().any(|rule| {
                    (v >= &rule[0].0 && v <= &rule[0].1) || (v >= &rule[1].0 && v <= &rule[1].1)
                })
            }) {
                Some(parsed_ticket)
            } else {
                None
            }
        })
        .collect();
    let mut per_field: Vec<(usize, Vec<usize>)> = (0..valid_tickets[0].len())
        .into_iter()
        .map(|i| {
            valid_tickets
                .iter()
                .map(|ticket| ticket[i])
                .collect::<Vec<usize>>()
        })
        .enumerate()
        .map(|(j, field_vals)| {
            (
                j,
                rules
                    .iter()
                    .enumerate()
                    .filter(|(_, rule)| {
                        field_vals.iter().all(|v| {
                            (v >= &rule[0].0 && v <= &rule[0].1)
                                || (v >= &rule[1].0 && v <= &rule[1].1)
                        })
                    })
                    .map(|(i, _)| i as usize)
                    .collect::<Vec<usize>>(),
            )
        })
        .collect::<Vec<(usize, Vec<usize>)>>();
    per_field.sort_by(|a, b| a.1.len().cmp(&b.1.len()));

    let mut result: usize = 1;
    let mut claimed_rule_index: HashMap<usize, (usize, String)> = HashMap::new();
    for (field_index, possible_vals) in per_field.iter() {
        let found_rule = possible_vals
            .iter()
            .filter(|v| !claimed_rule_index.contains_key(&v))
            .nth(0)
            .unwrap();
        claimed_rule_index.insert(
            *found_rule,
            (*field_index, rule_names[*found_rule].to_string()),
        );
    }
    for (_, val) in claimed_rule_index.iter() {
        if val.1.contains("departure") {
            result *= my_ticket[val.0];
        }
    }

    result
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
