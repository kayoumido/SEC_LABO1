use read_input::prelude::*;
use std::str::FromStr;
use termcolor::Color;

#[path = "../command.rs"]
pub mod command;

#[path = "../../utils.rs"]
pub mod utils;

#[path = "semantic_check.rs"]
mod semantic_check;

#[path = "syntatic_check.rs"]
mod syntatic_check;

use command::{GameCmd, MenuCmd};
use semantic_check::*;
use syntatic_check::*;

/// Main function to ask for user input
///
/// # Arguments
///
/// * `msg` - question/message to display to the user
/// * `err_msg` - error message to display when the users input fails the syntax check
/// * `syntatic_test` - function to perfor the syntax check. signature : fn(&str) -> bool
///
fn get_input(msg: &str, err_msg: &str, syntatic_test: fn(&str) -> bool) -> String {
    let input: String = input()
        .repeat_msg(msg)
        .add_err_test(move |x: &String| syntatic_test(&x), err_msg)
        .get();

    return input;
}

/// Ask the player for a menu command
pub fn ask_for_game_command() -> GameCmd {
    loop {
        let input_cmd = get_input(
            "What do you want to do ? ",
            "Unknown gane command",
            check_cmd_syntax,
        );

        if let Err(e) = check_game_cmd_semantic(&input_cmd) {
            println!("{}", e);
            continue;
        }

        return GameCmd::from_str(&input_cmd).unwrap();
    }
}

/// Ask the player for a menu command
pub fn ask_for_menu_command() -> MenuCmd {
    loop {
        let input_cmd = get_input(
            "What do you want to do ? ",
            "Unknown menu command",
            syntatic_check::check_cmd_syntax,
        );

        if let Err(e) = check_menu_cmd_semantic(&input_cmd) {
            println!("{}", e);
            continue;
        }

        return MenuCmd::from_str(&input_cmd).unwrap();
    }
}

/// Ask the player for a colour
pub fn ask_for_player_colour() -> Color {
    loop {
        let input_colour = get_input(
            "What colour would you like? \n(You can give a colour name or a rgb cod. e.g. Blue or (255, 51, 204) ",
            "Please enter a colour or a rgb code.",
            syntatic_check::check_colour_syntax,
        );

        let cleaned_input = utils::clean_str(&input_colour);
        if let Err(e) = check_colour_semantic(&cleaned_input) {
            println!("{}", e);
            continue;
        }

        return Color::from_str(&cleaned_input).unwrap();
    }
}

/// Ask the player for coordinates
pub fn ask_for_coordinates() -> (Option<u8>, Option<u8>) {
    loop {
        let input_coord = get_input(
            "Where do you want to go? \n(Enter coordinates (i.e. (x, y) or [x, y]) and max 4 squares) ",
            "Bad format, please respect the format (i.e. (x, y) or [x, y])",
            check_coord_syntax,
        );

        let clean_coord = utils::clean_str(&input_coord.to_string());
        let coords: Vec<&str> = clean_coord.split(',').collect();
        if let Err(e) = check_coordinate_semantic(&coords) {
            println!("{}", e);
            continue;
        }

        return (
            utils::parse_number(coords[0]),
            utils::parse_number(coords[1]),
        );
    }
}
