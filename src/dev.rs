use std::process::{Command, Stdio};
use std::io;
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::fs::read_to_string;
use std::io::Write;


#[derive(Serialize, Deserialize)]
struct Language {
    names: Vec<String>,
}


pub fn dev_mode(args: Vec<&str>) {
    let run_git_status:Vec<&str> = vec!["git", "status"];
    //let run_cargo_version:Vec<&str> = vec!["cargo", "version"];

    if args.len() < 2 {
        println!("options:");
        println!("--status    view a compact dashboard.");
        println!("--setup     set up the development environment.");
    } else if args[1] == "--status" {
        println!("your dashboard:");
        println!("git status:");

        run_external_command(run_git_status);

        let json_string = read_to_string("languages.json")
            .expect("could not read JSON file");

        // JSON in Rust-Struktur umwandeln
        let data: Language = serde_json::from_str(&json_string)
            .expect("something went wrong while parsing JSON");

        // Sprachen ausgeben
        println!("Saved languages:");
        for lang in data.names.iter() {
            println!("- {}", lang);
        }

    } else if args[1] == "--setup" {
        set_up_development_environment();
    }
}

fn set_up_development_environment() {
    let mut input:String = String::new();
    println!("Setting up the development environment...");
    println!("Please choose your prefered programming languages: for example: rust go python.");

    io::stdin().read_line(&mut input).expect("Failed to read line");
    let args: Vec<String> = input.trim().split_whitespace().map(|s| s.to_string()).collect();

    let data:Language = Language { names: args };
    
    let json = serde_json::to_string_pretty(&data).unwrap();
    let mut file = File::create("languages.json").expect("Unable to create file");
    file.write_all(json.as_bytes()).expect("Unable to write to file");
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
            println!("Error command is not found {}", e);
        }
    }
}
