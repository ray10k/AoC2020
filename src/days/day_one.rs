use std::fs;

fn setup(input_path: &str) -> Vec<u32> {
    let input_content = fs::read_to_string(input_path).expect("Error reading input file.");
    let mut retval:Vec<u32> = Vec::new();
    for number in input_content.lines(){
        if let Ok(num) = number.parse::<u32>() {
            retval.push(num);
        }
    };
    retval
}

fn first_star(numbers:Vec<u32>) -> String {
    for start in 0..numbers.len() {
        let one = numbers[start];
        for two in numbers[start+1..].iter() {
            let sum = one + *two;
            if sum == 2020 {
                let product = one * *two;
                return product.to_string();
            }
        }
    }
    "ERROR".to_string()
}

fn second_star()->String {
    "".to_string()
}

pub fn run_day(input_path: &str) {
    let numbers = setup(input_path);
    println!("Parsed number count: {}",numbers.len());
    let star_one = first_star(numbers);
    let star_two = second_star();
    println!("AoC 2020, day 1.\nStar 1 output: {star_one}.\nStar 2 output: {star_two}.");

}
