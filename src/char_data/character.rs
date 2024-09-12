use serde::{Deserialize, Serialize};
use super::stats::{MainStats, DependentStat};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Character {
    pub name: String,
    pub level: i32,
    pub main_stats: MainStats,
    pub background: String,
    pub class: String,
    pub skills: Vec<DependentStat>
}

impl Character {
    pub fn new(name: &str, main_stats: MainStats, skills: Vec<DependentStat>) -> Character {
        Character {
            name: String::from(name),
            level: 1,
            main_stats: main_stats,
            background: String::from("Squire"),
            class: String::from("Commander"),
            skills: skills
        }
    }

    pub fn zero() -> Character {
        Character {
            name: String::from(""),
            level: 1,
            main_stats: MainStats::zero(),
            background: String::from("Squire"),
            class: String::from("Commander"),
            skills: vec![
                DependentStat::default_skill("dex", "Acrobatics"),
                DependentStat::default_skill("int", "Arcana"),
                DependentStat::default_skill("str", "Athletics"),
                DependentStat::default_skill("int", "Crafting"),
                DependentStat::default_skill("cha", "Deception"),
                DependentStat::default_skill("cha", "Diplomacy"),
                DependentStat::default_skill("cha", "Intimidation"),
                DependentStat::default_skill("wis", "Medicine"),
                DependentStat::default_skill("wis", "Nature"),
                DependentStat::default_skill("int", "Occultism"),
                DependentStat::default_skill("cha", "Performance"),
                DependentStat::default_skill("wis", "Religion"),
                DependentStat::default_skill("int", "Society"),
                DependentStat::default_skill("dex", "Stealth"),
                DependentStat::default_skill("wis", "Survival"),
                DependentStat::default_skill("dex", "Thievery"),
            ]
        }
    }
}
