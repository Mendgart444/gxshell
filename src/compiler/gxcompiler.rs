use crate::compiler::lexer::Lexer;
use crate::compiler::parser::{ASTNode, Parser};
use std::fs;
use std::io::Write;
use std::process::Command;
use nu_ansi_term::Color::{Red, Blue};

pub struct Compiler;

impl Compiler {
    pub fn compile_to_rust(source_code: &str, output_filename: &str) {
        println!("{}", Blue.paint("[INFO] Starting process..."));
        println!("{}", Blue.paint("[INFO] Starting lexer..."));
        let mut lexer = Lexer::new(source_code);
        let tokens = lexer.tokenize();
        
        println!("{}", Blue.paint("[INFO] Starting Parser..."));

        let mut parser = Parser::new(tokens);
        if let Some(ast) = parser.parse() {
            println!("{}", Blue.paint("[INFO] Parsing the code..."));
            let rust_code = Compiler::generate_rust_code(&ast);

            let rust_file = format!("{}.rs", output_filename);
            if let Err(e) = fs::write(&rust_file, &rust_code) {
                eprintln!("{}", Red.paint(format!("Error: Failed to Write Rust File: {}", e)));
                return;
            }
            let error1: String = Red.paint("Error: could not compiler rust file").to_string();

            // 🚀 Rust-Code kompilieren
            println!("{}", Blue.paint("[INFO] compiling Rust code..."));
            let output = Command::new("rustc")
                .args([&rust_file, "-o", output_filename])
                .output()
                .expect(&error1);

            if !output.status.success() {
                eprintln!("{}", Red.paint(format!("rustc error: {}", output.status)));
                eprintln!("{}", Red.paint(format!("stdout: {}", String::from_utf8_lossy(&output.stdout))));
                eprintln!("{}", Red.paint(format!("stderr: {}", String::from_utf8_lossy(&output.stderr))));
                return;
            }

            // Temporäre Rust-Datei entfernen
            if let Err(e) = fs::remove_file(&rust_file) {
                eprintln!("{}", Red.paint(format!("Error: failed to delete temp file: {}", e)));
            }
        } else {
            eprintln!("{}", Red.paint("Error: Could not Parsing the code."));
        }
    }

    fn generate_rust_code(ast: &ASTNode) -> String {
        match ast {
            ASTNode::Block(body) => {
                let mut code: String = String::new();
                for node in body {
                    code.push_str(&Compiler::generate_rust_code(node));
                }
                code
            }
            ASTNode::Println(args) => {
                let formatted_string: String = args
                    .iter()
                    .map(|arg| Compiler::generate_rust_code(arg))
                    .collect::<Vec<String>>()
                    .join(", ");
                format!("    println!(\"{{}}\", {});\n", formatted_string)
            }
            ASTNode::Var(name, var_type, value) => {
                format!("    let {}:{} = {};\n", name, var_type, Compiler::generate_rust_code(value))
            }
            ASTNode::Function(name, return_type, params, body) => {
                let params_str = params
                    .iter()
                    .map(|(name, typ)| format!("{}: {}", name, typ))
                    .collect::<Vec<String>>()
                    .join(", ");
                
                let typ = if return_type == "void" { 
                    " ".to_string() 
                } else { 
                    format!(" -> {}", return_type) 
                };
            
                // Wenn `main`, dann kein `-> bool`
                if name == "main" {
                    format!(
                        "fn {}({}) {{\n{}\n}}\n",
                        name,
                        params_str,
                        Compiler::generate_rust_code(body)
                    )
                } else {
                    format!(
                        "fn {}({}) {} {{\n{}\n}}\n",
                        name,
                        params_str,
                        typ,
                        Compiler::generate_rust_code(body)
                    )
                }
            }
            ASTNode::FunctionCall(name, args) => {
                let args_str = args
                    .iter()
                    .map(|arg| Compiler::generate_rust_code(arg))
                    .collect::<Vec<String>>()
                    .join(", ");
                format!("    {}({});\n", name, args_str)
            }
            ASTNode::If(condition, body, else_body) => {
                let mut code = format!(
                    "if {} {{\n{}\n}}",
                    Compiler::generate_rust_code(condition),
                    Compiler::generate_rust_code(body)
                );
                if let Some(else_body) = else_body {
                    code.push_str(&format!(" else {{\n{}\n}}", Compiler::generate_rust_code(else_body)));
                }
                code
            }
            ASTNode::Return(expression) => format!("    return {};\n", Compiler::generate_rust_code(expression)),
            ASTNode::StringLiteral(value) => format!("\"{}\"", value),
            ASTNode::Commend => format!("{}", " "),
            ASTNode::Identifier(name) => name.clone(),
            ASTNode::Import(path) => {
                let module_path = path.replace("::", "/");
                let rust_file = format!("std/{}.rs", module_path);
                let rust_mod_file = format!("std/{}/mod.rs", module_path);

                if fs::metadata(&rust_file).is_ok() {
                    format!("use {};", path)
                } else if fs::metadata(&rust_mod_file).is_ok() {
                    format!("use {};", path)
                } else {
                    eprintln!("Warning: Module '{}' not found in std/", path);
                    String::new()
                }
            }

            _ => String::new(),
            
        }
    }

    pub fn crate_new_project(args: &str) {
        if args.trim().is_empty() {
            eprintln!("Error: Please provide a project name");
            return;
        }  
        let project_name = args.to_string();

        if let Err(e) = fs::create_dir(&project_name) {
            eprint!("{}", Red.paint(format!("Error: Failed to create project directory: {}", e)));
            return;
        }

        let main_file_path = format!("{}/src/main.gx", project_name);
        match fs::File::create(&main_file_path) {
            Ok(mut file) => {
                if let Err(e) = writeln!(file, "gxfn main() {{\n    println << \"Hello, World!\";\n}}") {
                    eprint!("{}", Red.paint(format!("Error: Failed to write main.gx file: {}", e)));
                }
            }
            Err(e) => {
                eprint!("{}", Red.paint(format!("Error: Failed to create main.gx file: {}", e)));
            }
        }
        // adding settings.gxconfig file
        let settings_file_path = format!("{}/settings.gxconfig", project_name);
        match fs::File::create(&settings_file_path) {
            Ok(mut file) => {
                if let Err(e) = writeln!(file, "name = {}\nversion = \"0.0.1\"\n\ndependencies \n{{ \n\n }}", project_name) {
                    eprint!("{}", Red.paint(format!("Error: Failed to write settings.gxconfig file: {}", e)));
                }
            }
            Err(e) => {
                eprint!("{}", Red.paint(format!("Error: Failed to create settings.gxconfig file: {}", e)));
            }
        }

    }
}


