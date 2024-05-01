use fancy_regex::Regex;


#[derive(Debug,Clone,PartialEq)]
enum Value {
    /// For the example input, and for star 2
    Trio(u8,u8,u8),
    /// Two rule-ids, in sequence.
    Duo(u8,u8),
    /// A single rule-id.
    Mono(u8),
    /// Character 'a'
    LiteralA,
    /// Character 'b'
    LiteralB,
}

#[derive(Debug,Clone,PartialEq)]
enum RuleType {
    /// Single rule-id or literal
    Single(u8,Value),
    /// Pair of rule-ids or values, separated by a pipe.
    Double(u8,Value,Value)
}

struct ParsedInput {
    rules:Vec<RuleType>,
    messages:Vec<String>
}

const MAX_DEPTH:usize = 27;

impl ParsedInput {

    fn rule_to_regex(&self, rule_id:u8) -> String {
        let mut retval = String::from("");
        //If one branch can cover at least this many
        //characters of input, there is no need to look further; The longest input
        //is covered at this point.
        self.render_rule(&mut retval, rule_id, 0);

        retval
    }

    fn render_rule(&self, regex:&mut String, rule_id:u8, depth:usize) -> usize {
        match self.id_to_rule(rule_id) {
            Some(RuleType::Single(_, v)) => {
                self.render_value(regex, v, depth)
            },
            Some(RuleType::Double(_, left, right)) => {
                regex.push('(');
                let a = self.render_value(regex, left, depth);
                regex.push('|');
                let b = self.render_value(regex, right, depth);
                regex.push(')');
                std::cmp::min(a,b)
            },
            None => depth,
        }
    }

    fn render_value(&self, regex:&mut String, val:&Value, depth:usize) -> usize {
        if depth >= MAX_DEPTH {
            return depth;
        }
        match val {
            Value::Trio(one, two, three) => {
                let a = self.render_rule(regex, *one, depth);
                let b = self.render_rule(regex, *two, depth);
                let c = self.render_rule(regex, *three, depth);
                std::cmp::min(a,std::cmp::min(b,c))
            },
            Value::Duo(left, right) => {
                let a = self.render_rule(regex, *left, depth);
                let b = self.render_rule(regex, *right, depth);
                std::cmp::min(a,b)
            },
            Value::Mono(val) => {
                self.render_rule(regex, *val, depth)
            },
            Value::LiteralA => {
                regex.push('a');
                depth + 1
            },
            Value::LiteralB => {
                regex.push('b');
                depth + 1
            },
        }

    }

    fn id_to_rule(&self, rule_id:u8) -> Option<&RuleType> {
        match self.rules.binary_search_by_key(&rule_id, |ele|{
            match ele {
                RuleType::Single(id, _)|RuleType::Double(id, _, _) => *id,
            }
        }){
            Ok(index) => Some(&self.rules[index]),
            Err(_) => None,
        }
    }

    fn with_alteration(&self, replacements:Vec<RuleType>) -> Self {
        let mut new_rules:Vec<RuleType> = self.rules.clone();
        for replacement in replacements {
            match replacement{
                RuleType::Single(id, _)|
                RuleType::Double(id, _, _) => {
                    match new_rules.binary_search_by_key(&id,|ele| match ele {
                        RuleType::Single(id, _)|RuleType::Double(id, _, _) => *id,
                    }) {
                        Ok(index) => new_rules[index] = replacement,
                        Err(index) => new_rules.insert(index,replacement),
                    }
                },
            }
        }
        Self{rules:new_rules,messages:self.messages.clone()}
    }
}

type State = ParsedInput;

