/*!
 * Functions to check if the user input is semanticly valid
 * 
 * # Author
 * Doran Kayoumi <doran.kayoumi@heig-vd.ch>
 */

use std::str::FromStr;
use termcolor::Color;

#[path = "../command.rs"]
pub mod command;

#[path = "../errors.rs"]
mod errors;

use command::{GameCmd, MenuCmd};
use errors::SemanticError;

/// Check if a given str is a semanticly a valid game command
/// i.e. GameCmd enum can parse it ^^'
/// 
/// # Arguments
/// 
/// * `cmd` - the str to check
/// 
pub fn check_game_cmd_semantic(cmd: &str) -> Result<(), SemanticError> {
    if let Err(_) = GameCmd::from_str(&cmd) {
        Err(SemanticError::InvalidGameCmdSemantic)
    } else {
        Ok(())
    }
}

/// Check if a given str is a semanticly a valid menu command
/// i.e. MenuCmd enum can parse it ^^'
/// 
/// # Arguments
/// 
/// * `cmd` - the str to check
/// 
pub fn check_menu_cmd_semantic(cmd: &str) -> Result<(), SemanticError> {
    if let Err(_) = MenuCmd::from_str(&cmd) {
        Err(SemanticError::InvalidMenuCmdSemantic)
    } else {
        Ok(())
    }
}

/// Check if a given str is a semanticly a valid coordinate
/// i.e. it only has two elements
/// 
/// # Arguments
/// 
/// * `coord` - the str to check
/// 
pub fn check_coordinate_semantic(coord: &Vec<&str>) -> Result<(), SemanticError> {
    if coord.len() != 2 {
        Err(SemanticError::InvalidCoordinateSemantic)
    } else {
        Ok(())
    }
}

/// Check if a given str is a semanticly a valid colour
/// i.e. Color enum can parse it ^^'
/// 
/// # Arguments
/// 
/// * `colour` - the str to check
/// 
pub fn check_colour_semantic(colour: &str) -> Result<(), termcolor::ParseColorError> {
    if let Err(e) = Color::from_str(&colour) {
        Err(e)
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    use super::*;

    #[rstest(
        input, 
        expected, 
        case("Move", Ok(())),
        case("Search", Ok(())),
        case("quit", Ok(())),
        case("1", Ok(())),
        case("3", Ok(())),
        case("Unknown", Err(SemanticError::InvalidGameCmdSemantic)),
        case("Start", Err(SemanticError::InvalidGameCmdSemantic)),
        case("16", Err(SemanticError::InvalidGameCmdSemantic)),
        case("M0v3", Err(SemanticError::InvalidGameCmdSemantic))
    )]
    fn test_check_game_cmd_semantic(input: &str, expected: Result<(), SemanticError>) {
        assert_eq!(check_game_cmd_semantic(input), expected);
    }

    #[rstest(
        input, 
        expected, 
        case("Start", Ok(())),
        case("About", Ok(())),
        case("quit", Ok(())),
        case("1", Ok(())),
        case("3", Ok(())),
        case("Unknown", Err(SemanticError::InvalidMenuCmdSemantic)),
        case("Move", Err(SemanticError::InvalidMenuCmdSemantic)),
        case("16", Err(SemanticError::InvalidMenuCmdSemantic)),
        case("M0v3", Err(SemanticError::InvalidMenuCmdSemantic))
    )]
    fn test_check_menu_cmd_semantic(input: &str, expected: Result<(), SemanticError>) {
        assert_eq!(check_menu_cmd_semantic(input), expected);
    }

    #[rstest(
        input, 
        expected, 
        case(&vec!["1", "2"], Ok(())),
        case(&vec!["15", "20"], Ok(())),
        case(&vec!["0", "0"], Ok(())),
        case(&vec!["1", "2", "5"], Err(SemanticError::InvalidCoordinateSemantic)),
    )]
    fn test_check_coordinate_semantic(input: &Vec<&str>, expected: Result<(), SemanticError>) {
        assert_eq!(check_coordinate_semantic(input), expected);
    }

    #[rstest(
        input, 
        expected, 
        case("Green", Ok(())),
        case("blue", Ok(())),
        case("magenta", Ok(())),
        case("255,51,204", Ok(())),

        // Note: I couldn't test invalid inputs because the `ParseColorError`
        //       requires a kind of `ParseColorErrorKind` but it's private.
        //       So instead of hitting my head against a wall, I choose to put this 
        //       aside for the time being (i.e. 24.05.21) and come back to it if I have time.
        // It should look something like this:
        // case("(255,51,204)", Err(termcolor::ParseColorError {
        //     kind: termcolor::ParseColorErrorKind::InvalidRgb,
        //     given: "(255,51,204)".to_string()
        // })),
    )]
    fn test_check_colour_semantic(input: &str, expected: Result<(), termcolor::ParseColorError>) {
        assert_eq!(check_colour_semantic(input), expected);
    }
}
