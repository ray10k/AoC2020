use std::{fs,fmt::Display};

#[derive(Clone)]
enum Absolute {
    North,
    East,
    South,
    West,
}

impl Display for Absolute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}",
        match self {
            Absolute::North => "North",
            Absolute::East => "East",
            Absolute::South => "South",
            Absolute::West => "West",
        })
    }
}

impl Absolute {
    fn rotate(&self, degrees:usize) -> Result<Self,()> {
        match (self,degrees) {
            (Absolute::North,270) | (Absolute::East,180) | (Absolute::South, 90) => Ok(Absolute::West),
            (Absolute::East,270) | (Absolute::South,180) | (Absolute::West, 90) => Ok(Absolute::North),
            (Absolute::South,270) | (Absolute::West,180) | (Absolute::North, 90) => Ok(Absolute::East),
            (Absolute::West,270) | (Absolute::North,180) | (Absolute::East, 90) => Ok(Absolute::South),
            _ => Err(())
        }
    }
}

#[derive(Clone)]
enum Relative {
    Left,
    Right,
}

impl Display for Relative {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}",
        match self {
            Relative::Left => "Left",
            Relative::Right => "Right",
        })
    }
}

enum Direction {
    Absolute(Absolute),
    Relative(Relative),
    Forward
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Absolute(a) => a.fmt(f),
            Direction::Relative(r) => r.fmt(f),
            Direction::Forward => write!(f,"Forward"),
        }
    }
}

struct Action {
    direction:Direction,
    magnitude:usize
}

impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}, {}",self.direction,self.magnitude)
    }
}

impl TryFrom<&str> for Action {
    type Error = String;
    
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.trim();
        if value.len() < 2 {
            return Err("Not enough information.".into());
        }
        let mut chars = value.chars();
        let dir = match chars.next() {
            Some('N') => Direction::Absolute(Absolute::North),
            Some('S') => Direction::Absolute(Absolute::South),
            Some('E') => Direction::Absolute(Absolute::East),
            Some('W') => Direction::Absolute(Absolute::West),
            Some('L') => Direction::Relative(Relative::Left),
            Some('R') => Direction::Relative(Relative::Right),
            Some('F') => Direction::Forward,
            _ => return Err("Unknown direction".into())
        };
        let mag = value[1..].parse::<usize>();
        match mag {
            Ok(m) => Ok(Action{direction:dir,magnitude:m}),
            Err(_) => Err("Could not parse magnitude.".into()),
        }
    }
}

struct Ship {
    facing:Absolute,
    x:isize,
    y:isize,
}

impl Default for Ship {
    fn default() -> Self {
        Self { facing: Absolute::East, x:0, y:0 }
    }
}

impl Display for Ship {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"Ship at {},{}; Facing {}.",self.x,self.y,self.facing)
    }
}

impl Ship {
    pub fn perform_action(&mut self, action:&Action) {
        let mag = action.magnitude as isize;
        let facing = self.facing.clone();
        match &action.direction {
            Direction::Absolute(abs) => self.slide(abs, mag),
            Direction::Relative(rel) => self.turn(rel,mag),
            Direction::Forward => self.slide(&facing,mag),
        }
    }

    pub fn move_towards(&mut self, wp:&Waypoint, repeats:isize) {
        self.x += wp.x * repeats;
        self.y += wp.y * repeats;
    }

    pub fn manhattan(&self) -> usize {
        let x:usize = self.x.abs().try_into().unwrap_or(0);
        let y:usize = self.y.abs().try_into().unwrap_or(0);
        x + y
    }

    fn slide(&mut self, direction:&Absolute, magnitude:isize) {
        match direction {
            Absolute::North => self.y += magnitude,
            Absolute::East => self.x += magnitude,
            Absolute::South => self.y -= magnitude,
            Absolute::West => self.x -= magnitude,
        }
    }

    fn turn(&mut self, direction:&Relative, amount:isize) {
        let amount = {
            amount.try_into().expect("Could not unpack amount.")
        };
        let result = match direction{
            Relative::Left => self.facing.rotate(360-amount),
            Relative::Right => self.facing.rotate(amount),
        };
        if let Ok(new_direction) = result {
            self.facing = new_direction;
        }
    }
}

struct Waypoint {
    x:isize,
    y:isize,
}

impl Display for Waypoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Waypoint at relative {},{}",self.x, self.y)
    }
}

impl Default for Waypoint {
    fn default() -> Self {
        Self { x: 10, y: 1 }
    }
}

impl Waypoint {
    pub fn perform_action(&mut self, action:&Action) {
        let mag = action.magnitude as isize;
        match &action.direction {
            Direction::Absolute(abs) => self.slide(abs, mag),
            Direction::Relative(rel) => self.turn(rel,mag),
            Direction::Forward => (),
        }
    }

    fn turn(&mut self, direction:&Relative, magnitude:isize) {
        let steps = match direction {
            Relative::Left => magnitude / 90,
            Relative::Right => 4 - (magnitude / 90),
        };
        //Simplified rotation matrices; the turns are always a multiple of 90 degrees.
        //Note: turns are counter-clockwise.
        let (new_x,new_y) = match steps % 4 {
            1 => (-self.y, self.x),
            2 => (-self.x,-self.y),
            3 => ( self.y,-self.x),
            _ => ( self.x, self.y)
        };

        self.x = new_x;
        self.y = new_y;
    }

    fn slide(&mut self, direction:&Absolute, magnitude:isize) {
        match direction{
            Absolute::North => self.y += magnitude,
            Absolute::East => self.x += magnitude,
            Absolute::South => self.y -= magnitude,
            Absolute::West => self.x -= magnitude,
        }
    }
}

fn setup(input_path:&str) -> Vec<Action> {
    let input = fs::read_to_string(input_path).expect("Could not read input");
    let mut retval:Vec<Action> = Vec::new();
    for line in input.lines() {
        match Action::try_from(line) {
            Ok(act) => retval.push(act),
            Err(_) => (),
        }
    }
    retval
}

fn star_one(initial_state:&Vec<Action>) -> String {
    let mut ship = Ship::default();
    for act in initial_state.iter() {
        ship.perform_action(act);
    }

    format!("{}",ship.manhattan())
}

fn star_two(initial_state:&Vec<Action>) -> String {
    let mut ship = Ship::default();
    let mut wp = Waypoint::default();
    for act in initial_state.iter(){
        if let Direction::Forward = act.direction {
            ship.move_towards(&wp, act.magnitude as isize);
        } else {
            wp.perform_action(act);
        }
    }

    format!("{}",ship.manhattan())
}

pub fn run_day(input_path:&str) {
    let initial_state = setup(input_path);
    let one = star_one(&initial_state);
    let two = star_two(&initial_state);
    println!("Day 11.\nStar one: {one}\nStar two: {two}");
}