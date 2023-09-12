use std::collections::HashMap;
use std::fs;
use std::fmt::{Display,Result as FmtResult};
#[derive(Copy,Clone,PartialEq,Debug)]
enum Tile{
    Occupied,
    Empty,
    Floor,
}

#[derive(Clone)]
struct WaitingArea {
    width:usize,
    states:Vec<Tile>,
}

impl WaitingArea {
    fn iter(&self) -> WaitingAreaIter {
        let new_area = self.states.to_vec();
        WaitingAreaIter { state: WaitingArea { width: self.width, states: new_area } }
    }

    fn iter_vision(&self) -> WaitingAreaVisionIter {
        WaitingAreaVisionIter::new(self)
    }

    fn height(&self) -> usize{
        self.states.len() / self.width
    }

    fn occupied_seats(&self) -> usize {
        let mut retval = 0;
        for tile in self.states.iter() {
            if let Tile::Occupied = *tile {
                retval += 1;
            }
        }
        retval
    }
}

impl Display for WaitingArea {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FmtResult {
        for row in self.states.chunks(self.width) {
            for tile in row {
                if let FmtResult::Err(x) = match tile {
                    Tile::Empty => write!(f,"L"),
                    Tile::Floor => write!(f,"."),
                    Tile::Occupied => write!(f,"#"),
                } {
                    return FmtResult::Err(x);
                }
            }
            if let FmtResult::Err(x) = writeln!(f,"") {
                return FmtResult::Err(x);
            }
        }
        return FmtResult::Ok(());
    }
}

struct WaitingAreaIter {
    state:WaitingArea
}

impl Iterator for WaitingAreaIter {
    type Item = WaitingArea;

    fn next(&mut self) -> Option<Self::Item> {
        let to_coords = |x:usize| (x%self.state.width,x/self.state.width);
        let to_index = |x:usize,y:usize| (x + (y * self.state.width));
        let mut new_state = self.state.states.to_vec();
        let height = self.state.height();
        for (middle,tile) in self.state.states.iter().enumerate() {
            if let Tile::Floor = *tile {
                continue;
            }
            let (x,y) = to_coords(middle);
            let mut occupied = 0; 
            for d_x in 0..=2 {
                if d_x == 0 && x == 0 {
                    continue;
                }
                if d_x == 2 && x+1 == self.state.width {
                    continue;
                }
                for d_y in 0..=2 {
                    if d_y == 0 && y == 0 {
                        continue;
                    }
                    if d_y == 2 && y+1 == height {
                        continue;
                    }
                    if d_x == 1 && d_y == 1{
                        continue;
                    }
                    let index = to_index((x+d_x)-1,(y+d_y)-1);
                    match self.state.states[index] {
                        Tile::Empty => (),
                        Tile::Floor => (),
                        Tile::Occupied => occupied += 1,
                    }
                }
            }
            
            if let Tile::Occupied = *tile  {
                if occupied >= 4 {
                    new_state[middle] = Tile::Empty;
                }
            } else if let Tile::Empty = *tile {
                if occupied == 0 {
                    new_state[middle] = Tile::Occupied;
                }
            }
        };
        if self.state.states == new_state {
            None
        } else {
            self.state.states = new_state;
            Some(self.state.clone())
        }
    }
}

struct LineIterator {
    d_x:isize,
    d_y:isize,
    x:isize,
    y:isize,
    height:isize,
    width:isize,
}

impl Iterator for LineIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let new_x = self.x + self.d_x;
        let new_y = self.y + self.d_y;
        if new_x < 0 || new_x >= self.width {
            None
        }
        else if new_y < 0 || new_y >= self.height {
            None
        }
        else {
            self.x = new_x;
            self.y = new_y;
            Some((new_x + (self.width*new_y)).try_into().expect("Could not convert."))
        }
    }
}

struct WaitingAreaVisionIter {
    state:WaitingArea,
    lookup:HashMap<usize,Vec<usize>>,
}

