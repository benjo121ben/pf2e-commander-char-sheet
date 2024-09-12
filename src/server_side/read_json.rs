use serde::Deserialize;
use leptos::logging::log;
use std::error::Error;
use std::fs::read_to_string;
use std::path::Path;
use crate::char_data::character::{Character, MainStats};

#[derive(Deserialize, Debug)]
pub struct User {
    fingerprint: String,
    location: String,
}

pub fn read_user_from_file<P: AsRef<Path>>(path: P) -> Result<Character, Box<dyn Error>> {
    // Open the file in read-only mode with buffer.
    log!("T0");
    let check_file_path_result = std::fs::exists(&path);
    match check_file_path_result {
        Ok(exists) => {
            if exists {
                let file_str = read_to_string(&path)?;
                log!("T1");
                let character = serde_json::from_str(&file_str)?;
                log!("T3");
                return Ok(character);
            }
            else {
                log!("Filepath does not exist, retun new char");
                return Ok(Character::new("", MainStats::zero()));
            }
        },
        Err(error) => {
            let errorstring = String::from(format!("There was an issue locating the path, this might be due to accessing rights. Cannot confirm or deny existence:\n{error}"));
            log!("{errorstring}");
            return Err(Box::from(error));
        },
    }
    

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