/*! Visual only `Map` functions

#
Add the missing part (`// TODO`).

You are free to modify anything, including the function parameters,
the code is provided as a support if desired.
*/
#[path = "player.rs"]
mod player;

use rand::Rng;
use std::{
    io::{self, Write},
    u32,
};
use termcolor::{BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};

use player::Player;

pub struct Map {
    size: (u8, u8),
    map: Vec<Vec<char>>,
    player: Player,
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

    pub fn new() -> Map {
        let size = (Map::DEFAULT_WIDTH, Map::DEFAULT_HEIGHT);
        let mut map = vec![vec![Map::EMPTY_CHAR; size.0 as usize]; size.1 as usize];

        let mut player = Player::new((0, 0), Color::Green);

        // generate the coordinates for the player and the treasure
        let mut rng = rand::thread_rng();
        let player_coordinates = (
            rng.gen_range(0..Map::DEFAULT_WIDTH) as u8,
            rng.gen_range(0..Map::DEFAULT_HEIGHT) as u8,
        );
        player.set_position(player_coordinates);

        let treasure_coordinates = (
            rng.gen_range(0..Map::DEFAULT_WIDTH) as u8,
            rng.gen_range(0..Map::DEFAULT_HEIGHT) as u8,
        );

        // place the player on the map
        map[player_coordinates.0 as usize][player_coordinates.1 as usize] = Map::PLAYER_CHAR;

        // DEBUG
        map[treasure_coordinates.0 as usize][treasure_coordinates.1 as usize] = Map::TREASURE_CHAR;

        Map {
            size,
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

        // Top row
        // buffer.set_color(ColorSpec::new().set_fg(Some(Map_COLOR)))?;
        write!(&mut buffer, "{:>4}", "⌜")?;
        for _ in 0..self.size.0 {
            write!(&mut buffer, "⎺-⎺")?;
        }
        writeln!(&mut buffer, "⌝")?;

        // Main grid
        for y in (0..self.size.1).rev() {
            write!(&mut buffer, "{:>2} ∣", y)?; // Side coordinates

            for x in 0..self.size.0 {
                let grid_c = self.map[x as usize][y as usize];
                let colour: Color;

                if grid_c == Map::PLAYER_CHAR {
                    colour = self.player.get_colour();
                } else {
                    colour = Map::MAP_COLOUR;
                }

                buffer.set_color(ColorSpec::new().set_fg(Some(colour)))?;
                write!(&mut buffer, "{:^3}", grid_c)?;
                buffer.set_color(ColorSpec::new().set_fg(Some(Color::White)))?;
            }

            writeln!(&mut buffer, "∣")?; // Side column
        }

        // Bottom row
        write!(&mut buffer, "{:>4}", "⌞")?;
        for _ in 0..self.size.0 {
            write!(&mut buffer, "_⎽_")?;
        }
        writeln!(&mut buffer, "⌟")?;

        // Bottom coordinates
        write!(&mut buffer, "{:4}", "")?;
        for x in 0..self.size.0 {
            write!(&mut buffer, "{:^3}", x)?;
        }
        writeln!(&mut buffer)?;

        buffer.set_color(ColorSpec::new().set_fg(Some(Color::White)))?;
        return bufwtr.print(&buffer);
    }
}
