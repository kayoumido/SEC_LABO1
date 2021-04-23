/*! Visual only `Map` functions

#
Add the missing part (`// TODO`).

You are free to modify anything, including the function parameters,
the code is provided as a support if desired.
*/
#[path = "player.rs"]
pub mod player;

#[path = "../../utils.rs"]
mod utils;

#[path="../errors.rs"]
mod errors;

use std::io::{self, Write};
use termcolor::{BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};

use player::Player;

use errors::MapError;

pub struct Map {
    map: Vec<Vec<char>>,
    player: Player,
    treasure_position: (u8, u8),
}

impl Map {
    pub const DEFAULT_HEIGHT: u8 = 15;
    pub const DEFAULT_WIDTH: u8 = 15;

    const PLAYER_CHAR: char = 'P';
    const EMPTY_CHAR: char = '.';
    const VISITED_CHAR: char = '~';

    const MAP_COLOUR: Color = Color::White;

    const MAX_PLAYER_MOVEMENT: u8 = 4;

    /// Returns a map.
    /// It will also "place" (i.e. generate coordinates) a treasure and the given `player`
    ///
    /// # Arguments
    ///
    /// * `player` - Player to "place" in the map
    ///
    pub fn new(mut player: Player) -> Self {
        let size = (Self::DEFAULT_WIDTH, Self::DEFAULT_HEIGHT);

        // generate the coordinates for the treasure
        let treasure_coordinates = utils::generate_coordinates(size.0, size.1);

        // generate the coordinates for the player
        // we just need to make sure that they aren't the same as the treasure
        let mut player_coordinates: (u8, u8);
        loop {
            player_coordinates = utils::generate_coordinates(size.0, size.1);

            if player_coordinates != treasure_coordinates {
                break;
            }
        }
        player.set_position(player_coordinates);

        Self {
            map: Self::init_map(size),
            player,
            treasure_position: treasure_coordinates,
        }
    }

    /// Prints the `Map` to `stdout`.
    ///
    /// When the function returns, the terminal color is `White`.
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
                let mut grid_c = self.map[y][x];
                let mut colour = Self::MAP_COLOUR;

                // check if we're about to display the player
                // so we can use the players colour and not the maps
                if (x as u8, y as u8) == self.player.get_position() {
                    grid_c = Self::PLAYER_CHAR;
                    colour = self.player.get_colour();
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

    /// Moves the `player` to the given new coordinates
    ///
    /// # Arguments
    ///
    /// * `new_coordinates` - A tuple of u8 that contains the new coordinates
    ///
    pub fn move_player(&mut self, new_coordinates: (u8, u8)) {
        self.player.set_position(new_coordinates);
    }

    /// Checks if the players current position is the same as the treasures
    pub fn search(&mut self) -> bool {
        let player_pos = self.player.get_position();
        if self.player.get_position() != self.treasure_position {
            self.map[player_pos.1 as usize][player_pos.0 as usize] = Self::VISITED_CHAR;
            false
        } else {
            true
        }
    }

    /// Check that a destination coordinate corresponds a valid movement.
    /// i.e. it doesn't exceed a movement of `MAX_PLAYER_MOVEMENT`
    ///
    /// # Arguments
    ///
    /// * `dest` - A tuple of u8 containing the destination coordinates
    pub fn is_valid_movement(&self, dest: (u8, u8)) -> Result<(), MapError> {
        let src = self.player.get_position();
        // we need to add 1 because the end of the range is still a valid movement
        let x_boundary_end = src.0 + Self::MAX_PLAYER_MOVEMENT + 1;
        let y_boudary_end = src.0 + Self::MAX_PLAYER_MOVEMENT + 1;

        let x_boundary_start: u8;
        let y_boundary_start: u8;
        // check the src is close to the edge (i.e. 0..MAX_MOVEMENT)
        // if so, we set the low x to 0 to avoid any overflows.
        if (0..Self::MAX_PLAYER_MOVEMENT).contains(&src.0) {
            x_boundary_start = 0;
        } else {
            x_boundary_start = src.0 - Self::MAX_PLAYER_MOVEMENT;
        }

        // same idea as before but this time for the low y
        if (0..Self::MAX_PLAYER_MOVEMENT).contains(&src.1) {
            y_boundary_start = 0;
        } else {
            y_boundary_start = src.1 - Self::MAX_PLAYER_MOVEMENT;
        }

        if (x_boundary_start..x_boundary_end).contains(&dest.0) && (y_boundary_start..y_boudary_end).contains(&dest.1) {
            Ok(())
        } else {
            Err(MapError::InvalidMovement)
        }
    }

    /// Check that a given coordinate is within the borad boundaries
    ///
    ///
    /// # Arguments
    ///
    /// * `coordinate` - A tuple of u8 containing the coordiante to check
    ///
    pub fn within_boundries(&self, coordinate: (Option<u8>, Option<u8>)) -> Result<(), MapError> {
        if (coordinate.0 != None && coordinate.1 != None) && (0..self.map[0].len()).contains(&(coordinate.0.unwrap() as usize)) && (0..self.map.len()).contains(&(coordinate.1.unwrap() as usize)) {
            Ok(())
        } else {
            Err(MapError::OutOfBoundsError)
        }
    }

    /// Returns the Euclidean distance between the player and the treasure
    pub fn distance_to_treasure(&self) -> f64 {
        utils::eucldean_distance(self.player.get_position(), self.treasure_position)
    }

    /// Creates a `Vec<Vec<char>` filled with `EMPTY_CHAR`
    ///
    /// # Arguments
    ///
    /// * `size` - A tuple of u8 containing the dimesions of the map
    fn init_map(size: (u8, u8)) -> Vec<Vec<char>> {
        vec![vec![Self::EMPTY_CHAR; size.0 as usize]; size.1 as usize]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn setup() -> Map {
        let player = Player::new((0, 0), Color::White);
        Map::new(player)
    }

    #[test]
    fn test_player_movement() {
        let mut map = setup();
        let new_position: (u8, u8) = (5, 5);
        let old_position = map.player.get_position();
        map.move_player(new_position);

        assert_ne!(new_position, old_position);
        assert_eq!(map.player.get_position(), new_position);
    }

    #[test]
    fn test_map_init() {
        let size = (3, 2);
        let expected_content = vec![Map::EMPTY_CHAR; size.0 as usize];
        let map = Map::init_map(size);

        assert!(map.contains(&expected_content));
        assert!(map[0].contains(&Map::EMPTY_CHAR));

        assert_eq!(map.len(), size.1 as usize);
        assert_eq!(map[0].len(), size.0 as usize);
    }

    #[test]
    fn test_treasure_searching() {
        let mut map = setup();
        assert!(!map.search());

        let treasure_pos = map.treasure_position;
        map.move_player(treasure_pos);

        assert!(map.search());
    }
}
