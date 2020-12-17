extern crate itertools;

use self::itertools::Itertools;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn main() {
    let lines = read_lines_as_str("./day16.input");
    let grouped_lines = &lines
        .into_iter()
        .group_by(|line| line == "")
        .into_iter()
        .filter_map(|(is_empty, line)| match !is_empty {
            true => Some(line.into_iter().collect::<Vec<String>>()),
            _ => None,
        })
        .collect::<Vec<Vec<String>>>();
    let rules = grouped_lines[0]
        .clone()
        .into_iter()
        .map(|rule| {
            let split_rule = rule.split(": ").collect::<Vec<&str>>();
            (
                split_rule[0].to_string(),
                split_rule[1]
                    .split(" or ")
                    .map(|s| {
                        let parse: Vec<usize> = s
                            .split("-")
                            .map(|i| i.parse::<usize>().expect("unable to parse int"))
                            .collect();
                        (parse[0], parse[1])
                    })
                    .collect::<Vec<(usize, usize)>>(),
            )
        })
        .collect::<Vec<(String, Vec<(usize, usize)>)>>();
    let my_ticket = grouped_lines[1][1]
        .split(",")
        .map(|i| i.parse::<usize>().expect("unable to parse int"))
        .collect::<Vec<usize>>();
    let nearby_tickets = grouped_lines[2][1..]
        .to_vec()
        .iter()
        .map(|t| {
            t.split(",")
                .map(|v| v.parse::<usize>().expect("invalid int"))
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    let answer1 = solve1(&rules, &nearby_tickets);
    println!("Answer 1 {}", answer1);
    let answer2 = solve2(&my_ticket, &rules, &nearby_tickets);
    println!("Answer 2 {}", answer2);
}

fn is_valid(val: usize, rule: &Vec<(usize, usize)>) -> bool {
    (val >= rule[0].0 && val <= rule[0].1) || (val >= rule[1].0 && val <= rule[1].1)
}

fn solve1(rules: &Vec<(String, Vec<(usize, usize)>)>, tickets: &Vec<Vec<usize>>) -> usize {
    tickets.iter().fold(0, |acc1, ticket| {
        acc1 + ticket
            .iter()
            .filter_map(
                |v| match !rules.iter().any(|(_, rule)| is_valid(*v, &rule)) {
                    true => Some(v),
                    _ => None,
                },
            )
            .fold(0, |acc2, j| acc2 + j)
    })
}

fn solve2(
    my_t: &Vec<usize>,
    rs: &Vec<(String, Vec<(usize, usize)>)>,
    ts: &Vec<Vec<usize>>,
) -> usize {
    let valid_ts = ts
        .iter()
        .filter_map(|t| {
            // t is ticket fields, f is a ticket field, rs is rules, r is a rule
            match t.iter().all(|f| rs.iter().any(|(_, r)| is_valid(*f, &r))) {
                true => Some(t),
                _ => None,
            }
        })
        .collect::<Vec<&Vec<usize>>>();
    let mut field_vals: Vec<(usize, Vec<usize>)> = (0..my_t.len())
        .into_iter()
        .map(|i| {
            // Iterate all ticket values per field and check if all values in that field
            // are valid for a rule. If so, that rule is a potential match to that field.
            // Return index of the field value and indices of potential matches to rules.
            (
                i,
                rs.iter()
                    .enumerate()
                    .filter_map(|(j, (_, r))| {
                        match valid_ts.iter().map(|t| t[i]).all(|v| is_valid(v, &r)) {
                            true => Some(j as usize),
                            _ => None,
                        }
                    })
                    .collect::<Vec<usize>>(),
            )
        })
        .collect::<Vec<(usize, Vec<usize>)>>();
    field_vals.sort_by(|a, b| a.1.len().cmp(&b.1.len()));

    let mut rule_i: HashMap<usize, (usize, &str)> = HashMap::new();
    for (field_index, possible_vals) in field_vals.iter() {
        let found_rule = possible_vals
            .iter()
            .find(|v| !rule_i.contains_key(&v))
            .unwrap();
        rule_i.insert(*found_rule, (*field_index, &rs[*found_rule].0));
    }
    rule_i
        .iter()
        .fold(1, |a, (_, v)| match v.1.contains("departure") {
            true => my_t[v.0] * a,
            _ => a * 1,
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
