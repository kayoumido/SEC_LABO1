// Note: Some of the `mod` imports weren't working w/o
//       giving the path to file. So for the sake of unifromitiy
//       the path was given for all of the `mod` imports.
#[path = "map.rs"]
mod map;

use std::str::FromStr;

use lazy_static::lazy_static;
use read_input::prelude::*;
use regex::Regex;
use termcolor::Color;

use map::Map;

fn main() {
    // let map = Map::new();

    // let colour = Color::from_str("blfue");

    let colour: Color;
    loop {
        lazy_static! {
            static ref re: Regex =
                Regex::new(r"([A-Za-z]+)|\((\d{1,3}), (\d{1,3}), (\d{1,3})\)?").unwrap();
        };

        let input_colour: String = input()
            .repeat_msg("What colour would you like? ")
            .add_err_test(
                |x: &String| re.is_match(&x),
                "Please enter a colour or a rgb code, please try again.",
            )
            .get();

        if let Err(err) = Color::from_str(&input_colour) {
            println!("Unknown colour, please try again.");
            continue;
        }

        colour = Color::from_str(&input_colour).unwrap();
        break;
    }

    println!("{:?}", colour);

    // map.print();
}
