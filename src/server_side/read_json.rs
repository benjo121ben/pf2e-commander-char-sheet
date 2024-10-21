use leptos::logging::log;
use serde::de::DeserializeOwned;
use std::hash::Hash;
use std::{collections::HashMap, error::Error};
use std::fs::read_to_string;
use std::path::Path;
use crate::char_data::character::{Character, SimpleCharacter};


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
            let errorstring = format!("There was an issue locating the path, this might be due to accessing rights. Cannot confirm or deny existence:\n{error}");
            log!("{errorstring}");
            return Err(Box::from(error));
        },
    }
    

}

pub fn read_vector_from_file<T: DeserializeOwned, P: AsRef<Path>>(path: P, debug_name: &str) -> Result<Vec<T>, Box<dyn Error>> {

    // Open the file in read-only mode with buffer.
    let check_file_path_result = std::fs::exists(&path);
    match check_file_path_result {
        Ok(exists) => {
            if exists {
                let file_str = read_to_string(&path)?;
                let objects: Vec<T> = serde_json::from_str(&file_str)?;
                return Ok(objects);
            }
            else {
                let errorstring = format!("Filepath does not exist: Cannot build {debug_name}");
                log!("{errorstring}");
                return Err(Box::from(errorstring));
            }
        },
        Err(error) => {
            let errorstring = format!("There was an issue locating the path, this might be due to accessing rights. Cannot confirm or deny existence:\n{error}");
            log!("{errorstring}");
            return Err(Box::from(error));
        },
    }
    

}

pub fn read_map_from_file<T: DeserializeOwned + Eq + Hash, D: DeserializeOwned + Eq + Hash, P: AsRef<Path>>(path: P, debug_name: &str) -> Result<HashMap<T, D>, Box<dyn Error>> {

    // Open the file in read-only mode with buffer.
    let check_file_path_result = std::fs::exists(&path);
    match check_file_path_result {
        Ok(exists) => {
            if exists {
                let file_str = read_to_string(&path)?;
                let objects: HashMap<T, D> = serde_json::from_str(&file_str)?;
                return Ok(objects);
            }
            else {
                let errorstring = format!("Filepath does not exist: Cannot build {debug_name}");
                log!("{errorstring}");
                return Err(Box::from(errorstring));
            }
        },
        Err(error) => {
            let errorstring = format!("There was an issue locating the path, this might be due to accessing rights. Cannot confirm or deny existence:\n{error}");
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