extern crate itertools;
extern crate regex;

use self::itertools::Itertools;
use self::regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn main() {
    let mut lines = read_lines_as_str("./day4.input");
    // Add an empty string at the end
    lines.push(String::from(""));
    /*
    Old solution
    let mut ans1 = 0;
    let mut ans2 = 0;
    let mut curr_pp = String::from("");
    for line in lines.iter() {
        if line == "" {
            if solve1(&curr_pp) {
                ans1 += 1;
            }
            if solve2(&curr_pp) {
                ans2 += 1;
            }
            curr_pp.clear();
        } else {
            curr_pp.push_str("\n");
            curr_pp.push_str(&line);
        }
    }
    */
    let passports: &Vec<String> = &lines
        .into_iter()
        .group_by(|line| line != "")
        .into_iter()
        .filter_map(|(not_empty, mut passport)| {
            if not_empty {
                Some(passport.join("\n"))
            } else {
                None
            }
        })
        //.filter(|(not_empty, _)| *not_empty)
        //.map(|(_, mut passport)| passport.join("\n"))
        .collect();
    let ans1 = passports
        .into_iter()
        .filter(|passport| solve1(passport))
        .count();
    let ans2 = passports
        .into_iter()
        .filter(|passport| solve2(passport))
        .count();
    println!("Answer 1: {} ", ans1);
    println!("Answer 2: {} ", ans2);
}

fn solve1(pp: &String) -> bool {
    let byr = "byr";
    let hgt = "hgt";
    let iyr = "iyr";
    let eyr = "eyr";
    let hcl = "hcl";
    let ecl = "ecl";
    let pid = "pid";
    return pp.contains(byr)
        && pp.contains(hgt)
        && pp.contains(iyr)
        && pp.contains(eyr)
        && pp.contains(hcl)
        && pp.contains(ecl)
        && pp.contains(pid);
}

fn solve2(pp: &String) -> bool {
    let byr = Regex::new(r"\bbyr:(19[2-9]\d|200[0-2])\b").unwrap();
    let hgt = Regex::new(r"\bhgt:(1([5-8]\d|9[0-3])cm|(59|6\d|7[0-6])in)\b").unwrap();
    let iyr = Regex::new(r"\biyr:20(1\d|20)\b").unwrap();
    let eyr = Regex::new(r"\beyr:20(2\d|30)\b").unwrap();
    let hcl = Regex::new(r"\bhcl:#[0-9a-fA-F]{6}\b").unwrap();
    let ecl = Regex::new(r"\becl:(amb|blu|brn|gry|grn|hzl|oth)\b").unwrap();
    let pid = Regex::new(r"\bpid:\d{9}\b").unwrap();
    return byr.is_match(&&pp)
        && hgt.is_match(&&pp)
        && iyr.is_match(&&pp)
        && eyr.is_match(&&pp)
        && hcl.is_match(&&pp)
        && ecl.is_match(&&pp)
        && pid.is_match(&&pp);
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
