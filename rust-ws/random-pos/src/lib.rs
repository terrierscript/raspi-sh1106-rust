
use rand::prelude::*;

pub fn random(max: i32) -> i32 {
    let rand : i32 = rand::random();
    let z = 1;
    return rand % max;
}
