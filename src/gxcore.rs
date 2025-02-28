use std::io::{self, Write};
use std::process::{Command, Stdio};
use std::env;
use std::fs;
use std::path::PathBuf;

pub fn start() {
    let mut input:String = String::new();
    
    
    println!("GXCORE All rigths served");
    loop {
        let current_dir:PathBuf = env::current_dir().unwrap_or(PathBuf::from("C:\\"));
        print!("{}>", current_dir.display());
        io::stdout().flush().unwrap();

        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let command:&str = input.trim();

        if command == "exit" {
            break;
        }

        execute_command(command);

    
  }
}

fn execute_command(command:&str){
    let parts:Vec<&str> = command.split_whitespace().collect();

      if parts.is_empty() {
          return;
      }

    match parts[0] {
        "bios" => bios(parts),
        "cd" => change_directory(parts),
        "dir" => list_directory(),
        "cls" => clear_screen(),
         _ => run_external_command(parts),
    }
}

fn change_directory(args: Vec<&str>) {
    if args.len() < 2 {
        println!("Usage: cd <path>");
        return;
    }
    let new_path = PathBuf::from(args[1]);
    if env::set_current_dir(&new_path).is_ok() {
        println!("Changed directory to {}", new_path.display());
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
    print!("\x1B[2J\x1B[1;1H");
}

fn bios(args: Vec<&str>) {
    let path = "/sys/firmware/efi/efivars/BootOrder-8be4df61-93ca-11d2-aa0d-00e098032b8c";

    match fs::read(path) {
        Ok(data) => {
            println!("BootOrder (raw): {:?}", &data);
        }
        Err(e) => {
            eprintln!("Fehler beim Lesen der UEFI-Variable: {}", e);
        }
    }
}

fn run_external_command(args: Vec<&str>) {
    if let Ok(mut child) = Command::new(args[0])
        .args(&args[1..])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
    {
        let _ = child.wait();
    } else {
        println!("Unknown command: {}", args[0]);
    }
}
