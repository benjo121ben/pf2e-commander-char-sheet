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
    pub weap_info: Option<WeaponInfo>
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WeaponInfo {
    pub damage: i32,
    pub w_type: WeaponType,
    pub d_type: String
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum WeaponType {
    Melee,
    Ranged
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GearType {
    Weapon,
    Armor,
    Gear
}