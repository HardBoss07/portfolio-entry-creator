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

    fn next_token(&mut self) -> Option<Token> {
        let chars: Vec<char> = self.text.chars().collect();
        
        while self.position < chars.len() {
            let current = chars[self.position];
            
            if "#-`".contains(current) {
                return Some(Token::Symbol(current));
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