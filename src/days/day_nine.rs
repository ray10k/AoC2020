use std::fs;
use std::cmp::{max,min};

const WINSIZE:usize = 25;

fn setup(input_path:&str) -> Vec<u64> {
    let mut retval:Vec<u64> = Vec::new();
    let input_string = fs::read_to_string(input_path).expect("Could not read file.");
    
    for line in input_string.lines() {
        retval.push(line.parse().expect(&format!("Could not parse <{line}>")[..]));
    }

    return retval;
}

fn star_one(values:&Vec<u64>) -> u64 {
    'outer: for window in values[..].windows(WINSIZE+1) {
        let target = window[WINSIZE];
        for a in 0..WINSIZE-1 {
            for b in a+1..WINSIZE {
                if (window[a] + window[b]) == target {
                    continue 'outer;
                }
            }
        }
        return target;
    }
    0
}

fn star_two(values:&Vec<u64>,target:u64) -> String {
    for start in 0..values.len()-1 {
        let mut min_val = u64::MAX;
        let mut max_val = 0;
        let mut sum = 0;
        for other in values[start..].iter() {
            sum += *other;
            min_val = min(min_val,*other);
            max_val = max(max_val,*other);
            if sum == target {
                return (min_val + max_val).to_string();
            }
            if sum > target {
                break;
            }
        }
    }
    "ERROR".to_string()
}

pub fn run_day(input_path:&str) {
    let values = setup(input_path);
    //Since star 2 requires whatever value came from star 1, it's a reasonable decision
    //to just pass that value along rather than having it start out as a String.
    let one = star_one(&values);
    let two = star_two(&values,one);
    let one = one.to_string();

    println!("Day 9.\nStar one: {one}\nStar two: {two}");
}