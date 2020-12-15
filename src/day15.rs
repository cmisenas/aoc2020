use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn main() {
    let lines = read_lines_as_str("./day15.input");
    let answer1 = solve1(&lines[0]);
    println!("Answer 1 {}", answer1);
    //let answer2 = solve2(&line);
    //println!("Answer 2 {}", answer2);
}

fn solve1(line: &String) -> usize {
    let mut numbers: Vec<usize> = line
        .split(",")
        .map(|l| l.parse::<usize>().expect("unable to parse int"))
        .collect();
    let initial_len = numbers.len();
    let mut mem: HashMap<usize, usize> = HashMap::new();
    for (i, n) in numbers.iter().enumerate() {
        mem.insert(*n, i + 1);
    }
    for i in numbers.len()..2020 {
        let prev_n = numbers[i - 1];
        if mem.contains_key(&prev_n) && i > initial_len {
            let next_n = i - mem.get(&prev_n).unwrap();
            numbers.push(next_n);
        } else {
            numbers.push(0);
        }
        mem.insert(prev_n, i);
    }
    numbers[numbers.len() - 1]
}

fn solve2(lines: &Vec<String>) -> i32 {
    0
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
