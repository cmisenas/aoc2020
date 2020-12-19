extern crate itertools;
extern crate regex;

use self::itertools::Itertools;
use self::regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn main() {
    let lines = read_lines_as_str("./day19.input");
    let grouped_lines = &lines
        .into_iter()
        .group_by(|line| line == "")
        .into_iter()
        .filter_map(|(is_empty, line)| match !is_empty {
            true => Some(line.into_iter().collect::<Vec<String>>()),
            _ => None,
        })
        .collect::<Vec<Vec<String>>>();
    let mut rules: HashMap<String, String> = HashMap::new();
    for rule in grouped_lines[0].iter() {
        let parsed_rule: Vec<&str> = rule.split(": ").collect();
        rules.insert(parsed_rule[0].to_string(), parsed_rule[1].to_string());
    }
    let messages: Vec<String> = grouped_lines[1].clone();
    let answer1 = solve1(&rules, &messages);
    println!("Answer 1 {}", answer1);
    // let answer2 = solve2(&lines);
    // println!("Answer 2 {}", answer2);
}

fn solve1(rules: &HashMap<String, String>, messages: &[String]) -> i32 {
    // Find rule 0
    // Keep substituting numbers until a and b rule is found
    // Expand the rule
    let parsed_rule = form_rule(String::from("0"), rules);
    let mut regex_str = String::from("\\b");
    regex_str.push_str(&parsed_rule);
    regex_str.push_str("\\b");
    println!("Parsed rule is {}", regex_str);
    let rule_regex = Regex::new(&regex_str).unwrap();
    let mut valid = 0;
    for message in messages.iter() {
        if rule_regex.is_match(&&message) {
            valid += 1;
        }
    }
    valid
}

fn form_rule(rule: String, rules: &HashMap<String, String>) -> String {
    let formed_rule = String::from("");
    let mut expanded: Vec<String> = Vec::new();
    let starting_rule = rules.get_key_value(&rule).unwrap();
    if starting_rule.1.contains('|') {
        let or_rule: Vec<&str> = starting_rule.1.split('|').map(|s| s.trim()).collect();
        let or_rule1: Vec<&str> = or_rule[0].split(' ').collect();
        let or_rule2: Vec<&str> = or_rule[1].split(' ').collect();
        let mut or_statement = String::from("(");
        for orule1 in or_rule1.iter() {
            or_statement.push_str(&form_rule(orule1.to_string(), rules));
        }
        or_statement.push('|');
        for orule2 in or_rule2.iter() {
            or_statement.push_str(&form_rule(orule2.to_string(), rules));
        }
        or_statement.push(')');
        expanded.push(or_statement.to_string());
    } else if starting_rule.1.contains('"') {
        expanded.push(starting_rule.1.replace('"', "").trim().to_string());
    } else {
        let and_rules: Vec<&str> = starting_rule.1.split(' ').map(|s| s.trim()).collect();
        let mut and_statement = String::from("");
        for arule in and_rules.iter() {
            and_statement.push_str(&form_rule(arule.to_string(), rules));
        }
        expanded.push(and_statement.to_string());
    }
    expanded.join("")
}

// fn solve2(rules: &[String], messages: &[String]) -> i32 {
//     0
// }

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
