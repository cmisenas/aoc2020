use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn main() {
    let lines = read_lines("./day1.input");
    /*
    let mut i = 0;
    while i < lines.len() {
        // TODO: Modify solution so this can solve part 2
        if let Some((a, b)) = vec![lines[i]; lines[i..].len()]
            .iter()
            .zip(lines[i + 1..].iter())
            .find(|(&a, &b)| a + b == 2020)
        {
            println!("Answer 1 {} * {} = {}", a, b, a * b);
            break;
        } else {
            i += 1;
        }
    }
    // Using for loop is considered more idiomatic vs for_each
    // See https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.for_each
    lines.iter().enumerate().for_each(|(i, a)| {
        lines[i + 1..].iter().enumerate().for_each(|(j, b)| {
            if a + b == 2020 {
                println!("HERE {} * {} = {}", a, b, a * b);
            }
            lines[j + 1..].iter().for_each(|c| {
                if a + b + c == 2020 {
                    println!("HERE {} * {} * {} = {}", a, b, c, a * b * c);
                }
            });
        });
    });
    */
    'l1: for (i, a) in lines.iter().enumerate() {
        for (j, b) in lines[i + 1..].iter().enumerate() {
            if a + b == 2020 {
                println!("Answer 1 {}", a * b);
                continue;
            }
            if a + b > 2020 {
                continue;
            }
            for c in &lines[j + 1..] {
                if a + b + c == 2020 {
                    println!("Answer 2 {}", a * b * c);
                    break 'l1;
                }
            }
        }
    }
}

fn read_lines<P>(filename: P) -> Vec<i64>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("Could not find file");
    let buf = io::BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line").parse::<i64>().unwrap())
        .collect()
}
