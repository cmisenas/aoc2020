use std::fs::File;
use std::io::{self, BufRead, Error, ErrorKind};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("./day1.input") {
        'l1: for (i, a) in lines.iter().enumerate() {
            for (j, b) in lines[i + 1..].iter().enumerate() {
                if a + b == 2020 {
                    println!("{} + {} = 2020", a, b);
                    println!("{} * {} = {}", a, b, a * b);
                    continue;
                }
                for c in &lines[j + 1..] {
                    if a + b + c == 2020 {
                        println!("{} + {} + {} = 2020", a, b, c);
                        println!("{} * {} * {} = {}", a, b, c, a * b * c);
                        break 'l1;
                    }
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
