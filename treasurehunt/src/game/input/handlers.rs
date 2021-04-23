use lazy_static::lazy_static;
use read_input::prelude::*;
use regex::Regex;
use std::str::FromStr;
use termcolor::Color;

#[path = "../command.rs"]
pub mod command;

#[path = "../../utils.rs"]
pub mod utils;

mod semantic_check;
mod syntatic_check;

use command::{GameCmd, MenuCmd};

fn get_input(msg: &str, err_msg: &str, syntatic_test: fn(&str) -> bool) -> String {
    let input: String = input()
        .repeat_msg(msg)
        .add_err_test(move |x: &String| syntatic_test(&x), err_msg)
        .get();

    return input;
}

fn clean_input(input: &String) -> String {
    input.replace(&['(', ')', '[', ']', ' '][..], "")
}

pub fn ask_for_game_command() -> GameCmd {
    loop {
        let input_cmd = get_input(
            "What do you want to do ? ",
            "Unknown gane command",
            syntatic_check::check_cmd_syntax,
        );

        if let Err(e) = semantic_check::check_game_cmd_semantic(&input_cmd) {
            println!("{}", e);
            continue;
        }

        return GameCmd::from_str(&input_cmd).unwrap();
    }
}

pub fn ask_for_menu_command() -> MenuCmd {
    loop {
        let input_cmd = get_input(
            "What do you want to do ? ",
            "Unknown menu command",
            syntatic_check::check_cmd_syntax,
        );

        if let Err(e) = semantic_check::check_menu_cmd_semantic(&input_cmd) {
            println!("{}", e);
            continue;
        }

        return MenuCmd::from_str(&input_cmd).unwrap();
    }
}

pub fn ask_for_player_colour() -> Color {
    loop {
        let input_colour = get_input(
            "What colour would you like? ",
            "Please enter a colour or a rgb code.",
            syntatic_check::check_colour_syntax,
        );

        let cleaned_input = clean_input(&input_colour);
        if let Err(e) = semantic_check::check_colour_semantic(&cleaned_input) {
            println!("{}", e);
            continue;
        }

        return Color::from_str(&cleaned_input).unwrap();
    }
}

pub fn ask_for_coordinates() -> (Option<u8>, Option<u8>) {
    loop {
        let input_coord = get_input(
            "Where do you want to go? ",
            "Bad format, please respect the format (i.e. (x, y) or [x, y])",
            syntatic_check::check_coord_syntax,
        );

        let clean_coord = clean_input(&input_coord.to_string());
        let coords: Vec<&str> = clean_coord.split(',').collect();
        if let Err(e) = semantic_check::check_coordinate_semantic(&coords) {
            println!("{}", e);
            continue; 
        }

        return (
            utils::parse_number(coords[0]),
            utils::parse_number(coords[1]),
        );
    }
}
