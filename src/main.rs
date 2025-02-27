mod gxinstaller;
mod gxcore;
mod env_var;
use std::io::{self, Write};
use std::process::{Command, Stdio};
use std::env;
use std::path::PathBuf;
//use crossterm::execute;

fn main() {
    let mut input:String = String::new();

    println!("GxShell v0.1.0, all rights served.");

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

fn run_gxinstaller(args: Vec<&str>) {
    if args.len() < 2 {
        println!("option not found. If you need help run: gxinstaller --help");
        return;
    } else if args.contains(&"--help")  {
        println!("Options:");
        println!("  --help              shows this message.");
        println!("  --version           shows the version of gxinstaller.");
        println!("  --install (option)  install the application or tool.");
        println!("  --list              shows a list of the applications or tools you can install.");
        println!("  --update            updates all tools and software.");

    } else if args.contains(&"--version")  {
        println!("gxinstaller version: {}", env_var::GXINSTALLER_VERSION);
    } else if args.contains(&"--list")  {
        println!("CyberGX");
        println!("C/C++");
        println!("Python");
        println!("GXManager");
        println!("GXGui designer");
        println!("GX IDE (is not only for CyberGX");
        println!("git");
    } else if args.contains(&"--install")  {
        if args.contains(&"CyberGX") {
            gxinstaller::install_cybergx_default()
        } else {
            println!("Package not found. to show avaiable packages: gxinstaller --list");
        }
    } else if args.contains(&"--update") {
        gxinstaller::update_all()
    }


}

fn run_gxcore(args: Vec<&str>) {
    if args.len() < 2 {
        println!("Error Option not found");
    } else if args.contains(&"--start") {
        println!("WARNING: IF YOU MAKE AN MISTAKE IN GXCORE THAN YOUR COMPUTER IS MAYBE UNUSEABLE!!!");
        gxcore::start();
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
