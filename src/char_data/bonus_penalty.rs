use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CalcType {
    Fixed,
    Calculated,
    Level
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BonusPenaltyType {
    Circumstance,
    Status,
    Item,
    Untyped
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BonusPenalty {
    pub selector: Vec<String>,
    pub calculation: BonusPenaltyCalculation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BonusPenaltyCalculation {
    pub calc_type: CalcType,
    pub value: Option<i32>,
    pub penalty_type: Option<BonusPenaltyType>,
}

