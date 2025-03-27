use std::{ffi::c_void, process::Command};

pub fn read_temperatures_windows() {
    let output = Command::new("wmic")
        .arg("temperature")
        .arg("get")
        .output()
        .expect("Failed to execute wmic");

    println!("{}", String::from_utf8_lossy(&output.stdout));
}

pub fn kill_process_windows(pid: u32) {
    let _ = Command::new("taskkill")
        .args(["/PID", &pid.to_string(), "/F"])
        .status();
}


pub fn add_user_windows(username: &str, password: &str) {
    let output = Command::new("net")
        .args(["user", username, password, "/add"])
        .output()
        .expect("Failed to execute net user");

    println!("{}", String::from_utf8_lossy(&output.stdout));
}


pub fn set_ip_address_windows(interface: &str, ip: &str, gateway: &str) {
    let output = Command::new("netsh")
        .args(["interface", "ip", "set", "address", interface, "static", ip, gateway])
        .output()
        .expect("Failed to execute netsh");

    println!("{}", String::from_utf8_lossy(&output.stdout));
}


pub fn create_partition_windows(disk_number: &str, size: &str) {
    let script = format!("select disk {}\ncreate partition primary size={}", disk_number, size);
    let output = Command::new("diskpart")
        .arg("/s")
        .arg(script)
        .output()
        .expect("Failed to execute diskpart");

    println!("{}", String::from_utf8_lossy(&output.stdout));
}


pub fn read_bios_info_windows() {
    let output = Command::new("wmic")
        .arg("bios")
        .arg("get")
        .arg("*")
        .output()
        .expect("Failed to execute wmic");

    println!("{}", String::from_utf8_lossy(&output.stdout));
}

pub fn change_boot_order_win(entry: &str) {
    let output = Command::new("bcdedit")
        .arg("/default")
        .arg(entry)
        .output()
        .expect("Failed to execute bcdedit");
    println!("{}", String::from_utf8_lossy(&output.stdout));
}

#[cfg(windows)]
pub fn set_uefi_variable(name: &str, value: &[u8]) -> bool {
    use winapi::um::winbase::SetFirmwareEnvironmentVariableA;

    unsafe {
        let name = std::ffi::CString::new(name).unwrap();
        let result = SetFirmwareEnvironmentVariableA(
            name.as_ptr(),
            std::ptr::null(),
            value.as_ptr() as *mut c_void,
            value.len() as u32,
        );
        result != 0
    }
}
