mod lexer;
mod parser;

use lexer::{Lexer, Token};
use parser::Parser;
use std::io::{self, Write};
use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Debug)]
enum CalculatorError {
    InvalidInput,
    DivideByZero,
    ParseError,
}

fn get_input() -> io::Result<String> {
    let mut input = String::new();
    io::stdout().flush()?;
    let _ = io::stdin().read_line(&mut input);
    Ok(input.trim().to_string())
}

fn get_precedence(token: &Token) -> Option<u8> {
    let op_precedence = HashMap::from([
        ('+', 0),
        ('-', 0),
        ('*', 1),
        ('/', 1),
    ]);

    match token {
        Token::PLUS => Some(*op_precedence.get(&'+').unwrap()),
        Token::MINUS => Some(*op_precedence.get(&'+').unwrap()),
        Token::TIMES => Some(*op_precedence.get(&'+').unwrap()),
        Token::DIVIDE => Some(*op_precedence.get(&'+').unwrap()),
        _ => None,
    }
}

fn get_postfix(input: &str) -> Result<Vec<Token>, CalculatorError> {
    let mut lexer = Lexer::new(&input);
    let mut tokens: Vec<Token> = vec![];
    let mut op_stack: VecDeque<Token> = VecDeque::new();
    loop {
        let token = lexer.next_token();
        match token {
            Token::NUMBER(_) => tokens.push(token),
            Token::PLUS | Token::MINUS | Token::TIMES | Token::DIVIDE => {
                if op_stack.is_empty() {
                    op_stack.push_front(token);
                    continue
                } else {
                    while get_precedence(op_stack.front().unwrap()).unwrap() > get_precedence(&token).unwrap() {
                        tokens.push(op_stack.pop_front().unwrap_or(Token::UNKNOWN));
                    }
                    op_stack.push_front(token);
                }
            }
            Token::EOF => break,
            Token::SPACE => continue,
            _ => return Err(CalculatorError::ParseError),
        }
    }

    while !op_stack.is_empty() {
        tokens.push(op_stack.pop_front().unwrap());
    }

    Ok(tokens)
}

fn main() {
    print!("Enter expression > ");
    let input = get_input().unwrap();
    let mut parser = Parser::new(&input);
    let _ = parser.get_expression_tree();
}
