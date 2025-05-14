pub struct BasicTrackStats {
    health: i32,
    willpower: i32,
}

impl BasicTrackStats {
    pub fn health(&self) -> i32 {
        self.health
    }

    pub fn set_health(&mut self, new_health: i32) {
        self.health = new_health;
    } 

    pub fn willpower(&self) -> i32 {
        self.willpower
    } 

    pub fn set_willpower(&mut self, new_willpower: i32) {
        self.willpower = new_willpower; 
    }
}

// Basic structs for basic standard human sheet with stats.
pub struct BasicBioInfo {
    pub name: String,
    pub known_name: String,
    pub age: String,
    pub desire: String,
    pub nature: String,
    pub demeanor: String,
    pub concept: String,
    pub chronicle: String,
    pub residence: String,
    pub group: String,
}

pub struct BasicAbilities {
    pub talents: BasicTalents,
    pub skills: BasicSkills,
    pub knowledges: BasicKnowledges,
}

pub struct BasicTalents {
    pub alertness: i32,
    pub athlethics: i32,
    pub awareness: i32,
    pub brawl: i32,
    pub dodge: i32,
    pub empathy: i32,
    pub expression: i32,
    pub intimidation: i32,
    pub streetwise: i32,
    pub subterfuge: i32,
}

pub struct BasicSkills {
    pub animal_ken: i32,
    pub crafts: i32,
    pub drive: i32,
    pub etiquette: i32,
    pub firearms: i32,
    pub leadership: i32,
    pub melee: i32,
    pub performance: i32,
    pub stealth: i32,
    pub survival: i32,
}

pub struct BasicKnowledges {
    pub computer: i32,
    pub enigmas: i32,
    pub investigation: i32,
    pub law: i32,
    pub linguistics: i32,
    pub lore: i32,
    pub medicine: i32,
    pub occult: i32,
    pub politics: i32,
    pub science: i32,
}
