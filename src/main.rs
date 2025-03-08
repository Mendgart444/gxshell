mod gxcore;
mod env_var;
mod updater;
mod cyber_gx_interpreter;
mod dev;

use cyber_gx_interpreter::lexer::Lexer;
//use cyber_gx_interpreter::interpreter::Interpreter;
use std::process::{Command, Stdio};
use std::env;
use std::path::PathBuf;
use nu_ansi_term::Color::{Green, Red, Blue};
use rustyline::Editor;

fn main()  {
    let mut rl:Editor<(), rustyline::history::FileHistory> = Editor::<(), _>::new().expect("Faild to launch editor.");
    let _ = rl.load_history(".hystory");

    updater::check_and_update();
    
    
    println!("{}", Red.paint(format!("GXShell version {}", env_var::GXSHELL_VERSION)));
    loop {
        let current_dir:PathBuf = env::current_dir().unwrap_or(PathBuf::from("C:\\"));
        let prompt = format!("{}> ", Green.paint(current_dir.display().to_string()));
        
        match rl.readline(&prompt) {
            Ok(line) => {
                let command = line.trim();
                if command == "exit" {
                    break;
                } else if  command == "update" {
                    updater::check_and_update();
                }
                let _ = rl.add_history_entry(command);
                execute_command(command);
            }

            Err(_) => break,
        }
    }
  
    let _ = rl.save_history(".hystory");



}

fn execute_command(command:&str) {
    let parts:Vec<&str> = command.split_whitespace().collect();

    if parts.is_empty() {
        return;
    }

    match parts[0] {
        "cd" => change_directory(parts),
        "gxcompiler" => {
            if parts.len() < 3 {
                println!("Usage: compile <source.gx> <output>");
                return;
            }
            let source_code = std::fs::read_to_string(parts[1]).expect("Failed to read source file");
            gxcompiler::compile(&source_code, parts[2]);
        },
        "update" => println!(" "),
        "dev" => dev::dev_mode(parts),
        "dir" => list_directory(),
        "cls" => clear_screen(),
        "version" => println!("{}", Green.paint(env_var::GXSHELL_VERSION)),
        "gxcore" => run_gxcore(parts),
        "gx" => run_interpreter(parts),
        _ => run_external_command(parts),
    }

}

fn change_directory(args: Vec<&str>) {
    if args.len() < 2 {
        println!("{}", Red.paint("Usage: cd <path>"));
        return;
    }
    let new_path = PathBuf::from(args[1]);
    if new_path.exists() && new_path.is_dir() {
        if let Err(e) = env::set_current_dir(&new_path) {
            println!("{}", Red.paint(format!("Faild to change dir {}", e)));
        }
    } else {
        println!("{}", Red.paint(format!("Directory not found: {}", new_path.display().to_string())));
    }
}

fn list_directory() {
    let current_dir = env::current_dir().unwrap();
    for entry in current_dir.read_dir().unwrap() {
        if let Ok(entry) = entry {
            println!("{}", entry.file_name().to_string_lossy());
        }
    }
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    println!("");
}

fn run_gxcore(args: Vec<&str>) {
    if args.len() < 2 {
        println!("{}", Red.paint("Error: start gxcore with --start"));
    } else if args[1] == "--start" {
        println!("{}", Red.paint("WARNING: IF YOU MAKE AN MISTAKE IN GXCORE THAN YOUR COMPUTER IS MAYBE UNUSEABLE!!!"));
        gxcore::start();
    }
}



fn run_interpreter(args: Vec<&str>) {
    if args.len() < 2 {
        println!("{}", Red.paint("Usage: gx <filename>"));
        return;
    }

    let filename = args[1];
    match std::fs::read_to_string(filename) {
        Ok(code) => {
            let mut lexer = Lexer::new(code);
            let tokens = lexer.tokenize();
            cyber_gx_interpreter::interpreter::Interpreter::execute(tokens);
        }
        Err(e) => {
            println!("{}", Red.paint(format!("Error: Faild to open: {}", e)));
        }
    }
}



fn run_external_command(args: Vec<&str>) {
    if args.is_empty() {
        return;
    }

    match Command::new(args[0])
        .args(&args[1..])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
    {
        Ok(mut child) => {
            let _ = child.wait();
        }

        Err(e) => {
            println!("{}", Red.paint(format!("Error command is not found {}", e)));
        }
    }
}
