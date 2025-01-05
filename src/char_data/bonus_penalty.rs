use std::{collections::HashMap, hash::Hash, vec};

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CalcType {
    Fixed,
    Calculated,
    ConditionValue
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum BonusPenaltyType {
    Circumstance,
    Status,
    Item,
    Untyped
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BonusPenalty {
    pub selector: Vec<String>,
    pub calculation: BonusPenaltyCalculation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BonusPenaltyCalculation {
    pub calc_type: CalcType,
    pub value: Option<i32>,
    pub penalty_type: BonusPenaltyType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatBonusPenalties {
    pub selector: String,
    pub bonuses: HashMap<BonusPenaltyType, i32>,
    pub penalties: HashMap<BonusPenaltyType, i32>
}

impl StatBonusPenalties {
    pub fn new() -> Self {
        StatBonusPenalties {
            selector: "".to_string(),
            bonuses: HashMap::new(),
            penalties: HashMap::new(),
        }
    }

    pub fn from(selector: &str, bp_type: BonusPenaltyType, value: i32) -> Self {
        let mut ret = StatBonusPenalties::new();
        ret.selector = selector.to_string();
        ret.add_bonus_penalty(&bp_type, value);

        return ret;
    }

    pub fn calculate_total(self: &Self) -> i32 {
        let bonus_total = self.bonuses.values().fold(
            0,
            |acc:i32, value:&i32| acc + *value
        );

        return self.penalties.values().fold(bonus_total, |acc:i32, value:&i32| acc + *value)
    }

    pub fn add_bonus_penalty(self: &mut Self, bp_type: &BonusPenaltyType, value: i32) {
        if value < 0 {
            self.add_penalty(bp_type, value);
        }
        else if value > 0 {
            self.add_bonus(bp_type, value);
        }
    }

    fn add_bonus(self: &mut Self, bp_type: &BonusPenaltyType, value: i32) {
        let existing_val_option = self.bonuses.get_mut(&bp_type);
        match existing_val_option {
            Some(existing_val) => {
                *existing_val = std::cmp::max(*existing_val, value);
            },
            None => {
                self.bonuses.insert(bp_type.clone(), value);
            },
        }
    }

    fn add_penalty(self: &mut Self, bp_type: &BonusPenaltyType, value: i32) {
        let existing_val_option = self.penalties.get_mut(&bp_type);
        match existing_val_option {
            Some(existing_val) => {
                *existing_val = std::cmp::min(*existing_val, value);
            },
            None => {
                self.penalties.insert(bp_type.clone(), value);
            },
        }
    }
}

impl BonusPenaltyCalculation {
    pub fn calculate_bonus_penalty_value (self: &Self, character_level: i32, condition_value: Option<i32>) -> Result <i32, String> {
        match self.calc_type {
            super::bonus_penalty::CalcType::Fixed => {
                self.value.ok_or(format!("expected value for fixed bonus penalty calculation for"))
            },
            super::bonus_penalty::CalcType::Calculated => {
                //this at the moment is just for the drained condition, which needs to be calculated using the character level
                let condition_value = condition_value.ok_or(format!("A calculated penalty requires the outer value to be present"))?;
                return Ok(-condition_value * character_level);
            },
            super::bonus_penalty::CalcType::ConditionValue => {
                return condition_value.map(|cond_val| -cond_val).ok_or(format!("A ConditionValue penalty requires the outer value to be present"))
            },
        }
    }
}

pub fn combine_selected_bonus_penalties(penalty_map: &HashMap<String, StatBonusPenalties>, selectors: &Vec<String>) -> StatBonusPenalties {
    let mut bp_vector = vec![];
    selectors.iter().map(|select| penalty_map.get(select)).for_each(|stat_bp_option|{
        if stat_bp_option.is_some() {
            bp_vector.push(stat_bp_option.unwrap().clone());
        }
    });
    return combine_stat_bonus_penalties(&bp_vector);
}

fn combine_stat_bonus_penalties(list_of_affectors: &Vec<StatBonusPenalties>) -> StatBonusPenalties {
    let mut ret = StatBonusPenalties::new();
    list_of_affectors.iter().for_each(|bp|{
        for (key, val) in bp.bonuses.iter() {
            ret.add_bonus_penalty(&key, *val);
        }

        for (key, val) in bp.penalties.iter() {
            ret.add_bonus_penalty(&key, *val);
        }
    });
    ret
}
