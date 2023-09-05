use std::{collections::HashMap, fs};


fn setup(input_path:&str) -> Vec<HashMap<String,String>> {
    let input_string = fs::read_to_string(input_path).expect("Could not read file.");
    let mut retval:Vec<HashMap<String,String>> = Vec::new();
    let mut current:HashMap<String,String> = HashMap::new();
    for line in input_string.lines() {
        if line == "" {
            retval.push(current);
            current = HashMap::new();
        }
        for phrase in line.split(' '){
            if phrase == "" {
                continue;
            }
            let colon = phrase.find(':').expect(&format!("Malformed input <{phrase}>"));
            current.insert(String::from(&phrase[..colon]), String::from(&phrase[colon+1 ..]));
        }
    }
    return retval
}

fn star_one(parsed_input:&Vec<HashMap<String,String>>) -> String {
    let expected_keys = ["byr","iyr","eyr","hgt","hcl","ecl","pid","cid"];
    let mut retval = 0;
    'outer_s1: for passport in parsed_input.iter() {
        for key in expected_keys {
            if !passport.contains_key(key) {
                if key != "cid" {
                    continue 'outer_s1;
                }
            }
        }
        retval += 1;
    }
    return retval.to_string();
}

fn star_two(parsed_input:&Vec<HashMap<String,String>>) -> String {
    type VerFunc = fn(&String) -> bool;
    let mut verifications:HashMap<String,VerFunc> = HashMap::new();
    verifications.insert("byr".to_string(),|l| {
        l.len() == 4 && {
            let ynum = l.parse::<u32>().unwrap_or(0); 
            ynum >= 1920 && ynum <= 2002
        }
    });
    verifications.insert("iyr".to_string(),|l| {
        l.len() == 4 && {
            let ynum = l.parse::<u32>().unwrap_or(0);
            ynum >= 2010 && ynum <= 2020
        }
    });
    verifications.insert("eyr".to_string(), |l| {
        l.len() == 4 && {
            let ynum = l.parse::<u32>().unwrap_or(0);
            ynum >= 2020 && ynum <= 2030
        }
    });
    verifications.insert("hgt".to_string(), |l| {
        if l.len() < 3 {
            return false;
        }
        let height = &l[..l.len()-2];
        let height = height.parse::<u32>().unwrap_or(0);
        if l[l.len()-2 ..] == "in"[..] {
            height >= 59 && height <= 76
        } else if l[l.len()-2 ..] == "cm"[..] {
            height >= 150 && height <= 193
        } else {
            false
        }
    });
    verifications.insert("hcl".to_string(),|l| {
        let hex_digits:Vec<char> = "0123456789abcdef".chars().collect();
        let mut chars = l.chars();
        l.len() == 7 && chars.next().or(Some('!')).unwrap() == '#' && chars.all(|c| hex_digits.contains(&c))
    });
    verifications.insert("ecl".to_string(), |l| {
        let valid_colors = ["amb","blu","brn","gry","grn","hzl","oth"];
        valid_colors.contains(&&l[..]) //Don't really get why the double-ref is showing up...
    });
    verifications.insert("pid".to_string(),|l| {
        l.len() == 9 && l.chars().all(|c| "0123456789".contains(c))
    });

    let mut retval:u32 = 0;
    'outer_s2: for passport in parsed_input.iter() {
        for (field, verify) in verifications.iter(){
            if !verify(passport.get(field).unwrap_or(&"".to_string())) {
                continue 'outer_s2;
            }
        }
        retval += 1;
    }

    return retval.to_string();
}


pub fn run_day(input_path:&str) {
    let parsed_input = setup(input_path);
    let one = star_one(&parsed_input);
    let two = star_two(&parsed_input);
    println!("Day 4.\nStar one: {one}\nStar two: {two}");
}