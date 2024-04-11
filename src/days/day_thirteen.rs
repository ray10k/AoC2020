use std::{fs, iter::zip};


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

fn star_two(bus_ids:&Vec<Option<usize>>) -> String {
    //Note to self; buckle down and figure out the ins and outs of the 'Chinese Remainder Theorem'
    //Right now, it just doesn't *click* in the slightest for me.
    //"Translating" this solve:
    //https://topaz.github.io/paste/#XQAAAQB3AgAAAAAAAAA0nArZJKsTURSG1rtiib9en3kR1665KCklh+ikQxKEJUvAOO/fsTBPeU7QZpeRChctU6gy4MkDmzB7czWmotIUnVNf/w9AUhmeknGsDx/hY8fCwtK2UcMUGeWk9xh892snCESyU2DQKjBf+Jbmm5WEt6dMWDHAvnPZrAB2zlSNnkXIEvIkLmFQeCCrvibJtBBWeKoaqz0EH/Z6sQ0c5buxYopGRAUeYGkzAbqZsT7rGBUaIpwr0ZNZjckFm1gIVzCejYg030CG45ncFIxWpvb87XA2r3oblIaC10da/xLJR4XWMfuoRujjhy0UOopAfZxwSMjt0D4Wkzdf8+R1Q58xVwWlyTGwhpAo4KPuGtXrfqWXxA96XT80Up+ZXu6o2E5NFdo8ZbH698kCwAnUBwAkkTzJzia4Z5ZM81AhKEjdLHlF4TF3skdyBP/3I/ye
    let mut real_busses:Vec<usize> = Vec::new();
    let mut offsets:Vec<usize> = Vec::new();
    for (index,bus) in bus_ids.iter().enumerate() {
        if let Some(id) = *bus {
            real_busses.push(id);
            let temp_a = index % id;
            offsets.push((id - temp_a)%id);
        }
    }
    let cycle_length = real_busses.iter().fold(1, |acc,ele| acc * ele);
    let cycle_busses:Vec<usize> = real_busses.iter().map(|x| cycle_length / *x).collect();

    let mut something:Vec<usize> = Vec::with_capacity(cycle_busses.len());

    for (real,cyc) in zip(real_busses.iter(),cycle_busses.iter()){
        let base = cyc % real;
        let mut x = 1;
        'inner: loop {
            if ((x*base)%real) == 1{
                break 'inner;
            }
            x += 1;
        }
        something.push(x);
    }

    let result = zip(zip(offsets.iter(),cycle_busses.iter()),something.iter())
        .map(|x| {x.0.0 * x.0.1 *x.1})
        .fold(0,|acc, ele| acc + ele) //Since Iterator.sum() doesn't work with usize.
        % cycle_length;

    format!("{result}")
}

pub fn run_day(input_path:&str) {
    let (earliest, bus_ids) = setup(input_path);
    let one = star_one(earliest,&bus_ids);
    let two = star_two(&bus_ids);
    println!("Day 13.\nStar one: {one}\nStar two: {two}");
}