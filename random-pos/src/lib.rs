
pub fn random(max: u32) -> u32 {
    let rand: u32 = rand::random();
    rand % max
}
