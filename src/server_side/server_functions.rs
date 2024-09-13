use leptos::*;

use leptos::logging::log;
use crate::server_side::read_json::{read_char_from_file, write_char_to_file};
use crate::char_data::character::Character;

#[server(GetChar, "/api")]
pub async fn get_char() -> Result<Character, ServerFnError> {
    let read_char_result = read_char_from_file("char.json");
    match read_char_result {
        Ok(read_char) => return Ok(read_char),
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
