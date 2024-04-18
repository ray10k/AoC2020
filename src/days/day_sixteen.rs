use std::collections::{HashMap, HashSet};

use itertools::Itertools;

type ValidRules = HashMap<String,SingleRule>;
type TicketNumbers = Vec<u16>;

struct SingleRule {
    start0: u16,
    end0: u16,
    start1: u16,
    end1: u16
}

impl SingleRule {
    fn valid(&self,number:&u16) -> bool {
        (*number >= self.start0 && *number <= self.end0) ||
        (*number >= self.start1 && *number <= self.end1)
    }
}

struct TrainNumbers {
    rules:ValidRules,
    own_ticket:TicketNumbers,
    other_tickets:Vec<TicketNumbers>,
}
type State = TrainNumbers;

impl TrainNumbers {
    pub fn any_valid(&self, number:u16) -> bool {
        for rule in self.rules.values() {
            if rule.valid(&number)
            {
                return true
            }
        }
        false
    }
}

fn setup(input_path:&str) -> State {
    let input_content = std::fs::read_to_string(input_path)
        .expect("Could not open input file.");

    let mut input = input_content.lines();
    let mut rules = ValidRules::new();
    for line in &mut input {
        if line.trim() == "" {
            break;
        }
        //format:
        //<field-name>: <a>-<b> or <c>-<d>
        let name_end = line.find(':').unwrap();
        let first_sep = line.find('-').unwrap();
        let first_break = (line[first_sep..]).find(' ').unwrap() + first_sep;
        let second_sep = line.rfind('-').unwrap();
        let second_break = line.rfind(' ').unwrap();

        let field_name = &line[0..name_end];
        let a = line[name_end+2..first_sep].parse::<u16>().unwrap();
        let b = line[first_sep+1..first_break].parse::<u16>().unwrap();
        let c = line[second_break+1..second_sep].parse::<u16>().unwrap();
        let d = line[second_sep+1..].parse::<u16>().unwrap();
        rules.insert(field_name.into(), SingleRule{start0:a,end0:b,start1:c,end1:d});
    }

    //skip the "your ticket:" line
    input.next();

    let own_ticket = input.next()
        .unwrap()
        .trim()
        .split(',')
        .map(|x| x.parse::<u16>().unwrap())
        .collect::<Vec<u16>>();

    //Skip empty line, "nearby tickets:" line.
    input.next();
    input.next();

    let mut other_tickets:Vec<TicketNumbers> = Vec::new();
    for line in input {
        let current_ticket = line.trim()
            .split(',')
            .map(|x| x.parse::<u16>().unwrap())
            .collect::<Vec<u16>>();
        other_tickets.push(current_ticket);
    }

    TrainNumbers{rules:rules,own_ticket:own_ticket,other_tickets:other_tickets}
}

fn star_one(initial_state:&State) -> String {
    let mut retval:usize = 0;
    'tick: for ticket in initial_state.other_tickets.iter() {
        for number in ticket.iter() {
            if !initial_state.any_valid(*number) {
                retval += *number as usize;
                continue 'tick;
            }
        }
    }

    format!("{retval}")
}

fn star_two(initial_state:&State) -> String {
    let mut valid_tickets:Vec<&Vec<u16>> = Vec::new();
    'tick: for ticket in initial_state.other_tickets.iter() {
        for number in ticket.iter() {
            if !initial_state.any_valid(*number) {
                continue 'tick;
            }
        }
        valid_tickets.push(ticket);
    }
    let mut possible_fields:HashMap<String,Vec<usize>> = HashMap::new();
    let field_count = initial_state.rules.len();
    for (field_name,rule) in initial_state.rules.iter() {
        let mut possible_positions:HashSet<usize> = HashSet::from_iter(0..field_count);
        //Eliminate indices that this field can't be in, because a ticket exists that does not allow it.
        for ticket in valid_tickets.iter(){
            for (index,value) in ticket.iter().enumerate() {
                if !rule.valid(value) {
                    possible_positions.remove(&index);
                }
            }
        }

        possible_fields.insert(field_name.clone(), possible_positions.iter().map(|x| *x).collect_vec());
    }
    //Next step: find the *only* possible remaining position for each ticket-field.
    //The fields and positions form an N-by-N grid, where each column and each row
    // must have exactly one "valid" mark. Assume that the rows are for the field names,
    // and the columns are for the field-order indices.
    //First: Check if there are any rows that only have one known-not-invalid position.
    //Second: Check if there are any columns that only have one known-not-invalid position.
    //Third: For any row and any column with a known-valid position, mark all other cells as known-invalid.
    //Forth: If no complete name-to-position mapping is known already, go back to First. Otherwise, break out.
    loop {
        //Step 1
        let known_positions:HashSet<usize> = possible_fields.values()
            .filter(|ele| ele.len() == 1)
            .map(|ele| ele[0])
            .collect();
        //Step 2
        /*let known_fields:HashSet<usize> = (0..field_count)
            .filter(|value|{
                possible_fields.values().filter(|v| v.contains(value)).count() == 1
            })
            .collect();
        print!("KF: {known_fields:?} ");*/
        //Step 3
        possible_fields.values_mut()
            .filter(|ele| ele.len() > 1)
            .for_each(|ele| ele.retain(|x| !known_positions.contains(x)));

        if possible_fields.values().all(|possible| possible.len() == 1) {
            break;
        }
    }
    let field_mapping:HashMap<String,usize> = possible_fields.iter().map(|ele| (ele.0.clone(),ele.1[0])).collect();
    let result = field_mapping.iter()
        .filter(|pair| pair.0.starts_with("departure"))
        .map(|pair| initial_state.own_ticket[*pair.1] as usize)
        .fold(1,|acc,ele| acc * ele);
    format!("{result}")
}

pub fn run_day(input_path:&str) {
    let initial_state = setup(input_path);
    let one = star_one(&initial_state);
    let two = star_two(&initial_state);
    println!("Day 16.\nStar one: {one}\nStar two: {two}");
}