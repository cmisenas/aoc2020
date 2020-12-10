use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn main() {
    let mut jolts = read_lines_as_int("./day10.input");

    // For the wall joltage
    jolts.push(0);
    jolts.sort();
    // For the in-device joltage
    jolts.push(jolts[jolts.len() - 1] + 3);

    let answer1 = solve1(&jolts);
    let answer2 = solve2(&jolts);

    println!("Answer 1 {}", answer1);
    println!("Answer 2 {}", answer2);
}

fn solve1(jolts: &Vec<i32>) -> i32 {
    let mut jolt1 = 0;
    let mut jolt3 = 0;

    for (i, jolt) in jolts.iter().skip(1).enumerate() {
        let dj = jolt - jolts[i];
        if dj == 1 {
            jolt1 += 1;
        } else {
            jolt3 += 1;
        }
    }
    jolt1 * jolt3
}

fn solve2(jolts: &Vec<i32>) -> usize {
    let mut inv: HashMap<i32, usize> = HashMap::new();
    let mut combinations: Vec<usize> = vec![0; jolts.len() - 1];
    combinations[jolts.len() - 2] = 1;
    for (i, jolt) in jolts.iter().enumerate() {
        inv.insert(*jolt, i);
    }

    for (i, jolt) in jolts.iter().take(jolts.len() - 2).rev().enumerate() {
        let mut sum = 0;
        for j in 1..4 {
            if let Some((_, v)) = inv.get_key_value(&(jolt + j)) {
                sum += combinations[*v];
            }
        }
        // Since we inversed, probably better to use for loop
        let index = jolts.len() - 2 - (i + 1);
        combinations[index] += sum;
    }
    combinations[0]
}

fn read_lines_as_int<P>(filename: P) -> Vec<i32>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("no such file");
    let buf = io::BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line").parse::<i32>().unwrap())
        .collect()
}
