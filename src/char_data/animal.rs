use serde::{Deserialize, Serialize};

use super::{hp::HpInfo, stats::Attributes};

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct Animal {
    pub hp_info: HpInfo,
    pub attributes: Attributes
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct SimpleAnimal {
    pub hp_info: HpInfo,
    pub attributes: Vec<i32>
}

impl Animal {
    pub fn new () -> Self{
        Animal {
            hp_info: HpInfo::new(0,0,1, 0),
            attributes: Attributes::zero()
        }
    }
}

impl From<&SimpleAnimal> for Animal {
    fn from(simple_animal: &SimpleAnimal) -> Self {
        Self {
            hp_info: simple_animal.hp_info.clone(),
            attributes: Attributes::from(simple_animal.attributes.clone())
        }
    }
} 

impl From<&Animal> for SimpleAnimal {
    fn from(animal: &Animal) -> Self {
        Self {
            hp_info: animal.hp_info.clone(),
            attributes: animal.attributes.as_number_vec()
        }
    }
} 