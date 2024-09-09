use serde::Deserialize;
use leptos::logging::log;
use std::error::Error;
use std::fs::read_to_string;
use std::io::BufReader;
use std::path::Path;

#[derive(Deserialize, Debug)]
pub struct User {
    fingerprint: String,
    location: String,
}

pub async fn read_user_from_file<P: AsRef<Path>>(path: P) -> Result<User, Box<dyn Error>> {
    // Open the file in read-only mode with buffer.
    log!("T0");
    let file = await read_to_string(path)?;
    log!("T1");
    
    // Read the JSON contents of the file as an instance of `User`.
    let u = serde_json::from_str(&file)?;
    log!("T3");

    // Return the `User`.
    Ok(u)
}