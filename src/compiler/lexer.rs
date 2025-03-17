use std::collections::HashSet;

#[derive(Debug, PartialEq)]
pub enum TokenType {
    Gxfn, Var, Println, String, Bool, Colon, 
    DoubleLessThan, Identifier, If, Else, Return, 
    Equal, OpenParen, CloseParen, Comma, End, 
    Indicator, Plus, Minus, Multiply, Divide,
    Number, Ampersand, Commend,
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
}

pub struct Lexer<'a> {
    source: &'a str,
    chars: std::iter::Peekable<std::str::Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Lexer {
            source,
            chars: source.chars().peekable(),
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::with_capacity(self.source.len() / 2);
        let keywords: HashSet<&str> = ["gxfn", "if", "var", "println", "return", "else", "true", "false"].iter().cloned().collect();

        while let Some(&c) = self.chars.peek() {
            match c {
                'a'..='z' | 'A'..='Z' => {
                    let ident = self.consume_while(|ch| ch.is_alphanumeric());
                    let token_type = match keywords.get(ident.as_str()) {
                        Some(&"gxfn") => TokenType::Gxfn,
                        Some(&"if") => TokenType::If,
                        Some(&"var") => TokenType::Var,
                        Some(&"println") => TokenType::Println,
                        Some(&"return") => TokenType::Return,
                        Some(&"else") => TokenType::Else,
                        Some(&"true") | Some(&"false") => TokenType::Bool,
                        _ => TokenType::Identifier,
                    };
                    tokens.push(Token { token_type, value: ident });
                }
                '0'..='9' => {
                    let num = self.consume_while(|ch| ch.is_numeric());
                    tokens.push(Token { token_type: TokenType::Number, value: num });
                }
                '=' => self.add_token(&mut tokens, TokenType::Equal),
                '(' => self.add_token(&mut tokens, TokenType::OpenParen),
                ')' => self.add_token(&mut tokens, TokenType::CloseParen),
                ';' => self.add_token(&mut tokens, TokenType::End),
                ':' => self.add_token(&mut tokens, TokenType::Indicator),
                '&' => self.add_token(&mut tokens, TokenType::Ampersand),
                '+' => self.add_token(&mut tokens, TokenType::Plus),
                '#' => self.add_token(&mut tokens, TokenType::Commend),
                '-' => self.add_token(&mut tokens, TokenType::Minus),
                '*' => self.add_token(&mut tokens, TokenType::Multiply),
                ',' => self.add_token(&mut tokens, TokenType::Comma),
                '/' => self.add_token(&mut tokens, TokenType::Divide),
                '"' => {
                    self.chars.next();
                    let string_val = self.consume_while(|ch| ch != '"');
                    self.chars.next();
                    tokens.push(Token { token_type: TokenType::String, value: string_val });
                }
                _ => { self.chars.next(); }
            }
        }
        tokens
    }

    fn consume_while<F>(&mut self, condition: F) -> String
    where F: Fn(char) -> bool {
        let mut result = String::new();
        while let Some(&ch) = self.chars.peek() {
            if !condition(ch) { break; }
            result.push(self.chars.next().unwrap());
        }
        result
    }

    fn add_token(&mut self, tokens: &mut Vec<Token>, token_type: TokenType) {
        tokens.push(Token { token_type, value: self.chars.next().unwrap().to_string() });
    }
}
