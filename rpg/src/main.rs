mod libs;

fn main() {
    let user_input = libs::input::read_input("What's your name adventurer?: ");

    let player1 = libs::character::Character{
        name:user_input.to_string(),
        level:1,
        strength:1,
        constitution:1,
    };

    println!("Welcome to RPG Test!");
    eprintln!("{}",player1);
}