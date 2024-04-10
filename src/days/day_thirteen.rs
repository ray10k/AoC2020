use std::fs;


fn setup(input_path:&str) -> (usize, Vec<Option<usize>>){
    let input = fs::read_to_string(input_path).expect("Could not read input.");
    let mut lines = input.lines();
    let earliest = lines.next()
        .expect("Input not large enough.")
        .trim()
        .parse::<usize>()
        .expect("Could not parse earliest timestamp.");
    let bus_ids:Vec<Option<usize>> = lines.next()
        .expect("Input not large enough.")
        .trim()
        .split(',')
        .map(|x| {
            match x {
                "x" => None,
                _ => Some(
                    x.parse::<usize>()
                        .expect("Could not parse bus id.")
                )
            }
        })
        .collect();
    (earliest,bus_ids)
}

fn star_one(earliest:usize, bus_ids:&Vec<Option<usize>>) -> String {
    let mut best_time = usize::MAX;
    let mut best_id = 0;
    for bus_id in bus_ids {
        if let Some(id) = bus_id {
            let wait_time = id - (earliest % *id);
            if wait_time < best_time {
                best_id = *id;
                best_time = wait_time;
            }
        }
    }
    format!("{}",best_time * best_id)
}

fn star_two(earliest:usize, bus_ids:&Vec<Option<usize>>) -> String {
    "".into()
}

pub fn run_day(input_path:&str) {
    let (earliest, bus_ids) = setup(input_path);
    let one = star_one(earliest,&bus_ids);
    let two = star_two(earliest,&bus_ids);
    println!("Day 13.\nStar one: {one}\nStar two: {two}");
}