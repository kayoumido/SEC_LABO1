use std::error;
use std::fmt;
use strum::EnumMessage;
use strum_macros;

#[derive(Debug, strum_macros::EnumMessage)]
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