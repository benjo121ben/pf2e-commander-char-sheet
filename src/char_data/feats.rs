use serde::{Deserialize, Serialize};

use super::traits::Trait;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FeatType{
    General,
    Ancestry,
    Skill,
    Class
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CharAdditionType{
    ProfIncrease,
    ProfSet,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharAddition {

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Feat{
    pub name: String,
    pub actions: i32, 
    pub feat_types: Vec<FeatType>,
    pub traits: Vec<Trait>,
    pub description: String,
    #[serde(default)]
    pub char_additions: Vec<CharAddition>
}