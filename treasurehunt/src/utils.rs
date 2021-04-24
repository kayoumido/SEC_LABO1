use rand::Rng;

/// Generate coordinates
///
/// # Arguments
///
/// * `max_x` - the max value the x coordinate can take (well max-1)
/// * `max_y` - the max value the y coordinate can take (well max-1)
///
pub fn generate_coordinates(max_x: usize, max_y: usize) -> (u8, u8) {
    let mut rng = rand::thread_rng();
    (rng.gen_range(0..max_x) as u8, rng.gen_range(0..max_y) as u8)
}

/// Calculate the euclidean distance between two coordinates
///
/// # Arguments
///
/// * `p` - the first coordinate (i.e. source)
/// * `q` - the second coordinate (i.e. destination)
pub fn euclidean_distance(p: (u8, u8), q: (u8, u8)) -> f64 {
    let p = (p.0 as f64, p.1 as f64);
    let q = (q.0 as f64, q.1 as f64);
    (((q.0 - p.0).powf(2.) + (q.1 - p.1).powf(2.)) as f64).sqrt()
}

/// Parses a string number (decimal or hex) to a decimal u8
///
// Note: This function comes from the termcolor lib
//       source: https://docs.rs/termcolor/1.1.2/src/termcolor/lib.rs.html#1850
pub fn parse_number(s: &str) -> Option<u8> {
    use std::u8;

    if s.starts_with("0x") {
        u8::from_str_radix(&s[2..], 16).ok()
    } else {
        u8::from_str_radix(&s, 10).ok()
    }
}

/// Cleans a string
/// i.e. it removes parentheses, brackets & spaces
///
/// # Arguments
///
/// * `s` - the string to clean
pub fn clean_str(s: &str) -> String {
    s.replace(&['(', ')', '[', ']', ' '][..], "")
}

#[cfg(test)]
mod test {
    use super::{clean_str, parse_number};
    use rstest::rstest;

    #[rstest(
        input,
        expected,
        case("8", Some(8)),
        case("255", Some(255)),
        case("0xFF", Some(255)),
        case("0xA", Some(10)),
        case("notanumber", None)
        ::trace
    )]
    fn test_parse_number(input: &str, expected: Option<u8>) {
        assert_eq!(parse_number(input), expected);
    }

    #[rstest(
        input,
        expected,
        case("Hello World", "HelloWorld"),
        case("(1, 2, 3)", "1,2,3"),
        case("[4,5,6]", "4,5,6"),
        case("nothing", "nothing")
        ::trace
    )]
    fn test_clean_str(input: &str, expected: &str) {
        assert_eq!(clean_str(input), expected);
    }
}
