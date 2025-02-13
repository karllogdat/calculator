#[derive(Debug)]
pub enum Token {
    PLUS,
    MINUS,
    TIMES,
    DIVIDE,
    NUMBER(f64),
    SPACE,
    EOF,
    UNKNOWN,
}

// Thanks chatgpt for making this shit unreadable
// (i dont understand rust lifetimes)
pub struct Lexer<'a> {
    input: std::str::Chars<'a>,
    current: Option<char>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut chars = input.chars();
        let current = chars.next();
        Self {
            input: chars,
            current,
        }
    }

    fn advance(&mut self) -> Option<char> {
        self.current = self.input.next();
        self.current
    }

    pub fn next_token(&mut self) -> Token {
        while let Some(c) = self.current {
            match c {
                ' ' | '\t' | '\n' => {
                    self.advance();
                    return Token::SPACE;
                }
                '0'..='9' | '.' => return self.next_number(),
                '+' => {
                    self.advance();
                    return Token::PLUS;
                }
                '-' => {
                    self.advance();
                    return Token::MINUS;
                }
                '*' => {
                    self.advance();
                    return Token::TIMES;
                }
                '/' => {
                    self.advance();
                    return Token::DIVIDE;
                }
                _ => {
                    self.advance();
                    return Token::UNKNOWN;
                }
            }
        }
        Token::EOF
    }

    fn next_number(&mut self) -> Token {
        let mut num_str = String::new();
        let mut has_dot = false;

        while let Some(c) = self.current {
            if c.is_numeric() {
                num_str.push(c);
            } else if c == '.' && !has_dot {
                num_str.push(c);
                has_dot = true;
            } else {
                break;
            }
            self.advance();
        }

        match num_str.parse::<f64>() {
            Ok(num) => Token::NUMBER(num),
            Err(_) => Token::UNKNOWN,
        }
    }
}
