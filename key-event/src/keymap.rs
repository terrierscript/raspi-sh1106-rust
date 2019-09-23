// use embedded_hal::digital::v2::InputPin;
use std::collections::HashMap;
use rppal::gpio::Trigger;

// use std::borrow::Borrow;
#[derive(Clone)]
pub struct Keymap {
    map: HashMap<u8, KeySetting>,
}

#[derive(Clone, Debug)]
pub struct KeySetting{
    pub name: String, 
    pub trigger: Trigger
}

fn get_keymap() -> HashMap<u8, KeySetting> {
    let hm: HashMap<_,_> = vec![
        (6,  "KEY_UP_PIN".to_string()   ,Trigger::FallingEdge),
        (19, "KEY_DOWN_PIN".to_string() ,Trigger::FallingEdge),
        (5,  "KEY_LEFT_PIN".to_string() ,Trigger::FallingEdge),
        (26, "KEY_RIGHT_PIN".to_string()    ,Trigger::FallingEdge),
        (13, "KEY_PRESS_PIN".to_string()    ,Trigger::RisingEdge),
        (21, "KEY1_PIN".to_string() ,Trigger::RisingEdge),
        (20, "KEY2_PIN".to_string() ,Trigger::RisingEdge),
        (16, "KEY3_PIN".to_string() ,Trigger::RisingEdge),
    ]
    .into_iter()
    .map(|(k, name, trigger)| {
        (k,KeySetting {
            name: name,
            trigger: trigger
        })
    })
    // .cloned()
    .collect();
    return hm;
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
    pub fn get_setting(&self, pin: u8) -> Option<&KeySetting> {
        match self.map.get(&pin) {
            Some(x) => Some(x),
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
                let key_setting = keymap.get_name(*k);
                println!("{:?}", key_setting);
            };
            x();
        }
    }
}
