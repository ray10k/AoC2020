use std::collections::HashMap;

#[derive(Clone,Default)]
struct Bitmask {
    mask_one:u64,
    mask_zero:u64
}

struct Assign {
    address:u64,
    initial_value:u64
}

impl Assign {
    fn mask(&self, mask:&Bitmask) -> u64 {
        //First, set all bits that should be 1.
        let temp = self.initial_value | mask.mask_one;
        //Second, set all bits that A: are set and B: are *not* masked out by the zero-mask.
        temp & !mask.mask_zero
    }
}

enum Operation {
    SetMask(Bitmask),
    AssignValue(Assign)
}

fn setup(input_path:&str) -> Vec<Operation> {
    let input = std::fs::read_to_string(input_path).expect("Could not open input file.");

    input.lines().map(|l|{
        let line = l.trim();
        if line.starts_with("mask") {
            let mask_start = &line[7..];
            let mut mask_one:u64 = 0;
            let mut mask_zero:u64 = 0;
            for (index,ch) in mask_start.chars().rev().enumerate() {
                match ch {
                    '1' => mask_one = mask_one | (1<<index),
                    '0' => mask_zero = mask_zero | (1<<index),
                    _ => ()
                }
            }
            Operation::SetMask(Bitmask{mask_one:mask_one, mask_zero:mask_zero})
        } else {
            let addr_end = line.find(']').expect("Malformed address");
            let val_start = line.rfind(' ').expect("Malformed value");
            let addr:u64 = line[4..addr_end].parse::<u64>().expect("Address did not parse.");
            let val:u64 = line[val_start..].trim().parse::<u64>().expect("Value did not parse.");
            Operation::AssignValue(Assign{address:addr, initial_value:val})
        }
    }).collect()
}

fn star_one(initial_state:&Vec<Operation>) -> String {
    let mut current_mask = Bitmask::default();
    let mut memory:HashMap<u64,u64> = HashMap::new();

    for op in initial_state.iter() {
        match op {
            Operation::SetMask(mask) => {
                current_mask = mask.clone();
            },
            Operation::AssignValue(value) => {
                let real_val = value.mask(&current_mask);
                memory.insert(value.address,real_val);
            },
        }
    }

    let result = memory.iter().map(|x| x.1).fold(0,|acc, ele| acc+ele);

    format!("{result}")
}

fn star_two(initial_state:()) -> String {
    

    "".into()
}

pub fn run_day(input_path:&str) {
    let initial_state = setup(input_path);
    let one = star_one(&initial_state);
    let two = star_two(());
    println!("Day 14.\nStar one: {one}\nStar two: {two}");
}