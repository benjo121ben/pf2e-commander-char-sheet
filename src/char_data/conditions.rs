use std::{cmp::Ordering, collections::HashMap};

use leptos::logging::log;
use serde::{Serialize, Deserialize};

use super::{bonus_penalty::BonusPenalty, character::Character};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FullConditionView {
    pub level: Option<i32>,
    pub name: String,
    pub active: bool,
    pub on_sheet: bool,
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
    pub bonus_penalty: Vec<BonusPenalty>,

    #[serde(default)]
    pub has_value: bool,

    #[serde(default)]
    pub added_on_remove: Vec<String>,

    pub increase_on_gain_by: Option<String>
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ForcedConditionEntry {
    pub name: String,
    pub value: Option<i32>,
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

pub fn get_condition_base_lvl (condition: &ConditionData, character: &Character) -> Option<i32>{
    if !condition.has_value {
        return None;
    }
    //this is relevant for conditions like dying that are increased by other conditions
    return condition.increase_on_gain_by.as_ref().and_then(|other_condition| {
        character.get_condition_lvl(&other_condition, true)
        .and_then(|lvl| Some(lvl + 1))
    }).or(Some(1))
}

fn add_full_to_hash_map (full_cond_map: &mut HashMap<String, FullConditionView>, insert_obj: &FullConditionView) {
    match full_cond_map.get_mut(&insert_obj.name) {
        Some(present_cond) => {
            present_cond.level = get_max_opt_level(present_cond.level, insert_obj.level);
            present_cond.forced = insert_obj.forced || present_cond.forced;
            present_cond.active = insert_obj.active || present_cond.active;
            present_cond.on_sheet = insert_obj.on_sheet || present_cond.on_sheet;
        },
        None => {
            full_cond_map.insert(insert_obj.name.clone(), insert_obj.clone());
        }
    }
}

fn add_forced_to_hash_map(condition_data_map: &HashMap<String, ConditionData>, full_cond_map: &mut HashMap<String, FullConditionView>, forced_condition: &ForcedConditionEntry) -> Result<(), String> {
    let forced_condition_data = get_condition_data(&forced_condition.name, condition_data_map)?;
    let insert_obj = FullConditionView {
        level: forced_condition.value,
        name: forced_condition.name.clone(),
        active: true,
        forced: true,
        condition_data: forced_condition_data.clone(),
        on_sheet: false,
    };
    add_full_to_hash_map(full_cond_map, &insert_obj);
    for child_forced_data in forced_condition_data.forced_conditions {
        let res = add_forced_to_hash_map(condition_data_map, full_cond_map, &child_forced_data);
        if res.is_err() {
            return res;
        }
    }
    return Ok(());
    
}

fn get_ordering(value: i32) -> Ordering {
    if value == 0 {
        Ordering::Equal
    }
    else if value > 0 {
        Ordering::Greater
    }
    else {
        Ordering::Less
    }
}


impl Character {
    pub fn get_all_conditions(self: & Self, condition_data_map: &HashMap<String, ConditionData>) -> Result<Vec<FullConditionView>, String>{
        let mut full_char_conditions: HashMap<String, FullConditionView> = HashMap::new();
        let condition_cmp = move |a: &FullConditionView, b: &FullConditionView| {
            let sheet_sort = get_ordering(b.on_sheet as i32 - a.on_sheet as i32);
            if sheet_sort == Ordering::Equal {
                a.name.cmp(&b.name)
            }
            else {
                sheet_sort
            }
        };
        for condition in self.conditions.clone() {
            let condition_data = get_condition_data(&condition.name, condition_data_map)?;
            let insert_obj = FullConditionView {
                level: condition.level,
                name: condition.name.clone(),
                active: condition.active,
                forced: false,
                condition_data: condition_data.clone(),
                on_sheet: true,
            };
            add_full_to_hash_map(&mut full_char_conditions, &insert_obj);

            if !condition.active {
                continue;
            }

            for forced_condition in condition_data.forced_conditions {
                let res = add_forced_to_hash_map(condition_data_map, &mut full_char_conditions, &forced_condition);
                if res.is_err() {
                    return Err(res.unwrap_err());
                }
            }
            
        }
        
        let mut ret_vec = vec![];
        for (_, condition_view) in full_char_conditions {
            ret_vec.push(
                condition_view
            );
        }
        ret_vec.sort_by(|a, b| condition_cmp(a,b));
        return Ok(ret_vec);
    }

    fn get_condition(self: &Self, condition_name: &str) -> Option<(CharacterConditionInfo, usize)>{
        self.conditions.iter().position(move|cond_info| cond_info.name == condition_name).and_then(|pos| {
            return Some((self.conditions.get(pos).cloned().unwrap(), pos));
        })
    }

    pub fn remove_condition (self: &mut Self, cond_name: &str){
        match self.get_condition(cond_name) {
            Some((_, index)) => {self.conditions.remove(index);},
            None => {log!("could not remove condition with name {cond_name}");}
        }
    }

    fn get_condition_lvl(self: &Self, condition_name: &str, ignore_inactive: bool) -> Option<i32> {
        self.get_condition(condition_name).and_then(|(cond_info, _)| {
            if ignore_inactive && !cond_info.active {
                return None;
            }
            cond_info.level.and_then(|level| {
                return Some(level);
            })
        })
    }

    pub fn add_condition(self: &mut Self, conditions_map: &HashMap<String, ConditionData>, add_condition_name: &str, increase_lvl: bool) {
        match self.conditions.iter().position(move|cond_info| cond_info.name == add_condition_name) {
            Some(pos) => {
                let current_cond = self.conditions.get(pos).unwrap();
                if !increase_lvl && current_cond.active {
                    return;
                }
                else if increase_lvl && current_cond.active {
                    self.increase_condition(add_condition_name);
                    return;
                }
                else { 
                    //present but not activated, easiest version is to just delete it and replace it
                    self.conditions.remove(pos);
                }
            },
            None => {}
        }


        match conditions_map.get(add_condition_name) {
            Some(cond_data) => {
                self.conditions.push(CharacterConditionInfo {
                    level: get_condition_base_lvl(cond_data, self), 
                    name: add_condition_name.to_string(), 
                    active: true
                });
            },
            None => log!("no condition data found. Error")
        }
    }

    pub fn increase_condition(self: &mut Self, cond_name: &str,) {
        self.conditions.iter_mut()
            .filter(move|cond_info| cond_info.name == cond_name)
            .for_each(move|cond_info| {
                match cond_info.level {
                    Some(lvl) => cond_info.level = Some(lvl + 1),
                    None => {}
                }
            })
        ;
    }
}