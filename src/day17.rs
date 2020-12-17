use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Clone, Debug)]
struct Cube {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Clone, Debug)]
struct HyperCube {
    x: i32,
    y: i32,
    z: i32,
    w: i32,
}

impl Cube {
    fn to_id(&self) -> String {
        self.x.to_string() + "," + &self.y.to_string() + "," + &self.z.to_string()
    }

    fn eq(&self, cube: &Cube) -> bool {
        self.x == cube.x && self.y == cube.y && self.z == cube.z
    }
}

impl HyperCube {
    fn to_id(&self) -> String {
        self.x.to_string()
            + ","
            + &self.y.to_string()
            + ","
            + &self.z.to_string()
            + ","
            + &self.w.to_string()
    }

    fn eq(&self, cube: &HyperCube) -> bool {
        self.x == cube.x && self.y == cube.y && self.z == cube.z && self.w == cube.w
    }
}

pub fn main() {
    let lines = read_lines_as_str("./day17.input");
    let answer1 = solve1(&lines);
    println!("Answer 1 {}", answer1);
    let answer2 = solve2(&lines);
    println!("Answer 2 {}", answer2);
}

fn get_neighbors(coor: &Cube) -> Vec<Cube> {
    let mut neighbors: Vec<Cube> = Vec::new();
    for x in -1..2 {
        for y in -1..2 {
            for z in -1..2 {
                if !(x == 0 && y == 0 && z == 0) {
                    neighbors.push(Cube {
                        x: coor.x + x,
                        y: coor.y + y,
                        z: coor.z + z,
                    });
                }
            }
        }
    }
    neighbors
}

fn convert_to_cube(lines: &Vec<String>) -> Vec<(bool, Cube)> {
    let z = 0;
    let mut cubes: Vec<(bool, Cube)> = Vec::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, cube) in line.chars().enumerate() {
            cubes.push((
                cube == '#',
                Cube {
                    x: x as i32,
                    y: y as i32,
                    z: z as i32,
                },
            ));
        }
    }
    cubes
}

fn solve1(lines: &Vec<String>) -> usize {
    let mut cubes_list = convert_to_cube(lines);
    let mut cube_state: HashMap<String, (bool, Cube)> = HashMap::new();
    for (active, cube) in cubes_list.iter() {
        cube_state.insert(cube.to_id(), (*active, cube.clone()));
    }
    for cycle in 0..6 {
        println!("Round {}", cycle + 1);
        let mut new_cube_state: HashMap<String, (bool, Cube)> = HashMap::new();
        for (active, cube) in cubes_list.iter() {
            let neighbors = get_neighbors(&cube);
            let active_neighbors = neighbors
                .iter()
                .filter(|n| {
                    if let Some(found_cube) = cube_state.get(&n.to_id()) {
                        found_cube.0
                    } else {
                        // Initial state should be false
                        false
                    }
                })
                .count();
            if *active && (active_neighbors == 2 || active_neighbors == 3) {
                // Remain active
                new_cube_state.insert(cube.to_id(), (true, cube.clone()));
            } else if !*active && active_neighbors == 3 {
                // Be active
                new_cube_state.insert(cube.to_id(), (true, cube.clone()));
            }

            for n in neighbors.iter() {
                let nn = get_neighbors(n);
                let n_state = if let Some(found_cube) = cube_state.get(&n.to_id()) {
                    (found_cube.0, found_cube.1.clone())
                } else {
                    // Initial state should be false
                    (false, n.clone())
                };
                let active_nn = nn
                    .iter()
                    .filter(|a| {
                        if let Some(found_cube) = cube_state.get(&a.to_id()) {
                            found_cube.0
                        } else {
                            // Initial state should be false
                            false
                        }
                    })
                    .count();
                if n_state.0 && (active_nn == 2 || active_nn == 3) {
                    // Remain active
                    new_cube_state.insert(n_state.1.to_id(), (true, n_state.1.clone()));
                } else if !n_state.0 && active_nn == 3 {
                    // Be active
                    new_cube_state.insert(n_state.1.to_id(), (true, n_state.1.clone()));
                }
            }
        }
        cubes_list.clear();
        for (_, a) in new_cube_state.iter() {
            cubes_list.push((a.0, a.1.clone()));
        }
        cube_state = new_cube_state.clone();
        println!("Active cubes {:?}", cube_state);
        println!("Active cubes # {:?}\n\n", cube_state.len());
    }
    cube_state.len()
}

