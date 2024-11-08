use std::collections::HashMap;

use leptos::logging::log;
use serde::{Serialize, Deserialize};

use super::{bonus_penalty::BonusPenaltyAmount, character::Character};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FullConditionView {
    pub level: Option<i32>,
    pub name: String,
    pub active: bool,
    pub forced: bool,
    pub condition_data: ConditionData
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CharacterConditionInfo {
    pub level: Option<i32>,
    pub name: String,
    pub active: bool
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConditionData {
    pub name: String,

    pub description: String,

    #[serde(default)]
    pub override_other: Vec<String>,

    #[serde(default)]
    pub forced_conditions: Vec<ForcedConditionEntry>,

    #[serde(default)]
    pub bonus_penalty: Vec<ConditionBonusPenalty>,

    #[serde(default)]
    pub has_value: bool,

    #[serde(default)]
    pub added_on_remove: Vec<String>,

    pub increase_on_gain_by: Option<String>,

    #[serde(default)]
    pub added_on_gain: Vec<String>,
}

#[derive( Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ForcedConditionEntry {
    pub name: String,
    pub value: Option<i32>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConditionBonusPenalty {
    pub selector: Vec<String>,
    pub amount: Option<BonusPenaltyAmount>,
}

fn get_max_opt_level(opt1:Option<i32>, opt2:Option<i32>) -> Option<i32>{
    let val = match (opt1, opt2) {
        (Some(val1), Some(val2)) => Some(std::cmp::max(val1, val2)),
        _ => None
    };
    val
}

pub fn get_condition_data(condition_name: &str, condition_data_map: &HashMap<String, ConditionData>) -> Result<ConditionData, String>{
    match condition_data_map.get(condition_name) {
        Some(found_condition) => Ok(found_condition.clone()),
        None => Err(format!("conditions::get_condition_data: could not find condition '{condition_name}' inside json data map"))
    }
}

fn add_full_to_hash_map (full_cond_map: &mut HashMap<String, FullConditionView>, insert_obj: &FullConditionView) {
    match full_cond_map.get_mut(&insert_obj.name) {
        Some(present_cond) => {
            present_cond.level = get_max_opt_level(present_cond.level, insert_obj.level);
            present_cond.forced = insert_obj.forced || present_cond.forced;
            present_cond.active = insert_obj.active || present_cond.active;
        },
        None => {
            full_cond_map.insert(insert_obj.name.clone(), insert_obj.clone());
        }
    }
}



impl Character {
    pub fn get_all_conditions(self: & Self, condition_data_map: &HashMap<String, ConditionData>) -> Result<Vec<FullConditionView>, String>{
        let mut full_char_conditions: HashMap<String, FullConditionView> = HashMap::new();

        for condition in self.conditions.clone() {
            let condition_data = get_condition_data(&condition.name, condition_data_map)?;
            let insert_obj = FullConditionView {
                level: condition.level,
                name: condition.name.clone(),
                active: condition.active,
                forced: false,
                condition_data: condition_data.clone(),
            };
            add_full_to_hash_map(&mut full_char_conditions, &insert_obj);

            for forced_condition in condition_data.forced_conditions {
                let forced_condition_data = get_condition_data(&condition.name, condition_data_map)?;
                let insert_obj = FullConditionView {
                    level: forced_condition.value,
                    name: forced_condition.name,
                    active: false,
                    forced: true,
                    condition_data: forced_condition_data,
                };
                add_full_to_hash_map(&mut full_char_conditions, &insert_obj);
            }
        }
        
        let mut ret_vec = vec![];
        for (_, condition_view) in full_char_conditions {
            ret_vec.push(
                condition_view
            );
        }
        let size = ret_vec.len();
        log!("conditions calculated {size}, {ret_vec:#?}");
        return Ok(ret_vec);
    }
}