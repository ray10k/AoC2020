use std::collections::HashMap;



fn setup(input_path:&str) -> Vec<u8> {
    std::fs::read_to_string(input_path)
        .expect("Could not open input file")
        .trim()
        .split(',')
        .map(|x| x.parse::<u8>().unwrap_or_default())
        .collect::<Vec<u8>>()
}

fn star_one(initial_state:&Vec<u8>) -> String {
    // Key: the number mentioned. Value: (The turn it was last mentioned on, the turn before that it was mentioned on).
    let mut last_mention:HashMap<usize,(usize,usize)> = HashMap::new();
    let mut most_recent:usize = 0;
    for (index,number) in initial_state.iter().enumerate() {
        last_mention.insert(*number as usize, (index + 1,0));
        most_recent = *number as usize;
    }

    for turn_number in initial_state.len()+1..=2020 {
        match last_mention.get(&most_recent) {
            Some((_,0))|None => { //Mentioned once/never. Call 0.
                let previous_zero = last_mention.get(&0).unwrap_or(&(0,0));
                most_recent = 0;
                last_mention.insert(most_recent, (turn_number,previous_zero.0));
            },
            Some(turn) => { //Mentioned at least twice.
                let difference = turn.0 - turn.1;
                let previous_call = last_mention.get(&difference).unwrap_or(&(0,0));
                most_recent = difference;
                last_mention.insert(most_recent,(turn_number,previous_call.0));
            },
        }
    }
    format!("{most_recent}")
}

fn star_two(initial_state:&Vec<u8>) -> String {
        // Key: the number mentioned. Value: (The turn it was last mentioned on, the turn before that it was mentioned on).
        let mut last_mention:HashMap<usize,(usize,usize)> = HashMap::new();
        let mut most_recent:usize = 0;
        for (index,number) in initial_state.iter().enumerate() {
            last_mention.insert(*number as usize, (index + 1,0));
            most_recent = *number as usize;
        }
    
        for turn_number in initial_state.len()+1..=30_000_000 {
            match last_mention.get(&most_recent) {
                Some((_,0))|None => { //Mentioned once/never. Call 0.
                    let previous_zero = last_mention.get(&0).unwrap_or(&(0,0));
                    most_recent = 0;
                    last_mention.insert(most_recent, (turn_number,previous_zero.0));
                },
                Some(turn) => { //Mentioned at least twice.
                    let difference = turn.0 - turn.1;
                    let previous_call = last_mention.get(&difference).unwrap_or(&(0,0));
                    most_recent = difference;
                    last_mention.insert(most_recent,(turn_number,previous_call.0));
                },
            }
        }
        format!("{most_recent}")
}

pub fn run_day(input_path:&str) {
    let initial_state = setup(input_path);
    let one = star_one(&initial_state);
    let two = star_two(&initial_state);
    println!("Day 15.\nStar one: {one}\nStar two: {two}");
}