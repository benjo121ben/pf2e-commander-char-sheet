use std;
use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Deserialize, Serialize)]
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


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MainStats {
    pub strength: MainStat,
    pub dexterity: MainStat,
    pub constitution: MainStat,
    pub wisdom: MainStat,
    pub intelligence: MainStat,
    pub charisma: MainStat,
}

impl MainStats {
    pub fn new (str: i32, dex: i32, con: i32, int: i32, wis: i32, cha: i32) -> MainStats{
        MainStats { 
            strength: MainStat::new("str", "Strength", "Str", str),
            dexterity: MainStat::new("dex", "Dexterity", "Dex", dex),
            constitution: MainStat::new("con", "Constitution", "Con", con),
            intelligence: MainStat::new("int", "Intelligence", "Int", int),
            wisdom: MainStat::new("wis", "Wisdom", "Wis", wis),
            charisma: MainStat::new("char", "Charisma", "Cha", cha),
         }
    }

    pub fn zero() -> MainStats{
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