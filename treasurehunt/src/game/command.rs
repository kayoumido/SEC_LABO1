use strum_macros::EnumString;

#[derive(PartialEq, Debug, EnumString)]
pub enum GameCmd {
    #[strum(serialize = "Move", serialize = "move", serialize = "1")]
    Move,
    #[strum(serialize = "Search", serialize = "search", serialize = "2")]
    Search,
    #[strum(serialize = "Quit", serialize = "quit", serialize = "3")]
    Quit,
}
#[derive(PartialEq, Debug, EnumString)]
pub enum MenuCmd {
    #[strum(serialize = "Start", serialize = "start", serialize = "1")]
    Start,
    #[strum(serialize = "About", serialize = "about", serialize = "2")]
    About,
    #[strum(serialize = "Quit", serialize = "quit", serialize = "3")]
    Quit,
}

#[cfg(test)]
mod test {
    use rstest::rstest;
    use std::str::FromStr;
    use strum;

    use super::*;

    #[rstest(
        input,
        expected,
        case("Start", Ok(MenuCmd::Start)),
        case("start", Ok(MenuCmd::Start)),
        case("1", Ok(MenuCmd::Start)),
        case("About", Ok(MenuCmd::About)),
        case("about", Ok(MenuCmd::About)),
        case("2", Ok(MenuCmd::About)),
        case("Quit", Ok(MenuCmd::Quit)),
        case("quit", Ok(MenuCmd::Quit)),
        case("3", Ok(MenuCmd::Quit)),
        case("UnknownCmd", Err(strum::ParseError::VariantNotFound)),
        case("5", Err(strum::ParseError::VariantNotFound)),
        ::trace
    )]
    fn test_menu_cmd_from_string(input: &str, expected: Result<MenuCmd, strum::ParseError>) {
        assert_eq!(MenuCmd::from_str(input), expected);
    }

    #[rstest(
        input,
        expected,
        case("Move", Ok(GameCmd::Move)),
        case("move", Ok(GameCmd::Move)),
        case("1", Ok(GameCmd::Move)),
        case("Search", Ok(GameCmd::Search)),
        case("search", Ok(GameCmd::Search)),
        case("2", Ok(GameCmd::Search)),
        case("Quit", Ok(GameCmd::Quit)),
        case("quit", Ok(GameCmd::Quit)),
        case("3", Ok(GameCmd::Quit)),
        case("UnknownCmd", Err(strum::ParseError::VariantNotFound)),
        case("5", Err(strum::ParseError::VariantNotFound)),
        ::trace
    )]
    fn test_game_cmd_from_string(input: &str, expected: Result<GameCmd, strum::ParseError>) {
        assert_eq!(GameCmd::from_str(input), expected);
    }
}
