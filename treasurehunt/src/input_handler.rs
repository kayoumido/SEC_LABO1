use lazy_static::lazy_static;
use read_input::prelude::*;
use regex::Regex;

#[path = "command.rs"]
pub mod command;

#[path = "utils.rs"]
pub mod utils;

use command::{CmdOrigin, PlayerCmd};
use std::ops::Range;
use std::str::FromStr;

use termcolor::Color;
use utils::parse_number;

fn clean_input(input: &String) -> String {
    input.replace(&['(', ')', '[', ']', ' '][..], "")
}

pub fn ask_for_player_colour() -> Color {
    loop {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^[A-Za-z]+$|^\((\d{1,3},\s*){2}\d{1,3}\)$|^\[(\d{1,3},\s*){2}\d{1,3}\]$|^(\d{1,3},\s*){2}\d{1,3}$").unwrap();
        };

        let input_colour: String = input()
            .repeat_msg("What colour would you like? ")
            .add_err_test(
                |x: &String| RE.is_match(&x),
                "Please enter a colour or a rgb code.",
            )
            .get();

        let cleaned_input = clean_input(&input_colour);
        if let Err(e) = Color::from_str(&cleaned_input) {
            println!("{}, please try again.", e);
            continue;
        }

        return Color::from_str(&cleaned_input).unwrap();
    }
}

pub fn ask_form_menu_command() -> PlayerCmd {
    ask_for_player_command(
        CmdOrigin::Menu,
        "What do you want to do ? ",
        "Unknown menu command",
    )
}

pub fn ask_form_game_command() -> PlayerCmd {
    ask_for_player_command(
        CmdOrigin::Game,
        "What do you want to do ? ",
        "Unknown game command",
    )
}

fn ask_for_player_command(origin: CmdOrigin, msg: &str, err_msg: &str) -> PlayerCmd {
    loop {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"([A-Za-z]+)|(\d+)").unwrap();
        };

        let input_cmd: String = input()
            .repeat_msg(msg)
            .add_err_test(|x: &String| RE.is_match(&x), err_msg)
            .get();

        if let Err(e) = PlayerCmd::from_str(&input_cmd, &origin) {
            println!("{}", e);
            continue;
        }

        return PlayerCmd::from_str(&input_cmd, &origin).unwrap();
    }
}

pub fn ask_for_coordinates() -> (Option<u8>, Option<u8>) {
    loop {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^\((\d+,\s*)+\d+\)$|^\[(\d+,\s*)+\d+\]$").unwrap();
        };

        let input_coord: String = input()
            .repeat_msg("Where do you want to go? ")
            .add_err_test(
                |x: &String| testing(x),
                "Bad format, please respect the format (i.e. (x, y) or [x, y])",
            )
            .get();

        let clean_coord = clean_input(&input_coord.to_string());
        let coords: Vec<&str> = clean_coord.split(',').collect();

        if coords.len() != 2 {
            println!(
                "Bad number of dimension in provided coordinate. Provided {}, 2 expected",
                coords.len()
            );
            continue;
        }

        return (parse_number(coords[0]), parse_number(coords[1]));
        // let x = parse_number(coords[0]);
        // let y = parse_number(coords[1]);

        // if (x == None || y == None)
        //     || !x_boundries.contains(&x.unwrap())
        //     || !y_boundries.contains(&y.unwrap())
        // {
        //     println!(
        //         "Out of bounds, given coordinates are out of the map ({}x{})",
        //         x_boundries.end, y_boundries.end
        //     );
        //     continue;
        // }

        // return (x.unwrap(), y.unwrap());
    }
}

fn testing(x: &String) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^\((\d+,\s*)+\d+\)$|^\[(\d+,\s*)+\d+\]$").unwrap();
    };

    RE.is_match(&x)
}
