mod gxcore;
mod env_var;
mod compiler;
mod dev;

use compiler::gxcompiler::Compiler;
use std::process::{Command, Stdio};
use std::{env, io};
use std::fs::File;
use std::io::BufRead;
use std::path::{Path, PathBuf};
use rustyline::Editor;
use nu_ansi_term::Color::{Red, LightRed, Yellow, Blue, Green};
use rustyline::completion::FilenameCompleter;
use rustyline::Config;




fn main()  {
    let config = Config::builder()
        .history_ignore_space(true)
        .build();
    let editor_err:String = format!("{}", Red.paint("Failed to launch editor."));
    let mut rl: Editor<FilenameCompleter, rustyline::history::FileHistory> = Editor::with_config(config).expect(&editor_err);
    let history_path: PathBuf = get_history_path();

    
    if rl.load_history(&history_path).is_err() {
        println!("{}", Red.paint("No .history data found"));
    }

    
    println!("{}", LightRed.paint(format!("GXShell version {}", env_var::GXSHELL_VERSION)));
    loop {
        let current_dir: PathBuf = env::current_dir().unwrap_or(PathBuf::from("C:\\"));
        let prompt = format!("{}> ", current_dir.display().to_string().trim());
        match rl.readline(&prompt) {
            Ok(line) => {
                let command = line.trim();
                if command == "exit" {
                    break;
                }

                let _ = rl.add_history_entry(command);
                execute_command(command);
            }

            Err(_) => break,
        }
    }

    if let Err(e) = rl.save_history(&history_path) {
        eprintln!("{}", Red.paint(format!("Error: Could not save history data: {}", e)));
    }
    
}

fn check_std_library() {
    let std_path: &Path = Path::new("std");
    let std_modules: Vec<&str> = vec![
        "std/io/stdin.rs",
        "std/io/stdout.rs",
        "std/gxmodules/gxmath.rs",
    ];

    let mut missing_modules: Vec<String> = Vec::new();

    for module in &std_modules {
        if !Path::new(module).exists() {
            missing_modules.push(module.to_string())
        }
    }

    if !std_path.exists() {
        println!("{}", Red.paint("Warning: The standard library (std/) is missing!"));
        println!("{}", Yellow.paint("Some CyberGX features may not work correctly."));
    } else if !missing_modules.is_empty() {
        println!("{}", Red.paint("Warning: Some standard library modules are missing!"));
        for module in missing_modules {
            println!("{}", Yellow.paint(format!("Missing: {}", module)));
        }
    }
}


fn execute_command(command: &str) {
    let parts: Vec<&str> = command.split_whitespace().collect();


    if parts.is_empty() {
        return;
    }

    match parts[0] {
        "cd" => change_directory(parts),
        "dev" => dev::dev_mode(parts),
        "dir" => list_directory(),
        "cls" => clear_screen(),
        "version_log" => println!("{}", Green.paint(
            "GXShell version 0.1.5\n
            Version Log:\n
            Cybergx: updated Parser and Compiler\n 
            Changes: -\n
            fixed issuses: -\n
            added features: Shell scripting\n"
        )),
        "version" => println!("{}", Green.paint(env_var::GXSHELL_VERSION)),
        "gxcore" => run_gxcore(parts),
        "gx" => run_compiler(parts),
        _ => run_external_command(parts)
    }
}

fn get_history_path() -> PathBuf {
    if cfg!(target_os = "windows") {
        env::var("USERPROFILE")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("."))
    } else {
        dirs::home_dir().unwrap_or_else(|| PathBuf::from("."))
    }
    .join(".history")
}



fn change_directory(args: Vec<&str>) {
    if args.len() < 2 {
        println!("{}", Blue.paint("Usage: cd <path>"));
        return;
    }

    let new_path = if args[1] == "~" {
        dirs::home_dir().unwrap_or(PathBuf::from("."))
    } else {
        PathBuf::from(args[1])
    };

    if new_path.exists() && new_path.is_dir() {
        if let Err(e) = env::set_current_dir(&new_path) {
            println!("{}", Red.paint(format!("Failed to change dir: {}", e)));
        }
    } else {
        println!("{}", Red.paint(format!("Directory not found: {}", new_path.display())));
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
        println!("{}", Red.paint("Error: start gxcore with --start"));
        return;
    } else if args[1] == "--start" {
        println!("{}", Yellow.paint("WARNING: IF YOU MAKE A MISTAKE IN GXCORE THEN YOUR COMPUTER MAY BE UNUSABLE!!!"));
        gxcore::start();
    }
}

fn run_compiler(args: Vec<&str>) {
    if args.len() < 3 {
        println!("{}", Blue.paint("Usage: gx <filename> <outputname>"));
        return;
    } else if args[1] == "--new" {
        if Path::new(&args[2]).exists() {
            println!("{}", Yellow.paint(format!("Warning: Project '{}' already exists!", args[2])));
            return;
        }

        Compiler::crate_new_project(&args[2]);
    }

    match std::fs::read_to_string(args[1]) {
        Ok(source_code) => {
            Compiler::compile_to_rust(&source_code, args[2]);
        }
        Err(e) => {
            println!("{}", Red.paint(format!("Error: Failed to read data {}:, {}", args[1], e)));
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
            println!("{}", Red.paint(format!("Error command not found: {}", e)));
        }
    }
}
