use leptos::*;

use crate::char_data::conditions::ConditionData;
use crate::char_data::feats::Feat;
use crate::server_side::read_json::{read_char_from_file, write_char_to_file, read_vector_from_file};
use crate::char_data::character::Character;
use std::collections::HashMap;
use std::process::Command;

use super::read_json::read_map_from_file;

#[server(GetChar, "/api", "GetJson", "get_char")]
pub async fn get_char() -> Result<Character, ServerFnError> {
    let read_char_result = read_char_from_file("saves/char.json");
    read_char_result.or_else(|error|{
        Err(ServerFnError::new(error.to_string()))
    })
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
pub async fn get_conditions() -> Result<Vec<ConditionData>, ServerFnError> {
    let read_cond_result = read_vector_from_file::<ConditionData,_>("resources/conditions.json", "Condition");
    read_cond_result.or_else(|error|{
        Err(ServerFnError::new(error.to_string()))
    })
}

#[server(GetFeats, "/api", "GetJson", "feats")]
pub async fn get_feats() -> Result<Vec<Feat>, ServerFnError> {
    let read_feat_result = read_vector_from_file::<Feat,_>("resources/feats.json", "Feat");
    read_feat_result.or_else(|error|{
        Err(ServerFnError::new(error.to_string()))
    })
}

#[server(GetTraits, "/api", "GetJson", "traits")]
pub async fn get_traits() -> Result<HashMap<String, String>, ServerFnError> {
    let read_trait_result = read_map_from_file::<String, String, _>("resources/traits.json", "Feat");
    read_trait_result.or_else(|error|{
        Err(ServerFnError::new(error.to_string()))
    })
}


#[server(PingServer, "/api", "GetJson", "ping")]
pub async fn ping_server() -> Result<i32, ServerFnError> {
    Ok(0)
}

#[server(StartV11, "/api", "GetJson", "start11")]
pub async fn start11() -> Result<i32, ServerFnError> {
    if cfg!(target_os = "linux") {
        let mut command = Command::new("bash");

        // Pass the script name as an argument
        command.arg("/home/benji-pi/public_scripts/start_F11.sh");

        // Execute the command
        // This will download a file called ncbi_dataset.zip in the current directory
        let _ = command.output();
    }
    Ok(0)
}

#[server(StartV12, "/api", "GetJson", "start12")]
pub async fn start12() -> Result<i32, ServerFnError> {
    if cfg!(target_os = "linux") {
        
        let mut command = Command::new("bash");

        // Pass the script name as an argument
        command.arg("/home/benji-pi/public_scripts/start_F12.sh");

        // Execute the command
        // This will download a file called ncbi_dataset.zip in the current directory
        let _ = command.output();
    }
    Ok(0)
}