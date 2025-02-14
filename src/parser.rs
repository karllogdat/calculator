use lexer::Lexer;

trait Evaluable {
    fn evaluate(&self) -> f46;
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

pub struct Parser {
    lexer: Lexer,
}
