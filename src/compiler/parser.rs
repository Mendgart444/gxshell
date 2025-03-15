use crate::compiler::lexer::{Token, TokenType};

#[derive(Debug)]
pub enum ASTNode {
    Main(Vec<ASTNode>),
    Println(Vec<ASTNode>), // println kann mehrere Argumente haben
    Var(String, Box<ASTNode>),
    Function(String, Vec<(String, String)>, Box<ASTNode>),
    FunctionCall(String, Vec<ASTNode>),
    #[allow(dead_code)] 
    If(Box<ASTNode>, Box<ASTNode>, Option<Box<ASTNode>>),
    #[allow(dead_code)]
    Bool(bool),
    #[allow(dead_code)]
    Return(Box<ASTNode>),
    StringLiteral(String),
    Identifier(String),
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
        // function
        if self.match_token(TokenType::Gxfn) {
            if let Some(fn_name) = self.parse_identifier() {
                let mut params = Vec::new();
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
                let body = self.parse_block();
                return Some(ASTNode::Function(fn_name, params, Box::new(body)));
            }
        // Println
        } else if self.match_token(TokenType::Println) {
            let mut args = Vec::new();
            while let Some(arg) = self.parse_expression() {
                args.push(arg);
                if !self.match_token(TokenType::DoubleLessThan) {
                    break;
                }
            }
            return Some(ASTNode::Println(args));
        // Variable
        } else if self.match_token(TokenType::Var) {
            if let Some(var_name) = self.parse_identifier() {
                if self.match_token(TokenType::Equal) {
                    if let Some(value) = self.parse_expression() {
                        return Some(ASTNode::Var(var_name, Box::new(value)));
                    }
                }
            }
        // If
        } else if self.match_token(TokenType::If) {
            if let Some(condition) = self.parse_expression() {
                let then_branch: ASTNode = self.parse_block();
                let else_branch: Option<Box<ASTNode>> = if self.match_token(TokenType::Else) {
                    Some(Box::new(self.parse_block()))
                } else {
                    None
                };
                return Some(ASTNode::If(Box::new(condition), Box::new(then_branch), else_branch));
            }
        // Return
        } else if self.match_token(TokenType::Return) {
            if let Some(value) = self.parse_expression() {
                return Some(ASTNode::Return(Box::new(value)));
            }
        // Identifier
        } else if self.match_token(TokenType::Identifier) {
            let func_name = self.tokens[self.pos - 1].value.clone();
            if self.match_token(TokenType::OpenParen) {
                let mut args = Vec::new();
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
        None
    }

    fn parse_expression(&mut self) -> Option<ASTNode> {
        if self.match_token(TokenType::String) {
            Some(ASTNode::StringLiteral(self.tokens[self.pos - 1].value.clone()))
        } else if self.match_token(TokenType::Identifier) {
            Some(ASTNode::Identifier(self.tokens[self.pos - 1].value.clone()))
        } else if self.match_token(TokenType::Bool) {
            let value: bool = self.tokens[self.pos - 1].value == "true";
            Some(ASTNode::Bool(value))
        } else {
            None
        }
    }

    fn match_token(&mut self, expected: TokenType) -> bool {
        if self.pos < self.tokens.len() && self.tokens[self.pos].token_type == expected {
            self.pos += 1;
            true
        } else {
            false
        }
    }

    fn parse_block(&mut self) -> ASTNode {
        let mut statements = Vec::new();
        while !self.match_token(TokenType::End) {
            if let Some(statement) = self.parse_statement() {
                statements.push(statement);
            }
        }
        ASTNode::Main(statements)
    }
    
    
    fn parse_identifier(&mut self) -> Option<String> {
        if self.match_token(TokenType::Identifier) {
            Some(self.tokens[self.pos - 1].value.clone())
        } else {
            None
        }
    }

}
