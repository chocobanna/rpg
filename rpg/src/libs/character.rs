use std::fmt;

pub struct Character {
    pub name: String,
    pub level: i32,
    pub strength: i16,
    pub constitution: i16,
}

impl fmt::Display for Character {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\nName: {}\n  Level: {}\n  STR: {}\n  CON: {}\n", self.name, self.level, self.strength, self.constitution)
    }
}