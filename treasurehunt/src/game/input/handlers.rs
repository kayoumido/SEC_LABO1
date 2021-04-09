use lazy_static::lazy_static;
use read_input::prelude::*;
use regex::Regex;

#[path = "../../command.rs"]
pub mod command;

use command::{CmdOrigin, PlayerCmd};

fn get_input(msg: &str, err_msg: &str, test: fn(&String) -> bool) -> String {
    let input: String = input()
        .repeat_msg(msg)
        .add_err_test(|x: &String| test(&x), err_msg)
        .get();

    return input;
}

pub fn ask_for_game_command() -> PlayerCmd {
    loop {
        let input_cmd = get_input("What do you want to do ? ", "Unknown menu command", foo);

        if let Err(e) = PlayerCmd::from_str(&input_cmd, &CmdOrigin::Game) {
            println!("{}", e);
            continue;
        }

        return PlayerCmd::from_str(&input_cmd, &CmdOrigin::Game).unwrap();
    }
}

fn foo(x: &String) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"([A-Za-z]+)|(\d+)").unwrap();
    };

    RE.is_match(x)
}
