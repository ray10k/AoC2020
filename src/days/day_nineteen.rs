
#[derive(Debug)]
enum Value {
    Duo(u8,u8),
    Mono(u8),
    LiteralA,
    LiteralB,
}

#[derive(Debug)]
enum RuleType {
    Single(u8,Value),
    Double(u8,Value,Value)
}

struct ParsedInput {
    rules:Vec<RuleType>,
    messages:Vec<String>
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
            let space = relevant.rfind(' ');
            if let Some(space) = space {
                let a = relevant[0..space].parse::<u8>().unwrap();
                let b = relevant[space+1 ..].parse::<u8>().unwrap();
                rules.push(RuleType::Single(rule_id, Value::Duo(a, b)));    
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
    
    "".into()
}

fn star_two(initial_state:&State) -> String {
    "".into()
}

pub fn run_day(input_path:&str) {
    let initial_state = setup(input_path);
    let one = star_one(&initial_state);
    let two = star_two(&initial_state);
    println!("Day 19.\nStar one: {one}\nStar two: {two}");
}