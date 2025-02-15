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

#[cfg(test)]
mod node_tests {
    use super::*;

    #[test]
    fn new_num_node() {
        let num_tok = Node::Number(0.0);
        let _actual = Node::new_num(Token::NUMBER(0.0)).unwrap();

        matches!(num_tok, _actual);
    }

    #[test]
    fn new_add_node() {
        let add_tok = Node::Add(Box::new(Node::Number(0.0)), Box::new(Node::Number(0.0)));
        let _actual = Node::new_operator(&Token::PLUS, Box::new(Node::Number(0.0)), Box::new(Node::Number(0.0))).unwrap();

        matches!(add_tok, _actual);
    }

    #[test]
    fn new_sub_node() {
        let sub_tok = Node::Subtract(Box::new(Node::Number(0.0)), Box::new(Node::Number(0.0)));
        let _actual = Node::new_operator(&Token::MINUS, Box::new(Node::Number(0.0)), Box::new(Node::Number(0.0))).unwrap();
        matches!(sub_tok, _actual);
    }

    #[test]
    fn new_mult_node() {
        let tok = Node::Multiply(Box::new(Node::Number(0.0)), Box::new(Node::Number(0.0)));
        let _actual = Node::new_operator(&Token::TIMES, Box::new(Node::Number(0.0)), Box::new(Node::Number(0.0))).unwrap();

        matches!(tok, _actual);
    }

    #[test]
    fn new_div_node() {
        let tok = Node::Divide(Box::new(Node::Number(0.0)), Box::new(Node::Number(0.0)));
        let _actual = Node::new_operator(&Token::DIVIDE, Box::new(Node::Number(0.0)), Box::new(Node::Number(0.0))).unwrap();
        matches!(tok, _actual);
    }

    #[test]
    fn eval_num() {
        if let Some(node) = Node::new_num(Token::NUMBER(727.0)) {
            assert_eq!(727.0, node.evaluate());
        }
    }
}
