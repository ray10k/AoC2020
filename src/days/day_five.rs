use std::fs;
use std::cmp;
use std::sync::atomic::{AtomicBool,Ordering};

static VERBOSE: AtomicBool = AtomicBool::new(false);
pub fn verbosity_set(verbose:bool) {
    VERBOSE.store(verbose, Ordering::Release);
}

fn setup(input_file:&str) -> Vec<String> {
    let mut retval:Vec<String> = Vec::new();
    let input_str = fs::read_to_string(input_file).expect("Could not read file.");
    for line in input_str.lines() {
        retval.push(String::from(line));
    }
    if VERBOSE.fetch_and(true, Ordering::Acquire) {
        println!("Day 5 parsing completed, {} items found.",retval.len());
    }
    return retval;
}

fn seat_id(input:&str,verbose:bool) -> u32 {
    let mut max:u32 = 127;
    let mut min:u32 = 0;
    for instruction in input[..7].chars() {
        let middle = min + ((max - min) / 2);
        if instruction == 'F' {
            max = middle;
        } else {
            min = middle + 1;
        }
        if verbose {
            print!("({};{})",min,max);
        }
    }
    let row = min;
    max = 7;
    min = 0;
    for instruction in input[7..].chars() {
        let middle = min + ((max-min)/2);
        if instruction == 'R' {
            min = middle + 1;
        } else {
            max = middle;
        }
        if verbose {
            print!("({min};{max})");
        }
    }
    let col = min;
    if verbose {
        println!("");
    }
    return (row * 8) + col;
}

fn star_one(input:&Vec<String>) -> String {
    let mut retval = 0;
    let verbose = VERBOSE.fetch_and(true, Ordering::Acquire);
    for sequence in input {
        if verbose {
            println!("Parsing pass {sequence}.")
        }
        //Column is now col_min.
        let seat_id = seat_id(&sequence,verbose);
        retval = cmp::max(retval, seat_id);
        if verbose {
            println!("\nInput {} yields seat id {seat_id} (row {}, col {})",String::from(sequence),seat_id/8,seat_id%8);
        }
    }
    return retval.to_string();
}

fn star_two(input:&Vec<String>) -> String {
    let mut numbers:Vec<u32> = Vec::new();
    let verbose = VERBOSE.fetch_and(true, Ordering::Acquire);
    for pass in input.iter() {
        numbers.push(seat_id(pass, verbose));
    }
    numbers.sort();
    if verbose {
        println!("Found {} numbers; first {} and last {}",numbers.len(),numbers[0],numbers[numbers.len()-1]);
    }
    let mut retval:u32 = 0;
    for win in numbers.windows(2) {
        if win[0]+1 != win[1] {
            retval = win[0]+1;
            break;
        }
    }
    retval.to_string()
}


pub fn run_day(input_file:&str) {
    let parsed_input = setup(input_file);
    let one = star_one(&parsed_input);
    let two = star_two(&parsed_input);
    println!("Day 5.\nStar one: {one}\nStar two: {two}");

}