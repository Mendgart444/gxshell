mod gxcore;
mod env_var;
mod updater;
mod cyber_gx_interpreter;
mod dev;

use cyber_gx_interpreter::gxcompiler::Compiler;
use std::process::{Command, Stdio};
use std::env;
use std::path::PathBuf;
use rustyline::Editor;

fn main()  {
    let mut rl: Editor<(), rustyline::history::FileHistory> = Editor::<(), _>::new().expect("Failed to launch editor.");
    let _ = rl.load_history(".history");

    
    println!("GXShell version {}", env_var::GXSHELL_VERSION);
    loop {
        let current_dir: PathBuf = env::current_dir().unwrap_or(PathBuf::from("C:\\"));
        let prompt = format!("{}> ", current_dir.display().to_string().trim());
        match rl.readline(&prompt) {
            Ok(line) => {
                let command = line.trim();
                if command == "exit" {
                    break;
                } else if command == "update" {
                    println!("Updater is not available yet.");
                }
                let _ = rl.add_history_entry(command);
                execute_command(command);
            }

            Err(_) => break,
        }
    }

    let _ = rl.save_history(".history");
    
}

fn execute_command(command: &str) {
    let parts: Vec<&str> = command.split_whitespace().collect();

    if parts.is_empty() {
        return;
    }

    match parts[0] {
        "cd" => change_directory(parts),
        "update" => println!(" "),
        "dev" => dev::dev_mode(parts),
        "dir" => list_directory(),
        "cls" => clear_screen(),
        "version" => println!("{}", env_var::GXSHELL_VERSION),
        "gxcore" => run_gxcore(parts),
        "gx" => run_compiler(parts),
        _ => run_external_command(parts),
    }
}

fn change_directory(args: Vec<&str>) {
    if args.len() < 2 {
        println!("Usage: cd <path>");
        return;
    }
    let new_path = PathBuf::from(args[1]);
    if new_path.exists() && new_path.is_dir() {
        if let Err(e) = env::set_current_dir(&new_path) {
            println!("Failed to change dir {}", e);
        }
    } else {
        println!("Directory not found: {}", new_path.display());
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
    #[cfg(windows)]
    {
        let _ = Command::new("cmd").arg("/c").arg("cls").status();
    }
    #[cfg(not(windows))]
    {
        let _ = Command::new("clear").status();
    }
}

fn run_gxcore(args: Vec<&str>) {
    if args.len() < 2 {
        println!("Error: start gxcore with --start");
    } else if args[1] == "--start" {
        println!("WARNING: IF YOU MAKE A MISTAKE IN GXCORE THEN YOUR COMPUTER MAY BE UNUSABLE!!!");
        gxcore::start();
    }
}

fn run_compiler(args: Vec<&str>) {
    if args.len() < 3 {
        println!("Usage: gx <filename> <outputname>");
        return;
    }

    match std::fs::read_to_string(args[1]) {
        Ok(source_code) => {
            Compiler::compile_to_rust(&source_code, args[2]);
        }
        Err(_) => {
            println!("Fehler: Datei konnte nicht gelesen werden.");
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
            println!("Error command not found: {}", e);
        }
    }
}
