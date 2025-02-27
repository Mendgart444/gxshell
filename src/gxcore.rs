use std::io::{self, Write};
use std::process::{Command, Stdio};
use std::env;
use windows::Win32::System::Firmware::GetFirmwareEnvironmentVariableA;
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
        "stepin" => run_stepin(parts),
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

fn run_stepin(args: Vec<&str>) {
    let mut in_bios:bool = false;
    if args.contains(&"--bios") {
        println!("gxcore entert bios.");
        println!("to show options type help --bios");
            in_bios = true;
    } else if in_bios && args.contains(&"UEFI_var") {
        let mut buffer = [0u8; 256];
        let result = unsafe {
            GetFirmwareEnvironmentVariableA("BootOrder\0", "{8BE4DF61-93CA-11D2-AA0D-00E098032B8C}\0", buffer.as_mut_ptr() as *mut _, buffer.len() as u32)
        };
        
        if result == 0 {
            println!("Error Faild to read UEFI-Variable!");
        } else {
            println!("BootOrder: {:?}", &buffer[..result as usize]);
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
