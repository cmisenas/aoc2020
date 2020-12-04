extern crate regex;

use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let lines = read_lines_as_str("./day4.input");
    let mut not_done = true;
    let mut valid_pp = 0;
    let mut curr_pp = "".to_string();
    for (i, line) in lines.iter().enumerate() {
        if line == "" || i + 1 == lines.len() {
            if i + 1 == lines.len() {
                curr_pp.push_str(&line);
            }
            println!("Passport: {} ", curr_pp);
            println!("Is valid?: {} ", is_valid_pp(curr_pp.to_string()));
            if (is_valid_pp(curr_pp.to_string())) {
                valid_pp = valid_pp + 1;
            }
            curr_pp.clear();
        } else {
            curr_pp.push_str("\n");
            curr_pp.push_str(&line);
        }
    }
    println!("valid passports: {}", valid_pp);
}

fn is_valid_pp(pp: String) -> bool {
    // let cid = "cid:";
    let byr = Regex::new(r"byr:").unwrap();
    let iyr = Regex::new(r"iyr:").unwrap();
    let eyr = Regex::new(r"eyr:").unwrap();
    let hgt = Regex::new(r"hgt:").unwrap();
    let hcl = Regex::new(r"hcl:[0-9A-F]+").unwrap();
    let ecl = Regex::new(r"ecl:(amb|blu|brn|gry|grn|hzl|oth)").unwrap();
    let pid = Regex::new(r"pid:\d{9}").unwrap();
    return pp.contains("byr:")
        && pp.contains("iyr:")
        && pp.contains("eyr:")
        && pp.contains("hgt:")
        && hcl.is_match(&&pp)
        && ecl.is_match(&&pp)
        && pid.is_match(&&pp);
}

fn is_valid_byr(pp: String) -> bool {
    true
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
