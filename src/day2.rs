use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn main() {
    let lines = read_lines("./day2.input");
    let ans1 = lines
        .iter()
        .filter(|l| is_valid(l.to_string(), first_rule))
        .count();
    let ans2 = lines
        .iter()
        .filter(|l| is_valid(l.to_string(), second_rule))
        .count();
    println!("Answer 1: {} ", ans1);
    println!("Answer 2: {} ", ans2);
}

fn is_valid(entry: String, f: fn(lo: usize, hi: usize, chr: char, p: &str) -> bool) -> bool {
    match entry.split_whitespace().collect::<Vec<&str>>().as_slice() {
        [range, chr, p] => {
            let r = range
                .split("-")
                .map(|n| n.parse::<usize>().expect("Could not be parsed"))
                .collect::<Vec<usize>>();

            let ch = chr.chars().nth(0).unwrap();
            f(r[0], r[1], ch, p)
        }
        _ => false,
    }
}

fn first_rule(lo: usize, hi: usize, chr: char, pword: &str) -> bool {
    let occ = pword.chars().filter(|&c| c == chr).count();
    return occ >= lo && occ <= hi;
}

fn second_rule(lo: usize, hi: usize, chr: char, pword: &str) -> bool {
    let pw1 = pword.chars().nth(lo - 1).unwrap();
    let pw2 = pword.chars().nth(hi - 1).unwrap();
    return (pw1 == chr && pw2 != chr) || (pw2 == chr && pw1 != chr);
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
