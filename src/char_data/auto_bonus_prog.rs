use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Default, Deserialize, PartialEq, Eq)]
pub struct AbpData {
    pub attack_pot : i32,
    #[serde(default)]
    pub skill_pot : HashMap<String, i32>,
    pub def_pot : i32,
    pub attack_dice : i32,
}