#[derive(Debug, PartialEq)]
pub enum TokenType {
    Println,
    Operator,
    Number,
    Plus,
    String,
    Placeholder,
    End,
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
}

pub struct Lexer {
    source: String,
    pos: usize,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Self { source, pos: 0 }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        let chars: Vec<char> = self.source.chars().collect();
        
        while self.pos < chars.len() {
            if self.source[self.pos..].starts_with("println") {
                tokens.push(Token { token_type: TokenType::Println, value: "println".to_string() });
                self.pos += 7;
            } else if self.source[self.pos..].starts_with("<<") {
                tokens.push(Token { token_type: TokenType::Operator, value: "<<".to_string() });
                self.pos += 2;
            } else if self.source[self.pos..].starts_with("{}") {
                tokens.push(Token { token_type: TokenType::Placeholder, value: "{}".to_string() });
                self.pos += 2;
            } else if chars[self.pos].is_digit(10) {
                let start = self.pos;
                while self.pos < chars.len() && chars[self.pos].is_digit(10) {
                    self.pos += 1;
                }
                tokens.push(Token { token_type: TokenType::Number, value: self.source[start..self.pos].to_string() });
            } else if chars[self.pos] == '+' {
                tokens.push(Token { token_type: TokenType::Plus, value: "+".to_string() });
                self.pos += 1;
            } else if chars[self.pos] == '"' {
                let start = self.pos + 1;
                self.pos += 1;
                while self.pos < chars.len() && chars[self.pos] != '"' {
                    self.pos += 1;
                }
                tokens.push(Token { token_type: TokenType::String, value: self.source[start..self.pos].to_string() });
                self.pos += 1;
            } else {
                self.pos += 1;
            }
        }
        
        tokens.push(Token { token_type: TokenType::End, value: "".to_string() });
        tokens
    }
}
