use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn main() {
    let lines = read_lines_as_str("./day18.input");
    let answer1 = solve1(&lines);
    let answer2 = solve2(&lines);
    println!("Answer 1 {}", answer1);
    println!("Answer 2 {}", answer2);
}

fn parse_expr1(terms: Vec<String>) -> i64 {
    let mut left = 0;
    let mut op = "";
    for t in terms.iter() {
        let int = t.parse::<i64>();
        if !int.is_err() {
            if left == 0 {
                left = int.unwrap();
            } else {
                let right = int.unwrap();
                left = match op {
                    "*" => left * right,
                    _ => left + right,
                };
                op = "";
            }
        } else {
            op = t;
        }
    }
    left
}

fn parse_expr_with_paren(terms: Vec<String>, is_part_a: bool) -> i64 {
    let mut pointer = 0;
    let mut terms_to_reduce: Vec<String> = Vec::new();
    let mut reduced_terms: Vec<String> = Vec::new();
    let mut start_paren_count = 0;
    let mut end_paren_count = 0;
    while pointer < terms.len() {
        let current_term = terms[pointer].to_string();
        // Paren statement already started
        if start_paren_count > 0 {
            if current_term.contains("(") {
                start_paren_count += current_term.chars().filter(|&c| c == '(').count();
                terms_to_reduce.push(current_term);
            } else if current_term.contains(")") {
                end_paren_count += current_term.chars().filter(|&c| c == ')').count();
                terms_to_reduce.push(current_term);
                // We've found the end of the parenthesis statement
                if start_paren_count == end_paren_count {
                    // Remove the leading paren
                    terms_to_reduce[0] = terms_to_reduce[0].get(1..).unwrap().to_string();
                    // Remove the closing paren
                    let x = terms_to_reduce.len() - 1;
                    let y = terms_to_reduce[x].len();
                    terms_to_reduce[x] = terms_to_reduce[x].get(0..y - 1).unwrap().to_string();
                    let result = parse_expr_with_paren(terms_to_reduce.clone(), is_part_a);
                    reduced_terms.push(result.to_string());
                    terms_to_reduce.clear();
                    start_paren_count = 0;
                    end_paren_count = 0;
                }
            } else {
                terms_to_reduce.push(current_term);
            }
        // Start of a paren statement
        } else if current_term.contains("(") {
            start_paren_count += current_term.chars().filter(|&c| c == '(').count();
            terms_to_reduce.push(current_term);
        } else {
            reduced_terms.push(current_term);
        }
        pointer += 1;
    }
    if is_part_a {
        parse_expr1(reduced_terms)
    } else {
        parse_expr2(reduced_terms)
    }
}

fn solve1(lines: &Vec<String>) -> i64 {
    let mut sum = 0;
    for line in lines.iter() {
        let result = parse_expr_with_paren(line.split(" ").map(|l| l.to_string()).collect(), true);
        sum += result;
    }
    sum
}

fn parse_expr2(terms: Vec<String>) -> i64 {
    let mut reduced_terms = terms.clone();
    while reduced_terms.contains(&String::from("+")) {
        let (index, _) = reduced_terms
            .iter()
            .enumerate()
            .find(|(_, t)| t.to_string() == String::from("+"))
            .unwrap();
        let result = reduced_terms[index - 1].parse::<i64>().unwrap()
            + reduced_terms[index + 1].parse::<i64>().unwrap();
        reduced_terms.insert(index - 1, result.to_string());
        reduced_terms.remove(index);
        reduced_terms.remove(index);
        reduced_terms.remove(index);
    }
    parse_expr1(reduced_terms.clone())
}

fn solve2(lines: &Vec<String>) -> i64 {
    let mut sum = 0;
    for line in lines.iter() {
        let result = parse_expr_with_paren(line.split(" ").map(|l| l.to_string()).collect(), false);
        sum += result;
    }
    sum
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
