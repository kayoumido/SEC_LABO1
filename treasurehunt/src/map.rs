/*! Visual only `Map` functions

#
Add the missing part (`// TODO`).

You are free to modify anything, including the function parameters,
the code is provided as a support if desired.
*/
#[path = "player.rs"]
pub mod player;

#[path = "utils.rs"]
mod utils;

use std::io::{self, Write};
use termcolor::{BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};

pub struct Map {
    map: Vec<Vec<char>>,
    player: player::Player,
    treasure_position: (u8, u8),
}

impl Map {
    const DEFAULT_HEIGHT: u8 = 15;
    const DEFAULT_WIDTH: u8 = 15;

    const PLAYER_CHAR: char = 'P';
    const EMPTY_CHAR: char = '.';
    const VISITED_CHAR: char = '~';
    const TREASURE_CHAR: char = 'X';

    const MAP_COLOUR: Color = Color::White;

    pub fn new(mut player: player::Player) -> Self {
        let size = (Self::DEFAULT_WIDTH, Self::DEFAULT_HEIGHT);

        let treasure_coordinates = utils::generate_coordinates(size.0, size.1);
        let player_coordinates = utils::generate_coordinates(size.0, size.1);
        player.set_position(player_coordinates);
        // let player = Player::new(player_coordinates, Color::Green);

        let map = Self::init_map(size, player_coordinates, treasure_coordinates);

        Self {
            map,
            player,
            treasure_position: treasure_coordinates,
        }
    }

    /// Prints the `Map` to `stdout`.
    ///
    /// When the function returns, the terminal color is `White`.
    /// This functions requires definition of the `Map_WIDTH`, `Map_HEIGHT` and `Map_COLOR` constants
    pub fn print(&self) -> io::Result<()> {
        let bufwtr = BufferWriter::stdout(ColorChoice::Always);
        let mut buffer = bufwtr.buffer();

        let size = (self.map.len(), self.map[0].len());

        // Top row
        buffer.set_color(ColorSpec::new().set_fg(Some(Self::MAP_COLOUR)))?;
        write!(&mut buffer, "{:>4}", "⌜")?;
        for _ in 0..size.0 {
            write!(&mut buffer, "⎺-⎺")?;
        }
        writeln!(&mut buffer, "⌝")?;

        // Main grid
        for y in (0..size.1).rev() {
            write!(&mut buffer, "{:>2} ∣", y)?; // Side coordinates

            for x in 0..size.0 {
                let grid_c = self.map[x as usize][y as usize];
                let colour: Color;

                // check if we're about to display the player
                // so we can use the players colour and not the maps
                if grid_c == Self::PLAYER_CHAR {
                    colour = self.player.get_colour();
                } else {
                    colour = Self::MAP_COLOUR;
                }

                buffer.set_color(ColorSpec::new().set_fg(Some(colour)))?;
                write!(&mut buffer, "{:^3}", grid_c)?;
            }

            buffer.set_color(ColorSpec::new().set_fg(Some(Self::MAP_COLOUR)))?;
            writeln!(&mut buffer, "∣")?; // Side column
        }

        // Bottom row
        write!(&mut buffer, "{:>4}", "⌞")?;
        for _ in 0..size.0 {
            write!(&mut buffer, "_⎽_")?;
        }
        writeln!(&mut buffer, "⌟")?;

        // Bottom coordinates
        write!(&mut buffer, "{:4}", "")?;
        for x in 0..size.0 {
            write!(&mut buffer, "{:^3}", x)?;
        }
        writeln!(&mut buffer)?;

        buffer.set_color(ColorSpec::new().set_fg(Some(Color::White)))?;
        return bufwtr.print(&buffer);
    }

    pub fn move_player(&mut self, new_position: (u8, u8)) {
        self.player.set_position(new_position);
    }

    fn init_map(
        size: (u8, u8),
        player_coordinates: (u8, u8),
        treasure_coordinates: (u8, u8), // debug
    ) -> Vec<Vec<char>> {
        let mut map = vec![vec![Self::EMPTY_CHAR; size.0 as usize]; size.1 as usize];

        // place the player on the map
        map[player_coordinates.0 as usize][player_coordinates.1 as usize] = Self::PLAYER_CHAR;

        // DEBUG
        map[treasure_coordinates.0 as usize][treasure_coordinates.1 as usize] = Self::TREASURE_CHAR;

        map
    }

    pub fn restart() {}
}
