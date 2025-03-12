use std::process::{Command, Stdio};
use serde::{Serialize, Deserialize};


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

        

    } else if args[1] == "--setup" {
        set_up_development_environment();
    }
}

fn set_up_development_environment() {
    
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
            println!("Failed to remove temporary Rust file: {}", e);
        }
    }
}
