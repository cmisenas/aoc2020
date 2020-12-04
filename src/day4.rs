extern crate regex;

use self::regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn main() {
    let lines = read_lines_as_str("./day4.input");
    let mut valid_pp = 0;
    let mut curr_pp = "".to_string();
    for (i, line) in lines.iter().enumerate() {
        if line == "" || i + 1 == lines.len() {
            if i + 1 == lines.len() {
                curr_pp.push_str("\n");
                curr_pp.push_str(&line);
            }
            if is_valid_pp(curr_pp.to_string()) {
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
