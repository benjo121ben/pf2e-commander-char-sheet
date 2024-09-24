use std::collections::HashMap;

use leptos::*;

use crate::char_data::conditions::Condition;
use crate::server_side::read_json::{read_char_from_file, write_char_to_file, read_traits_from_file};
use crate::char_data::character::Character;

use super::read_json::read_conditions_from_file;

#[server(GetChar, "/api")]
pub async fn get_char() -> Result<Character, ServerFnError> {
    let read_char_result = read_char_from_file("char.json");
    match read_char_result {
        Ok(read_char) => return Ok(read_char),
        Err(error) => return Err(ServerFnError::new(error.to_string())),
    }
}

#[server(GetConditions, "/api")]
pub async fn get_conditions() -> Result<Vec<Condition>, ServerFnError> {
    let read_cond_result = read_conditions_from_file("resources/conditions.json");
    match read_cond_result {
        Ok(conditions) => return Ok(conditions),
        Err(error) => return Err(ServerFnError::new(error.to_string())),
    }
}

#[server(GetTraits, "/api")]
pub async fn get_traits() -> Result<HashMap<String, String>, ServerFnError> {
    let read_trait_result = read_traits_from_file("resources/traits.json");
    match read_trait_result {
        Ok(traits) => return Ok(traits),
        Err(error) => return Err(ServerFnError::new(error.to_string())),
    }
}

#[server(SetChar, "/api")]
pub async fn set_char(char: Character) -> Result<i32, ServerFnError> {
    let result = write_char_to_file("char.json", &char);
    match result {
        Ok(_) => Ok(0),
        Err(error) => Err(ServerFnError::new(error)),
    }
}
