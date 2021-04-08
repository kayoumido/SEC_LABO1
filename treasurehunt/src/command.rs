/*!

Note:
The follwoing is strongly inspired by the termcolor lib Color for all the numeric str and error handeling
*/

#[path = "utils.rs"]
mod utils;

use utils::parse_number;

use std::error;
use std::fmt;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlayerCmd {
    Start,
    Info,
    Move,
    Search,
    Quit,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CmdOrigin {
    Menu,
    Game,
}

impl PlayerCmd {
    pub fn from_str_numeric(s: &str, origin: &CmdOrigin) -> Result<Self, ParseCmdError> {
        if let Some(n) = parse_number(s) {
            match origin {
                CmdOrigin::Menu => Self::parse_menu_num_cmd(n),
                CmdOrigin::Game => Self::parse_game_num_cmd(n),
            }
        } else {
            Err(ParseCmdError {
                kind: ParseCmdErrorKind::InvalidCmd,
                given: s.to_string(),
            })
        }
    }

    pub fn from_str(s: &str, origin: &CmdOrigin) -> Result<Self, ParseCmdError> {
        match origin {
            &CmdOrigin::Menu => match &*s.to_lowercase() {
                "start" => Ok(Self::Start),
                "info" => Ok(Self::Info),
                "quit" => Ok(Self::Quit),
                _ => Self::from_str_numeric(s, origin),
            },
            &CmdOrigin::Game => match &*s.to_lowercase() {
                "move" => Ok(Self::Move),
                "search" => Ok(Self::Search),
                "quit" => Ok(Self::Quit),
                _ => Self::from_str_numeric(s, origin),
            },
        }
    }

    fn parse_menu_num_cmd(n: u8) -> Result<Self, ParseCmdError> {
        match n {
            1 => Ok(Self::Start),
            2 => Ok(Self::Info),
            3 => Ok(Self::Quit),
            _ => Err(ParseCmdError {
                kind: ParseCmdErrorKind::InvalidCmdNumber,
                given: n.to_string(),
            }),
        }
    }

    fn parse_game_num_cmd(n: u8) -> Result<Self, ParseCmdError> {
        match n {
            1 => Ok(Self::Move),
            2 => Ok(Self::Search),
            3 => Ok(Self::Quit),
            _ => Err(ParseCmdError {
                kind: ParseCmdErrorKind::InvalidCmdNumber,
                given: n.to_string(),
            }),
        }
    }
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParseCmdError {
    kind: ParseCmdErrorKind,
    given: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum ParseCmdErrorKind {
    InvalidCmd,
    InvalidCmdNumber,
}

impl ParseCmdError {
    /// Return the string that couldn't be parsed as a valid color.
    pub fn invalid(&self) -> &str {
        &self.given
    }
}

impl error::Error for ParseCmdError {
    fn description(&self) -> &str {
        use self::ParseCmdErrorKind::*;
        match self.kind {
            InvalidCmd => "unrecognized command",
            InvalidCmdNumber => "invalid command number",
        }
    }
}

impl fmt::Display for ParseCmdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use self::ParseCmdErrorKind::*;
        match self.kind {
            InvalidCmd => write!(
                f,
                "unrecognized command name '{}'. Choose from: \
                 start, info and quit if you're in the main menu \
                 otherwise move, search and quit.",
                self.given
            ),
            InvalidCmdNumber => write!(
                f,
                "unrecognized command number '{}', \
                 you should use the numbers indicated in the menu",
                self.given
            ),
        }
    }
}

#[cfg(test)]
mod test {
    use super::{CmdOrigin, ParseCmdError, ParseCmdErrorKind, PlayerCmd};

    #[test]
    fn test_cmd_from_string_ok() {
        let menu_cmd = PlayerCmd::from_str("Start", &CmdOrigin::Menu);
        let game_cmd = PlayerCmd::from_str("Move", &CmdOrigin::Game);

        assert_eq!(menu_cmd, Ok(PlayerCmd::Start));
        assert_eq!(game_cmd, Ok(PlayerCmd::Move));
    }

    #[test]
    fn test_from_decimal_ok() {
        let menu_cmd = PlayerCmd::from_str("2", &CmdOrigin::Menu);
        let game_cmd = PlayerCmd::from_str("2", &CmdOrigin::Game);

        assert_eq!(menu_cmd, Ok(PlayerCmd::Info));
        assert_eq!(game_cmd, Ok(PlayerCmd::Search));
    }

    #[test]
    fn test_from_hex_ok() {
        let menu_cmd = PlayerCmd::from_str("0x2", &CmdOrigin::Menu);
        let game_cmd = PlayerCmd::from_str("0x2", &CmdOrigin::Game);

        assert_eq!(menu_cmd, Ok(PlayerCmd::Info));
        assert_eq!(game_cmd, Ok(PlayerCmd::Search));
    }

    #[test]
    fn test_unknown_cmd_from_string_error() {
        let menu_cmd = PlayerCmd::from_str("unknown", &CmdOrigin::Menu);
        let game_cmd = PlayerCmd::from_str("unknown", &CmdOrigin::Game);

        let expected = Err(ParseCmdError {
            kind: ParseCmdErrorKind::InvalidCmd,
            given: "unknown".to_string(),
        });

        assert_eq!(menu_cmd, expected);
        assert_eq!(game_cmd, expected);
    }

    #[test]
    fn test_unknown_cmd_from_decimal_error() {
        let menu_cmd = PlayerCmd::from_str("45", &CmdOrigin::Menu);
        let game_cmd = PlayerCmd::from_str("45", &CmdOrigin::Game);

        let expected = Err(ParseCmdError {
            kind: ParseCmdErrorKind::InvalidCmdNumber,
            given: "45".to_string(),
        });

        assert_eq!(menu_cmd, expected);
        assert_eq!(game_cmd, expected);
    }

    #[test]
    fn test_unknown_cmd_from_hex_error() {
        let menu_cmd = PlayerCmd::from_str("0xFF", &CmdOrigin::Menu);
        let game_cmd = PlayerCmd::from_str("0xFF", &CmdOrigin::Game);

        let expected = Err(ParseCmdError {
            kind: ParseCmdErrorKind::InvalidCmdNumber,
            given: "255".to_string(),
        });

        assert_eq!(menu_cmd, expected);
        assert_eq!(game_cmd, expected);
    }

    #[test]
    fn test_game_command_string_as_menu_cmd_error() {
        let cmd = PlayerCmd::from_str("Move", &CmdOrigin::Menu);

        assert_eq!(
            cmd,
            Err(ParseCmdError {
                kind: ParseCmdErrorKind::InvalidCmd,
                given: "Move".to_string(),
            })
        );
    }

    #[test]
    fn test_menu_command_string_as_game_cmd_error() {
        let cmd = PlayerCmd::from_str("Start", &CmdOrigin::Game);

        assert_eq!(
            cmd,
            Err(ParseCmdError {
                kind: ParseCmdErrorKind::InvalidCmd,
                given: "Start".to_string(),
            })
        );
    }
}
