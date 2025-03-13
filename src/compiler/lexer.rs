#[derive(Debug, PartialEq)]
pub enum TokenType {
    Gxfn, Var, Println, String, #[allow(dead_code)] Bool, Colon, 
    DoubleLessThan, Identifier, #[allow(dead_code)] If, #[allow(dead_code)] Else, #[allow(dead_code)] Return, 
    Equal, OpenParen, CloseParen, Comma, End, 
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
}

pub struct Lexer {
    source: String,
    position: usize,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Lexer { source, position: 0 }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while self.position < self.source.len() {
            let current_char = self.source.chars().nth(self.position).unwrap();

            match current_char {
                'g' if self.source[self.position..].starts_with("gxfn") => {
                    tokens.push(Token { token_type: TokenType::Gxfn, value: "gxfn".to_string() });
                    self.position += 4;
                }
                'v' if self.source[self.position..].starts_with("var") => {
                    tokens.push(Token { token_type: TokenType::Var, value: "var".to_string() });
                    self.position += 3;
                }
                'p' if self.source[self.position..].starts_with("println") => {
                    tokens.push(Token { token_type: TokenType::Println, value: "println".to_string() });
                    self.position += 7;
                }
                '=' => {
                    tokens.push(Token { token_type: TokenType::Equal, value: "=".to_string() });
                    self.position += 1;
                }
                '(' => {
                    tokens.push(Token { token_type: TokenType::OpenParen, value: "(".to_string() });
                    self.position += 1;
                }
                ')' => {
                    tokens.push(Token { token_type: TokenType::CloseParen, value: ")".to_string() });
                    self.position += 1;
                }
                ';' => {
                    tokens.push(Token { token_type: TokenType::End, value: ";".to_string() });
                    self.position += 1;
                }
                '"' => {
                    let start = self.position + 1;
                    if let Some(end) = self.source[start..].find('"') {
                        let end = end + start;
                        let value = self.source[start..end].to_string();
                        tokens.push(Token { token_type: TokenType::String, value });
                        self.position = end + 1;
                    }
                }
                _ if current_char.is_alphabetic() => {
                    let start = self.position;
                    while self.position < self.source.len() && self.source.chars().nth(self.position).unwrap().is_alphanumeric() {
                        self.position += 1;
                    }
                    let value = self.source[start..self.position].to_string();
                    tokens.push(Token { token_type: TokenType::Identifier, value });
                }
                _ => self.position += 1,
            }
        }
        tokens
    }
}
