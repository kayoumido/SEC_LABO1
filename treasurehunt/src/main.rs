/*!
 * First labratory for the Secure Coding course at the HEIG-VD.
 *
 * This project (crate?) is an implementation of a small treasure hunting game.
 *
 * The player has to move arround the map and search each square for the treasure
 *
 * # Author
 * Doran Kayoumi <doran.kayoumi@heig-vd.ch>
 *
 * # Notes
 *
 * For some reason when compiling there's a warning saying that most of the
 * functions aren't used..but it isn't the case. I think it's linked to how the
 * different files are imported, but I don't know how to fix it Q_Q.
 */

#[path = "game/engine.rs"]
mod engine;

#[path = "game/input/handlers.rs"]
mod handlers;

#[path = "game/objects/map.rs"]
mod map;

fn main_menu() {
    println!();
    println!("Main Menu");
    println!("---------");
    println!("1. Start");
    println!("2. About");
    println!("3. Quit");
}

fn about() {
    println!("About");
    println!("-----");
    println!("The objective is pretty simple, you need to find the treasure that is hidden");
    println!("Here are the commands you can use to find it:");

    println!();

    println!("1. Movements");
    println!("This is the action you'll use to move around the map");
    println!("How to use");
    println!("----------");
    println!("You can perform this action by entering the number in the displayed menu or simply typing \"Move\"");
    println!("Then you'll need to indicate the coordinates you wish to do. Decimal and hexadecimal numbers are accepted.");
    println!(
        "/!\\ You can only move {} squares at a time",
        map::Map::MAX_PLAYER_MOVEMENT
    );
    println!();
    println!("Example");
    println!("-------");
    println!("What do you want to do? Move");
    println!("Where do you want to go? (5, 5)");
    println!();
    println!("What do you want to do? Move");
    println!("Where do you want to go? (0xE, 0x5)");
    println!("You can only move 4 squares at a time");

    println!();

    println!("2. Search");
    println!("This is the action you'll use to see if you found the treasure");
    println!("How to use");
    println!("----------");
    println!("You can perform this action by entering the number in the displayed menu or simply typing \"Search\"");
    println!(
        "Note: The squares you've already searched will be marked with {}",
        map::Map::VISITED_CHAR
    );
    println!("If you're result reulted in nothing, the eucldean distance to the treasure will be displayed");
    println!();
    println!("Example");
    println!("-------");
    println!("What do you want to do? Search");
    println!("Dayum..your search resulted in nothing :/");
    println!("Here's how far you're from the treasure: 5.000");

    println!();

    println!("3. Quit");
    println!("Well...uh...you'll use this to quit the game");
    println!("How to use");
    println!("----------");
    println!("You can perform this action by entering the number in the displayed menu or simply typing \"Quit\"");
}

fn main() {
    println!("Welcome treasure hunter!");
    println!("---------------------");
    loop {
        main_menu();
        match handlers::ask_for_menu_command() {
            handlers::command::MenuCmd::Start => {
                engine::play();
                break;
            }
            handlers::command::MenuCmd::About => about(),
            handlers::command::MenuCmd::Quit => {
                break;
            }
        }
    }

    println!("Goodbye!");
}
