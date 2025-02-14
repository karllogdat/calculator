use core::num;
use std::collections::VecDeque;
use std::fmt::Debug;

use crate::lexer::{self, Lexer, Token};

enum ParseError {
    InvalidTokenError,
    IncompleteOperandError,
}

#[derive(Debug, Clone)]
enum Node {
    Number(f64),
    Add(Box<Node>, Box<Node>),
    Subtract(Box<Node>, Box<Node>),
    Multiply(Box<Node>, Box<Node>),
    Divide(Box<Node>, Box<Node>),
}

impl Node {
    fn evaluate(&self) -> f64 {
        match self {
            Node::Number(n) => *n,
            Node::Add(l, r) => l.evaluate() + r.evaluate(),
            Node::Subtract(l, r) => l.evaluate() - r.evaluate(),
            Node::Multiply(l, r) => l.evaluate() * r.evaluate(),
            Node::Divide(l, r) => l.evaluate() / r.evaluate(),
        }
    }
}

pub struct Parser<'a> {
    lexer: Lexer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            lexer: (Lexer::new(input)),
        }
    }

    fn get_expression_tree(&mut self) -> Result<Box<Node>, ParseError> {
        let mut output_queue: VecDeque<Box<Node>> = VecDeque::new();
        let mut op_stack: VecDeque<Token> = VecDeque::new();
        loop {
            let token = self.lexer.next_token();
            match token {
                Token::EOF => break,
                Token::SPACE => continue,
                Token::NUMBER(value) => {
                    output_queue.push_front(Box::new(Node::Number(value)));
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

                            match token {
                                Token::PLUS => {
                                    output_queue.push_front(Box::new(Node::Add(left, right)))
                                }
                                _ => return Err(ParseError::InvalidTokenError),
                            }
                        }
                    }
                }
                Token::UNKNOWN => return Err(ParseError::InvalidTokenError),
            }
        }

        Ok(Box::new(*output_queue.front().unwrap().clone()))
    }
}
