use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn main() {
    let lines = read_lines_as_str("./day14.input");
    let answer1 = solve1(&lines);
    println!("Answer 1 {}", answer1);
    // let mut answer2 = 0;
    // println!("Answer 2 {}", answer2);
}

fn solve1(lines: &Vec<String>) -> usize {
    let mut sum = 0;
    let mut current_bitmask = String::from("");
    let mut bitmask_group: Vec<String> = Vec::new();
    let mut memory: HashMap<usize, usize> = HashMap::new();
    for (index, line) in lines.iter().enumerate() {
        if line.contains("mask =") && bitmask_group.len() == 0 {
            current_bitmask = line.strip_prefix("mask = ").unwrap().to_string();
        } else if index == lines.len() - 1
            || lines.iter().nth(index + 1).unwrap().contains("mask =")
        {
            bitmask_group.push(line.to_string());
            // evaluate bitmask_group and current_bitmask
            for write_bit in bitmask_group.iter() {
                let (key, val) = parse_bitmask(current_bitmask.to_string(), write_bit.to_string());
                memory.insert(key, val);
            }
            bitmask_group.clear();
        } else {
            bitmask_group.push(line.to_string());
        }
    }

    for (_, mem) in memory {
        sum += mem;
    }
    sum
}

fn parse_bitmask(bitmask: String, write_bit: String) -> (usize, usize) {
    let parse_write_bit: Vec<&str> = write_bit.split("] = ").collect();
    let mem_addr: usize = parse_write_bit[0]
        .strip_prefix("mem[")
        .unwrap()
        .parse::<usize>()
        .expect("invalid memory address");
    let val: usize = parse_write_bit[1]
        .parse::<usize>()
        .expect("invalid value to write to memory");
    let mut binary_val = format!("{:036b}", val);
    for (index, bit) in bitmask.chars().enumerate() {
        if bit != 'X' {
            let mut tmp_binary_val: Vec<char> = binary_val.chars().collect::<Vec<char>>();
            tmp_binary_val[index] = bit;
            binary_val = tmp_binary_val.into_iter().collect::<String>();
        }
    }
    (mem_addr, usize::from_str_radix(&binary_val, 2).unwrap())
}

fn solve2() -> i32 {
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

// #[test]
// fn check_solve_1_sample() {
//     // assert_eq!(solve1(), );
// }
//
// #[test]
// fn check_solve_2_sample() {
//     // assert_eq!(solve1(), );
// }
