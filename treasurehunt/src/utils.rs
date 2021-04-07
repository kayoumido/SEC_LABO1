use rand::Rng;

pub fn generate_coordinates(max_x: u8, max_y: u8) -> (u8, u8) {
    let mut rng = rand::thread_rng();
    (rng.gen_range(0..max_x) as u8, rng.gen_range(0..max_y) as u8)
}
