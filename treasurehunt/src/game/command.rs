use strum_macros::EnumString;

#[derive(EnumString)]
pub enum GameCmd {
    #[strum(serialize = "Move", serialize = "move", serialize = "1")]
    Move,
    #[strum(serialize = "Search", serialize = "search", serialize = "2")]
    Search,
    #[strum(serialize = "Quit", serialize = "quit", serialize = "3")]
    Quit,
}
#[derive(EnumString)]
pub enum MenuCmd {
    #[strum(serialize = "Start", serialize = "start", serialize = "1")]
    Start,
    #[strum(serialize = "Info", serialize = "info", serialize = "2")]
    Info,
    #[strum(serialize = "Quit", serialize = "quit", serialize = "3")]
    Quit,
}
