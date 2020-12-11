use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn main() {
    let lines = read_lines_as_str("./day11.input");

    let answer1 = solve1(&lines);
    println!("Answer 1 {}", answer1);
    let answer2 = solve2(&lines);
    println!("Answer 2 {}", answer2);
}

fn solve1(layout: &Vec<String>) -> usize {
    count_occupied_seats(seat_round(layout))
}

fn seat_round(layout: &Vec<String>) -> Vec<String> {
    let mut current_row: String = "".to_string();
    let mut prev_layout: Vec<String> = layout.iter().cloned().collect();
    let mut new_layout: Vec<String> = Vec::new();
    let mut started = false;
    while !started || !new_layout.eq(&prev_layout) {
        if !started {
            started = true;
        } else {
            prev_layout = new_layout.iter().cloned().collect();
        }
        new_layout = Vec::new();
        for (i, row) in prev_layout.iter().enumerate() {
            for (j, seat) in row.chars().collect::<Vec<char>>().into_iter().enumerate() {
                if seat.to_string() == "#" && check_should_be_empty(&prev_layout, i, j) {
                    current_row.push('L');
                } else if seat.to_string() == "L" && check_should_be_occupied(&prev_layout, i, j) {
                    current_row.push('#');
                } else {
                    current_row.push(seat);
                }
            }
            new_layout.push(current_row.to_string());
            current_row.clear();
        }
    }
    new_layout
}

fn check_should_be_occupied(layout: &Vec<String>, r: usize, c: usize) -> bool {
    let adj_seats = get_adj_seats(layout.len(), layout[0].len(), r, c);
    let condition = adj_seats.iter().all(|(_, pos)| {
        if let Some((i, j)) = pos {
            let seat = layout[*i].chars().nth(*j).unwrap().to_string();
            seat != "#"
        } else {
            true
        }
    });
    condition
}

fn check_should_be_empty(layout: &Vec<String>, r: usize, c: usize) -> bool {
    let adj_seats = get_adj_seats(layout.len(), layout[0].len(), r, c);
    let seat_str = layout[r].chars().nth(c).unwrap().to_string();
    let first_c = seat_str == "#";
    let second_c = adj_seats
        .iter()
        .filter(|(_, pos)| {
            if let Some((i, j)) = pos {
                let seat = layout[*i].chars().nth(*j).unwrap().to_string();
                seat == "#"
            } else {
                false
            }
        })
        .count()
        >= 4;
    first_c && second_c
}

fn get_adj_seats(
    height: usize,
    width: usize,
    row: usize,
    col: usize,
) -> HashMap<String, Option<(usize, usize)>> {
    let mut adj_seats: HashMap<String, Option<(usize, usize)>> = HashMap::new();
    let r = row as i32;
    let c = col as i32;
    let h = height as i32;
    let w = width as i32;
    adj_seats.insert(
        String::from("down"),
        if r + 1 < h {
            Some((row + 1, col))
        } else {
            None
        },
    );
    adj_seats.insert(
        String::from("up"),
        if r - 1 >= 0 {
            Some((row - 1, col))
        } else {
            None
        },
    );
    adj_seats.insert(
        String::from("left"),
        if c - 1 >= 0 {
            Some((row, col - 1))
        } else {
            None
        },
    );
    adj_seats.insert(
        String::from("right"),
        if c + 1 < w {
            Some((row, col + 1))
        } else {
            None
        },
    );
    adj_seats.insert(
        String::from("nw"),
        if r - 1 >= 0 && c - 1 >= 0 {
            Some((row - 1, col - 1))
        } else {
            None
        },
    );
    adj_seats.insert(
        String::from("ne"),
        if r - 1 >= 0 && c + 1 < w {
            Some((row - 1, col + 1))
        } else {
            None
        },
    );
    adj_seats.insert(
        String::from("sw"),
        if r + 1 < h && c - 1 >= 0 {
            Some((row + 1, col - 1))
        } else {
            None
        },
    );
    adj_seats.insert(
        String::from("se"),
        if r + 1 < h && c + 1 < w {
            Some((row + 1, col + 1))
        } else {
            None
        },
    );
    adj_seats
}

