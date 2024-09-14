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
    name: String,
    actions: i32, 
    feat_types: Vec<FeatType>,
    traits: Vec<Trait>,
    description: String,
    #[serde(default)]
    char_additions: Vec<CharAddition>
}