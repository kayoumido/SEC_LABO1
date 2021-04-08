use termcolor::Color;

#[derive(Debug, Copy, Clone)]
pub struct Player {
    position: (u8, u8),
    colour: Color,
}

impl Player {
    pub fn new(position: (u8, u8), colour: Color) -> Self {
        Self { position, colour }
    }

    pub fn set_position(&mut self, new_pos: (u8, u8)) {
        self.position = new_pos;
    }

    pub fn get_position(self) -> (u8, u8) {
        self.position
    }

    pub fn get_colour(&self) -> Color {
        self.colour
    }
}
