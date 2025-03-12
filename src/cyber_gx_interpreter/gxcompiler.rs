use crate::cyber_gx_interpreter::lexer::Lexer;
use crate::cyber_gx_interpreter::parser::{ASTNode, Parser};
use std::fs;
use std::process::Command;

pub struct Compiler;

impl Compiler {
    pub fn compile_to_rust(source_code: &str, output_filename: &str) {
        let mut lexer = Lexer::new(source_code.to_string());
        let tokens = lexer.tokenize();

        let mut parser = Parser::new(tokens);
        if let Some(ast) = parser.parse() {
            let rust_code = Compiler::generate_rust_code(&ast);

            let rust_file = format!("{}.rs", output_filename);
            if let Err(e) = fs::write(&rust_file, &rust_code) {
                eprintln!("Failed to write Rust file: {}", e);
                return;
            }

            // Kompilieren mit rustc
            let output = Command::new("rustc")
                .args([&rust_file, "-o", output_filename])
                .output()
                .expect("Failed to compile Rust code");

            if !output.status.success() {
                eprintln!("rustc failed with status: {}", output.status);
                eprintln!("stdout: {}", String::from_utf8_lossy(&output.stdout));
                eprintln!("stderr: {}", String::from_utf8_lossy(&output.stderr));
                return;
            }

            if let Err(e) = fs::remove_file(&rust_file) {
                eprintln!("Failed to remove temporary Rust file: {}", e);
            }
        } else {
            eprintln!("Failed to parse source code.");
        }
    }

    fn generate_rust_code(ast: &ASTNode) -> String {
        match ast {
            ASTNode::Main(body) => {
                let mut code = String::from("fn main() {\n");
                for node in body {
                    code.push_str(&Compiler::generate_rust_code(node));
                }
                code.push_str("}\n");
                code
            }
            ASTNode::Println(value) => format!("    println!(\"{}\");\n", value),
            ASTNode::Var(name, value) => format!("    let {} = {};\n", name, Compiler::generate_rust_code(value)),
            ASTNode::Function(name, params, body) => {
                let params_str = params.iter()
                    .map(|(name, typ)| format!("{}: {}", name, typ))
                    .collect::<Vec<String>>()
                    .join(", ");
                format!("fn {}({}) -> bool {{\n{}\n}}\n", name, params_str, Compiler::generate_rust_code(body))
            }
            ASTNode::If(condition, body, else_body) => {
                let mut code = format!("if {} {{\n{}\n}}", Compiler::generate_rust_code(condition), Compiler::generate_rust_code(body));
                if let Some(else_body) = else_body {
                    code.push_str(&format!(" else {{\n{}\n}}", Compiler::generate_rust_code(else_body)));
                }
                code
            }
            ASTNode::Return(expression) => format!("return {};\n", Compiler::generate_rust_code(expression)),
            ASTNode::Bool(value) => format!("{}", value),
            ASTNode::Identifier(name) => name.clone(),
            _ => String::new(),
        }
    }
}