use std;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MainStat{
    id: String,
    name: String,
    abbrv: String,
    pub value: i32
}

impl MainStat{
    pub fn new(id: &str, name: &str, abbrv: &str, val: i32) -> MainStat {
        MainStat{
            id: String::from(id),
            name: String::from(name),
            abbrv: String::from(abbrv),
            value: val
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_abbr(&self) -> &str {
        &self.abbrv
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }
}

impl std::cmp::PartialEq for MainStat {
    // Required method
    fn eq(&self, other: &Self) -> bool {
        return self.id == other.id && self.abbrv == other.abbrv && self.value == other.value;
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MainStats {
    pub strength: MainStat,
    pub dexterity: MainStat,
    pub constitution: MainStat,
    pub wisdom: MainStat,
    pub intelligence: MainStat,
    pub charisma: MainStat,
}

impl MainStats {
    pub fn new() -> MainStats{
        MainStats { 
            strength: MainStat::new("str", "Strength", "Str", 0),
            dexterity: MainStat::new("dex", "Dexterity", "Dex", 0),
            constitution: MainStat::new("con", "Constitution", "Con", 0),
            wisdom: MainStat::new("wis", "Wisdom", "Wis", 0),
            intelligence: MainStat::new("int", "Intelligence", "Int", 0),
            charisma: MainStat::new("char", "Charisma", "Cha", 0),
         }
    }

    pub fn as_vec(&self) -> Vec<MainStat> {
        vec![
            self.strength.clone(),
            self.dexterity.clone(),
            self.constitution.clone(),
            self.wisdom.clone(),
            self.intelligence.clone(),
            self.charisma.clone()
        ]
    }

    fn mut_vec(&mut self) -> Vec<&mut MainStat>{
        vec![
            &mut self.strength,
            &mut self.dexterity,
            &mut self.constitution,
            &mut self.wisdom,
            &mut self.intelligence,
            &mut self.charisma
        ]
    } 

    pub fn set_stat(&mut self, id: &str, value: i32){
        for s in self.mut_vec() {
            if s.get_id() == id {
                s.value = value;
            }
        }
    }

    pub fn get_stat(&self, id: &str) -> std::option::Option::<MainStat> {
        for s in self.as_vec() {
            if s.get_id() == id {
                return std::option::Option::Some(s.clone());
            }
        }
        return None;
    }

    pub fn get_stat_val(&self, id: &str) -> std::option::Option::<i32> {
        for s in self.as_vec() {
            if s.get_id() == id {
                return std::option::Option::Some(s.value);
            }
        }
        return None;
    }
}



#[derive(Debug, Clone)]
pub struct DependentStat{

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Character {
    pub name: String,
    pub level: i32,
    pub main_stats: MainStats,
    pub background: String,
    pub class: String,
}

impl Character {
    pub fn new(name: &str) -> Character {
        Character {
            name: String::from(name),
            level: 1,
            main_stats: MainStats::new(),
            background: String::from("Squire"),
            class: String::from("Commander"),
        }
    }
}
