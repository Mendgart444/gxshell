use std::fs;
use std::process::Command;

pub fn compile_to_rust(source_code: &str, output_filename: &str) {
    let mut rust_code = String::from("fn main() {\n");

    for line in source_code.lines() {
        let line = line.trim();
        if line.starts_with("println << ") {
            let content = line.trim_start_matches("println << ").trim_end_matches(";");
            rust_code.push_str(&format!("    println!({});\n", content));
        }
    }

    rust_code.push_str("}\n");

    let rust_file = format!("{}.rs", output_filename);
    fs::write(&rust_file, &rust_code).expect("Failed to write Rust file");

    // Kompilieren mit rustc
    let _ = Command::new("rustc")
        .args([&rust_file, "-o", output_filename])
        .output()
        .expect("Failed to compile Rust code");

    fs::remove_file(rust_file).ok(); // Temporäre Datei löschen
}
