use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use leptos::logging::log;
use super::{bonus_penalty::StatBonusPenalties, character::Character, proficiency::ProficiencyLevel};


#[derive(Debug, Clone, Deserialize, Serialize, Eq)]
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


#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
pub struct Attributes {
    pub strength: Attribute,
    pub dexterity: Attribute,
    pub constitution: Attribute,
    pub wisdom: Attribute,
    pub intelligence: Attribute,
    pub charisma: Attribute
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

    pub fn get_stat(&self, id: &str) -> Result<Attribute, String> {
        if id == "key" {
            return Ok(self.intelligence.clone())
        }
        for s in self.as_vec() {
            if s.get_id() == id {
                return Ok(s.clone());
            }
        }
        return Err(format!("Could not find Attribute with id:{id}"));
    }

    pub fn get_stat_val(&self, id: &str) -> Result<i32, String> {
        if id == "key" {
            return Ok(self.intelligence.value)
        }
        for s in self.as_vec() {
            if s.get_id() == id {
                return Ok(s.value);
            }
        }
        return Err(format!("Attributes Could not find value for attribute id: {id}"));
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProficiencyType {
    Save,
    Skill,
    Lore,
    Armor,
    Weapon,
    Spell,
    ClassDC,
    Perception
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq)]
pub struct CalculatedStat{
    pub p_type: ProficiencyType,
    pub attribute: String, 
    pub name: String, 
    pub proficiency: ProficiencyLevel,
}

impl CalculatedStat {
    pub fn default_skill(attribute: &str, name: &str) -> CalculatedStat {
        CalculatedStat {
            p_type: ProficiencyType::Skill,
            attribute: String::from(attribute),
            name: String::from(name),
            proficiency: ProficiencyLevel::Untrained,
        }
    }

    pub fn default_save(attribute: &str, name: &str) -> CalculatedStat {
        CalculatedStat {
            p_type: ProficiencyType::Save,
            attribute: String::from(attribute),
            name: String::from(name),
            proficiency: ProficiencyLevel::Trained,
        }
    }

    pub fn new(p_type: ProficiencyType, attribute: &str, name: &str, proficiency: ProficiencyLevel) -> CalculatedStat {
        CalculatedStat {
            p_type: p_type,
            attribute: String::from(attribute),
            name: String::from(name),
            proficiency: proficiency,
        }
    }

    pub fn default_array() -> Vec<CalculatedStat> {
        vec![
                CalculatedStat::default_skill("dex", "Acrobatics"),
                CalculatedStat::default_skill("int", "Arcana"),
                CalculatedStat::default_skill("str", "Athletics"),
                CalculatedStat::default_skill("int", "Crafting"),
                CalculatedStat::default_skill("cha", "Deception"),
                CalculatedStat::default_skill("cha", "Diplomacy"),
                CalculatedStat::default_skill("cha", "Intimidation"),
                CalculatedStat::default_skill("wis", "Medicine"),
                CalculatedStat::default_skill("wis", "Nature"),
                CalculatedStat::default_skill("int", "Occultism"),
                CalculatedStat::default_skill("cha", "Performance"),
                CalculatedStat::default_skill("wis", "Religion"),
                CalculatedStat::default_skill("int", "Society"),
                CalculatedStat::default_skill("dex", "Stealth"),
                CalculatedStat::default_skill("wis", "Survival"),
                CalculatedStat::default_skill("dex", "Thievery"),
                CalculatedStat::default_save("con", "Fortitude"),
                CalculatedStat::default_save("dex", "Reflex"),
                CalculatedStat::default_save("wis", "Will"),
                CalculatedStat::new(ProficiencyType::Armor, "dex", "Unarmored", ProficiencyLevel::Trained),
                CalculatedStat::new(ProficiencyType::Armor, "dex", "Light", ProficiencyLevel::Untrained),
                CalculatedStat::new(ProficiencyType::Armor, "dex", "Medium", ProficiencyLevel::Untrained),
                CalculatedStat::new(ProficiencyType::Armor, "dex", "Heavy", ProficiencyLevel::Untrained),
                CalculatedStat::new(ProficiencyType::Weapon, "str", "Simple", ProficiencyLevel::Untrained),
                CalculatedStat::new(ProficiencyType::Weapon, "str", "Martial", ProficiencyLevel::Untrained),
                CalculatedStat::new(ProficiencyType::Weapon, "str", "Advanced", ProficiencyLevel::Untrained),
                CalculatedStat::new(ProficiencyType::Perception, "wis", "Perception", ProficiencyLevel::Untrained),
                CalculatedStat::new(ProficiencyType::ClassDC, "key", "ClassDC", ProficiencyLevel::Trained),
            ]
    }

    pub fn calculate_stat(self: &Self, character: &Character, bp_map: &HashMap<String, StatBonusPenalties>) -> i32 {
        let attribute_name = self.attribute.clone();
        let char_attributes = &character.attributes;
        let get_attribute_result: Result<Attribute, String> = char_attributes.get_stat(&attribute_name);
        let skill_auto_bonus_prog = match character.abp_data.skill_pot.get(&self.name) {
            Some(value) => *value,
            None => 0,
        };
        //we check this because some feat may override which stat to use for a skill
        let override_attribute = character.override_prof.get(&self.name).and_then(|override_name| {
            return char_attributes.get_stat(&override_name).ok();
        });
        match get_attribute_result {
            Ok(default_attribute) => {
                let mut attribute_used = default_attribute;
                if override_attribute.is_some() && override_attribute.clone().unwrap().value > attribute_used.value{
                    attribute_used = override_attribute.unwrap();
                }
                attribute_used.value + skill_auto_bonus_prog + self.proficiency.get_bonus(character.level)
            },
            Err(err) => {log!("{err}"); return -99},
        }
    }
}

impl PartialEq for CalculatedStat {
    fn eq(&self, other: &Self) -> bool {
        let val = self.p_type == other.p_type && self.attribute == other.attribute && self.name == other.name && self.proficiency == other.proficiency;
        val
    }
}