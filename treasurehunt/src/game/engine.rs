#[path = "objects/map.rs"]
pub mod map;

#[path = "input/handlers.rs"]
mod handlers;

/// Display the Game menu
fn game_menu() {
    println!();
    println!("Game Menu");
    println!("---------");
    println!("1. Move");
    println!("2. Search");
    println!("3. Quit");
}

/// Main game loop
pub fn play() {
    print!("Before you can start your hunt..");
    let colour = handlers::ask_for_player_colour();
    let mut map = map::Map::new(map::player::Player::new((0, 0), colour));

    if let Err(_) = map.print() {
        println!("Something bad happened");
        return;
    }

    loop {
        game_menu();
        match handlers::ask_for_game_command() {
            handlers::command::GameCmd::Move => loop {
                let coordinates = handlers::ask_for_coordinates();

                if let Err(e) = map.within_boundries(coordinates) {
                    println!("{}", e);
                    continue;
                }

                let coord = (coordinates.0.unwrap(), coordinates.1.unwrap());
                if let Err(e) = map.is_valid_movement(coord) {
                    println!("{}", e);
                    continue;
                }

                map.move_player(coord);
                break;
            },
            handlers::command::GameCmd::Search => {
                if !map.search() {
                    println!("Dayum..your search resulted in nothing :/");
                    println!(
                        "Here's how far you're from the treasure: {:.3}",
                        map.distance_to_treasure()
                    )
                } else {
                    println!("Congratz! You've found the treasure");
                    break;
                }
            }
            handlers::command::GameCmd::Quit => break,
        }

        if let Err(_) = map.print() {
            println!("Something bad happened");
            return;
        }
    }
}
