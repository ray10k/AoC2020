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

fn first_star(numbers:&Vec<u32>) -> String {
    assert!(numbers.len() >= 2);
    let end = numbers.len()-1;
    for start in 0..end {
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

fn second_star(numbers:&Vec<u32>)->String {
    assert!(numbers.len() >= 3);
    let end = numbers.len()-2;
    for first in 0..end {
        let one = numbers[first];
        for second in first+1 .. end+1{
            let two = numbers[second];
            for three in numbers[second+1 ..].iter(){
                let sum = one + two + *three;
                if sum == 2020 {
                    let product = one * two * *three;
                    return product.to_string();
                }
            }
        }
    }

    "ERROR".to_string()
}

pub fn run_day(input_path: &str) {
    let numbers = setup(input_path);
    let star_one = first_star(&numbers);
    let star_two = second_star(&numbers);
    println!("day 1.\nStar one: {star_one}\nStar two: {star_two}");

}
