use serde::{Deserialize, Serialize};
use super::{proficiency::ProficiencyLevel, stats::{DependentStat, MainStats}};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Character {
    pub name: String,
    pub level: i32,
    pub main_stats: MainStats,
    pub background: String,
    pub class: String,
    pub skills: Vec<DependentStat>,
    pub saves: Vec<DependentStat>
}

impl Character {
    pub fn new(name: &str, main_stats: MainStats, skills: Vec<DependentStat>, saves: Vec<DependentStat>) -> Character {
        Character {
            name: String::from(name),
            level: 1,
            main_stats: main_stats,
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
    fn get_appropriate_skill(skill_name: &str) -> (String, bool){
        let mut is_lore = false;
        let main_stat = match skill_name {
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
        return (String::from(main_stat), is_lore);
    }
}

impl From<SimpleCharacter> for Character{
    fn from(value: SimpleCharacter) -> Self {
        let mut ret_val = Character {
            name: value.name.clone(),
            level: value.level,
            main_stats: value.main_stats.clone(),
            background: value.background.clone(),
            class: value.class.clone(),
            skills: vec![],
            saves: vec![]
        };

        for skill_tuple in value.skills {
            let (main_stat, is_lore) = Character::get_appropriate_skill(skill_tuple.0.as_str());
            ret_val.skills.push(DependentStat::new(&main_stat, skill_tuple.0.as_str(), skill_tuple.1, is_lore))
        }

        ret_val.saves = DependentStat::make_new_saves(
            value.saves.get(0).unwrap().clone(),
            value.saves.get(1).unwrap().clone(),
            value.saves.get(2).unwrap().clone()
        );
        return ret_val;
    }
}

impl From<&SimpleCharacter> for Character{
    fn from(value: &SimpleCharacter) -> Self {
        let mut ret_val = Character {
            name: value.name.clone(),
            level: value.level,
            main_stats: value.main_stats.clone(),
            background: value.background.clone(),
            class: value.class.clone(),
            skills: vec![],
            saves: vec![]
        };

        for skill_tuple in value.skills.clone() {
            let (main_stat, is_lore) = Character::get_appropriate_skill(skill_tuple.0.as_str());
            ret_val.skills.push(DependentStat::new(&main_stat, skill_tuple.0.as_str(), skill_tuple.1, is_lore))
        }

        ret_val.saves = DependentStat::make_new_saves(
            value.saves.get(0).unwrap().clone(),
            value.saves.get(1).unwrap().clone(),
            value.saves.get(2).unwrap().clone()
        );
        return ret_val;
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimpleCharacter {
    pub name: String,
    pub level: i32,
    pub main_stats: MainStats,
    pub background: String,
    pub class: String,
    pub skills: Vec<(String, ProficiencyLevel)>,
    pub saves: Vec<ProficiencyLevel>
}



impl From<Character> for SimpleCharacter{
    fn from(value: Character) -> Self {
        let mut ret_val = SimpleCharacter {
            name: value.name.clone(),
            level: value.level,
            main_stats: value.main_stats.clone(),
            background: value.background.clone(),
            class: value.class.clone(),
            skills: vec![],
            saves: vec![]
        };

        ret_val.skills.extend(value.skills.into_iter().map(|s: DependentStat| return (s.name, s.proficiency)));
        ret_val.saves.extend(value.saves.into_iter().map(|s: DependentStat| return s.proficiency));
        return ret_val;
    }
}

impl From<&Character> for SimpleCharacter{
    fn from(value: &Character) -> Self {
        let mut ret_val = SimpleCharacter {
            name: value.name.clone(),
            level: value.level,
            main_stats: value.main_stats.clone(),
            background: value.background.clone(),
            class: value.class.clone(),
            skills: vec![],
            saves: vec![]
        };

        ret_val.skills.extend(value.skills.clone().into_iter().map(|s: DependentStat| return (s.name, s.proficiency)));
        ret_val.saves.extend(value.saves.clone().into_iter().map(|s: DependentStat| return s.proficiency));
        return ret_val;
    }
}

