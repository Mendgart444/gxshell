mod gxcore;
mod env_var;
mod linux;
mod windows;

use std::process::{Command, Stdio};
use std::env;
use std::path::PathBuf;
use nu_ansi_term::Color::{Red, LightRed, Yellow, Blue, Green};
use rustyline::completion::FilenameCompleter;
use rustyline::highlight::MatchingBracketHighlighter;
use rustyline::hint::HistoryHinter;
use rustyline::validate::MatchingBracketValidator;
use rustyline::Helper;
use rustyline::validate::Validator;
use rustyline::highlight::Highlighter;
use rustyline::completion::Completer;
use rustyline::hint::Hinter;
use rustyline::{Config, EditMode, Editor};
 

pub struct GXShellHelper {
    pub completer: FilenameCompleter,
    pub highlighter: MatchingBracketHighlighter,
    pub hinter: HistoryHinter,
    pub validator: MatchingBracketValidator,
}

// Implementiere die benötigten Traits
impl Completer for GXShellHelper {
    type Candidate = String;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        ctx: &rustyline::Context<'_>,
    ) -> rustyline::Result<(usize, Vec<String>)> {
        let completions: (usize, Vec<rustyline::completion::Pair>) = self.completer.complete(line, pos, ctx)?;
        let string_completions: Vec<String> = completions.1.into_iter().map(|p| p.display).collect();
        Ok((completions.0, string_completions))
    }
}

impl Hinter for GXShellHelper {
    type Hint = String;
    fn hint(&self, line: &str, pos: usize, _ctx: &rustyline::Context<'_>) -> Option<String> {
        self.hinter.hint(line, pos, _ctx)
    }
}

impl Highlighter for GXShellHelper {}

impl Validator for GXShellHelper {}

impl Helper for GXShellHelper {}

fn main()  {
    let config: Config = Config::builder().edit_mode(EditMode::Emacs).build();

    let helper: GXShellHelper = GXShellHelper {
        completer: FilenameCompleter::new(),
        highlighter: MatchingBracketHighlighter::new(),
        hinter: HistoryHinter {},
        validator: MatchingBracketValidator::new(),
    };

    let mut rl: Editor<GXShellHelper, rustyline::history::FileHistory> = Editor::with_config(config)
        .expect("Failed to create editor");

    rl.set_helper(Some(helper));

    // History laden
    let history_path: PathBuf = get_history_path();
    if rl.load_history(&history_path).is_err() {
        println!("No history found.");
    }
    
    println!("{}", LightRed.paint(format!("GXShell version {}", env_var::GXSHELL_VERSION)));
    loop {
        let current_dir: PathBuf = env::current_dir().unwrap_or(PathBuf::from("C:\\"));
        let prompt: String = format!("{}> ", current_dir.display().to_string().trim());
        match rl.readline(&prompt) {
            Ok(line) => {
                let command = line.trim();
                if command == "exit" {
                    break;
                }
                rl.save_history(&history_path).expect("Error could not save history");

                execute_command(command);
            }

            Err(_) => break,
        }
    }

    
}

fn execute_command(command: &str) {
    let parts: Vec<&str> = command.split_whitespace().collect();


    if parts.is_empty() {
        return;
    }

    match parts[0] {
        "cd" => change_directory(parts),
        "dir" => list_directory(),
        "cls" => clear_screen(),
        "info" => println!("
        version: {}\n
        about:   GXShell is a shell for Dev's and GX\n
        author:  Raffael\n
        ", env_var::GXSHELL_VERSION),
        "version" => println!("{}", Green.paint(env_var::GXSHELL_VERSION)),
        "gxcore" => run_gxcore(parts),
        _ => run_external_command(parts)
    }
}

fn get_history_path() -> PathBuf {
    if cfg!(target_os = "windows") {
        env::var("USERPROFILE")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("."))
    } else {
        dirs::home_dir().unwrap_or_else(|| PathBuf::from("."))
    }
    .join(".history")
}



fn change_directory(args: Vec<&str>) {
    if args.len() < 2 {
        println!("{}", Blue.paint("Usage: cd <path>"));
        return;
    }

    let new_path: PathBuf = if args[1] == "~" {
        dirs::home_dir().unwrap_or(PathBuf::from("."))
    } else {
        PathBuf::from(args[1])
    };

    if new_path.exists() && new_path.is_dir() {
        if let Err(e) = env::set_current_dir(&new_path) {
            println!("{}", Red.paint(format!("Failed to change dir: {}", e)));
        }
    } else {
        println!("{}", Red.paint(format!("Directory not found: {}", new_path.display())));
    }
}

fn list_directory() {
    let current_dir: PathBuf = env::current_dir().unwrap();
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

fn run_gxcore(args: Vec<&str>) {
    if args.len() < 2 {
        println!("{}", Red.paint("Error: start gxcore with --start"));
        return;
    } else if args[1] == "--start" {
        println!("{}", Yellow.paint("WARNING: IF YOU MAKE A MISTAKE IN GXCORE THEN YOUR COMPUTER MAY BE UNUSABLE!!!"));
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
            println!("{}", Red.paint(format!("Error command not found: {}", e)));
        }
    }
}
