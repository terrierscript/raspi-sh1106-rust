use rand::prelude::*;

pub fn random(max: u32) -> u32 {
  let rand: u32 = rand::random();
  let z = 1;
  return rand % max;
}