fn count_occupied_seats(layout: Vec<String>) -> usize {
    layout.iter().fold(0, |acc, row| {
        acc + row.bytes().filter(|r| *r == b'#').count()
    })
}

fn solve2(layout: &Vec<String>) -> usize {
    count_occupied_seats(seat_round2(layout))
}

fn seat_round2(layout: &Vec<String>) -> Vec<String> {
    let mut current_row: String = "".to_string();
    let mut prev_layout: Vec<String> = layout.iter().cloned().collect();
    let mut new_layout: Vec<String> = Vec::new();
    let mut started = false;
    while !started || !new_layout.eq(&prev_layout) {
        if !started {
            started = true;
        } else {
            prev_layout = new_layout.iter().cloned().collect();
        }
        new_layout = Vec::new();
        for (i, row) in prev_layout.iter().enumerate() {
            for (j, seat) in row.chars().collect::<Vec<char>>().into_iter().enumerate() {
                if seat.to_string() == "#" && check_should_be_empty2(&prev_layout, i, j) {
                    current_row.push('L');
                } else if seat.to_string() == "L" && check_should_be_occupied2(&prev_layout, i, j) {
                    current_row.push('#');
                } else {
                    current_row.push(seat);
                }
            }
            new_layout.push(current_row.to_string());
            current_row.clear();
        }
    }
    new_layout
}

fn check_should_be_occupied2(layout: &Vec<String>, r: usize, c: usize) -> bool {
    let neighbors: Vec<bool> = vec![
        is_n_empty(layout, r, c),
        is_e_empty(layout, r, c),
        is_s_empty(layout, r, c),
        is_w_empty(layout, r, c),
        is_ne_empty(layout, r, c),
        is_se_empty(layout, r, c),
        is_nw_empty(layout, r, c),
        is_sw_empty(layout, r, c),
    ];
    let condition = neighbors.iter().all(|is_empty| *is_empty);
    condition
}

fn check_should_be_empty2(layout: &Vec<String>, r: usize, c: usize) -> bool {
    let neighbors: Vec<bool> = vec![
        is_n_empty(layout, r, c),
        is_e_empty(layout, r, c),
        is_s_empty(layout, r, c),
        is_w_empty(layout, r, c),
        is_ne_empty(layout, r, c),
        is_se_empty(layout, r, c),
        is_nw_empty(layout, r, c),
        is_sw_empty(layout, r, c),
    ];
    let seat_str = layout[r].chars().nth(c).unwrap().to_string();
    let first_c = seat_str == "#";
    let second_c = neighbors.iter().filter(|&&is_empty| !is_empty).count() >= 5;
    first_c && second_c
}

fn is_n_empty(layout: &Vec<String>, r: usize, c: usize) -> bool {
    let mut edge_not_hit = true;
    let mut occupied_found = false;
    let mut i = r as i32;
    while edge_not_hit && !occupied_found {
        i -= 1;
        if i < 0 {
            edge_not_hit = false;
            continue;
        }
        let adj = layout[i as usize].chars().nth(c).unwrap().to_string();
        if adj == "L" {
            break;
        } else {
            occupied_found = adj == "#"
        }
    }
    !occupied_found
}

fn is_s_empty(layout: &Vec<String>, r: usize, c: usize) -> bool {
    let mut edge_not_hit = true;
    let mut occupied_found = false;
    let mut i = r as i32;
    let h = layout.len() as i32;
    while edge_not_hit && !occupied_found {
        i += 1;
        if i >= h {
            edge_not_hit = false;
            continue;
        }
        let adj = layout[i as usize].chars().nth(c).unwrap().to_string();
        if adj == "L" {
            break;
        } else {
            occupied_found = adj == "#"
        }
    }
    !occupied_found
}

