use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProficiencyLevel{
    Untrained,
    Half,
    Trained,
    Expert, 
    Master,
    Legendary
}

impl ProficiencyLevel {
    pub fn get_bonus(&self, level: i32) -> i32{
        match *self {
            ProficiencyLevel::Untrained => 0,
            ProficiencyLevel::Half => f64::floor(level as f64/2.0f64) as i32,
            ProficiencyLevel::Trained => level + 2,
            ProficiencyLevel::Expert => level + 4,
            ProficiencyLevel::Master => level + 6,
            ProficiencyLevel::Legendary => level + 8,
        }
    }
}

impl fmt::Display for ProficiencyLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}