use rand::Rng;

pub fn generate_coordinates(max_x: u8, max_y: u8) -> (u8, u8) {
    let mut rng = rand::thread_rng();
    (rng.gen_range(0..max_x) as u8, rng.gen_range(0..max_y) as u8)
}

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
