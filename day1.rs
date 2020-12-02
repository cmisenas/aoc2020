use std::fs::File;
use std::io::{self, BufRead, Error, ErrorKind};
use std::path::Path;

fn main() {
    // O(n*n) complexity. Might be able to improve by sorting?
    if let Ok(lines) = read_lines("./day1.input") {
        'outer: for (i, outer) in lines.iter().enumerate() {
            for inner in &lines[i + 1..] {
                if inner + outer == 2020 {
                    println!("{} + {} = 2020", inner, outer);
                    println!("{} * {} = {}", inner, outer, inner * outer);
                    break 'outer;
                }
            }
        }
    }
}

fn read_lines<P>(filename: P) -> Result<Vec<i64>, Error>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let buf = io::BufReader::new(file);
    buf.lines()
        .map(|l| l.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}
