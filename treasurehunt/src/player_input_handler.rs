use lazy_static::lazy_static;
use read_input::prelude::*;
use regex::Regex;

#[path = "player_command.rs"]
mod player_command;

use std::str::FromStr;
use termcolor::Color;

use player_command::{CmdOrigin, PlayerCmd};

pub fn ask_for_player_colour() -> Color {
    loop {
        lazy_static! {
            static ref re: Regex =
                Regex::new(r"([A-Za-z]+)|\((\d{1,3}), (\d{1,3}), (\d{1,3})\)?").unwrap();
        };

        let input_colour: String = input()
            .repeat_msg("What colour would you like? ")
            .add_err_test(
                |x: &String| re.is_match(&x),
                "Please enter a colour or a rgb code.",
            )
            .get();

        if let Err(_) = Color::from_str(&input_colour) {
            println!("Unknown colour, please try again.");
            continue;
        }

        return Color::from_str(&input_colour).unwrap();
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
