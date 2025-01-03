use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::{bonus_penalty::StatBonusPenalties, character::Character, stats::CalculatedStat};


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Gear {
    pub name: String,
    pub g_type: GearType,
    #[serde(default)]
    pub traits: Vec<String>,
    pub proficiency: Option<String>,
    pub invested: Option<bool>,
    pub description: String,
    pub weap_info: Option<WeaponInfo>
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WeaponInfo {
    pub damage: i32,
    pub w_type: WeaponType,
    pub d_type: String
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum WeaponType {
    Melee,
    Ranged
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GearType {
    Weapon,
    Armor,
    Gear
}

pub struct AttackData {
    pub traits: Vec<String>,
    pub attribute_att_bonus: i32,
    pub prof_att_bonus: i32,
    pub item_att_bonus: i32,
    pub bonus_penalty_adjustment: i32,
    pub map: i32,
    pub dice_amount: i32,
    pub dice_size: i32,
    pub attribute_damage_bonus: i32,
    pub item_damage_bonus: i32,
    pub dam_type: String
}

impl AttackData {
    pub fn get_full_attack_bonus(self: &Self) -> i32 {
        self.attribute_att_bonus + self.prof_att_bonus + self.item_att_bonus
    }

    pub fn get_full_damage_bonus(self: &Self) -> i32 {
        self.attribute_damage_bonus + self.item_damage_bonus
    }
}

pub fn get_weapon_proficiency (character_data: &Character, weapon: &Gear) -> Result<CalculatedStat, String>{
    match weapon.proficiency.clone() {
        Some(prof_name) => {
            let stat_opt = character_data.get_prof_obj_from_name(&prof_name);
            match stat_opt {
                Some(prof) => Ok(prof),
                None => {let name_copy = prof_name.clone(); Err(format!("get_weapon_proficiency: Gear {0}. Could not find a proficiency with name {name_copy}", weapon.name))}
            }
        },
        None => Err(format!("Gear {0} does not have a weapon proficiency assigned", weapon.name)),
    }
}

pub fn get_weapon_attack_data(character_data: &Character, bp_map: &HashMap<String, StatBonusPenalties>, weapon: &Gear) -> Result<AttackData, String> {
        let mut stat: CalculatedStat = get_weapon_proficiency(&character_data, &weapon)?;
        let weap_info = match weapon.weap_info.clone() {
            Some(data) => Ok(data),
            None => Err(String::from("get_weapon_attack_data: We expect weapon_data by now. It was checked before")),
        }?;
        stat.attribute = if weap_info.w_type == WeaponType::Melee {"str"} else {"dex"}.to_string();
        let (base_bonus, bp_adjustment) = stat.calculate_stat(&character_data, bp_map);
        let bonus_progression_proficiency = character_data.abp_data.attack_pot;
        let map = if weapon.traits.contains(&"Agile".to_string()) {4} else {5};

        let dice_amount = character_data.abp_data.attack_dice;
        let dice_size = weap_info.damage;
        let mut stat_bonus_dmg = 0;
        if weap_info.w_type == WeaponType::Melee || weapon.traits.iter().any(|t|t=="Propulsive") {
            stat_bonus_dmg = character_data.attributes.get_stat_val("str")?;
            if weapon.traits.iter().any(|t|t=="Propulsive") {
                stat_bonus_dmg /= 2;
            }
        }
        //TODO add options for item runes
        Ok(AttackData{
            traits: weapon.traits.clone(),
            attribute_att_bonus: base_bonus,
            prof_att_bonus: bonus_progression_proficiency,
            item_att_bonus: 0,
            map,
            dice_amount,
            dice_size,
            attribute_damage_bonus: stat_bonus_dmg,
            item_damage_bonus: 0,
            dam_type: weap_info.d_type,
            bonus_penalty_adjustment: bp_adjustment,
        })
        
}