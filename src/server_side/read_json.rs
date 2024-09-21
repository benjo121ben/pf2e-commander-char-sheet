use leptos::logging::log;
use std::error::Error;
use std::fs::read_to_string;
use std::path::Path;
use crate::char_data::{character::{Character, SimpleCharacter}, conditions::Condition};


pub fn read_char_from_file<P: AsRef<Path>>(path: P) -> Result<Character, Box<dyn Error>> {

    // Open the file in read-only mode with buffer.
    let check_file_path_result = std::fs::exists(&path);
    match check_file_path_result {
        Ok(exists) => {
            if exists {
                let file_str = read_to_string(&path)?;
                let character: SimpleCharacter = serde_json::from_str(&file_str)?;
                return Ok(Character::from(character));
            }
            else {
                log!("Filepath does not exist, return new char");
                return Ok(Character::zero());
            }
        },
        Err(error) => {
            let errorstring = String::from(format!("There was an issue locating the path, this might be due to accessing rights. Cannot confirm or deny existence:\n{error}"));
            log!("{errorstring}");
            return Err(Box::from(error));
        },
    }
    

}

pub fn read_conditions_from_file<P: AsRef<Path>>(path: P) -> Result<Vec<Condition>, Box<dyn Error>> {

    // Open the file in read-only mode with buffer.
    let check_file_path_result = std::fs::exists(&path);
    match check_file_path_result {
        Ok(exists) => {
            if exists {
                let file_str = read_to_string(&path)?;
                let conditions: Vec<Condition> = serde_json::from_str(&file_str)?;
                return Ok(conditions);
            }
            else {
                let errorstring = String::from(format!("Filepath does not exist: Cannot build Conditions"));
                log!("{errorstring}");
                return Err(Box::from(errorstring));
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
    match serde_json::to_string_pretty(&SimpleCharacter::from(character)) {
        Ok(json) => {
            match std::fs::write(path, json) {
                Ok(_) => {
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