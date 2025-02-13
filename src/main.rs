mod lexer;

use lexer::{Lexer, Token};
use std::io::{self, Write};

fn get_input() -> io::Result<String> {
    let mut input = String::new();
    io::stdout().flush()?;
    let _ = io::stdin().read_line(&mut input);
    Ok(input.trim().to_string())
}

fn main() {
    print!("Enter expression > ");
    let input = get_input().unwrap();
    let mut lexer = Lexer::new(&input);
    loop {
        let token = lexer.next_token();
        match token {
            Token::EOF => break,
            Token::SPACE => continue,
            _ => println!("{:?}", token),
        }
    }
}
