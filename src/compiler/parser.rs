use crate::compiler::lexer::{Token, TokenType};
use rayon::prelude::*;

#[derive(Debug)]
pub enum ASTNode {
    Println(Box<[ASTNode]>),
    Var(String, String, Box<ASTNode>),
    Function(String, String, Vec<(String, String)>, Box<ASTNode>),
    FunctionCall(String, Box<[ASTNode]>),
    If(Box<ASTNode>, Box<ASTNode>, Option<Box<ASTNode>>),
    Return(Box<ASTNode>),
    StringLiteral(String),
    Number(i32),
    Identifier(String),
    Block(Box<[ASTNode]>),
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
        let statements: Vec<ASTNode> = (0..self.tokens.len())
            .into_par_iter()
            .filter_map(|_| self.parse_statement())
            .collect();
        
        Some(ASTNode::Block(statements.into_boxed_slice()))
    }

    fn parse_statement(&mut self) -> Option<ASTNode> {
        if let Some(token) = self.tokens.get(self.pos) {
            match token.token_type {
                TokenType::Println => {
                    self.pos += 1;
                    let args = self.parse_arguments();
                    return Some(ASTNode::Println(args.into_boxed_slice()));
                }
                TokenType::Var => {
                    self.pos += 1;
                    if let Some(var_name) = self.parse_identifier() {
                        if self.match_token(TokenType::Equal) {
                            if let Some(value) = self.parse_expression() {
                                return Some(ASTNode::Var(var_name, String::new(), Box::new(value)));
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        None
    }

    fn parse_arguments(&mut self) -> Vec<ASTNode> {
        let mut args = Vec::new();
        while let Some(arg) = self.parse_expression() {
            args.push(arg);
            if !self.match_token(TokenType::DoubleLessThan) {
                break;
            }
        }
        args
    }

    fn parse_expression(&mut self) -> Option<ASTNode> {
        if let Some(token) = self.tokens.get(self.pos) {
            self.pos += 1;
            return match &token.token_type {
                TokenType::String => Some(ASTNode::StringLiteral(token.value.clone())),
                TokenType::Identifier => Some(ASTNode::Identifier(token.value.clone())),
                TokenType::Number => Some(ASTNode::Number(token.value.parse().unwrap())),
                _ => None,
            };
        }
        None
    }

    fn parse_identifier(&mut self) -> Option<String> {
        if let Some(token) = self.tokens.get(self.pos) {
            if token.token_type == TokenType::Identifier {
                self.pos += 1;
                return Some(token.value.clone());
            }
        }
        None
    }

    fn match_token(&mut self, expected: TokenType) -> bool {
        if let Some(token) = self.tokens.get(self.pos) {
            if token.token_type == expected {
                self.pos += 1;
                return true;
            }
        }
        false
    }
}
