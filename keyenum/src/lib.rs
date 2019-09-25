use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[derive(FromPrimitive, Clone, Copy, PartialEq, Eq,Hash)]
pub enum KeyEnum {
    KeyUpPin = 6,
    KeyDownPin = 19,
    KeyLeftPin = 5,
    KeyRightPin = 26,
    KeyPressPin = 13,
    Key1Pin = 21,
    Key2Pin = 20,
    Key3Pin = 16,
}

pub fn from_u8(num: u8) -> Option<KeyEnum> {
    FromPrimitive::from_u8(num)
}
