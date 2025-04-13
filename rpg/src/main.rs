mod modules;
mod menus;

fn main() {
    modules::screen_fx::clear_screen();
    println!("Welcome to D&D Digital Campaign V0.1");
    let player1 = menus::character_creation::create_character();

    menus::main_menu::main_menu()
}
