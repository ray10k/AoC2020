use std::collections::HashMap;

#[derive(Clone,Default)]
struct Bitmask {
    mask_one:u64,
    mask_zero:u64,
    mask_float:u64,
    float_offsets:Vec<u8> //Series of offsets, counting from LSB.
}

impl Bitmask {
    fn float_mask(&self) -> FloatMask {
        FloatMask{progress:0,offsets:&self.float_offsets}
    }
}

struct FloatMask<'a> {
    progress:u64,
    offsets:&'a Vec<u8>
}

impl <'a> Iterator for FloatMask<'a> {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.progress as usize >= (1 << self.offsets.len()) {
            None
        } else {
            let initial = self.progress;
            let mut mask = 0;
            self.progress += 1;
            for (index, offset) in self.offsets.iter().enumerate() {
                mask = mask | ((initial & (1<<index))<<(*offset - index as u8));
            }
            Some(mask)
        }
    }
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
            let mut mask_float:u64 = 0;
            let mut float_offsets:Vec<u8> = Vec::new();
            for (index,ch) in mask_start.chars().rev().enumerate() {
                match ch {
                    '1' => mask_one = mask_one | (1<<index),
                    '0' => mask_zero = mask_zero | (1<<index),
                    'X' => {
                        mask_float = mask_float | (1<<index);
                        float_offsets.push((index&0xff) as u8)},
                    _ => ()
                }
            }
            Operation::SetMask(Bitmask{mask_one:mask_one, mask_zero:mask_zero,mask_float:mask_float,float_offsets:float_offsets})
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

fn star_two(initial_state:&Vec<Operation>) -> String {
    let mut current_mask = Bitmask::default();
    let mut memory:HashMap<u64,u64> = HashMap::new();
    
    for op in initial_state.iter() {
        match op {
            Operation::SetMask(mask) => {
                current_mask = mask.clone();
            },
            Operation::AssignValue(value) => {
                let memory_address = (value.address | current_mask.mask_one) & !current_mask.mask_float;
                memory.insert(memory_address, value.initial_value);
                for floater in current_mask.float_mask() {
                    memory.insert(memory_address | floater, value.initial_value);
                }
            }
        }
    }

    let result = memory.iter().map(|x| *x.1).fold(0,|acc, ele| acc + ele);

    format!("{result}")
}

pub fn run_day(input_path:&str) {
    let initial_state = setup(input_path);
    let one = star_one(&initial_state);
    let two = star_two(&initial_state);
    println!("Day 14.\nStar one: {one}\nStar two: {two}");
}