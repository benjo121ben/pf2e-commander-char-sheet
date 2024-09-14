use serde::{Deserialize, Serialize};
use leptos::logging::log;
use super::{character::Character, proficiency::ProficiencyLevel};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Attribute{
    id: String,
    name: String,
    abbrv: String,
    pub value: i32
}

impl Attribute{
    pub fn new(id: &str, name: &str, abbrv: &str, val: i32) -> Attribute {
        Attribute{
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

impl std::cmp::PartialEq for Attribute {
    // Required method
    fn eq(&self, other: &Self) -> bool {
        return self.id == other.id && self.abbrv == other.abbrv && self.value == other.value;
    }
}


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Attributes {
    pub strength: Attribute,
    pub dexterity: Attribute,
    pub constitution: Attribute,
    pub wisdom: Attribute,
    pub intelligence: Attribute,
    pub charisma: Attribute,
}

impl Attributes {
    pub fn new (str: i32, dex: i32, con: i32, int: i32, wis: i32, cha: i32) -> Attributes{
        Attributes { 
            strength: Attribute::new("str", "Strength", "Str", str),
            dexterity: Attribute::new("dex", "Dexterity", "Dex", dex),
            constitution: Attribute::new("con", "Constitution", "Con", con),
            intelligence: Attribute::new("int", "Intelligence", "Int", int),
            wisdom: Attribute::new("wis", "Wisdom", "Wis", wis),
            charisma: Attribute::new("cha", "Charisma", "Cha", cha),
         }
    }

    pub fn zero() -> Attributes{
        Attributes { 
            strength: Attribute::new("str", "Strength", "Str", 0),
            dexterity: Attribute::new("dex", "Dexterity", "Dex", 0),
            constitution: Attribute::new("con", "Constitution", "Con", 0),
            intelligence: Attribute::new("int", "Intelligence", "Int", 0),
            wisdom: Attribute::new("wis", "Wisdom", "Wis", 0),
            charisma: Attribute::new("cha", "Charisma", "Cha", 0),
         }
    }

    pub fn as_vec(&self) -> Vec<Attribute> {
        vec![
            self.strength.clone(),
            self.dexterity.clone(),
            self.constitution.clone(),
            self.intelligence.clone(),
            self.wisdom.clone(),
            self.charisma.clone()
        ]
    }

    pub fn as_number_vec(&self) -> Vec<i32> {
        vec![
            self.strength.value,
            self.dexterity.value,
            self.constitution.value,
            self.intelligence.value,
            self.wisdom.value,
            self.charisma.value
        ]
    }

    fn mut_vec(&mut self) -> Vec<&mut Attribute>{
        vec![
            &mut self.strength,
            &mut self.dexterity,
            &mut self.constitution,
            &mut self.intelligence,
            &mut self.wisdom,
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

    pub fn get_stat(&self, id: &str) -> std::option::Option::<Attribute> {
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

impl From<Vec<i32>> for Attributes {
    fn from(vector: Vec<i32>) -> Self {
        let mut ret_val = Self::zero();
        if vector.len() < 6 {
            panic!("Not enough values to create attributes from vector");
        }
        else{
            ret_val.strength.value = vector[0];
            ret_val.dexterity.value = vector[1];
            ret_val.constitution.value = vector[2];
            ret_val.intelligence.value = vector[3];
            ret_val.wisdom.value = vector[4];
            ret_val.charisma.value = vector[5];
            return ret_val;
        }
    }
}

impl From<&Vec<i32>> for Attributes {
    fn from(vector: &Vec<i32>) -> Self {
        let mut ret_val = Self::zero();
        if vector.len() < 6 {
            panic!("Not enough values to create attributes from vector");
        }
        else{
            ret_val.strength.value = vector[0];
            ret_val.dexterity.value = vector[1];
            ret_val.constitution.value = vector[2];
            ret_val.intelligence.value = vector[3];
            ret_val.wisdom.value = vector[4];
            ret_val.charisma.value = vector[5];
            return ret_val;
        }
    }
}



#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependentStat{
    pub attribute: String, 
    pub name: String, 
    pub proficiency: ProficiencyLevel,
    pub lore: bool
}

impl DependentStat {
    pub fn default_skill(attribute: &str, name: &str) -> DependentStat {
        DependentStat {
            attribute: String::from(attribute),
            name: String::from(name),
            proficiency: ProficiencyLevel::Untrained,
            lore: false
        }
    }

    pub fn new(attribute: &str, name: &str, proficiency: ProficiencyLevel, lore: bool) -> DependentStat {
        DependentStat {
            attribute: String::from(attribute),
            name: String::from(name),
            proficiency: proficiency,
            lore
        }
    }

    pub fn make_new_saves(fortitude: ProficiencyLevel, reflex: ProficiencyLevel, will: ProficiencyLevel) -> Vec<DependentStat> {
        return vec![
            DependentStat::new("con", "Fortitude", fortitude, false),
            DependentStat::new("dex", "Reflex", reflex, false),
            DependentStat::new("wis", "Will", will, false),
        ]
    }

    pub fn calculate_stat(self: &Self, character: &Character) -> i32 {
        let attribute_name = self.attribute.clone();
        let char_attributes = &character.attributes;
        let base_val = char_attributes.get_stat(&attribute_name);
        match base_val {
            Some(val) => val.value + self.proficiency.get_bonus(character.level),
            None => {log!("There was an error getting attribute: {attribute_name}"); return -99},
        }
    }
}