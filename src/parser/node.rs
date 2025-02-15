use super::super::lexer::Token;

#[derive(Debug, Clone)]
pub enum Node {
    Number(f64),
    Add(Box<Node>, Box<Node>),
    Subtract(Box<Node>, Box<Node>),
    Multiply(Box<Node>, Box<Node>),
    Divide(Box<Node>, Box<Node>),
}

impl Node {
    pub fn new_num(token: Token) -> Option<Box<Self>> {
        match token {
            Token::NUMBER(n) => Some(Box::new(Node::Number(n))),
            _ => None,
        }
    }

    pub fn new_operator(token: &Token, left: Box<Node>, right: Box<Node>) -> Option<Box<Self>> {
        match token {
            Token::PLUS => Some(Box::new(Node::Add(left, right))),
            Token::MINUS => Some(Box::new(Node::Subtract(left, right))),
            Token::TIMES => Some(Box::new(Node::Multiply(left, right))),
            Token::DIVIDE => Some(Box::new(Node::Divide(left, right))),
            _ => None,
        }
    }

    pub fn evaluate(&self) -> f64 {
        match self {
            Node::Number(n) => *n,
            Node::Add(l, r) => l.evaluate() + r.evaluate(),
            Node::Subtract(l, r) => l.evaluate() - r.evaluate(),
            Node::Multiply(l, r) => l.evaluate() * r.evaluate(),
            Node::Divide(l, r) => l.evaluate() / r.evaluate(),
        }
    }
}
