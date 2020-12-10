use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn main() {
    let mut jolts = read_lines_as_int("./day10.input");
    let mut answer1 = 0;
    let mut answer2 = 0;
    let mut jolt1 = 0;
    let mut jolt2 = 0;
    let mut jolt3 = 0;

    // For the wall joltage
    jolts.push(0);
    jolts.sort();
    // For the in-device joltage
    jolts.push(jolts[jolts.len() - 1] + 3);

    for (i, jolt) in jolts.iter().skip(1).enumerate() {
        let dj = jolt - jolts[i];
        if dj == 1 {
            jolt1 += 1;
        } else if dj == 2 {
            jolt2 += 1;
        } else {
            jolt3 += 1;
        }
    }
    answer1 = jolt1 * jolt3;

    println!("Answer 1 {}", answer1);
    println!("Answer 2 {}", answer2);
}

fn solve1() -> i32 {
    0
}

fn solve2() -> i32 {
    0
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
