use serde::{Serialize, Deserialize};

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Condition {
    pub name: String,
    pub description: String,
    #[serde(rename = "override")]
    #[serde(default)]
    pub override_field: Vec<String>,
    #[serde(rename = "forced_conditions")]
    #[serde(default)]
    pub forced_conditions: Vec<ForcedCondition>,
    #[serde(default)]
    pub penalty: Vec<Penalty>,
    #[serde(rename = "has_value")]
    #[serde(default)]
    pub has_value: bool,
    #[serde(rename = "added_on_remove")]
    #[serde(default)]
    pub added_on_remove: Vec<String>,
    #[serde(rename = "increase_on_gain_by")]
    pub increase_on_gain_by: Option<String>,
    #[serde(rename = "added_on_gain")]
    #[serde(default)]
    pub added_on_gain: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ForcedCondition {
    pub name: String,
    pub value: Option<i32>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Penalty {
    pub selector: Vec<String>,
    pub amount: Option<Amount>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Amount {
    #[serde(rename = "calc_type")]
    pub calc_type: String,
    pub value: Option<i32>,
    #[serde(rename = "penalty_type")]
    pub penalty_type: Option<String>,
}

