use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let lines = read_lines_as_str("./day4.input");
    let mut valid_pp = 0;
    let mut curr_pp = "".to_string();
    for (i, line) in lines.iter().enumerate() {
        if line == "" || i + 1 == lines.len() {
            if i + 1 == lines.len() {
                curr_pp.push_str(&line);
            }
            println!("Passport: {} ", curr_pp);
            println!("Is valid?: {} ", is_valid_pp(curr_pp.to_string()));
            if is_valid_pp(curr_pp.to_string()) {
                valid_pp = valid_pp + 1;
            }
            curr_pp.clear();
        } else {
            curr_pp.push_str("\n");
            curr_pp.push_str(&line);
        }
    }
    println!("valid passports: {}", valid_pp);
}

fn is_valid_pp(pp: String) -> bool {
    return pp.contains("byr:")
        && pp.contains("iyr:")
        && pp.contains("eyr:")
        && pp.contains("hgt:")
        && pp.contains("hcl:")
        && pp.contains("ecl:")
        && pp.contains("pid:");
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
