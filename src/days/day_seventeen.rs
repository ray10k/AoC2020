use std::collections::HashSet;
use std::ops::Add;
use std::{isize, usize};



#[derive(PartialEq, Eq, Clone, Copy, Hash)]
struct Coordinate {
    x:isize,
    y:isize,
    z:isize,
    w:isize
}

impl Add for Coordinate {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Coordinate {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w
        }
    }
}

impl Add for &Coordinate{
    type Output = Coordinate;

    fn add(self, rhs: Self) -> Self::Output {
        Coordinate {
            x:self.x + rhs.x,
            y:self.y + rhs.y,
            z:self.z + rhs.z,
            w:self.w + rhs.w
        }
    }
}

impl Coordinate {
    const fn default() -> Self {
        Coordinate{
            x: 0,
            y: 0,
            z: 0,
            w: 0
        }
    }

    const fn new_2d(x:usize, y:usize) -> Self {
        Coordinate {
            x:x as isize,
            y:y as isize,
            z:0,
            w:0
        }
    }

    const fn new_3d(x:isize, y:isize, z:isize) -> Self {
        Coordinate {
            x:x,
            y:y,
            z:z,
            w:0
        }
    }

    const fn new_4d(x:isize, y:isize, z:isize, w:isize) -> Self {
        Coordinate {
            x:x,
            y:y,
            z:z,
            w:w
        }
    }

    fn neighbors_3d(&self) -> CoordinateNeighbourIterator {
        CoordinateNeighbourIterator(self.clone(),0)
    }

    fn neighbors_4d(&self) -> CoordinateHyperNeighborIterator {
        CoordinateHyperNeighborIterator(self.clone(),0)
    }
}

const fn make_3d_lookup() -> [Coordinate;26] {
    let def = Coordinate::default();
    let mut retval = [def;26];
    let mut counter:isize = 0;
    let mut index:usize = 0;
    
    loop { //const functions can't have for-loops. Adjust.
        if index == 26 {
            break;
        }
        let a = (counter % 3) - 1;
        let b = ((counter/3)%3) - 1;
        let c = ((counter/9)%3) - 1;
        counter += 1;
        if a == 0 && b == 0 && c == 0 {
            continue;
        }
        retval[index] = Coordinate::new_3d(a, b, c);
        index += 1;
    }

    retval
}

const fn make_4d_lookup() -> [Coordinate;80] {
    let def = Coordinate::default();
    let mut retval = [def;80];
    let mut counter:isize = 0;
    let mut index:usize = 0;
    
    loop {
        if index == 80 {
            break;
        }
        let a = (counter % 3) - 1;
        let b = ((counter/3)%3) - 1;
        let c = ((counter/9)%3) - 1;
        let d = ((counter/27)%3) - 1;
        counter += 1;
        if a == 0 && b == 0 && c == 0 && d == 0{
            continue;
        }
        retval[index] = Coordinate::new_4d(a, b, c, d);
        index += 1;
    }

    retval
}

struct CoordinateNeighbourIterator(Coordinate,usize);

impl Iterator for CoordinateNeighbourIterator {
    type Item = Coordinate;

    fn next(&mut self) -> Option<Self::Item> {
        if self.1 >= GEN_NEIGH_3D.len() {
            None
        } else {
            let index = self.1;
            self.1 += 1;
            Some(&self.0 + &GEN_NEIGH_3D[index])
        }
    }
}

struct CoordinateHyperNeighborIterator(Coordinate,usize);

impl Iterator for CoordinateHyperNeighborIterator {
    type Item = Coordinate;

    fn next(&mut self) -> Option<Self::Item> {
        if self.1 >= GEN_NEIGH_4D.len() {
            None
        } else {
            let index = self.1;
            self.1 += 1;
            Some(&self.0 + &GEN_NEIGH_4D[index])
        }
    }
}

const GEN_NEIGH_3D:[Coordinate;26] = make_3d_lookup();
const GEN_NEIGH_4D:[Coordinate;80] = make_4d_lookup();

fn gol_step(initial:HashSet<Coordinate>) -> HashSet<Coordinate> {
    let mut retval:HashSet<Coordinate> = HashSet::new();

    for live in initial.iter() {
        for adjacent in live.neighbors_3d() {
            let mut neighbor_count:u8 = 0;
            for to_check in adjacent.neighbors_3d() {
                if initial.contains(&to_check) {
                    neighbor_count += 1;
                }
            }

            if initial.contains(&adjacent) {
                if neighbor_count == 2 || neighbor_count == 3 {
                    retval.insert(adjacent);
                }
            } else if neighbor_count == 3 {
                retval.insert(adjacent);
            }
        }
    }
    retval
}

fn gol_4d_step(initial:HashSet<Coordinate>) -> HashSet<Coordinate> {
    let mut retval:HashSet<Coordinate> = HashSet::new();

    for live in initial.iter() {
        for adjacent in live.neighbors_4d() {
            let mut neighbor_count:u8 = 0;
            for to_check in adjacent.neighbors_4d() {
                if initial.contains(&to_check) {
                    neighbor_count += 1;
                }
            }

            if initial.contains(&adjacent) {
                if neighbor_count == 2 || neighbor_count == 3 {
                    retval.insert(adjacent);
                }
            } else if neighbor_count == 3 {
                retval.insert(adjacent);
            }
        }
    }
    retval
}

type State = Vec<Coordinate>;

fn setup(input_path:&str) -> State {
    let mut retval = Vec::new();
    std::fs::read_to_string(input_path)
        .expect("could not open input file")
        .lines()
        .enumerate()
        .for_each(|(y,line)| {
            line.trim()
                .chars()
                .enumerate()
                .for_each(|(x,symbol)|{
                    if symbol == '#' {
                        retval.push(Coordinate::new_2d(x, y));
                    }
                })
        });
    retval
}

fn star_one(initial_state:&State) -> String {
    let mut field:HashSet<Coordinate> = HashSet::from_iter(initial_state.iter().cloned());

    for _ in 0..6 {
        field = gol_step(field);
    }

    format!("{}",field.len())
}

fn star_two(initial_state:&State) -> String {
    let mut field:HashSet<Coordinate> = HashSet::from_iter(initial_state.iter().cloned());

    for _ in 0..6 {
        field = gol_4d_step(field);
    }

    format!("{}",field.len())
}

pub fn run_day(input_path:&str) {
    let initial_state = setup(input_path);
    let one = star_one(&initial_state);
    let two = star_two(&initial_state);
    println!("Day 17.\nStar one: {one}\nStar two: {two}");
}