mod env;
use std::io::{self, Write};

fn main() {
    #[allow(unused_mut)]
    let mut current_dir:&str = "C:";
    let mut command:String = String::new();

    println!("GXCMD version: {}, all rights served", env::VERSION);

    loop {
        print!("{}/>", current_dir);

        io::stdout().flush().expect("Something went wrong while flushing stdout");

        io::stdin()
            .read_line(&mut command)
            .expect("something went wrong");

        let command: &str = command.trim();

        if command.eq_ignore_ascii_case("exit") {
            break;
        }

        if command == "version" {
            println!("{}", env::VERSION);
        } else if command == "help" {
            println!("Go to https://mendgart444.github.io/gxweb/")
        } else {
            println!("command: {} not found.", command);
        }
    }
}
