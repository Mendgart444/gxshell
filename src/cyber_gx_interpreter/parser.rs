use crate::cyber_gx_interpreter::lexer::{Token, TokenType};

#[derive(Debug)]
pub enum ASTNode {
    Main(Vec<ASTNode>),
    Println(String),
    Number(i32),
    Plus(Box<ASTNode>, Box<ASTNode>),
    Var(String, Box<ASTNode>),
    Function(String, Vec<(String, String)>, Box<ASTNode>),
    If(Box<ASTNode>, Box<ASTNode>, Option<Box<ASTNode>>),
    Return(Box<ASTNode>),
    Bool(bool),
    Identifier(String),
    // Weitere AST-Knoten hier hinzufügen
}

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    pub fn parse(&mut self) -> Option<ASTNode> {
        self.parse_main()
    }

    fn parse_main(&mut self) -> Option<ASTNode> {
        if self.match_token(TokenType::Main) {
            let mut body = Vec::new();
            while !self.match_token(TokenType::End) {
                let node = self.parse_statement();
                if let Some(node) = node {
                    body.push(node);
                } else {
                    return None;
                }
            }
            Some(ASTNode::Main(body))
        } else {
            None
        }
    }

    fn parse_statement(&mut self) -> Option<ASTNode> {
        if self.match_token(TokenType::Println) {
            if let Some(value) = self.next_token().and_then(|t| {
                if let TokenType::String = t.token_type {
                    Some(t.value.clone())
                } else {
                    None
                }
            }) {
                if self.match_token(TokenType::DoubleLessThan) {
                    if let Some(identifier) = self.next_token().and_then(|t| {
                        if let TokenType::Identifier = t.token_type {
                            Some(t.value.clone())
                        } else {
                            None
                        }
                    }) {
                        if self.match_token(TokenType::End) {
                            return Some(ASTNode::Println(format!("{} {}", value, identifier)));
                        }
                    }
                }
            }
        } else if self.match_token(TokenType::Var) {
            if let Some(var_name) = self.next_token().and_then(|t| {
                if let TokenType::Identifier = t.token_type {
                    Some(t.value.clone())
                } else {
                    None
                }
            }) {
                if self.match_token(TokenType::Colon) {
                    if let Some(var_type) = self.next_token().and_then(|t| {
                        if let TokenType::Identifier = t.token_type {
                            Some(t.value.clone())
                        } else {
                            None
                        }
                    }) {
                        if self.match_token(TokenType::DoubleLessThan) {
                            let expr = self.parse_expression();
                            if let Some(expr) = expr {
                                if self.match_token(TokenType::End) {
                                    return Some(ASTNode::Var(var_name.to_string(), Box::new(expr)));
                                }
                            }
                        }
                    }
                }
            }
        } else if self.match_token(TokenType::Gxfn) {
            if let Some(fn_name) = self.next_token().and_then(|t| {
                if let TokenType::Identifier = t.token_type {
                    Some(t.value.clone())
                } else {
                    None
                }
            }) {
                let mut params = Vec::new();
                if self.match_token(TokenType::Colon) {
                    if let Some(return_type) = self.next_token().and_then(|t| {
                        if let TokenType::Identifier = t.token_type {
                            Some(t.value.clone())
                        } else {
                            None
                        }
                    }) {
                        while !self.match_token(TokenType::End) {
                            let param_name_token = self.next_token();
                            if let Some(param_name) = self.next_token().and_then(|t| {
                                if let TokenType::Identifier = t.token_type {
                                    Some(t.value.clone())
                                } else {
                                    None
                                }
                            }) {
                                if self.match_token(TokenType::Colon) {
                                    let param_type_token = self.next_token();
                                    if let Some(param_type) = self.next_token().and_then(|t| {
                                        if let TokenType::Identifier = t.token_type {
                                            Some(t.value.clone())
                                        } else {
                                            None
                                        }
                                    }) {
                                        params.push((param_name.to_string(), param_type.to_string()));
                                    }
                                }
                            }
                        }
                        let body = self.parse_statement();
                        if let Some(body) = body {
                            return Some(ASTNode::Function(fn_name.to_string(), params, Box::new(body)));
                        }
                    }
                }
            }
        } else if self.match_token(TokenType::If) {
            let condition = self.parse_expression();
            if let Some(condition) = condition {
                let body = self.parse_statement();
                if let Some(body) = body {
                    let else_body = if self.match_token(TokenType::Else) {
                        self.parse_statement()
                    } else {
                        None
                    };
                    return Some(ASTNode::If(Box::new(condition), Box::new(body), else_body.map(Box::new)));
                }
            }
        } else if self.match_token(TokenType::Return) {
            let expression = self.parse_expression();
            if let Some(expression) = expression {
                if self.match_token(TokenType::End) {
                    return Some(ASTNode::Return(Box::new(expression)));
                }
            }
        }
        None
    }

    fn parse_expression(&mut self) -> Option<ASTNode> {
        // Implementieren Sie die Logik zum Parsen von Ausdrücken
        None
    }

    fn match_token(&mut self, expected: TokenType) -> bool {
        if self.pos < self.tokens.len() && self.tokens[self.pos].token_type == expected {
            self.pos += 1;
            true
        } else {
            false
        }
    }

    fn next_token(&mut self) -> Option<&Token> {
        if self.pos < self.tokens.len() {
            let token = &self.tokens[self.pos];
            self.pos += 1;
            Some(token)
        } else {
            None
        }
    }
}