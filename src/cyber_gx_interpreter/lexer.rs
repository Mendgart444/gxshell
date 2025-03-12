#[derive(Debug, PartialEq)]
pub enum TokenType {
    Gxfn,
    Var,
    Main,
    Println,
    String,
    Number,
    Plus,
    Placeholder,
    End,
    Colon,
    DoubleLessThan,
    Identifier,
    If,
    Else,
    Return,
    Bool,
    // Weitere Token-Typen hier hinzufügen
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
            if let Some(current_char) = self.source.chars().nth(self.position) {
                match current_char {
                    'g' if self.source[self.position..].starts_with("gxfn") => {
                        tokens.push(Token {
                            token_type: TokenType::Gxfn,
                            value: "gxfn".to_string(),
                        });
                        self.position += 4;
                    }
                    'v' if self.source[self.position..].starts_with("var") => {
                        tokens.push(Token {
                            token_type: TokenType::Var,
                            value: "var".to_string(),
                        });
                        self.position += 3;
                    }
                    'm' if self.source[self.position..].starts_with("main") => {
                        tokens.push(Token {
                            token_type: TokenType::Main,
                            value: "main".to_string(),
                        });
                        self.position += 4;
                    }
                    'p' if self.source[self.position..].starts_with("println") => {
                        tokens.push(Token {
                            token_type: TokenType::Println,
                            value: "println".to_string(),
                        });
                        self.position += 7;
                    }
                    ':' => {
                        tokens.push(Token {
                            token_type: TokenType::Colon,
                            value: ":".to_string(),
                        });
                        self.position += 1;
                    }
                    '<' if self.source[self.position..].starts_with("<<") => {
                        tokens.push(Token {
                            token_type: TokenType::DoubleLessThan,
                            value: "<<".to_string(),
                        });
                        self.position += 2;
                    }
                    '"' => {
                        let start = self.position + 1;
                        if let Some(end) = self.source[start..].find('"') {
                            let end = end + start;
                            let value = self.source[start..end].to_string();
                            tokens.push(Token {
                                token_type: TokenType::String,
                                value,
                            });
                            self.position = end + 1;
                        } else {
                            // Fehlerbehandlung für fehlendes schließendes Anführungszeichen
                            eprintln!("Lexer Error: Missing closing quote for string literal");
                            return tokens;
                        }
                    }
                    '0'..='9' => {
                        while self.position < self.source.len() {
                            if let Some(c) = self.source.chars().nth(self.position) {
                                if c.is_digit(10) {
                                    self.position += 1;
                                } else {
                                    break;
                                }
                            } else {
                                break;
                            }
                        }
                    }
                    '+' => {
                        tokens.push(Token {
                            token_type: TokenType::Plus,
                            value: "+".to_string(),
                        });
                        self.position += 1;
                    }
                    '{' => {
                        tokens.push(Token {
                            token_type: TokenType::Placeholder,
                            value: "{}".to_string(),
                        });
                        self.position += 2;
                    }
                    ';' => {
                        tokens.push(Token {
                            token_type: TokenType::End,
                            value: ";".to_string(),
                        });
                        self.position += 1;
                    }
                    ',' => {
                        tokens.push(Token {
                            token_type: TokenType::End,
                            value: ",".to_string(),
                        });
                        self.position += 1;
                    }
                    'i' if self.source[self.position..].starts_with("if") => {
                        tokens.push(Token {
                            token_type: TokenType::If,
                            value: "if".to_string(),
                        });
                        self.position += 2;
                    }
                    'e' if self.source[self.position..].starts_with("else") => {
                        tokens.push(Token {
                            token_type: TokenType::Else,
                            value: "else".to_string(),
                        });
                        self.position += 4;
                    }
                    'r' if self.source[self.position..].starts_with("return") => {
                        tokens.push(Token {
                            token_type: TokenType::Return,
                            value: "return".to_string(),
                        });
                        self.position += 6;
                    }
                    'b' if self.source[self.position..].starts_with("bool") => {
                        tokens.push(Token {
                            token_type: TokenType::Bool,
                            value: "bool".to_string(),
                        });
                        self.position += 4;
                    }
                    _ if current_char.is_alphabetic() => {
                        let start = self.position;
                        while self.position < self.source.len() && self.source.chars().nth(self.position).unwrap().is_alphanumeric() {
                            self.position += 1;
                        }
                        let value = self.source[start..self.position].to_string();
                        tokens.push(Token {
                            token_type: TokenType::Identifier,
                            value,
                        });
                    }
                    _ => {self.position += 1;}
                } 
            
            } else {
                break;
            }

            
        }

        tokens
    }
}