use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;
use std::collections::HashMap;

pub fn main() {
    let lines = read_lines_as_str("./day6.input");
    let mut sum_1 = 0;
    let mut sum_2 = 0;
    let mut curr_group = Vec::new();

    for line in lines {
        if line == "" {
            let ans = calc_group(&curr_group);
            sum_1 = sum_1 + ans[0];
            sum_2 = sum_2 + ans[1];
            curr_group.clear();
        } else {
            curr_group.push(line);
        }
    }
    println!("Sum 1 {}", sum_1);
    println!("Sum 2 {}", sum_2);
}

fn calc_group(group: &Vec<String>) -> Vec<usize> {
    let mut answers = HashSet::new();
    let mut answers_count = HashMap::new();
    for a in group {
        for b in a.chars() {
            let c = answers_count.entry(b).or_insert(0);
            *c += 1;
            answers.insert(b);
        }
    }
    let mut c = 0;
    for (k, ac) in answers_count.iter() {
        if *ac == group.len() {
            c = c + 1;
        }
    }
    vec![answers.len(), c]
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
