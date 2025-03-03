use std::process::{Command, Stdio};
use std::env;
use std::fs;
use std::path::PathBuf;
use sysinfo::System;
use rustyline::Editor;


pub fn start() {


    let mut rl = Editor::<(), _>::new().expect("Faild to launch editor.");
    let _ = rl.load_history(".hystory");
    
    
    println!("GXCORE All rigths served");
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

fn execute_command(command:&str){
    let parts:Vec<&str> = command.split_whitespace().collect();

      if parts.is_empty() {
          return;
      }

    match parts[0] {
        "bios" => {
            if is_admin() {
                bios(parts);
            } else {
                println!("Error, you need admin rights for this");
            }
        },
        "sysinfo" => system_info(),
        "cd" => change_directory(parts),
        "dir" => list_directory(),
        "cls" => clear_screen(),
         _ => run_external_command(parts),
    }
}
#[cfg(target_os = "windows")]
fn is_admin() -> bool {
    use winapi::um::processthreadsapi::OpenProcessToken;
    use winapi::um::securitybaseapi::GetTokenInformation;
    use winapi::um::winnt::{TokenElevation, HANDLE, TOKEN_ELEVATION, TOKEN_QUERY};
    use winapi::um::handleapi::CloseHandle;
    use winapi::um::processthreadsapi::GetCurrentProcess;
    use std::ptr;
    use std::mem;




    unsafe {
        let process = GetCurrentProcess();
        let mut token: HANDLE = ptr::null_mut();

        if OpenProcessToken(process, TOKEN_QUERY, &mut token) == 0 {
            return false;
        }

        let mut elevation = TOKEN_ELEVATION { TokenIsElevated: 0 };
        let mut size = mem::size_of::<TOKEN_ELEVATION>() as u32;

        let success = GetTokenInformation(
            token,
            TokenElevation,
            &mut elevation as *mut _ as *mut _,
            size,
            &mut size,
        );

        CloseHandle(token); // WICHTIG: Handle schließen, um Memory-Leaks zu vermeiden

        success != 0 && elevation.TokenIsElevated != 0
    }
}

#[cfg(not(target_os = "windows"))]
fn is_admin() -> bool {
    std::env::var("USER").unwrap_or_default() == "root"
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

fn bios(args: Vec<&str>) {
    if args.len() < 2 {
        println!("Error that command is not found in the bios syntax.");
        return;
    } else if args[1] == "--read_var"{
        let path = "/sys/firmware/efefivars/BootOrder-8be4df61-93ca-11d2-aa0d-00e098032b8c";

        match fs::read(path) {
            Ok(data) => {
                println!("BootOrder (raw): {:?}", &data);
            }
            Err(e) => {
                eprintln!("Error: could not read UEFI-Variable: {}", e);
            }
        
        }
    }
}

fn system_info() {
    let mut sys = System::new();
    sys.refresh_all();

    println!("CPU usage: {:.2}%", sys.global_cpu_info().cpu_usage());
    println!("RAM {}/{} MB", sys.used_memory() / 1024, sys.total_memory() / 1024);
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
