use std::collections::HashMap;
use std::fmt::{Display,Formatter};


type ValidRules = HashMap<String,(u16,u16,u16,u16)>;
type TicketNumbers = Vec<u16>;

struct TrainNumbers {
    rules:ValidRules,
    own_ticket:TicketNumbers,
    other_tickets:Vec<TicketNumbers>,
}
type State = TrainNumbers;

impl TrainNumbers {
    pub fn any_valid(&self, number:u16) -> bool {
        for (name,rule) in self.rules.iter() {
            if (number >= rule.0 && number <= rule.1) || (number >= rule.2 && number <= rule.3)
            {
                return true
            }
        }
        false
    }
}

impl Display for TrainNumbers {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"Rules: {:?}\nOwn ticket: {:?}\nOther tickets: {:?}",self.rules,self.own_ticket,self.other_tickets)
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
        rules.insert(field_name.into(), (a,b,c,d));
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
    "".into()
}

pub fn run_day(input_path:&str) {
    let initial_state = setup(input_path);
    let one = star_one(&initial_state);
    let two = star_two(&initial_state);
    println!("Day 16.\nStar one: {one}\nStar two: {two}");
}