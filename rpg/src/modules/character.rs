use std::fmt;

pub struct Character {
    pub name: String,
    pub level: i32,
    pub strength: i16,
    pub dexerity: i16,
    pub constitution: i16,
    pub intelligence: i16,
    pub wisdom: i16,
    pub charisma: i16,
}

impl fmt::Display for Character {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\nName: {}\n  Level: {}\n  Strength: {}\n  Dexterity: {}\n  Constitution: {}\n  Intelligence: {}\n  Wisdom: {}\n  Charisma: {}\n",
        self.name, self.level, self.strength, self.dexerity, self.constitution, self.intelligence, self.wisdom, self.charisma)
    }
}