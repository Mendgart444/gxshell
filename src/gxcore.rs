use crate::linux;
use crate::windows;
use std::process::{Command, Stdio};
use std::env;
#[cfg(not(windows))]
use std::fs;
use std::path::PathBuf;
use rustyline::Editor;
use nu_ansi_term::Color::{Red, LightRed, Blue};



pub fn start() {


    let mut rl = Editor::<(), _>::new().expect("Faild to launch editor.");
    let history_path: PathBuf = get_history_path();
    let _ = rl.load_history(&history_path);
    
    
    
    println!("{}", LightRed.paint("GXCORE version 0.1.0"));
    loop {
        let current_dir:PathBuf = env::current_dir().unwrap_or(PathBuf::from("C:\\"));
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
  
    let _ = rl.save_history(&history_path);



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
                println!("{}", Red.paint("Error, you need admin rights for this"));
            }
        },
        "add-user" => {
            if is_admin() {
                if cfg!(target_os = "windows") {
                    windows::add_user_windows(parts[1], parts[2]);
                } else {
                    linux::add_user_linux(parts[1]);
                }
            } else {
                println!("{}", Red.paint("Error: you need admin rights for this"));
            }
        },
        "sysinfo" => linux::system_info(),
        "set-ip-adress" => {
            if is_admin() {
                if cfg!(target_os = "windows") {
                    windows::set_ip_address_windows(parts[1], parts[2], parts[3]);
                } else {
                    linux::set_ip_address_linux(parts[1], parts[2]);
                }
            } else {
                println!("{}", Red.paint("Error: you need admin rights for this"));
            }
        },
        "kill-process" => {
            if cfg!(target_os = "windows") {
                windows::kill_process_windows(parts[1].parse::<u32>().expect("Invaild Number"));
            } else {
                linux::kill_process_linux(parts[1].parse::<u32>().expect("Invaild Number"));
            }
        },
        "cd" => change_directory(parts),
        "dir" => list_directory(),
        "cls" => clear_screen(),
         _ => run_external_command(parts),
    }
}

fn get_history_path() -> PathBuf {
    if cfg!(target_os = "windows") {
        env::var("USERPROFILE")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("."))
    } else {
        #[cfg(target_os = "linux")]
        use std::fs;
        dirs::home_dir().unwrap_or_else(|| PathBuf::from(".")) 
    }
        .join(".history")
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
        println!("{}", Blue.paint("Usage: cd <path>"));
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

fn bios(args: Vec<&str>) {
    if args.len() < 2 {
        println!("{}", Red.paint("Error that command is not found in the bios syntax."));
        return;
    }
    #[cfg(not(windows))]
    if args[1] == "--read-var"{
        let path = "/sys/firmware/efefivars/BootOrder-8be4df61-93ca-11d2-aa0d-00e098032b8c";

        match fs::read(path) {
            Ok(data) => {
                println!("BootOrder (raw): {:?}", &data);
            }
            Err(e) => {
                eprintln!("{}", Red.paint(format!("Error: could not read UEFI-Variable: {}", e)));
            }
        
        }
    } else if args[1] == "--set-var" {
        linux::set_uefi_variable(args[2] as &str, args[3].as_bytes());
    } else if args[1] == "--change-boot-order" {
        linux::change_boot_order_linux(args[2] as &str);
    } else if args[1] == "--show-temp-and-fans" {
        linux::read_temperatures_linux();
    } else if args[1] == "--gpu-fan-speed" {
        set_nvidia_fan_speed(args[2].parse::<u32>().expect("Invaild Number"));
    } else if args[1] == "--create-partition" {
        linux::create_partition_linux(args[2], args[3], args[4]);
    } else if args[1] == "--read-bios-info" {
        linux::read_bios_info_linux();
    }

    #[cfg(windows)]
    if args[1] == "--read_var"{
        println!("under construct");
    } else if args[1] == "--set-var" {
        windows::set_uefi_variable(args[2] as &str, args[3].as_bytes());
    } else if args[1] == "--change-boot-order" {
        windows::change_boot_order_win(args[2] as &str);
    } else if args[1] == "--read-bios-info" {
        windows::read_bios_info_windows();
    } else if args[1] == "--show--temp-and-fans" {
        windows::read_temperatures_windows();
    } else if args[1] == "--gpu-fan-speed" {
        set_nvidia_fan_speed(args[2].parse::<u32>().expect("Invaild Number"));
    } else if args[1] == "--create-partition" {
        windows::create_partition_windows(args[2], args[3]);
    }


}

fn set_nvidia_fan_speed(speed: u32) {
    let output = Command::new("nvidia-settings")
        .args(["-a", &format!("GPUFanControlState=1")])
        .args(["-a", &format!("GPUTargetFanSpeed={}", speed)])
        .output()
        .expect("Failed to execute nvidia-settings");

    println!("{}", String::from_utf8_lossy(&output.stdout));
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
            println!("{}", Red.paint(format!("Error command {} is not found as internal or external command: {}", args[0], e)));
        }
    }
}
