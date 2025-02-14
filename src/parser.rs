use core::num;
use std::collections::VecDeque;
use std::fmt::Debug;

use crate::lexer::{self, Lexer, Token};

enum ParseError {
    InvalidTokenError,
    IncompleteOperandError,
}

trait Evaluable: Debug {
    fn evaluate(&self) -> f64;
}

#[derive(Debug)]
struct NumberNode {
    value: f64,
}

#[derive(Debug)]
struct AddNode {
    left: Box<dyn Evaluable>,
    right: Box<dyn Evaluable>,
}

#[derive(Debug)]
struct SubtractNode {
    left: Box<dyn Evaluable>,
    right: Box<dyn Evaluable>,
}

#[derive(Debug)]
struct MultiplyNode {
    left: Box<dyn Evaluable>,
    right: Box<dyn Evaluable>,
}

#[derive(Debug)]
struct DivideNode {
    left: Box<dyn Evaluable>,
    right: Box<dyn Evaluable>,
}

impl Evaluable for NumberNode {
    fn evaluate(&self) -> f64 {
        self.value
    }
}

impl Evaluable for AddNode {
    fn evaluate(&self) -> f64 {
        self.left.evaluate() + self.right.evaluate()
    }
}

impl Evaluable for SubtractNode {
    fn evaluate(&self) -> f64 {
        self.left.evaluate() - self.right.evaluate()
    }
}

impl Evaluable for MultiplyNode {
    fn evaluate(&self) -> f64 {
        self.left.evaluate() * self.right.evaluate()
    }
}

impl Evaluable for DivideNode {
    fn evaluate(&self) -> f64 {
        self.left.evaluate() / self.right.evaluate()
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

    fn get_expression_tree(&mut self) -> Result<&Box<dyn Evaluable>, ParseError> {
        let mut output_queue: VecDeque<Box<dyn Evaluable>> = VecDeque::new();
        let mut op_stack: VecDeque<Token> = VecDeque::new();
        loop {
            let token = self.lexer.next_token();
            match token {
                Token::EOF => break,
                Token::SPACE => continue,
                Token::NUMBER(value) => {
                    output_queue.push_front(Box::new(NumberNode { value }));
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
                                    output_queue.push_front(Box::new(AddNode { left, right }))
                                }
                                _ => return Err(ParseError::InvalidTokenError),
                            }
                        }
                    }
                }
                Token::UNKNOWN => return Err(ParseError::InvalidTokenError),
            }
        }
    }
}
