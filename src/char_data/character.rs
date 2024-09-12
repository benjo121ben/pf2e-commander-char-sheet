use std;
use serde::{Deserialize, Serialize};
use crate::char_data::stats::MainStats;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Character {
    pub name: String,
    pub level: i32,
    pub main_stats: MainStats,
    pub background: String,
    pub class: String,
}

impl Character {
    pub fn new(name: &str, main_stats: MainStats) -> Character {
        Character {
            name: String::from(name),
            level: 1,
            main_stats: main_stats,
            background: String::from("Squire"),
            class: String::from("Commander"),
        }
    }
}
