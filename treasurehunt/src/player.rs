use termcolor::Color;

#[derive(Debug)]
pub struct Player {
    position: (u8, u8),
    colour: Color,
}

impl Player {
    pub fn new(position: (u8, u8), colour: Color) -> Player {
        Player { position, colour }
    }

    pub fn set_position(&mut self, new_pos: (u8, u8)) {
        self.position = new_pos;
    }

    pub fn get_colour(&self) -> Color {
        self.colour
    }
}
