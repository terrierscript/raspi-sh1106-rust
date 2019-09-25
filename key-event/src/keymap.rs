// use embedded_hal::digital::v2::InputPin;
use rppal::gpio::Trigger;
use std::collections::HashMap;
use keyenum::KeyEnum;

// use std::borrow::Borrow;
#[derive(Clone)]
pub struct Keymap {
    map: HashMap<KeyEnum, Trigger>,
}

fn get_keymap() -> HashMap<KeyEnum, Trigger> {
    let hm: HashMap<KeyEnum, Trigger> = vec![
        (KeyEnum::KeyUpPin, Trigger::FallingEdge),
        (KeyEnum::KeyDownPin, Trigger::FallingEdge),
        (KeyEnum::KeyLeftPin, Trigger::FallingEdge),
        (KeyEnum::KeyRightPin, Trigger::FallingEdge),
        (KeyEnum::KeyPressPin, Trigger::RisingEdge),
        (KeyEnum::Key1Pin, Trigger::RisingEdge),
        (KeyEnum::Key2Pin, Trigger::RisingEdge),
        (KeyEnum::Key3Pin, Trigger::RisingEdge),
    ]
    .iter()
    .cloned()
    .collect();
    return hm;
}

impl Keymap {
    pub fn new() -> Self {
        Keymap { map: get_keymap() }
    }

    pub fn keys(&self) -> Vec<&KeyEnum> {
        self.map.keys().collect::<Vec<&KeyEnum>>()
    }

    pub fn get_setting(&self, pin: u8) -> Option<&Trigger> {
         keyenum::from_u8(pin).map_or(None, |p| 
            self.map.get(&p).or(None)
        )
    }
}

#[cfg(test)]
mod test {
    use crate::Keymap;

    #[test]
    pub fn test1() {
        let keymap = Keymap::new();
        for k in keymap.keys().into_iter() {
            let x = || {
                let key_setting = keymap.get_setting(*k as u8);
                println!("{:?}", key_setting);
            };
            x();
        }
    }
}
