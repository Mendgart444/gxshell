use self_update::backends::github::Update;
use self_update::cargo_crate_version;
use nu_ansi_term::Color::{Green, Red};
use std::error::Error;

const REPO_OWNER: &str = "Mendgart444";  // Dein GitHub Benutzername/Organisation
const REPO_NAME: &str = "gxshell";         // Dein Repository-Name

/// Prüft auf Updates und installiert automatisch die neueste Version
pub fn check_and_update() {
    println!("{}", Green.paint("Checking for updates..."));

    match perform_update() {
        Ok(()) => println!("{}", Green.paint("Update successful! Restart GXShell to apply the changes...")),
        Err(e) => println!("{}", Red.paint(format!("Error: faild to update: {}", e))),
    }
}

/// Lädt das neueste Release von GitHub herunter und ersetzt die alte Datei
fn perform_update() -> Result<(), Box<dyn Error>> {
    let status = Update::configure()
        .repo_owner(REPO_OWNER)
        .repo_name(REPO_NAME)
        .bin_name("gxcmd") // Name der Binärdatei auf GitHub Releases
        .show_download_progress(true)
        .current_version(cargo_crate_version!()) // Liest die aktuelle Version aus `Cargo.toml`
        .build()?
        .update()?;

    if status.updated() {
        println!("{}", Green.paint("update successful!"));
    } else {
        println!("{}", Green.paint("GXShell is already up to date!"));
    }
    
    Ok(())
}
