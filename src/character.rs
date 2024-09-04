use leptos::html::Main;

#[derive(Debug, Clone)]
pub struct MainStats {
    pub strength: i32,
    pub dexterity: i32,
    pub constitution: i32,
    pub wisdom: i32,
    pub intelligence: i32,
    pub charisma: i32,
}

#[derive(Debug, Clone)]
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
            main_stats: MainStats {
                strength: 0,
                dexterity: 0,
                constitution: 0,
                wisdom: 0,
                intelligence: 0,
                charisma: 0,
            },
            background: String::new(),
            class: String::new(),
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = String::from(name);
    }
}
