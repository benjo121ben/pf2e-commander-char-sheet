use leptos::*;

use leptos::logging::log;
use crate::server_side::read_json::{read_user_from_file, write_char_to_file};
use crate::char_data::character::{Character, MainStats};

#[server(GetChar, "/api")]
pub async fn get_char() -> Result<Character, ServerFnError> {
    let ketra = Character::new(
        "Ketrania Valenzia Adriaste Uth Viharin VII", 
        MainStats::new(3, 1, 1, 3, 0, 1)
    );
    let test = read_user_from_file("test.json");
    println!("{test:?}");
    return Ok(ketra);
}

#[server(SetChar, "/api")]
pub async fn set_char(char: Character) -> Result<i32, ServerFnError> {
    log!("Arrived {char:?}");
    let result = write_char_to_file("char.json", &char);
    match result {
        Ok(_) => Ok(0),
        Err(error) => Err(ServerFnError::new(error)),
    }
    
}
