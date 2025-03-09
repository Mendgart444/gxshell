#[allow(unused_imports)]
use crate::cyber_gx_interpreter::lexer::{Token, TokenType, Lexer};
use std::fs;
use std::process::Command;

pub struct Compiler;

impl Compiler {
    pub fn compile_to_rust(source_code: &str, output_filename: &str) {
        let mut rust_code:String = String::from("fn main() {\n");

        let mut lexer:Lexer = Lexer::new(source_code.to_string());
        let tokens:Vec<Token> = lexer.tokenize();

        for token in tokens {
            match token.token_type {
                TokenType::Println => {
                    rust_code.push_str("    println!(\"");
                }
                TokenType::String => {
                    rust_code.push_str(&token.value);
                }
                TokenType::Operator if token.value == "<<" => {
                    rust_code.push_str("{}");
                }
                TokenType::Number => {
                    rust_code.push_str(&token.value);
                }
                TokenType::Plus => {
                    rust_code.push_str(" + ");
                }
                TokenType::Placeholder => {
                    rust_code.push_str("{}");
                }
                _ => {}
            }
        }

        rust_code.push_str("}\n");

        let rust_file = format!("{}.rs", output_filename);
        fs::write(&rust_file, &rust_code).expect("Failed to write Rust file");

        // Kompilieren mit rustc
        let _ = Command::new("rustc")
            .args([&rust_file, "-o", output_filename])
            .output()
            .expect("Failed to compile Rust code");

        fs::remove_file(rust_file).ok(); // Temporäre Datei löschen
    }
}
