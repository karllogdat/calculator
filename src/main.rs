mod lexer;
mod parser;

use parser::parser::Parser;

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
    let mut parser = Parser::new(&input);
    let res = parser.get_expression_tree();
    match res {
        Ok(root) => println!("{}", root.evaluate()),
        Err(e) => println!("An error occured: {:?}", e),
    }
}
