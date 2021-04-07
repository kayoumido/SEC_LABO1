/*!

Note:
The follwoing is strongly inspired by the termcolor lib Color for all the numeric str and error handeling
*/

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
        /// Parses a string number (decimal or hex) to a decimal u8
        ///
        // Note: This function comes from the termcolor lib
        //       source: https://docs.rs/termcolor/1.1.2/src/termcolor/lib.rs.html#1850
        fn parse_number(s: &str) -> Option<u8> {
            use std::u8;

            if s.starts_with("0x") {
                u8::from_str_radix(&s[2..], 16).ok()
            } else {
                u8::from_str_radix(s, 10).ok()
            }
        }

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

/// An error from parsing an invalid color specification.
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
