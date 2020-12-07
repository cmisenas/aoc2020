extern crate regex;

use self::regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct BagRule {
    bag: String,
    qt: i32,
}

struct BagRules {
    bag: String,
    rules: HashMap<String, i32>,
}

pub fn main() {
    let lines = read_lines_as_str("./day7.input");
    let mut answer1 = 0;
    let mut answer2 = 0;
    let mut final_rules = HashMap::new();
    let main_bag = "shiny gold";

    for line in lines {
        let rule = get_bag_rule(line);
        final_rules.insert(rule.bag, rule.rules);
    }

    answer1 = solve1(main_bag.to_string(), final_rules.clone());
    answer2 = solve2(main_bag.to_string(), final_rules.clone());

    println!("Answer 1 {}", answer1);
    println!("Answer 2 {}", answer2);
}

fn solve1(main_bag: String, rules: HashMap<String, HashMap<String, i32>>) -> i32 {
    let mut count = 0;
    let mut keep_search = true;
    let mut search_these: HashSet<String> = HashSet::new();
    let mut counted: HashSet<String> = HashSet::new();
    search_these.insert(main_bag.to_string());

    while keep_search {
        for b in search_these.clone().iter() {
            for (bag, rule) in &rules {
                let rule_bags: Vec<String> = rule.keys().cloned().collect();
                if rule_bags.contains(b) && !counted.contains(bag) {
                    search_these.insert(bag.to_string());
                    counted.insert(bag.to_string());
                    count += 1;
                }
                search_these.remove(b);
            }
        }
        if search_these.len() == 0 {
            keep_search = false;
        }
    }
    count
}

fn solve2(main_bag: String, rules: HashMap<String, HashMap<String, i32>>) -> i32 {
    let mut count = 0;
    let empty_hm = HashMap::new();
    let bags_within: HashMap<String, i32> = rules.get(&main_bag).unwrap_or(&empty_hm).clone();
    for (bw, qt) in &bags_within {
        count += qt;
        count += qt * solve2(bw.to_string(), rules.clone());
    }
    count
}

fn get_bag_rule(rule: String) -> BagRules {
    let a: Vec<&str> = rule.split(" bags contain ").collect();
    let b: String = a[1].to_string();
    let c: Vec<&str> = b.split(", ").collect();
    let mut bag_rule = HashMap::new();
    for d in c {
        let parsed = parse_bag_rule(d.to_string());
        bag_rule.insert(parsed.bag, parsed.qt);
    }
    BagRules {
        bag: a[0].to_string(),
        rules: bag_rule,
    }
}

fn parse_bag_rule(rule: String) -> BagRule {
    let a = rule.find(' ').unwrap();
    let qt = rule
        .get(0..a)
        .expect("invalid string qt")
        .parse()
        .unwrap_or(0);
    let bag = rule.get(a..).expect("invalid string bag");
    let strip_bag = Regex::new(r"\bbags?\b\.?").unwrap();
    BagRule {
        bag: strip_bag.replace(bag, "").trim().to_string(),
        qt: qt,
    }
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
