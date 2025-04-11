use std::fmt;

pub struct Character {
    pub name: String,
    pub level: i32,
}

impl fmt::Display for Character {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Character: {} (Level: {})", self.name, self.level)
    }
}