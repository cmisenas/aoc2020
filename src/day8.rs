use std::collections::HashMap;
/**
extern crate regex;

use self::regex::Regex;
*/
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

enum Instruction {
    Noop,
    Jmp,
    Acc,
}

pub fn main() {
    let lines = read_lines_as_str("./day8.input");
    let mut answer1 = 0;
    let mut answer2 = 0;
    let mut ins_set: Vec<(String, i32)> = Vec::new();

    for line in lines {
        // println!("Line {}", line);
        ins_set.push(parse_ins(line));
    }
    answer1 = solve1(ins_set);

    println!("Answer 1 {}", answer1);
    println!("Answer 2 {}", answer2);
}

fn solve1(ins_set: Vec<(String, i32)>) -> i32 {
    let mut acc_n: i32 = 0;
    let mut visited: HashSet<i32> = HashSet::new();
    let nop = "nop";
    let jmp = "jmp";
    let acc = "acc";
    let mut x: i32 = 0;
    let mut no_loop = true;
    while no_loop {
        if !visited.contains(&x) {
            let (curr_ins, curr_n) = &ins_set[x as usize];
            visited.insert(x);
            if curr_ins == jmp {
                x += curr_n;
            } else if curr_ins == acc {
                acc_n += curr_n;
                x += 1;
            } else {
                x += 1;
            }
        } else {
            no_loop = false;
        }
    }
    acc_n
}

fn solve2() -> usize {
    0
}

fn parse_ins(line: String) -> (String, i32) {
    let a: Vec<&str> = line.split(" ").collect();
    let b: String = a[0].to_string();
    // println!("Parsing {} and {}", a[0], a[1]);
    let c: i32 = a[1].parse::<i32>().expect("Unable to parse num");
    (b, c)
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
