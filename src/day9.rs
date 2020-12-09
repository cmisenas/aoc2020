use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn main() {
    let lines = read_lines_as_int("./day9.input");
    let preamble = 25;
    let answer1 = solve1(preamble, &lines);
    let answer2 = solve2(answer1, &lines);

    println!("Answer 1 {}", answer1);
    println!("Answer 2 {}", answer2);
}

fn solve1(preamble: usize, nums: &Vec<i64>) -> i64 {
    let (i, _) = nums[..nums.len() - preamble]
        .iter()
        .enumerate()
        .find(|(i, _)| !is_valid(nums[preamble + i], nums[*i..*i + preamble].to_vec()))
        .expect("No answer yielded");
    nums[i + preamble]
}

fn is_valid(num: i64, nums: Vec<i64>) -> bool {
    nums[..nums.len() - 1]
        .iter()
        .enumerate()
        .any(|(i, x)| nums[i + 1..].iter().any(|y| x + y == num))
}

fn solve2(num: i64, nums: &Vec<i64>) -> i64 {
    let mut size = 2;
    let mut index = 0;
    while size < nums.len() - 1 {
        let result = nums[..nums.len() - size]
            .iter()
            .enumerate()
            .find(|(i, _)| nums[*i..*i + size].iter().sum::<i64>() == num);
        if let Some((i, _)) = result {
            index = i;
            break;
        } else {
            size += 1;
        }
    }
    let mut range: Vec<i64> = nums[index..index + size].to_vec();
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
