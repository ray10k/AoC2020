use std::{collections::HashMap, fmt::Display};


struct Tile{
    id:u16,
    contents:[u16;10]
}

struct ClassifiedTile {
    tile:Tile,
    class:TileClass,
}

enum FlipDirection {
    None,
    Horizontal,
    Vertical,
    Both
}

enum Rotation {
    None,
    Clockwise,
    HalfTurn,
    CounterClockwise,
}

enum TileClass {
    Corner,
    Edge,
    Center,
}

fn mirror_num(straight:u16) -> u16 {
    let mut retval = 0;
    for x in 0..10 {
        retval |= ((straight >> x) & 1) << (9 - x);
    }
    retval
}

impl Tile {
    fn top_num(&self) -> u16 {
        self.contents[0]
    }

    fn bottom_num(&self) -> u16 {
        mirror_num(self.contents[9])
    }

    fn left_num(&self) -> u16 {
        let mut left = 0;
        for x in 0..10 {
            left |= (self.contents[x] & 1) << x;
        }
        mirror_num(left)
    }

    fn right_num(&self) -> u16 {
        let mut right = 0;
        for x in 0..10 {
            right |= ((self.contents[x] & 0x200)>>9) << x;
        }
        right
    }

    fn transform(&self, flip:FlipDirection, rotate:Rotation) -> Self {
        //Flip first, rotate last.
        let mut new_data = self.contents.clone();

        if let (FlipDirection::Both,Rotation::HalfTurn) = (&flip,&rotate) {
            return Self {
                contents:new_data,
                id:self.id
            };
        }

        match flip {
            FlipDirection::None => (),
            FlipDirection::Horizontal => {
                for i in 0..10 {
                    new_data[i] = mirror_num(new_data[i]);
                }
            },
            FlipDirection::Vertical => {
                for i in 0..5 {
                    let temp = new_data[10-i];
                    new_data[10-i] = new_data[i];
                    new_data[i] = temp;
                }
            },
            FlipDirection::Both => {
                for i in 0..5 {
                    let temp = mirror_num(new_data[10-i]);
                    new_data[10-i] = mirror_num(new_data[i]);
                    new_data[i] = temp
                }
            },
        }

        match rotate {
            Rotation::None => (),
            Rotation::Clockwise => {

            },
            Rotation::HalfTurn => {
                for i in 0..5 {
                    let temp = mirror_num(new_data[10-i]);
                    new_data[10-i] = mirror_num(new_data[i]);
                    new_data[i] = temp
                }
            },
            Rotation::CounterClockwise => todo!(),
        }


        Self{
            contents : new_data,
            id: self.id
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f,"Tile {}:",self.id)?;
        for num in self.contents.iter() {
            for i in 0..10 {
                if (*num & (1 << i)) != 0 {
                    write!(f,"#")?;
                } else {
                    write!(f,".")?;
                }
            }
            writeln!(f,"")?;
        }
        std::fmt::Result::Ok(())
    }
}

type State = Vec<Tile>;

fn setup(input_path:&str) -> State {
    assert_eq!(0x3ff,mirror_num(0x3ff));
    assert_eq!(0x001,mirror_num(0x200));
    assert_eq!(0x00f,mirror_num(0x3c0));
    assert_eq!(0x201,mirror_num(0x201));
    assert_eq!(0x3c1,mirror_num(0x20f));

    let data = std::fs::read_to_string(input_path).expect("Could not open input file.");
    let mut lines = data.lines();
    let mut retval:State = Vec::new();

    'mainloop: loop{
        let firstline = lines.next();
        if let None = firstline {
            break 'mainloop;
        }
        let firstline = firstline.unwrap().trim();
        let space = firstline.find(' ').unwrap_or(4) + 1;
        let colon = firstline.find(':').unwrap_or(9);
        let tile_id = firstline[space..colon].parse::<u16>().expect("Malformed tile id.");
        let mut tile_data = [0u16;10];
        for x in 0..10 {
            let line = lines.next().expect("Tile too short.").trim();
            let mut bits:u16 = 0;
            for (position,c) in line.chars().enumerate() {
                if c == '#' {
                    bits |= 1 << position;
                }
            }
            tile_data[x] = bits;
        }
        retval.push(Tile{
            id:tile_id,
            contents:tile_data,
        });
        lines.next();
    }

    retval
}

fn star_one(initial_state:&State) -> String {
    println!("Sorting through {} tiles...",initial_state.len());
    //Map tile ID to edge-values, clockwise, and mirrored edge-values.
    let edges:HashMap<u16,[u16;8]> = initial_state.iter()
        .map(|item|{
            let id = item.id;
            let edges = [
                item.top_num(),item.right_num(),item.bottom_num(),item.left_num(),
                mirror_num(item.top_num()), mirror_num(item.right_num()),
                mirror_num(item.bottom_num()),mirror_num(item.left_num())
            ];
            (id,edges)
        })
        .collect();
    let ids:Vec<u16> = edges.keys().map(|x| *x).collect();
    let mut corners:Vec<u16> = Vec::with_capacity(4);
    let mut edge_count = 0;
    let mut center_count = 0;
    
    for i in 0..ids.len() {
        let current_sides = &edges.get(&ids[i]).expect("Ghost ID found!")[0..4];
        let mut connecting_sides = 0;
        'tile: for other in ids.iter() {
            if *other == ids[i] {
                continue 'tile;
            }
            let current_other = edges.get(other).expect("Ghost ID found!");
            for x in current_sides.iter() {
                if current_other.contains(x) {
                    connecting_sides += 1;
                    continue 'tile;
                }
            }
        }
        println!("id {}; sides:{connecting_sides}",ids[i]);
        match connecting_sides {
            2 => corners.push(ids[i]),
            3 => edge_count += 1,
            _ => center_count += 1,
        }
    }

    println!("Found {} corner-tiles, {edge_count} edge-tiles and {center_count} center-tiles.",corners.len());

    let retval = corners.iter().fold(1,|acc, ele| acc * *ele as usize);

    format!("{retval}")
}

fn star_two(initial_state:&State) -> String {
    "".into()
}

pub fn run_day(input_path:&str) {
    let initial_state = setup(input_path);
    let one = star_one(&initial_state);
    let two = star_two(&initial_state);
    println!("Day 20.\nStar one: {one}\nStar two: {two}");
}