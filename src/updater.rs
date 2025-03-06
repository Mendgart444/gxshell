use reqwest;
use semver::Version;
use nu_ansi_term::Color::{Green, Red};
use std::error::Error;



const UPDATE_URL:&str = "https://mendgart444.github.io/gxweb/gxshell_version.txt";

const CURRENT_VERSION:&str = "0.1.0";

pub async fn check_for_updates() {
    match get_latest_version().await {
        Ok(lastest_version) => {
            let current_version = Version::parse(CURRENT_VERSION).unwrap_or_else(|_| Version::new(0,0,0));

            if lastest_version > current_version {
                println!("{}", Red.paint(format!("Update available lastest version: {}", lastest_version)));
                println!("{}", Green.paint("Updating..."));
            } else {
                println!("{}", Green.paint("Lastest version is installed."));
            }

        }
        Err(e) => println!("{}", Red.paint(format!("Error update check faild: {}", e))),
    }
}

async fn get_latest_version() -> Result<Version, Box<dyn Error>> {
    let response = reqwest::get(UPDATE_URL).await?.text().await?;
    let version = Version::parse(response.trim())?;
    Ok(version)
}