fn get_hyper_neighbors(coor: &HyperCube) -> Vec<HyperCube> {
    let mut neighbors: Vec<HyperCube> = Vec::new();
    for x in -1..2 {
        for y in -1..2 {
            for z in -1..2 {
                for w in -1..2 {
                    if !(x == 0 && y == 0 && z == 0 && w == 0) {
                        neighbors.push(HyperCube {
                            x: coor.x + x,
                            y: coor.y + y,
                            z: coor.z + z,
                            w: coor.w + w,
                        });
                    }
                }
            }
        }
    }
    neighbors
}

fn convert_to_hypercube(lines: &Vec<String>) -> Vec<(bool, HyperCube)> {
    let z = 0;
    let w = 0;
    let mut cubes: Vec<(bool, HyperCube)> = Vec::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, cube) in line.chars().enumerate() {
            cubes.push((
                cube == '#',
                HyperCube {
                    x: x as i32,
                    y: y as i32,
                    z: z as i32,
                    w: w as i32,
                },
            ));
        }
    }
    cubes
}

fn solve2(lines: &Vec<String>) -> usize {
    let mut cubes_list = convert_to_hypercube(lines);
    let mut cube_state: HashMap<String, (bool, HyperCube)> = HashMap::new();
    for (active, cube) in cubes_list.iter() {
        cube_state.insert(cube.to_id(), (*active, cube.clone()));
    }
    for cycle in 0..6 {
        println!("Round {}", cycle + 1);
        let mut new_cube_state: HashMap<String, (bool, HyperCube)> = HashMap::new();
        for (active, cube) in cubes_list.iter() {
            let neighbors = get_hyper_neighbors(&cube);
            let active_neighbors = neighbors
                .iter()
                .filter(|n| {
                    if let Some(found_cube) = cube_state.get(&n.to_id()) {
                        found_cube.0
                    } else {
                        // Initial state should be false
                        false
                    }
                })
                .count();
            if *active && (active_neighbors == 2 || active_neighbors == 3) {
                // Remain active
                new_cube_state.insert(cube.to_id(), (true, cube.clone()));
            } else if !*active && active_neighbors == 3 {
                // Be active
                new_cube_state.insert(cube.to_id(), (true, cube.clone()));
            }

            for n in neighbors.iter() {
                let nn = get_hyper_neighbors(n);
                let n_state = if let Some(found_cube) = cube_state.get(&n.to_id()) {
                    (found_cube.0, found_cube.1.clone())
                } else {
                    // Initial state should be false
                    (false, n.clone())
                };
                let active_nn = nn
                    .iter()
                    .filter(|a| {
                        if let Some(found_cube) = cube_state.get(&a.to_id()) {
                            found_cube.0
                        } else {
                            // Initial state should be false
                            false
                        }
                    })
                    .count();
                if n_state.0 && (active_nn == 2 || active_nn == 3) {
                    // Remain active
                    new_cube_state.insert(n_state.1.to_id(), (true, n_state.1.clone()));
                } else if !n_state.0 && active_nn == 3 {
                    // Be active
                    new_cube_state.insert(n_state.1.to_id(), (true, n_state.1.clone()));
                }
            }
        }
        cubes_list.clear();
        for (_, a) in new_cube_state.iter() {
            cubes_list.push((a.0, a.1.clone()));
        }
        cube_state = new_cube_state.clone();
        println!("Active cubes {:?}", cube_state);
        println!("Active cubes # {:?}\n\n", cube_state.len());
    }
    cube_state.len()
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
