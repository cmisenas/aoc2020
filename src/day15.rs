use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn main() {
    let lines = read_lines_as_str("./day15.input");
    let (answer1, answer2) = solve(&lines[0]);
    println!("Answer 1 {}", answer1);
    println!("Answer 2 {}", answer2);
}

fn solve(line: &String) -> (usize, usize) {
    let numbers: Vec<usize> = line
        .split(",")
        .map(|l| l.parse::<usize>().expect("unable to parse int"))
        .collect();
    let mut ans1 = 0;
    let initial_len = numbers.len();
    let mut mem: HashMap<usize, usize> = HashMap::new();
    for (i, n) in numbers.iter().enumerate() {
        mem.insert(*n, i + 1);
    }
    let mut prev_n = numbers[initial_len - 1];
    for i in initial_len..30000000 {
        let next_n = if let Some(val) = mem.get(&prev_n) {
            i - val
        } else {
            0
        };
        mem.insert(prev_n, i);
        if i == 2019 {
            ans1 = next_n;
        }
        prev_n = next_n;
    }
    (ans1, prev_n)
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