fn is_e_empty(layout: &Vec<String>, r: usize, c: usize) -> bool {
    let mut edge_not_hit = true;
    let mut occupied_found = false;
    let mut j = c as i32;
    let w = layout[0].len() as i32;
    while edge_not_hit && !occupied_found {
        j += 1;
        if j >= w {
            edge_not_hit = false;
            continue;
        }
        let adj = layout[r].chars().nth(j as usize).unwrap().to_string();
        if adj == "L" {
            break;
        } else {
            occupied_found = adj == "#"
        }
    }
    !occupied_found
}

fn is_w_empty(layout: &Vec<String>, r: usize, c: usize) -> bool {
    let mut edge_not_hit = true;
    let mut occupied_found = false;
    let mut j = c as i32;
    while edge_not_hit && !occupied_found {
        j -= 1;
        if j < 0 {
            edge_not_hit = false;
            continue;
        }
        let adj = layout[r].chars().nth(j as usize).unwrap().to_string();
        if adj == "L" {
            break;
        } else {
            occupied_found = adj == "#"
        }
    }
    !occupied_found
}

fn is_se_empty(layout: &Vec<String>, r: usize, c: usize) -> bool {
    let mut edge_not_hit = true;
    let mut occupied_found = false;
    let mut i = r as i32;
    let mut j = c as i32;
    let h = layout.len() as i32;
    let w = layout[0].len() as i32;
    while edge_not_hit && !occupied_found {
        i += 1;
        j += 1;
        if i >= h || j >= w {
            edge_not_hit = false;
            continue;
        }
        let adj = layout[i as usize]
            .chars()
            .nth(j as usize)
            .unwrap()
            .to_string();
        if adj == "L" {
            break;
        } else {
            occupied_found = adj == "#"
        }
    }
    !occupied_found
}

fn is_sw_empty(layout: &Vec<String>, r: usize, c: usize) -> bool {
    let mut edge_not_hit = true;
    let mut occupied_found = false;
    let mut i = r as i32;
    let mut j = c as i32;
    let h = layout.len() as i32;
    while edge_not_hit && !occupied_found {
        i += 1;
        j -= 1;
        if i >= h || j < 0 {
            edge_not_hit = false;
            continue;
        }
        let adj = layout[i as usize]
            .chars()
            .nth(j as usize)
            .unwrap()
            .to_string();
        if adj == "L" {
            break;
        } else {
            occupied_found = adj == "#"
        }
    }
    !occupied_found
}

fn is_nw_empty(layout: &Vec<String>, r: usize, c: usize) -> bool {
    let mut edge_not_hit = true;
    let mut occupied_found = false;
    let mut i = r as i32;
    let mut j = c as i32;
    while edge_not_hit && !occupied_found {
        i -= 1;
        j -= 1;
        if i < 0 || j < 0 {
            edge_not_hit = false;
            continue;
        }
        let adj = layout[i as usize]
            .chars()
            .nth(j as usize)
            .unwrap()
            .to_string();
        if adj == "L" {
            break;
        } else {
            occupied_found = adj == "#"
        }
    }
    !occupied_found
}

fn is_ne_empty(layout: &Vec<String>, r: usize, c: usize) -> bool {
    let mut edge_not_hit = true;
    let mut occupied_found = false;
    let mut i = r as i32;
    let mut j = c as i32;
    let w = layout[0].len() as i32;
    while edge_not_hit && !occupied_found {
        i -= 1;
        j += 1;
        if i < 0 || j >= w {
            edge_not_hit = false;
            continue;
        }
        let adj = layout[i as usize]
            .chars()
            .nth(j as usize)
            .unwrap()
            .to_string();
        if adj == "L" {
            break;
        } else {
            occupied_found = adj == "#"
        }
    }
    !occupied_found
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