impl WaitingAreaVisionIter {
    fn new(initial_state:&WaitingArea) -> Self {
        let mut lookup = HashMap::new();

        let height:isize = initial_state.height().try_into().expect("Could not convert height.");
        let width:isize = initial_state.width.try_into().expect("Could not convert width.");
        for (tile,index) in initial_state.states.iter().zip(0..) {
            if let Tile::Floor = tile {
                continue;
            }
            let mut visible:Vec<usize> = Vec::new();
            let x = index % width;
            let y = index / width;
            for d_x in -1..=1 {
                if d_x == -1 && x == 0 {
                    continue;
                }
                if d_x == 1 && x+1 == width {
                    continue;
                }
                'inner: for d_y in -1..=1 {
                    if d_x == 0 && d_y == 0 {
                        continue;
                    }
                    if d_y == -1 && y == 0 {
                        continue;
                    }
                    if d_y == 1 && y+1 == width {
                        continue;
                    }
                    
                    let line = LineIterator{x:x, y:y, width:width, height:height, d_x:d_x, d_y:d_y};
                    for index in line{
                        match initial_state.states[index] {
                            Tile::Empty => {visible.push(index); continue 'inner;},
                            Tile::Occupied => {visible.push(index); continue 'inner;},
                            _ => (),
                        };
                    }
                }
            }
            lookup.insert(index.try_into().expect("Index in negative!"), visible);
        };

        WaitingAreaVisionIter { state: WaitingArea { width: initial_state.width, states: initial_state.states.to_vec() }, lookup: lookup }
    }
}

impl Iterator for WaitingAreaVisionIter {
    type Item = WaitingArea;

    fn next(&mut self) -> Option<Self::Item> {
        let mut new_state = self.state.states.to_vec();
        for (index,visibles) in self.lookup.iter() {
            let mut occupied = 0;

            for to_check in visibles.iter() {
                match self.state.states[*to_check] {
                    Tile::Occupied => {occupied += 1},
                    _ => (),
                };
            }
            match self.state.states[*index] {
                Tile::Floor => {panic!("There is no chair in position {}",*index)},
                Tile::Empty => {
                    if occupied == 0 {
                        new_state[*index] = Tile::Occupied;
                    }
                },
                Tile::Occupied => {
                    if occupied >= 5 {
                        new_state[*index] = Tile::Empty;
                    }
                }
            }
        };
        


        if self.state.states == new_state {
            None
        } else {
            self.state.states = new_state;
            Some(self.state.clone())
        }
    }
}

fn setup(input_path:&str) -> WaitingArea {
    let input_str = fs::read_to_string(input_path).expect("Could not read file.");
    let mut width:usize = 0;
    let mut tiles:Vec<Tile> = Vec::new();
    for line in input_str.lines() {
        width = line.len();
        for letter in line.chars() {
            match letter {
                '.' => tiles.push(Tile::Floor),
                'L' => tiles.push(Tile::Empty),
                _ => panic!("ERROR: Could not parse character '{letter}'"),
            }
        }
    }
    WaitingArea { width: width, states: tiles }
}

fn star_one(initial_state:&WaitingArea) -> String {
    let final_state = initial_state.iter().last();
    if let Some(s) = final_state {
        s.occupied_seats().to_string()
    } else {
        "ERROR".to_string()
    }
}

fn star_two(initial_state:&WaitingArea) -> String {
    println!("{initial_state}\n");
    let iterator = initial_state.iter_vision();

    if let Some(s) = iterator.last() {
        println!("{s}");
        s.occupied_seats().to_string()
    } else {
        "ERROR".to_string()
    }
}

pub fn run_day(input_path:&str){
    let initial_state = setup(input_path);
    let one = star_one(&initial_state);
    let two = star_two(&initial_state);
    println!("Day 11.\nStar one: {one}\nStar two: {two}");
}