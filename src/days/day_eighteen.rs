
#[derive(Debug)]
enum Token {
    Literal(u8),
    Add,
    Mult,
    Open,
    Close
}

type State = Vec<Vec<Token>>;

fn setup(input_path:&str) -> State {
    std::fs::read_to_string(input_path)
        .expect("Could not open input file")
        .lines()
        .map(|line|{
            line.trim()
                .chars()
                .filter_map(|c| 
                    match c {
                        '+' => Some(Token::Add),
                        '*' => Some(Token::Mult),
                        '(' => Some(Token::Open),
                        ')' => Some(Token::Close),
                        ' ' => None,
                        n if "0123456789".contains(n) => Some(Token::Literal(n.to_digit(10).map_or(0, |x| x as u8))),
                        _ => panic!("unknown character")
                    }
                ).collect()
        })
        .collect()
}

fn reverse_polish_calc(input:Vec<&Token>) -> usize {
    let mut stack:Vec<usize> = Vec::new();
    for token in input {
        match token {
            Token::Literal(x) => stack.push(*x as usize),
            Token::Add => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(a+b);
            },
            Token::Mult => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(a*b);
            },
            Token::Open|Token::Close => panic!("Parentheses in equation!!"),
        }
    }
    if stack.len() != 1 {
        panic!("Equation did not yield a single result!");
    }
    stack[0]
}

fn star_one(initial_state:&State) -> String {
    let mut retval:usize = 0;
    for statement in initial_state.iter() {
        let mut stack:Vec<&Token> = Vec::with_capacity(statement.len()/2);
        let mut output:Vec<&Token> = Vec::with_capacity(statement.len());
        for item in statement.iter(){ //Shunting-yard algo
            match item {
                Token::Literal(_) => {
                    output.push(item);
                    while let Some(Token::Add)|Some(Token::Mult) = stack.last() {
                        output.push(stack.pop().unwrap());
                    }
                },
                Token::Add|Token::Mult|Token::Open => stack.push(item),
                Token::Close => {
                    loop {
                        if let Some(Token::Open) = stack.last() {
                            stack.pop();
                            if let Some(&Token::Add|&Token::Mult) = stack.last() {
                                output.push(stack.pop().unwrap())
                            }
                            break;
                        }
                        if let None = stack.last() {
                            break;
                        }
                        output.push(stack.pop().unwrap());
                    }
                },
            }
        }
        for op in stack.into_iter().rev() {
            output.push(op);
        }
        
        //println!("final: {}",stack[0]);
        retval += reverse_polish_calc(output);
    }
    format!("{retval}")
}

fn star_two(initial_state:&State) -> String {
    let mut retval:usize = 0;
    for line in initial_state.iter() {
        let mut stack:Vec<&Token> = Vec::with_capacity(line.len()/2);
        let mut output:Vec<&Token> = Vec::with_capacity(line.len());

        for token in line.iter() {
            match token {
                Token::Literal(_) => {
                    output.push(token);
                },
                Token::Add | Token::Open => {
                    stack.push(token);
                },
                Token::Mult => {
                    while let Some(Token::Add) = stack.last() {
                        output.push(stack.pop().unwrap());
                    }
                    stack.push(token);
                },
                Token::Close => {
                    loop {
                        if let Some(Token::Open) = stack.last() {
                            stack.pop();
                            break;
                        }
                        let top = stack.pop();
                        if let None = top {
                            break;
                        }
                        output.push(top.unwrap());
                    }
                },
            }
        }
        for op in stack.into_iter().rev() {
            output.push(op);
        }
        //print!("{output:?} -> ");
        let result = reverse_polish_calc(output);
        //println!("{result}");
        retval += result;
    }
    format!("{retval}")
}

pub fn run_day(input_path:&str) {
    let initial_state = setup(input_path);
    let one = star_one(&initial_state);
    let two = star_two(&initial_state);
    println!("Day 18.\nStar one: {one}\nStar two: {two}");
}