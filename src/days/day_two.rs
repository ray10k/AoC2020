use std::fs;
use std::convert::TryFrom;

struct PasswordLine{
    min:usize,
    max:usize,
    character:char,
    password:String,
}

impl TryFrom<&str> for PasswordLine {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        //check format
        if value.matches('-').count() != 1 {
            return Err("format mismatch; exactly 1 - is expected.");
        }
        if value.matches(' ').count() != 2 {
            return Err("format mismatch; exactly 2 spaces are expected.");
        }
        if value.matches(':').count() != 1 {
            return Err("format mismatch; exactly 1 : is expected.");
        }
        let parts: Vec<&str> = value.split([' ',':','-'].as_ref()).collect();
        let min:usize = parts[0].parse().unwrap();
        let max:usize = parts[1].parse().unwrap();
        let character = parts[2].chars().collect::<Vec<char>>()[0];
        let password = parts[4].to_owned();
        return Ok(PasswordLine { min, max: max, character: character, password: password });
    }
}

fn setup<'a>(input_path:&str) -> Vec<PasswordLine>{
    let mut retval:Vec<PasswordLine> = Vec::<PasswordLine>::new();
    let input_content = fs::read_to_string(input_path).unwrap();
    for line in input_content.lines() {
        if let Ok(pass_line) = PasswordLine::try_from(line) {
            retval.push(pass_line);
        }
    }

    retval
}

fn first_star(input:&Vec<PasswordLine>) -> String {
    let mut correct_count:u32 = 0;
    for pw in input{
        let letter_count = pw.password.matches(pw.character).count();
        if letter_count >= pw.min && letter_count <= pw.max {
            correct_count += 1;
        }
    }
    return correct_count.to_string();
}

fn second_star(input:&Vec<PasswordLine>) -> String {
    let mut correct_count:u32 = 0;

    for pw in input {
        let characters:Vec<char> = pw.password.chars().collect();
        let mut matches = 0;
        if characters[pw.min-1] == pw.character
            { matches += 1; }
        if characters[pw.max-1] == pw.character
            { matches += 1; }
        if matches == 1 
            { correct_count += 1;}
    }

    return correct_count.to_string();
}

pub fn run_day(input_path:&str) {
    let data = setup(input_path);
    let first = first_star(&data);
    let second = second_star(&data);

    println!("Day 2.\nStar one: {first}.\nStar two: {second}.");
}