use std::io::{self, Write};

pub fn print(text: &str) {
    print!("{}", text);
    io::stdout().flush().expect("Failed to flush");
}

pub fn println(text: &str) {
    println!("{}", text);
}