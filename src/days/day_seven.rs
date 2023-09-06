use std::collections::{HashMap, HashSet};
use std::fs; 
use std::str::FromStr;


struct BagRule {
    color:String,
    contains:Vec<(String,u32)>
}

impl FromStr for BagRule {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sep_position = s.find(" bags contain ");
        if let None = sep_position {
            return Err("Separator not found, string is malformed.".to_string());
        }
        let sep_position = sep_position.unwrap();
        let color = s[..sep_position].to_string();
        let sep_end = sep_position + " bags contain ".len();
        let mut contains:Vec<(String,u32)> = Vec::new();
        if let Some(_) = s.find(" no ") {
            return Ok(BagRule{color,contains});
        }
        for bag in s[sep_end..].split(", ") {
            let mut end_offset = 4; 
            let sub_split = bag.find(' ').unwrap();
            let amount:u32 = bag[..sub_split].parse().unwrap();
            if amount != 1 {
                end_offset += 1;
            }
            if bag.ends_with('.') {
                end_offset += 1;
            }
            let other_color:String = bag[sub_split+1..bag.len()-end_offset].to_string();
            contains.push((other_color,amount));
        }
        return Ok(BagRule{color,contains});
    }
}

impl ToString for BagRule{
    fn to_string(&self) -> String {
        if self.contains.is_empty() {
            let mut retval = String::from(&self.color);
            retval.push_str(" bags contain no further bags.");
            return retval;
        }
        let mut retval = String::from("Bagging rule for ");
        retval.push_str(&self.color);
        retval.push_str(" bags: ");
        for (color, amount) in &self.contains {
            retval.push_str(&amount.to_string());
            retval.push_str(" times a ");
            retval.push_str(color);
            retval.push_str(" bag, ");
        }
        retval.replace_range(retval.len()-2.., ".");
        return retval;
    }
}

fn setup(input_path:&str) -> Vec<BagRule> {
    let str_input = fs::read_to_string(input_path).expect("Could not read file.");
    let mut retval:Vec<BagRule> = Vec::new();
    for rule in str_input.lines(){
        if let Ok(p_rule) = BagRule::from_str(rule) {
            retval.push(p_rule);
        }
    }
    return retval;
}

fn star_one(input:&Vec<BagRule>) -> String {
    //General idea: build a 'reverse graph' and collect all bags, starting from 'shiny gold'.
    let mut reverse_tree:HashMap<&str,Vec<&str>> = HashMap::new();
    for rule in input {
        for (other,_) in &rule.contains {
            let key = &(*other)[..];
            if !reverse_tree.contains_key(key) {
                reverse_tree.insert(&other,Vec::<&str>::new());
            }
            reverse_tree.get_mut(key).expect("Should never happen!").push(&rule.color);
        }
    };
    let mut containing_bags:HashSet<&str> = HashSet::new();
    let mut to_check = Vec::new();
    to_check.push("shiny gold");
    let mut next_round: HashSet<&str> = HashSet::new();
    while !&to_check.is_empty() {
        next_round.clear();
        for item in to_check.iter() {
            if let Some(container) = reverse_tree.get(*item) {
                next_round.extend(container.iter());
            }
        }
        for checked in containing_bags.iter() {
            next_round.remove(*checked);
        }
        containing_bags.extend(next_round.iter());
        to_check.clear();
        to_check.extend(next_round.iter());
    }

    containing_bags.len().to_string()
}

fn star_two(input:&Vec<BagRule>) -> String {
    //We've done backward traversal, time for forward traversal.
    //Assuming no loops (as in, red contains blue contains red)
    //Approach: Depth-first traversal until the first empty bag,
    //track total with a stack. Recursion?

    //Need to build a forward tree this time.
    let mut tree:HashMap<&str,Vec<(&str,u32)>> = HashMap::new();
    for rule in input {
        let mut containing = Vec::new();
        for (other,count) in rule.contains.iter() {
            containing.push((&other[..],*count));
        }
        tree.insert(&rule.color,containing);
    }

    fn visit_bag(to_visit:&str,tree:&HashMap<&str,Vec<(&str,u32)>>) -> u32 {
        let containing = tree.get(to_visit).unwrap();
        if containing.is_empty() {
            return 1;
        } else {
            let mut total = 1;
            for (other, amount) in containing.iter() {
                let subtotal = *amount * visit_bag(*other, &tree);
                total += subtotal;
            }
            total
        }
    }

    return (visit_bag("shiny gold", &tree)-1).to_string();
}


pub fn run_day(input_path:&str) {
    let rules = setup(input_path);
    let one = star_one(&rules);
    let two = star_two(&rules);

    println!("Day 7.\nStar one: {one}\nStar two: {two}");
}