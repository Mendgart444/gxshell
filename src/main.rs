mod gxinstaller;
mod gxcore;
mod env_var;
use std::process::{Command, Stdio};
use std::env;
use std::path::PathBuf;
use rustyline::Editor;
//use crossterm::execute;

fn main() {
    let mut rl = Editor::<(), _>::new().expect("Faild to launch editor.");
    let _ = rl.load_history(".hystory");
    
    
    println!("GXShell version 0.1.0 All rigths served");
    loop {
        let current_dir:PathBuf = env::current_dir().unwrap_or(PathBuf::from("C:\\"));
        let prompt = format!("{}> ", current_dir.display());
        
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
  
    let _ = rl.save_history(".hystory");


}

fn execute_command(command:&str) {
    let parts:Vec<&str> = command.split_whitespace().collect();

    if parts.is_empty() {
        return;
    }

    match parts[0] {
        "cd" => change_directory(parts),
        "dir" => list_directory(),
        "cls" => clear_screen(),
        "version" => println!("version 0.1.0"),
        "gxinstaller" => run_gxinstaller(parts),
        "gxcore" => run_gxcore(parts),
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
            println!("Faild to change dir {}", e);
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
    print!("\x1B[2J\x1B[1;1H");
}

fn run_gxinstaller(args: Vec<&str>) {
    if args.len() < 2 {
        println!("option not found. If you need help run: gxinstaller --help");
        return;
    } else if args[1] == "--help"  {
        println!("Options:");
        println!("  --help              shows this message.");
        println!("  --version           shows the version of gxinstaller.");
        println!("  --install (option)  install the application or tool.");
        println!("  --list              shows a list of the applications or tools you can install.");
        println!("  --update            updates all tools and software.");

    } else if args[1] == "--version"  {
        println!("gxinstaller version: {}", env_var::GXINSTALLER_VERSION);
    } else if args[1] == "--list"  {
        println!("CyberGX");
        println!("C/C++");
        println!("Python");
        println!("GXManager");
        println!("GXGui designer");
        println!("GX IDE (is not only for CyberGX");
        println!("git");
    } else if args[1] == "--install"  {
        if args[2] == "CyberGX" {
            gxinstaller::install_cybergx_default()
        } else {
            println!("Package not found. to show avaiable packages: gxinstaller --list");
        }
    } else if args[1] == "--update" {
        gxinstaller::update_all()
    }


}

fn run_gxcore(args: Vec<&str>) {
    if args.len() < 2 {
        println!("Error: This is not an available option in gxcore.");
    } else if args[1] == "--start" {
        println!("WARNING: IF YOU MAKE AN MISTAKE IN GXCORE THAN YOUR COMPUTER IS MAYBE UNUSEABLE!!!");
        gxcore::start();
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
            println!("Error command {} is not found as internal or external command: {}", args[0], e);
        }
    }
}
