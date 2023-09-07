use std::fs;



fn setup(input_path:&str) -> Vec<u16> {
    let input_string = fs::read_to_string(input_path).expect("Could not read file");
    let mut retval = Vec::new();

    for line in input_string.lines() {
        let num:u16 = line.parse().expect(&format!("Could not parse number {line}"));
        retval.push(num);
    }

    return retval;
}

fn star_one(adapters:&Vec<u16>) -> String {
    let mut ordered = adapters.to_vec();
    ordered.sort();
    let mut deltas: [u16;3] = [0;3];
    deltas[0] = 1;
    deltas[2] = 1;
    for window in ordered.windows(2) {
        //print!("{}-{} ",window[0],window[1]);
        let delta = (window[1] - window[0]) - 1;
        deltas[delta as usize] += 1;
    }
    (deltas[0] * deltas[2]).to_string()
}

fn star_two(adapters:&Vec<u16>) -> String {
    let maximum = *adapters.iter().max().unwrap();
    let mut sorted_adapters = adapters.to_vec();
    sorted_adapters.push(0);
    sorted_adapters.sort();
    //Build a *reverse* lookup table (index -> all possible previous adapters)
    //Build a route-total table (index -> number of ways to reach that adapter from 0)

    let mut lookup_table:Vec<Vec<usize>> = Vec::new();
    for _ in 0..=(maximum+3) {
        lookup_table.push(Vec::new());
    }
    for (index,v) in lookup_table.iter_mut().enumerate(){
        let min = {
            if index <= 3 {
                0
            } else {
                index as u16 - 3
            }
        };
        for adapter in sorted_adapters.iter() {
            if *adapter >= min && *adapter <= index as u16 {
                v.push(*adapter as usize);
            }
        }
    }

    let mut route_total:Vec<u64> = vec![0;(maximum+1) as usize];
    route_total[0] = 1;
    for i in sorted_adapters.iter() {
        let mut total = 0;
        for adapter in lookup_table[*i as usize].iter() {
            total += route_total[*adapter];
        }
        route_total[*i as usize] = total;
    }

    route_total.last().unwrap().to_string()
}

pub fn run_day(input_path:&str) {
    let values = setup(input_path);
    let one = star_one(&values);
    let two = star_two(&values);

    println!("Day 10.\nStar one: {one}\nStar two: {two}");
}