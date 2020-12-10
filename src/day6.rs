extern crate itertools;

use self::itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn main() {
    let lines = read_lines_as_str("./day6.input");
    /* Old answer
    let mut sum_1 = 0;
    let mut sum_2 = 0;
    let mut curr_group = Vec::new();

    for line in lines {
        if line == "" {
            let ans = calc_group(&curr_group);
            sum_1 += ans[0];
            sum_2 += ans[1];
            curr_group.clear();
        } else {
            curr_group.push(line);
        }
    }
    */
    let (sum_1, sum_2) = &lines
        .into_iter()
        .group_by(|line| line != "")
        .into_iter()
        .filter_map(|(not_empty, answers)| {
            if not_empty {
                Some(calc_group(answers.collect::<Vec<String>>()))
            } else {
                None
            }
        })
        .fold((0, 0), |(acc1, acc2), (a1, a2)| (acc1 + a1, acc2 + a2));
    println!("Answers 1 {}", sum_1);
    println!("Answers 2 {}", sum_2);
}

fn calc_group(group: Vec<String>) -> (usize, usize) {
    let mut answers = HashSet::new();
    let mut answers_count = HashMap::new();
    for a in group.iter() {
        for b in a.chars() {
            let c = answers_count.entry(b).or_insert(0);
            *c += 1;
            answers.insert(b);
        }
    }
    let c = answers_count
        .iter()
        .filter(|(_, ac)| **ac == group.len())
        .count();
    (answers.len(), c)
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
