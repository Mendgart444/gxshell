use crate::compiler::lexer::{Token, TokenType};

#[derive(Debug)]
pub enum ASTNode {
    Println(Vec<ASTNode>),
    Var(String, String, Box<ASTNode>),
    Function(String, String, Vec<(String, String)>, Box<ASTNode>),
    FunctionCall(String, Vec<ASTNode>),
    If(Box<ASTNode>, Box<ASTNode>, Option<Box<ASTNode>>),
    Return(Box<ASTNode>),
    StringLiteral(String),
    #[allow(dead_code)]
    Number(i32),
    Identifier(String),
    Block(Vec<ASTNode>),
    #[allow(dead_code)]
    Commend,
    Import(ImportNode),
}

#[derive(Debug)]
pub struct ImportNode {
    pub module_path: String,
    pub is_standard_lib: bool,
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
        self.parse_statement()
    }

    fn parse_statement(&mut self) -> Option<ASTNode> {
        if let Some(token) = self.tokens.get(self.pos) {
            match token.token_type {
                TokenType::Gxfn => {
                    self.pos += 1;
                    if let Some(fn_name) = self.parse_identifier() {
                        let mut params: Vec<(String, String)> = Vec::new();
                        let mut return_type: String = String::new();
                        if self.match_token(TokenType::Indicator) {
                            if let Some(type_name) = self.parse_identifier() {
                                return_type.push_str(&type_name);
                            }
                        }
                        if self.match_token(TokenType::OpenParen) {
                            while let Some(param_name) = self.parse_identifier() {
                                if self.match_token(TokenType::Colon) {
                                    if let Some(param_type) = self.parse_identifier() {
                                        params.push((param_name, param_type));
                                    }
                                }
                                if !self.match_token(TokenType::Comma) {
                                    break;
                                }
                            }
                            self.match_token(TokenType::CloseParen);
                        }
                        let body:ASTNode = self.parse_block();
                        return Some(ASTNode::Function(fn_name, return_type, params, Box::new(body)));
                    }
                }
                TokenType::Import => {
                    self.pos += 1;
                    let mut path = String::new();
                    let mut is_std = false;

                    if self.match_token(TokenType::Identifier) {
                        path.push_str(&self.tokens[self.pos - 1].value);

                        if path == "std" {
                            is_std = true;
                        }

                        if self.match_token(TokenType::Colon) && self.match_token(TokenType::Colon) {
                            if self.match_token(TokenType::Identifier) {
                                path.push_str("::");
                                path.push_str(&self.tokens[self.pos - 1].value);
                                return Some(ASTNode::Import(ImportNode {
                                    module_path: path,
                                    is_standard_lib: is_std,
                                }));
                            }
                        }
                    }

                    eprintln!("Error: Invalid import syntax");
                    return None;
                }
                TokenType::Println => {
                    self.pos += 1;
                    let mut args = Vec::new();
                    while let Some(arg) = self.parse_expression() {
                            args.push(arg);
                            
                            if !self.match_token(TokenType::DoubleLessThan) {
                                break;
                            }
                    }       
            return Some(ASTNode::Println(args));
                }
                TokenType::Var => {
                    self.pos += 1;
                    if let Some(var_name) = self.parse_identifier() {
                        let mut var_type:String = String::new();
                        let is_reference:bool = if self.match_token(TokenType::Ampersand) {
                            true
                        } else {
                            false
                        };
                        
                        if self.match_token(TokenType::Indicator) {
                            if let Some(type_name) = self.parse_identifier() {
                                if is_reference {
                                    let ref_type: String = format!("&{}", type_name);
                                    var_type.push_str(&ref_type);
                                } else {
                                    var_type.push_str(&type_name);
                                }
                            }
                        }

                        if self.match_token(TokenType::Equal) {
                            if let Some(value) = self.parse_expression() {
                                return Some(ASTNode::Var(var_name, var_type, Box::new(value)));
                            }
                        }
                    }
                }
                TokenType::If => {
                    self.pos += 1;
                    if let Some(condition) = self.parse_expression() {
                        let then_branch: ASTNode = self.parse_block();
                        let else_branch: Option<Box<ASTNode>> = if self.match_token(TokenType::Else) {
                            Some(Box::new(self.parse_block()))
                        } else {
                            None
                        };
                        return  Some(ASTNode::If(Box::new(condition), Box::new(then_branch), else_branch));
                    }
                }
                TokenType::Return => {
                    self.pos += 1;
                    if let Some(value) = self.parse_expression() {
                        return Some(ASTNode::Return(Box::new(value)));
                    }
                }
                TokenType::Identifier => {
                    self.pos += 1;
                    let func_name = self.tokens[self.pos - 1].value.clone();
                    if self.match_token(TokenType::OpenParen) {
                        let mut args: Vec<ASTNode> = Vec::new();
                        while let Some(arg) = self.parse_expression() {
                            args.push(arg);
                            if !self.match_token(TokenType::Comma) {
                                break;
                            }
                        }
                        if self.match_token(TokenType::CloseParen) {
                            return Some(ASTNode::FunctionCall(func_name, args));
                        }
        
                        return Some(ASTNode::Println(args));
                    }
                }
                _ => {}
            }
        }
        None
    }

    #[allow(dead_code)]

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

    fn parse_block(&mut self) -> ASTNode {
        let mut statements: Vec<ASTNode> = Vec::new();
        
        while !self.match_token(TokenType::End) {
            if let Some(statement) = self.parse_statement() {
                statements.push(statement);
            }
        }
        ASTNode::Block(statements)
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
