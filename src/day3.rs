use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn main() {
    let lines = read_lines_as_str("./day3.input");
    let s1 = get_trees_enc(1, 1, &lines);
    let s2 = get_trees_enc(3, 1, &lines);
    let s3 = get_trees_enc(5, 1, &lines);
    let s4 = get_trees_enc(7, 1, &lines);
    let s5 = get_trees_enc(1, 2, &lines);
    println!(
        "{} * {} * {} * {} * {} = {}",
        s1,
        s2,
        s3,
        s4,
        s5,
        s1 * s2 * s3 * s4 * s5
    );
}

fn get_trees_enc(right: usize, down: usize, lines: &Vec<String>) -> usize {
    let mut row = 0;
    let mut col = 0;
    let mut trees = 0;
    let len = lines[0].len();
    let end_row = lines.len() - 1;
    while row != end_row {
        row = row + down;
        col = (col + right) % len;
        if lines[row].chars().nth(col).unwrap().to_string() == "#" {
            trees = trees + 1;
        }
    }
    return trees;
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
