use serde::{Deserialize, Serialize};
use super::{proficiency::ProficiencyLevel, stats::{ProficiencyStat, ProficiencyStatType, Attributes}, feats::Feat};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Character {
    pub name: String,
    pub level: i32,
    pub attributes: Attributes,
    pub background: String,
    pub class: String,
    #[serde(default)]
    pub proficiencies: Vec<ProficiencyStat>,
    #[serde(default)]
    pub feats: Vec<Feat>
}

impl Character {
    pub fn zero() -> Character {
        Character {
            name: String::from(""),
            level: 1,
            attributes: Attributes::zero(),
            background: String::from("Squire"),
            class: String::from("Commander"),
            proficiencies: ProficiencyStat::default_array(),
            feats: vec![]
        }
    }
}

impl Character {
    fn get_attribute_and_lore_flag_from_skill_name(skill_name: &str, p_type: &ProficiencyStatType) -> String{
        return String::from(match p_type {
            ProficiencyStatType::Save => {
                match skill_name {
                    "Fortitude" => "con",
                    "Reflex" => "dex",
                    "Will" => "wis",
                    _ => {panic!("This save does not exist {skill_name}");}
                }
            },
            ProficiencyStatType::Skill => {
                match skill_name {
                    "Acrobatics" => "dex",
                    "Arcana" => "int",
                    "Athletics" => "str",
                    "Crafting" => "int",
                    "Deception" => "cha",
                    "Diplomacy" => "cha",
                    "Intimidation" => "cha",
                    "Medicine" => "wis",
                    "Nature" => "wis",
                    "Occultism" => "int",
                    "Performance" => "cha",
                    "Religion" => "wis",
                    "Society" => "int",
                    "Stealth" => "dex",
                    "Survival" => "wis",
                    "Thievery" => "dex",
                    _ => {panic!("This skill does not exist {skill_name}");}
                }
            },
            ProficiencyStatType::Lore => "int",
            ProficiencyStatType::Armor => "dex",
            ProficiencyStatType::Weapon => "str",
            ProficiencyStatType::Spell => "key",
            ProficiencyStatType::ClassDC => "key",
            ProficiencyStatType::Perception => "wis",
        });
        
    }

    pub fn get_skill_obj_from_skill_name(self: &Self, skill_name: &str) -> Option<ProficiencyStat>{
        for skill in &self.proficiencies {
            if skill.name == skill_name {
                return Some((*skill).clone());
            }
        }
        return None;
    }

    pub fn get_skill_obj_indx_from_skill_name(self: &Self, skill_name: &str) -> Option<usize>{
        for (indx, skill) in self.proficiencies.iter().enumerate() {
            if skill.name == skill_name {
                return Some(indx);
            }
        }
        return None;
    }
}

impl From<SimpleCharacter> for Character{
    fn from(simp_char: SimpleCharacter) -> Self {
        let mut ret_val = Character {
            name: simp_char.name.clone(),
            level: simp_char.level,
            attributes: Attributes::from(&simp_char.attributes),
            background: simp_char.background.clone(),
            class: simp_char.class.clone(),
            proficiencies: vec![],
            feats: vec![]
        };

        for skill_tuple in simp_char.proficiencies {
            let attribute = Character::get_attribute_and_lore_flag_from_skill_name(skill_tuple.0.as_str(), &skill_tuple.1);
            ret_val.proficiencies.push(ProficiencyStat::new(skill_tuple.1, &attribute, skill_tuple.0.as_str(), skill_tuple.2))
        }

        return ret_val;
    }
}

impl From<&SimpleCharacter> for Character{
    fn from(simp_char: &SimpleCharacter) -> Self {
        let mut ret_val = Character {
            name: simp_char.name.clone(),
            level: simp_char.level,
            attributes: Attributes::from(&((*simp_char).attributes)),
            background: simp_char.background.clone(),
            class: simp_char.class.clone(),
            proficiencies: vec![],
            feats: vec![]
        };

        for skill_tuple in simp_char.proficiencies.clone() {
            let attribute = Character::get_attribute_and_lore_flag_from_skill_name(skill_tuple.0.as_str(), &skill_tuple.1);
            ret_val.proficiencies.push(ProficiencyStat::new(skill_tuple.1, &attribute, skill_tuple.0.as_str(), skill_tuple.2))
        }

        return ret_val;
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimpleCharacter {
    pub name: String,
    pub level: i32,
    pub attributes: Vec<i32>,
    pub background: String,
    pub class: String,
    #[serde(default)]
    pub proficiencies: Vec<(String, ProficiencyStatType, ProficiencyLevel)>,
    #[serde(default)]
    pub feats: Vec<Feat>
}



impl From<Character> for SimpleCharacter{
    fn from(ref_char: Character) -> Self {
        let mut ret_val = SimpleCharacter {
            name: ref_char.name.clone(),
            level: ref_char.level,
            attributes: ref_char.attributes.as_number_vec(),
            background: ref_char.background.clone(),
            class: ref_char.class.clone(),
            proficiencies: vec![],
            feats: vec![]
        };

        ret_val.proficiencies.extend(ref_char.proficiencies.into_iter().map(|s: ProficiencyStat| return (s.name, s.p_type, s.proficiency)));
        return ret_val;
    }
}

impl From<&Character> for SimpleCharacter{
    fn from(ref_char: &Character) -> Self {
        let mut ret_val = SimpleCharacter {
            name: ref_char.name.clone(),
            level: ref_char.level,
            attributes: ref_char.attributes.as_number_vec(),
            background: ref_char.background.clone(),
            class: ref_char.class.clone(),
            proficiencies: vec![],
            feats: vec![]
        };

        ret_val.proficiencies.extend(ref_char.proficiencies.clone().into_iter().map(|s: ProficiencyStat| return (s.name, s.p_type, s.proficiency)));
        return ret_val;
    }
}

