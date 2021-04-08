// Note: Some of the `mod` imports weren't working w/o
//       giving the path to file. So for the sake of unifromitiy
//       the path was given for all of the `mod` imports.
#[path = "map.rs"]
mod map;

#[path = "utils.rs"]
mod utils;

#[path = "player.rs"]
mod player;

#[path = "command.rs"]
mod command;

#[path = "input_handler.rs"]
mod input_handler;

use command::PlayerCmd;
use map::Map;

fn main() {
    // Print welcome message
    // Print main menu
    // Ask user for input
    // 1. Start game
    // 2. Print About
    // 3. Quit
    //
    // Start game
    // Ask for player colour
    // init map

    let colour = input_handler::ask_for_player_colour();
    let mut map = Map::new(map::player::Player::new((0, 0), colour));

    if let Err(_) = map.print() {
        println!("Something bad happened");
    }

    let mut prev_cmd: PlayerCmd;
    loop {
        let cmd = input_handler::ask_form_game_command();

        match cmd {
            input_handler::command::PlayerCmd::Move => loop {
                let coordinates = input_handler::ask_for_coordinates(
                    0..Map::DEFAULT_WIDTH,
                    0..Map::DEFAULT_WIDTH,
                );
                let player_coord = map.get_player_position();

                let possible_x_start: u8;
                if player_coord.0 < 4 {
                    possible_x_start = 0
                } else {
                    possible_x_start = player_coord.0 - 4;
                }

                let possible_x_end: u8;
                if player_coord.0 > Map::DEFAULT_WIDTH - 4 {
                    possible_x_end = Map::DEFAULT_WIDTH;
                } else {
                    possible_x_end = player_coord.0 + 4;
                }

                let possible_y_start: u8;
                if player_coord.1 < 4 {
                    possible_y_start = 0
                } else {
                    possible_y_start = player_coord.1 - 4;
                }

                let possible_y_end: u8;
                if player_coord.1 > Map::DEFAULT_HEIGHT - 4 {
                    possible_y_end = Map::DEFAULT_HEIGHT;
                } else {
                    possible_y_end = player_coord.1 + 4;
                }

                let possible_x_coord = possible_x_start..possible_x_end;
                let possible_y_coord = possible_y_start..possible_y_end;

                if !possible_x_coord.contains(&coordinates.0)
                    || !possible_y_coord.contains(&coordinates.1)
                {
                    println!("You can only move 4 squares at a time!");
                    continue;
                }

                map.move_player(coordinates);
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
                }
            }
            input_handler::command::PlayerCmd::Quit => break,
            // if we ever get here that means something teribly wrong happened
            _ => break,
        }

        if let Err(_) = map.print() {
            println!("Something bad happened");
        }
    }
}
