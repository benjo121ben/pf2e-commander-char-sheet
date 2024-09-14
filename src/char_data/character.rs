use serde::{Deserialize, Serialize};
use super::{proficiency::ProficiencyLevel, stats::{DependentStat, Attributes}};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Character {
    pub name: String,
    pub level: i32,
    pub attributes: Attributes,
    pub background: String,
    pub class: String,
    pub skills: Vec<DependentStat>,
    pub saves: Vec<DependentStat>
}

impl Character {
    pub fn new(name: &str, attributes: Attributes, skills: Vec<DependentStat>, saves: Vec<DependentStat>) -> Character {
        Character {
            name: String::from(name),
            level: 1,
            attributes,
            background: String::from("Squire"),
            class: String::from("Commander"),
            skills: skills,
            saves: saves
        }
    }

    pub fn zero() -> Character {
        Character {
            name: String::from(""),
            level: 1,
            attributes: Attributes::zero(),
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
            ],
            saves: DependentStat::make_new_saves(
                ProficiencyLevel::Untrained, 
                ProficiencyLevel::Untrained, 
                ProficiencyLevel::Untrained
            )
        }
    }
}

impl Character {
    fn get_attribute_and_lore_flag_from_skill_name(skill_name: &str) -> (String, bool){
        let mut is_lore = false;
        let attribute = match skill_name {
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
            _ => {is_lore = true; "int"}

        };
        return (String::from(attribute), is_lore);

        
    }

    pub fn get_skill_obj_from_skill_name(self: &Self, skill_name: &str) -> Option<DependentStat>{
        for skill in &self.skills {
            if skill.name == skill_name {
                return Some((*skill).clone());
            }
        }
        return None;
    }

    pub fn get_skill_obj_indx_from_skill_name(self: &Self, skill_name: &str) -> Option<usize>{
        for (indx, skill) in self.skills.iter().enumerate() {
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
            skills: vec![],
            saves: vec![]
        };

        for skill_tuple in simp_char.skills {
            let (attribute, is_lore) = Character::get_attribute_and_lore_flag_from_skill_name(skill_tuple.0.as_str());
            ret_val.skills.push(DependentStat::new(&attribute, skill_tuple.0.as_str(), skill_tuple.1, is_lore))
        }

        ret_val.saves = DependentStat::make_new_saves(
            simp_char.saves.get(0).unwrap().clone(),
            simp_char.saves.get(1).unwrap().clone(),
            simp_char.saves.get(2).unwrap().clone()
        );
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
            skills: vec![],
            saves: vec![]
        };

        for skill_tuple in simp_char.skills.clone() {
            let (attribute, is_lore) = Character::get_attribute_and_lore_flag_from_skill_name(skill_tuple.0.as_str());
            ret_val.skills.push(DependentStat::new(&attribute, skill_tuple.0.as_str(), skill_tuple.1, is_lore))
        }

        ret_val.saves = DependentStat::make_new_saves(
            simp_char.saves.get(0).unwrap().clone(),
            simp_char.saves.get(1).unwrap().clone(),
            simp_char.saves.get(2).unwrap().clone()
        );
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
    pub skills: Vec<(String, ProficiencyLevel)>,
    pub saves: Vec<ProficiencyLevel>
}



impl From<Character> for SimpleCharacter{
    fn from(ref_char: Character) -> Self {
        let mut ret_val = SimpleCharacter {
            name: ref_char.name.clone(),
            level: ref_char.level,
            attributes: ref_char.attributes.as_number_vec(),
            background: ref_char.background.clone(),
            class: ref_char.class.clone(),
            skills: vec![],
            saves: vec![]
        };

        ret_val.skills.extend(ref_char.skills.into_iter().map(|s: DependentStat| return (s.name, s.proficiency)));
        ret_val.saves.extend(ref_char.saves.into_iter().map(|s: DependentStat| return s.proficiency));
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
            skills: vec![],
            saves: vec![]
        };

        ret_val.skills.extend(ref_char.skills.clone().into_iter().map(|s: DependentStat| return (s.name, s.proficiency)));
        ret_val.saves.extend(ref_char.saves.clone().into_iter().map(|s: DependentStat| return s.proficiency));
        return ret_val;
    }
}

