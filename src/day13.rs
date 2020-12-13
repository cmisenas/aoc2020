use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn main() {
    let lines = read_lines_as_str("./day13.input");
    let earliest = lines.iter().nth(0).unwrap().parse::<i64>().unwrap();
    let bus_times: Vec<i64> = lines
        .iter()
        .nth(1)
        .unwrap()
        .split(",")
        .filter(|&t| t != "x")
        .map(|t| t.parse::<i64>().unwrap())
        .collect();
    let answer1 = solve1(earliest, &bus_times);
    println!("Answer 1 {}", answer1);
    let answer2 = solve2(&bus_times);
    println!("Answer 2 {}", answer2);
}

fn solve1(earliest: i64, bus_times: &Vec<i64>) -> i64 {
    let mut earliest_bus_time = 0;
    let mut earliest_bus_id = 0;
    for b in bus_times {
        let mut time = 0;
        while time < earliest {
            time += b;
        }
        if earliest_bus_id == 0 || earliest_bus_time > time {
            earliest_bus_id = *b;
            earliest_bus_time = time;
        }
    }
    earliest_bus_id * (earliest_bus_time - earliest)
}

fn solve2(bus_times: &Vec<i64>) -> i64 {
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
