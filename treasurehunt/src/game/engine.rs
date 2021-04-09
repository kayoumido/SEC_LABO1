#[path = "objects/map.rs"]
pub mod map;

#[path = "../input_handler.rs"]
mod input_handler;

#[path = "input/handlers.rs"]
mod handlers;

pub fn play() {
    let colour = input_handler::ask_for_player_colour();
    let mut map = map::Map::new(map::player::Player::new((0, 0), colour));

    if let Err(_) = map.print() {
        println!("Something bad happened");
        return;
    }

    loop {
        let cmd = input_handler::ask_form_game_command();

        match cmd {
            input_handler::command::PlayerCmd::Move => loop {
                let coordinates = input_handler::ask_for_coordinates();

                if map.within_boundries(coordinates) {
                    println!(
                        "Out of bounds, given coordinates are out of the map ({}x{})",
                        map::Map::DEFAULT_WIDTH,
                        map::Map::DEFAULT_HEIGHT
                    );
                    continue;
                }

                let coord = (coordinates.0.unwrap(), coordinates.1.unwrap());

                if map.is_valid_movement(coord) {
                    println!("You can only move 4 squares at a time!");
                    continue;
                }

                map.move_player(coord);
                break;
            },
            input_handler::command::PlayerCmd::Search => {
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
            input_handler::command::PlayerCmd::Quit => break,
            // if we ever get here that means something teribly wrong happened
            _ => break,
        }

        if let Err(_) = map.print() {
            println!("Something bad happened");
            return;
        }
    }
}
