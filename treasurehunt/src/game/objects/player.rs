/*!
 * Struct to contain the player information (i.e. current position and colour)
 *
 * # Author
 * Doran Kayoumi <doran.kayoumi@heig-vd.ch>
 */
use termcolor::Color;
#[derive(Debug, Copy, Clone)]
pub struct Player {
    position: (u8, u8),
    colour: Color,
}

impl Player {
    /// Returns a new player with a colour and a
    pub fn new(position: (u8, u8), colour: Color) -> Self {
        Self { position, colour }
    }

    /// Changes the players coordinates
    ///
    /// # Arguments
    ///
    /// * `new_pos` - A tuple of u8 that holds the new coordinates
    ///
    pub fn set_position(&mut self, new_pos: (u8, u8)) {
        self.position = new_pos;
    }

    /// Returns the players current coordinates
    pub fn get_position(self) -> (u8, u8) {
        self.position
    }

    /// Returns the players colour
    pub fn get_colour(&self) -> Color {
        self.colour
    }
}
