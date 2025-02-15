use crate::parser::node::Node;
use std::collections::VecDeque;

use crate::lexer::{Lexer, Token};

pub enum ParseError {
    InvalidTokenError,
    IncompleteOperandError,
}

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    tree: Option<Box<Node>>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            lexer: (Lexer::new(input)),
            tree: None,
        }
    }

    pub fn get_expression_tree(&mut self) -> Result<Box<Node>, ParseError> {
        let mut output_queue: VecDeque<Box<Node>> = VecDeque::new();
        let mut op_stack: VecDeque<Token> = VecDeque::new();
        loop {
            let token = self.lexer.next_token();
            match token {
                Token::EOF => break,
                Token::SPACE => continue,
                Token::NUMBER(_) => {
                    let node = Node::new_num(token);
                    if let Some(node) = node {
                        output_queue.push_front(node);
                    }
                }
                Token::PLUS | Token::MINUS | Token::TIMES | Token::DIVIDE => {
                    if op_stack.is_empty() {
                        op_stack.push_front(token);
                        continue;
                    } else {
                        while token.get_precedence() < op_stack.front().unwrap().get_precedence() {
                            let right = output_queue.pop_front();
                            match right {
                                None => return Err(ParseError::IncompleteOperandError),
                                Some(_) => (),
                            }
                            let right = right.unwrap();
                            let left = output_queue.pop_front();
                            match left {
                                None => return Err(ParseError::IncompleteOperandError),
                                Some(_) => (),
                            }
                            let left = left.unwrap();
                            let node = Node::new_operator(&token, left, right);
                            if let Some(node) = node {
                                output_queue.push_front(node);
                            }
                        }
                        op_stack.push_front(token);
                    }
                }
                Token::UNKNOWN => return Err(ParseError::InvalidTokenError),
            }
        }

        while !op_stack.is_empty() {
            let token = op_stack.pop_front().unwrap();
            let right = output_queue.pop_front();
            let left = output_queue.pop_front();
            if left.is_none() || right.is_none() {
                return Err(ParseError::IncompleteOperandError);
            }
            let node = Node::new_operator(&token, left.unwrap(), right.unwrap());
            if let Some(node) = node {
                output_queue.push_front(node);
            }
        }

        println!("The evaluated smth is {}", output_queue.front().unwrap().evaluate());
        Ok(Box::new(*output_queue.front().unwrap().clone()))
    }
}
