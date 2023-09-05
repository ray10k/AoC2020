use std::fs;
use std::collections::HashSet;
use std::sync::atomic::{AtomicBool,Ordering};

static VERBOSE: AtomicBool = AtomicBool::new(false);
pub fn verbosity_set(verbose:bool) {
    VERBOSE.store(verbose, Ordering::Release);
}

fn setup(input_file:&str) -> Vec<Vec<Vec<char>>> { //groups, persons, answers.
    let input_data = fs::read_to_string(input_file).expect("Could not read file.");
    let mut retval:Vec<Vec<Vec<char>>> = Vec::new();
    let mut group:Vec<Vec<char>> = Vec::new();
    let verbose = VERBOSE.fetch_and(true, Ordering::Acquire);
    for person in input_data.lines() {
        if person == "" {
            if verbose {
                println!("Group {:?} parsed.",group);
            }
            retval.push(group);
            group = Vec::new();
        } else {
            group.push(person.chars().collect());
        }
    }
    if verbose {
        println!("Final group {:?} parsed.",group);
    }
    retval.push(group);

    return retval;
}

fn star_one(input:&Vec<Vec<Vec<char>>>) -> String {
    let mut retval = 0;
    for group in input.iter() {
        let mut answers:HashSet<char> = HashSet::new();
        for person in group {
            for answer in person {
                answers.insert(*answer);
            }
        }
        retval += answers.len();
    }
    retval.to_string()
}

fn star_two(input:&Vec<Vec<Vec<char>>>) -> String {
    let mut retval = 0;
    
    let mut yesses:HashSet<char> = HashSet::new();
    for group in input.iter() {
        yesses.extend(group[0].iter());
        if group.len() > 1 {
            for answer in group[1..].iter() {
                let mut answers:HashSet<char> = HashSet::new();
                answers.extend(answer);
                yesses.retain(|x| answers.contains(x));
            }
        }
        retval += yesses.len();
        yesses.clear();
    }

    retval.to_string()
}

pub fn run_day(input_file:&str) {
    let parsed_data = setup(input_file);
    let one = star_one(&parsed_data);
    let two = star_two(&parsed_data);

    println!("Day six.\nStar one: {one}\nStar two: {two}");
}