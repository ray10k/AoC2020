use std::{str::FromStr, fs, collections::HashSet};

#[derive(Copy,Clone)]
enum Instruction {
    Nop(i32),
    Acc(i32),
    Jmp(i32)
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let opcode = &s[..3];
        let mut value:i32 = s[5..].parse().unwrap();
        let sign = &s[4..5] == "+";
        if !sign {
            value *= -1;
        };
        match opcode {
            "nop" => Ok(Instruction::Nop(value)),
            "acc" => Ok(Instruction::Acc(value)),
            "jmp" => Ok(Instruction::Jmp(value)),
            &_ => Err(String::from(format!("Unknown opcode {opcode}"))),
        }
    }
}

impl ToString for Instruction {
    fn to_string(&self) -> String {
        match self {
            Instruction::Nop(x) => format!("NOP ({})",x),
            Instruction::Acc(x) => format!("ACC ({})",x),
            Instruction::Jmp(x) => format!("JMP ({})",x),
        }
    }
}

fn setup(input_path:&str) -> Vec<Instruction> {
    let input_string = fs::read_to_string(input_path).expect("Could not read file.");
    let mut retval:Vec<Instruction> = Vec::new();
    for line in input_string.lines() {
        if let Ok(instr) = Instruction::from_str(line) {
            retval.push(instr);
        }
    }
    return retval;
}

fn star_one(program:&Vec<Instruction>) -> String {
    let mut visited:HashSet<usize> = HashSet::new();
    let mut acc:i32 = 0;
    let mut pc:usize = 0;
    loop {
        let current = &program[pc];
        visited.insert(pc);
        match current {
            Instruction::Nop(_) => {pc += 1;},
            Instruction::Acc(x) => {acc += x; pc += 1;},
            Instruction::Jmp(x) => {pc = (*x + pc as i32) as usize ;},
        }
        if visited.contains(&pc) {
            break;
        }
    }
    acc.to_string()
}

fn star_two(program:&Vec<Instruction>) -> String {
    let mut visited:HashSet<usize> = HashSet::new();
    let mut acc:i32;
    let mut pc:usize;
    let mut changed:usize = 0;

    'outer: loop {
        if let Instruction::Acc(_) = program[changed] {
            changed += 1;
            continue;
        }

        visited.clear();
        acc = 0;
        pc = 0;
        loop {
            if visited.contains(&pc) {
                break;
            }
            if pc >= program.len() {
                break 'outer;
            }
            visited.insert(pc);
            let mut instruction = program[pc];
            if pc == changed {
                match instruction {
                    Instruction::Acc(_) => {panic!("SHOULD NEVER HAPPEN!!!");},
                    Instruction::Jmp(x) => {instruction = Instruction::Nop(x);},
                    Instruction::Nop(x) => {instruction = Instruction::Jmp(x);},
                }
            }
            match instruction {
                Instruction::Nop(_) => {pc += 1;},
                Instruction::Acc(x) => {acc += x;pc += 1;},
                Instruction::Jmp(x) => {pc = (x + pc as i32) as usize}
            }
        }
        changed += 1;
    }
    acc.to_string()
}

pub fn run_day(input_path:&str) {
    let input = setup(input_path);
    let one = star_one(&input);
    let two = star_two(&input);

    println!("Day 8.\nStar one: {one}\nStar two: {two}")
}