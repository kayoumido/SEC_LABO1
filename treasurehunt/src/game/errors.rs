use std::error;
use std::fmt;
use strum::EnumMessage;
use strum_macros;

#[derive(PartialEq, Debug, strum_macros::EnumMessage)]
pub enum SemanticError {
    #[strum(message = "Unknown game command, please try again")]
    #[strum(
        detailed_message = "There was an error parsing the inputed game command\nPlease try again"
    )]
    InvalidGameCmdSemantic,

    #[strum(message = "Unknown menu command, please try again")]
    #[strum(detailed_message = "There was an error parsing the inputed menu command")]
    InvalidMenuCmdSemantic,

    #[strum(message = "Bad dimension in provided coordinate (two expected), please try again")]
    #[strum(
        detailed_message = "Bad dimension in provided coordinate (two expected), please try again"
    )]
    InvalidCoordinateSemantic,
}

impl fmt::Display for SemanticError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.get_message().unwrap())
    }
}

impl error::Error for SemanticError {
    fn description(&self) -> &str {
        self.get_detailed_message().unwrap()
    }
}

#[derive(PartialEq, Debug, strum_macros::EnumMessage)]
pub enum MapError {
    #[strum(message = "Coordinates are out of bounds (bounds are 0..14)")]
    #[strum(detailed_message = "The given commands are out of the map")]
    OutOfBoundsError,

    #[strum(message = "You can only move 4 squares at a time")]
    #[strum(detailed_message = "The given coordinates are too far from your original posstion")]
    InvalidMovement,
}

impl fmt::Display for MapError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.get_message().unwrap())
    }
}

impl error::Error for MapError {
    fn description(&self) -> &str {
        self.get_detailed_message().unwrap()
    }
}
