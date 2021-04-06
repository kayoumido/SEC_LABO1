#[path = "player.rs"]
mod player;

use player::Player;

#[derive(Debug)]
pub struct Tile {
    player: Option<Player>,
    has_treasure: bool,
    visited: bool,
}

impl Tile {
    pub fn new() -> Tile {
        Tile {
            player: None,
            has_treasure: false,
            visited: false,
        }
    }

    pub fn set_has_treasure(&mut self, state: bool) {
        self.has_treasure = state;
    }
}
