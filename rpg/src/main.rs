// Import
use std::io;

mod character;

fn main() {
    println!("What's your name adventurer?");
    let mut input = String::new(); 

    io::stdin().read_line(&mut input).expect("How the actual fu--. You typed nothing gj.");

    let input = input.trim();

    let player1 = character::Character{
        name:input.to_string(),
        level:1,
    };

    println!("Welcome to RPG Test! V0.1");
    eprintln!("{}",player1);
}