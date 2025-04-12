pub fn create_character() -> crate::modules::character::Character {
    let name = crate::modules::input::read_input("What's your name adventurer?: ");
    
    crate::modules::character::Character {
        name,
        level: 1,
        strength: 1,
        dexerity: 1,
        constitution: 1,
        intelligence: 1,
        wisdom: 1,
        charisma: 1,
    }
}
