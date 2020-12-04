use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn main() {
    let mut valid = 0;
    if let lines = read_lines("./day2.input") {
        for line in lines {
            if is_valid(line.to_string()) {
                valid = valid + 1;
            }
        }
    }
    println!("Valid pwords: {} ", valid);
}

fn is_valid(entry: String) -> bool {
    match entry.split_whitespace().collect::<Vec<&str>>().as_slice() {
        [range, chr, pword] => second_rule(range.to_string(), chr.to_string(), pword.to_string()),
        _ => false,
    }
}

fn first_rule(range: String, chr: String, pword: String) -> bool {
    let r = range.split("-").collect::<Vec<&str>>();
    let min = r[0].parse::<usize>().unwrap();
    let max = r[1].parse::<usize>().unwrap();
    let ch = chr.chars().nth(0).unwrap();
    let occ = pword.chars().filter(|&c| c == ch).count();
    return occ >= min && occ <= max;
}

fn second_rule(pos: String, chr: String, pword: String) -> bool {
    let r = pos.split("-").collect::<Vec<&str>>();
    let pos1 = r[0].parse::<usize>().unwrap();
    let pos2 = r[1].parse::<usize>().unwrap();
    let ch = chr.chars().nth(0).unwrap();
    let pw1 = pword.chars().nth(pos1 - 1).unwrap();
    let pw2 = pword.chars().nth(pos2 - 1).unwrap();
    return (pw1 == ch && pw2 != ch) || (pw2 == ch && pw1 != ch);
}

fn read_lines<P>(filename: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("no such file");
    let buf = io::BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
