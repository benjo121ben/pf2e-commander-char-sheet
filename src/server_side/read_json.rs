use serde::Deserialize;
use leptos::logging::log;
use std::error::Error;
use std::fs::{read_to_string, };
use std::io::BufReader;
use std::path::Path;
use crate::char_data::character::Character;

#[derive(Deserialize, Debug)]
pub struct User {
    fingerprint: String,
    location: String,
}

pub fn read_user_from_file<P: AsRef<Path>>(path: P) -> Result<User, Box<dyn Error>> {
    // Open the file in read-only mode with buffer.
    log!("T0");
    let file_str = read_to_string(path)?;
    log!("T1");
    
    // Read the JSON contents of the file as an instance of `User`.
    let u = serde_json::from_str(&file_str)?;
    log!("T3");

    // Return the `User`.
    Ok(u)
}

pub fn write_char_to_file<P: AsRef<Path>>(path: P, character: &Character) -> Result<(), Box<dyn Error>>{
    match serde_json::to_string_pretty(character) {
        Ok(json) => {
            match std::fs::write(path, json) {
                Ok(_) => {
                    println!("Save complete");
                    return Ok(());
                },
                Err(error) => {
                    println!("Error occurred during File writing: {error}");
                    return Err(Box::new(error));
                },
            }
        },
        Err(error) => { 
            println!("Error occurred during Serialization {error}");
            return Err(Box::new(error));
        }
    };
}