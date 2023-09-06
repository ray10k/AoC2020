use std::fs;
use std::collections::HashSet;

#[derive(PartialEq,Eq,Hash)]
struct TreeLoc {
    x:usize,
    y:usize,
}


fn setup(input_path:&str) -> (HashSet<TreeLoc>,usize,usize) {
    let mut retval:HashSet<TreeLoc> = HashSet::new();
    let mut height:usize = 0;
    let mut width:usize = 0;
    let input_contents = fs::read_to_string(input_path).expect("Could not open input file.");
    for (y,line) in input_contents.lines().enumerate() {
        for (x,letter) in line.chars().enumerate() {
            if letter == '#' {
                retval.insert(TreeLoc { x: x, y: y });
            }
            width = line.len();
        }
        height = y;
    }
    return (retval,height,width);
}

fn sled(trees:&HashSet<TreeLoc>,height:usize,width:usize,h_slope:usize,v_slope:usize) -> u32 {
    let mut tree_count: u32 = 0;
    for(steps,y) in (0..=height).step_by(v_slope).enumerate() {
        let x = (steps * h_slope) % width;
        if trees.contains(&TreeLoc { x: x, y: y }) {
            //println!("Hit a tree {x},{y}");
            tree_count += 1;
        }
    }
    tree_count
}

fn first_star(trees:&HashSet<TreeLoc>,height:usize,width:usize) -> String {
    sled(trees,height,width,3,1).to_string()
}

fn second_star(trees:&HashSet<TreeLoc>,height:usize,width:usize) -> String {
    let mut retval: u64 = 1;
    let speeds = [(1,1),(3,1),(5,1),(7,1),(1,2)];
    for (v_x,v_y) in speeds {
        let hits = u64::from(sled(trees,height,width,v_x,v_y));
        //println!("Hit {hits} trees, going {v_x} right and {v_y} down.");
        retval *= hits;
    }
    retval.to_string()
}

pub fn run_day(input_path:&str) {
    let (input,height,width) = setup(input_path);
    let first = first_star(&input,height,width);
    let second = second_star(&input,height,width);

    println!("Day 3.\nStar one: {first}\nStar two: {second}")
}