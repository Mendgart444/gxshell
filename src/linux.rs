use nu_ansi_term::Color::Green;
use sysinfo::System;
use std::process::Command;
use std::fs;

pub fn system_info() {
    let mut sys = System::new();
    sys.refresh_all();

    println!("{}", Green.paint(format!("CPU usage: {:.2}%", sys.global_cpu_info().cpu_usage())));
    println!("{}", Green.paint(format!("RAM {}/{} MB", sys.used_memory() / 1024, sys.total_memory() / 1024)));
}

pub fn kill_process_linux(pid: u32) {
    let _ = Command::new("kill")
        .arg(format!("{}", pid))
        .status();
}


pub fn add_user_linux(username: &str) {
    let output = Command::new("useradd")
        .arg(username)
        .output()
        .expect("Failed to execute useradd");

    println!("{}", String::from_utf8_lossy(&output.stdout));
}


pub fn set_ip_address_linux(interface: &str, ip: &str) {
    let output = Command::new("ip")
        .args(["addr", "add", ip, "dev", interface])
        .output()
        .expect("Failed to execute ip addr add");

    println!("{}", String::from_utf8_lossy(&output.stdout));
}


pub fn create_partition_linux(device: &str, start: &str, end: &str) {
    let output = Command::new("parted")
        .args([device, "mkpart", "primary", start, end])
        .output()
        .expect("Failed to execute parted");

    println!("{}", String::from_utf8_lossy(&output.stdout));
}


pub fn read_bios_info_linux() {
    let output = Command::new("dmidecode")
        .arg("--type")
        .arg("bios")
        .output()
        .expect("Failed to execute dmidecode");

    println!("{}", String::from_utf8_lossy(&output.stdout));
}

pub fn set_uefi_variable(name: &str, value: &[u8]) -> std::io::Result<()> {
   let path = format!("/sys/firmware/efi/efivars/{}", name);
   fs::write(path, value)?;
   Ok(())
}

pub fn read_temperatures_linux() {
    let output = Command::new("sensors")
        .output()
        .expect("Failed to execute sensors");

    println!("{}", String::from_utf8_lossy(&output.stdout));
}


pub fn change_boot_order_linux(new_order: &str) {
    use std::process::Command;

    let output = Command::new("efibootmgr")
        .arg("-o")
        .arg(new_order)
        .output()
        .expect("Failed to execute efibootmgr");

    println!("{}", String::from_utf8_lossy(&output.stdout));
}
