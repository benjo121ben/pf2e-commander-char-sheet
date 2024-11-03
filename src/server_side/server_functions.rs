use leptos::*;

use crate::char_data::conditions::Condition;
use crate::char_data::feats::Feat;
use crate::server_side::read_json::{read_char_from_file, write_char_to_file, read_vector_from_file};
use crate::char_data::character::Character;
use std::collections::HashMap;

use super::read_json::read_map_from_file;

#[server(GetChar, "/api", "GetJson", "get_char")]
pub async fn get_char() -> Result<Character, ServerFnError> {
    let read_char_result = read_char_from_file("saves/char.json");
    match read_char_result {
        Ok(read_char) => return Ok(read_char),
        Err(error) => return Err(ServerFnError::new(error.to_string())),
    }
}

#[server(SetChar, "/api", "Url", "set_char")]
pub async fn set_char(char: Character) -> Result<i32, ServerFnError> {
    let result = write_char_to_file("saves/char.json", &char);
    match result {
        Ok(_) => Ok(0),
        Err(error) => Err(ServerFnError::new(error)),
    }
}


#[server(GetConditions, "/api", "GetJson", "conditions")]
pub async fn get_conditions() -> Result<Vec<Condition>, ServerFnError> {
    let read_cond_result = read_vector_from_file::<Condition,_>("resources/conditions.json", "Condition");
    match read_cond_result {
        Ok(conditions) => return Ok(conditions),
        Err(error) => return Err(ServerFnError::new(error.to_string())),
    }
}

#[server(GetFeats, "/api", "GetJson", "feats")]
pub async fn get_feats() -> Result<Vec<Feat>, ServerFnError> {
    let read_feat_result = read_vector_from_file::<Feat,_>("resources/feats.json", "Feat");
    match read_feat_result {
        Ok(feats) => return Ok(feats),
        Err(error) => return Err(ServerFnError::new(error.to_string())),
    }
}

#[server(GetTraits, "/api", "GetJson", "traits")]
pub async fn get_traits() -> Result<HashMap<String, String>, ServerFnError> {
    let read_trait_result = read_map_from_file::<String, String, _>("resources/traits.json", "Feat");
    match read_trait_result {
        Ok(traits) => return Ok(traits),
        Err(error) => return Err(ServerFnError::new(error.to_string())),
    }
}


#[server(PingServer, "/api", "GetJson", "ping")]
pub async fn ping_server() -> Result<i32, ServerFnError> {
    Ok(0)
}