pub mod modules;

fn main_menu() {
    let mut i = 0;
    while i < 1 {
        modules::screen_fx::clear_screen();
        println!("D&D Digital Campaign V0.1\n");
        eprintln!("{}", player1);

        let user_input = modules::input::read_input("Actions\n  Add/Remove Combat Exit\nWhat do you want to do: ");
    
        if user_input.trim().eq_ignore_ascii_case("exit") {
            println!("Exiting program.");
            std::process::exit(0);
        } else if user_input.trim().eq_ignore_ascii_case("add/remove") {
            let _ = modules::input::read_input("Actions\n  Stats Items Skills Titles Back\nWhat do you want to do: ");
        }
    }
}