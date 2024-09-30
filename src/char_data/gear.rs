use serde::{Deserialize, Serialize};



#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Gear {
    pub name: String,
    pub g_type: GearType,
    #[serde(default)]
    pub traits: Vec<String>,
    pub proficiency: Option<String>,
    pub invested: Option<bool>,
    pub description: String,
    pub damage: Option<i32>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GearType {
    Weapon,
    Armor,
    Gear
}