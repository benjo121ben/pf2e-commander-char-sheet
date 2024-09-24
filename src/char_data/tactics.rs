use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tactic {
    pub name: String,
    pub actions: i64,
    pub selected: bool,
    pub traits: Vec<String>,
    pub description: String,
}