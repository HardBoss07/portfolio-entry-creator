use crate::enums::Token;

pub struct Lexer {
    text: String,
    position: usize,
}

impl Lexer {
    pub fn new(text: String) -> Self {
        Self {
            text,
            position: 0,
        }
    }

    fn inc(&mut self) {
        self.position += 1;
    }

    fn next_token(&mut self) -> Option<Token> {
        let chars: Vec<char> = self.text.chars().collect();
        
        while self.position < chars.len() {
            let current = chars[self.position];
            
            if current.is_whitespace() {
                self.inc();
                continue;
            }

            if "#-`()/".contains(current) {
                self.inc();
                return Some(Token::Symbol(current));
            }

            if current.is_alphanumeric() {
                let start = self.position;
                while self.position < chars.len() && chars[self.position] != '\n' {
                    self.inc();
                }

                return Some(Token::Text((&self.text[start..self.position]).to_string()))
            }
        }

        None
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while let Some(token) = self.next_token() {
            tokens.push(token);
        }

        tokens
    }
}