fn setup(input_path:&str) -> State {
    let mut rules:Vec<RuleType> = Vec::new();
    let mut messages:Vec<String> = Vec::new();
    let input_data = std::fs::read_to_string(input_path).expect("Could not open input file");
    let mut line_itr = input_data.lines();
    loop {
        let line = line_itr.next().unwrap().trim();
        if line == "" {
            break
        }
        let colon = line.find(':').unwrap();
        let rule_id = line[0..colon].parse::<u8>().unwrap();
        if line.contains('"') {
            //must be a literal.
            let last_quote = line.rfind('"').unwrap();
            if &line[last_quote-1..last_quote] == "a" {
                rules.push(RuleType::Single(rule_id, Value::LiteralA));
            } else {
                rules.push(RuleType::Single(rule_id, Value::LiteralB));
            }
            continue;
        }
        if line.contains("|") {
            //pair
            let splitpoint = line.find('|').unwrap();
            let left = line[colon+1 .. splitpoint].trim();
            let right = line[splitpoint+1 ..].trim();
            let v_left:Value;
            let v_right:Value;
            if left.contains(' ') {
                let first_space = left.find(' ').unwrap();
                let a = left[0..first_space].parse::<u8>().unwrap();
                let b = left[first_space+1 ..].parse::<u8>().unwrap();
                v_left = Value::Duo(a, b);
            } else {
                let a = left.parse::<u8>().unwrap();
                v_left = Value::Mono(a);
            }

            if right.contains(' ') {
                let last_space = right.find(' ').unwrap();
                let c = right[0..last_space].parse::<u8>().unwrap();
                let d = right[last_space+1 ..].parse::<u8>().unwrap();
                v_right = Value::Duo(c, d);
            } else {
                let c = right.parse::<u8>().unwrap();
                v_right = Value::Mono(c);
            }
            rules.push(RuleType::Double(rule_id, v_left,v_right));
        } else {
            //single
            let relevant = line[colon+1..].trim();
            let space = relevant.find(' ');
            if let Some(space) = space {
                let space_count = relevant.chars().filter(|c| *c == ' ').count();
                let a = relevant[0..space].parse::<u8>().unwrap();
                if space_count == 2 {
                    let other_space = relevant.rfind(' ').unwrap();
                    let b = relevant[space+1 .. other_space].parse::<u8>().unwrap();
                    let c = relevant[other_space+1 ..].parse::<u8>().unwrap();
                    rules.push(RuleType::Single(rule_id, Value::Trio(a, b, c)));
                } else {
                    let b = relevant[space+1 ..].parse::<u8>().unwrap();
                    rules.push(RuleType::Single(rule_id, Value::Duo(a, b)));    
                }
            } else {
                let a = relevant.parse::<u8>().unwrap();
                rules.push(RuleType::Single(rule_id, Value::Mono(a)));
            }
        }
    }
    rules.sort_by_key(|r| match r {
        RuleType::Single(x, _) => *x,
        RuleType::Double(x, _, _) => *x,
    });
    for line in line_itr {
        messages.push(String::from(line.trim()));
    }
    ParsedInput{
        rules:rules,
        messages:messages
    }
}

fn star_one(initial_state:&State) -> String {

    let pattern = initial_state.rule_to_regex(0);
    let pattern = format!("^{pattern}$");
    let pattern = Regex::new(&pattern).expect("Invalid ruleset.");
    let mut retval = 0;

    for msg in initial_state.messages.iter(){
        if let Result::Ok(true) = pattern.is_match(&msg) {
            retval += 1;
        }
    }
    
    format!("{retval}")
}

fn star_two(initial_state:&State) -> String {
    let initial_state = initial_state.with_alteration(vec![
        RuleType::Double(8, Value::Mono(42), Value::Duo(42, 8)),
        RuleType::Double(11, Value::Duo(42,31),Value::Trio(42, 11, 31))
    ]);
    let patt_42_s = format!("^({})",initial_state.rule_to_regex(42));
    let patt_42 = Regex::new(&patt_42_s).expect("invalid expression");
    let patt_31_s = format!("^({})",initial_state.rule_to_regex(31));
    let patt_31 = Regex::new(&patt_31_s).expect("Invalid expression");
    let mut retval = 0;
    for line in initial_state.messages.iter() {
        let mut starting_point:usize = 0;
        let mut count_l = 0;
        loop {
            if let Ok(Some(match_)) = patt_42.find(&line[starting_point..]) {
                count_l += 1;
                starting_point += match_.end();
            } else {
                break;
            }
        };
        let mut count_r = 0;
        loop {
            if let Ok(Some(match_)) = patt_31.find(&line[starting_point..]) {
                count_r += 1;
                starting_point += match_.end();
            } else {
                break;
            }
        }
        if count_l > count_r && count_r > 0 && starting_point == line.len() {
            retval += 1;
        };
    }

    format!("{retval}")
}

pub fn run_day(input_path:&str) {
    let initial_state = setup(input_path);
    let one = star_one(&initial_state);
    let two = star_two(&initial_state);
    println!("Day 19.\nStar one: {one}\nStar two: {two}");
}