use serde::{Serialize, Deserialize};

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Condition {
    pub name: String,
    pub description: String,
    #[serde(rename = "override")]
    #[serde(default)]
    pub override_field: Vec<String>,
    #[serde(default)]
    pub forced_conditions: Vec<ForcedCondition>,
    #[serde(default)]
    pub penalty: Vec<Penalty>,
    #[serde(default)]
    pub has_value: bool,
    #[serde(default)]
    pub added_on_remove: Vec<String>,
    pub increase_on_gain_by: Option<String>,
    #[serde(default)]
    pub added_on_gain: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ForcedCondition {
    pub name: String,
    pub value: Option<i32>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Penalty {
    pub selector: Vec<String>,
    pub amount: Option<Amount>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Amount {
    #[serde(rename = "calc_type")]
    pub calc_type: String,
    pub value: Option<i32>,
    #[serde(rename = "penalty_type")]
    pub penalty_type: Option<String>,
}

