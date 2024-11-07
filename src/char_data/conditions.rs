use serde::{Serialize, Deserialize};

use super::bonus_penalty::BonusPenalty;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Condition {
    pub name: String,

    pub description: String,

    #[serde(default)]
    pub override_other: Vec<String>,

    #[serde(default)]
    pub forced_conditions: Vec<ForcedCondition>,

    #[serde(default)]
    pub bonus_penalty: Vec<BonusPenalty>,

    #[serde(default)]
    pub has_value: bool,

    #[serde(default)]
    pub added_on_remove: Vec<String>,

    pub increase_on_gain_by: Option<String>,

    #[serde(default)]
    pub added_on_gain: Vec<String>,
}

#[derive( Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ForcedCondition {
    pub name: String,
    pub value: Option<i32>,
}