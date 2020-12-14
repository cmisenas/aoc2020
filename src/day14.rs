extern crate itertools;

use self::itertools::Itertools;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn main() {
    let lines = read_lines_as_str("./day14.input");
    let (answer1, answer2) = solve(&lines);
    println!("Answer 1 {}", answer1);
    println!("Answer 2 {}", answer2);
}

fn solve(lines: &Vec<String>) -> (usize, usize) {
    let mut memory1: HashMap<usize, usize> = HashMap::new();
    let mut memory2: HashMap<usize, usize> = HashMap::new();
    let mask_groups: &Vec<Vec<String>> = &lines
        .into_iter()
        .enumerate()
        .group_by(|(_, line)| line.contains("mask ="))
        .into_iter()
        .map(|(_, bgroup)| {
            bgroup
                .into_iter()
                .map(|(_, b)| b.to_string())
                .collect::<Vec<String>>()
        })
        .collect();
    for (i, current_bitmask) in mask_groups.iter().enumerate().step_by(2) {
        let bitmask_group = &mask_groups.iter().nth(i + 1).unwrap();
        let parsed_bitmask = current_bitmask[0]
            .strip_prefix("mask = ")
            .unwrap()
            .to_string();
        for write_bit in bitmask_group.iter() {
            let (key, val) = parse_bitmask(&parsed_bitmask, write_bit.to_string());
            memory1.insert(key, val);
            let (addrs, val) = parse_bitmask2(&parsed_bitmask, write_bit.to_string());
            for addr in addrs.iter() {
                memory2.insert(*addr, val);
            }
        }
    }
    (
        memory1.iter().fold(0, |acc, (_, mem)| acc + mem),
        memory2.iter().fold(0, |acc, (_, mem)| acc + mem),
    )
}

fn parse_bitmask(bitmask: &String, write_bit: String) -> (usize, usize) {
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
            binary_val = replace_char_in_str(binary_val, bit, index);
        }
    }
    (mem_addr, usize::from_str_radix(&binary_val, 2).unwrap())
}

fn parse_bitmask2(bitmask: &String, write_bit: String) -> (Vec<usize>, usize) {
    let parse_write_bit: Vec<&str> = write_bit.split("] = ").collect();
    let mem_addr: usize = parse_write_bit[0]
        .strip_prefix("mem[")
        .unwrap()
        .parse::<usize>()
        .expect("invalid memory address");
    let val: usize = parse_write_bit[1]
        .parse::<usize>()
        .expect("invalid value to write to memory");
    let mut binary_val = format!("{:036b}", mem_addr);
    let mut all_addresses: Vec<usize> = Vec::new();
    for (index, bit) in bitmask.chars().enumerate() {
        if bit != '0' {
            binary_val = replace_char_in_str(binary_val, bit, index);
        }
    }
    let tmp_all_addresses = get_all_addresses(binary_val.to_string());
    for tmp in tmp_all_addresses.iter() {
        all_addresses.push(usize::from_str_radix(&tmp, 2).unwrap());
    }
    (all_addresses, val)
}

fn get_all_addresses(bitmask: String) -> Vec<String> {
    let mut result = Vec::new();
    let first_x = bitmask.find('X');
    let count_x = bitmask.chars().filter(|&c| c == 'X').count();
    if first_x.is_none() {
        result.push(bitmask);
    } else if count_x == 1 {
        result.push(bitmask.replace("X", "0"));
        result.push(bitmask.replace("X", "1"));
    } else {
        let str1 = replace_char_in_str(bitmask.to_string(), '0', first_x.unwrap());
        let str2 = replace_char_in_str(bitmask.to_string(), '1', first_x.unwrap());
        let mut replaced1 = get_all_addresses(str1);
        let mut replaced2 = get_all_addresses(str2);
        result.append(&mut replaced1);
        result.append(&mut replaced2);
    }
    result
}

fn replace_char_in_str(orig: String, repl: char, index: usize) -> String {
    let mut tmp_str: Vec<char> = orig.chars().collect::<Vec<char>>();
    tmp_str[index] = repl;
    tmp_str.into_iter().collect::<String>()
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
