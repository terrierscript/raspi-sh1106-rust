use num_derive::FromPrimitive;    
use num_traits::FromPrimitive;

#[derive(FromPrimitive)]
pub enum KeyEnum {
  KEY_UP_PIN= 6,
  KEY_DOWN_PIN= 19,
  KEY_LEFT_PIN= 5,
  KEY_RIGHT_PIN= 26,
  KEY_PRESS_PIN= 13,
  KEY1_PIN= 21,
  KEY2_PIN= 20,
  KEY3_PIN    = 16,
}

pub fn from_u8(num: u8) -> Option<KeyEnum> {
  return FromPrimitive::from_u8(num);
}