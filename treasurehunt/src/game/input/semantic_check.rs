use std::str::FromStr;
use termcolor::Color;

#[path = "../command.rs"]
pub mod command;

#[path = "../errors.rs"]
mod errors;

use command::{GameCmd, MenuCmd};
use errors::SemanticError;

pub fn check_game_cmd_semantic(cmd: &str) -> Result<(), SemanticError> {
    if let Err(_) = GameCmd::from_str(&cmd) {
        Err(SemanticError::InvalidGameCmdSemantic)
    } else {
        Ok(())
    }
}

pub fn check_menu_cmd_semantic(cmd: &str) -> Result<(), SemanticError> {
    if let Err(_) = MenuCmd::from_str(&cmd) {
        Err(SemanticError::InvalidMenuCmdSemantic)
    } else {
        Ok(())
    }
}

pub fn check_coordinate_semantic(coord: &Vec<&str>) -> Result<(), SemanticError> {
    if coord.len() != 2 {
        Err(SemanticError::InvalidCoordinateSemantic)
    } else {
        Ok(())
    }
}

pub fn check_colour_semantic(colour: &str) -> Result<(), termcolor::ParseColorError> {
    if let Err(e) = Color::from_str(&colour) {
        Err(e)
    } else {
        Ok(())
    }
}
