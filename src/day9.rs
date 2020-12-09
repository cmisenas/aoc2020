use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn main() {
    let lines = read_lines_as_int("./day9.input");
    let mut answer1 = 0;
    let preamble = 25;
    let mut answer2 = 0;
    answer1 = solve1(preamble, lines.clone());
    answer2 = solve2(answer1, lines.clone());

    println!("Answer 1 {}", answer1);
    println!("Answer 2 {}", answer2);
}

fn solve1(preamble: usize, nums: Vec<i64>) -> i64 {
    let mut prev_nums: Vec<i64> = Vec::new();
    let mut start = 0;
    let mut index: usize = preamble;
    let mut curr_num = nums[index];
    prev_nums.extend(nums[start..index].iter().cloned());
    while is_valid(curr_num, prev_nums.clone()) && index < nums.len() - 1 {
        prev_nums.push(nums[index]);
        prev_nums.remove(0);
        index += 1;
        start += 1;
        curr_num = nums[index];
    }
    curr_num
}

fn is_valid(num: i64, nums: Vec<i64>) -> bool {
    for (i, x) in nums[..nums.len() - 1].iter().enumerate() {
        for y in nums[i + 1..].iter() {
            if x + y == num {
                return true;
            }
        }
    }
    false
}

fn solve2(num: i64, nums: Vec<i64>) -> i64 {
    let mut size = 2;
    let mut start = 0;
    let mut range: Vec<i64> = Vec::new();
    range.extend(nums[start..start + size].iter().cloned());
    let mut not_found = true;
    while not_found {
        let sum = range.iter().sum::<i64>();
        if sum == num {
            not_found = false;
        } else {
            if (start + 1 + size) >= nums.len() {
                start = 0;
                size += 1;
            } else {
                start += 1;
            }
            range.clear();
            range.extend(nums[start..start + size].iter().cloned());
        }
    }
    range.sort();
    range[0] + range[range.len() - 1]
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
