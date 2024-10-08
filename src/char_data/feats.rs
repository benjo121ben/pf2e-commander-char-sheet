use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum FeatType{
    General,
    Ancestry,
    Skill,
    Class
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum CharAdditionType{
    ProfIncrease,
    ProfSet,

}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct CharAddition {

}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct Feat{
    pub name: String,
    pub actions: i32, 
    pub feat_types: Vec<FeatType>,
    pub traits: Vec<String>,
    pub description: String,
    #[serde(default)]
    pub char_additions: Vec<CharAddition>
}