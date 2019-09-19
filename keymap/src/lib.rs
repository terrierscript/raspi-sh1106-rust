// use embedded_hal::digital::v2::InputPin;
use std::collections::HashMap;
// use std::borrow::Borrow;

pub struct Keymap {
    map: HashMap<u8, &'static str>,
}

fn get_keymap() -> HashMap<u8, &'static str> {
    [
        (6, "KEY_UP_PIN"),
        (19, "KEY_DOWN_PIN"),
        (5, "KEY_LEFT_PIN"),
        (26, "KEY_RIGHT_PIN"),
        (13, "KEY_PRESS_PIN"),
        (21, "KEY1_PIN"),
        (20, "KEY2_PIN"),
        (16, "KEY3_PIN"),
    ]
    .iter()
    .cloned()
    .collect()
}

impl Keymap {
    pub fn new() -> Self {
        Keymap { map: get_keymap() }
    }

    pub fn keys(&self) -> Vec<&u8> {
        self.map.keys().collect::<Vec<&u8>>()
        // .clone()
        // .borrow()
    }
    pub fn get_name(&self, pin: u8) -> Option<String> {
        match self.map.get(&pin) {
            Some(x) => Some(x.to_string()),
            None => None,
        }
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
                let name = keymap.get_name(*k);
                println!("{:?}", name);
            };
            x();
        }
    }
}
