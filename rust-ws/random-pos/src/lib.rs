#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        random();
    }
}
use rand::Rng;

pub fn random(max: i32) -> i32 {
    let mut rng = rand::thread_rng();
    let z = 1;
    return rng.gen<i32>() % max;
}
