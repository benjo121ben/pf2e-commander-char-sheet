use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BonusPenaltyCalcType {
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
    pub amount: BonusPenaltyAmount,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BonusPenaltyAmount {
    pub calc_type: BonusPenaltyCalcType,
    pub value: Option<i32>,
    pub penalty_type: Option<BonusPenaltyType>,
}

