use std::io::{self, Write};

enum Token {
    PLUS,
    MINUS,
    TIMES,
    DIVIDE,
    NUMBER(f64),
    SPACE,
    UNKNOWN,
}

enum Operation {
    ADD(f64, f64),
    SUBTRACT(f64, f64),
    MULTIPLY(f64, f64),
    DIVIDE(f64, f64),
}

fn get_input() -> io::Result<String> {
    let mut input = String::new();
    io::stdout().flush()?;
    let _ = io::stdin().read_line(&mut input);
    Ok(input.trim().to_string())
}

fn tokenize(expression: &str) -> io::Result<Vec<Token>> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = expression.chars().collect();

    for char in chars {
        if char.is_numeric() {
            tokens.push(Token::NUMBER(char.to_digit(10).unwrap() as f64));
            continue;
        }
            
        match char {
            '+' => tokens.push(Token::PLUS),
            '-' => tokens.push(Token::MINUS),
            '*' => tokens.push(Token::TIMES),
            '/' => tokens.push(Token::DIVIDE),
            ' ' => tokens.push(Token::SPACE),
            _ => tokens.push(Token::UNKNOWN),
        }
    }

    Ok(tokens)
}

fn evaluate(tokens: &Vec<Token>) -> f64 {
    todo!();
}

fn main() {
    println!("Enter an expression: ");
    let string = get_input().unwrap();
    let tokens = tokenize(&string.as_str()).unwrap();
    for token in tokens {
        match token {
            Token::NUMBER(num) => println!("{}", num),
            Token::PLUS => println!("+"),
            Token::MINUS => println!("-"),
            Token::TIMES => println!("*"),
            Token::DIVIDE => println!("/"),
            _ => (),
        }
    }
}
