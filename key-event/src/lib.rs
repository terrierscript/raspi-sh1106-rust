extern crate rppal;
extern crate sh1106;

use rppal::gpio::{Gpio, InputPin, Trigger};

// use std::borrow::Borrow;
// use std::collections::HashMap;
mod keymap;
use crate::keymap::Keymap;

pub trait CallbackFn: FnMut(String, i8) + Send + 'static + Copy {}

pub struct Pins {
    pub input_pin: InputPin,
    pub name: String,
}
pub fn get_pins<C>(mut key_event: C) -> Vec<InputPin>
where
    C: FnMut(String, i8) + Send + 'static + Copy,
{
    let gpio = Gpio::new().expect("Failed Gpio::new");
    let keymap = Keymap::new();
    let pins: Vec<InputPin> = keymap
        .keys()
        .into_iter()
        .map(move |pid| {
            let mut p = gpio.get(*pid).expect("get pin").into_input_pullup();
            let kk = Keymap::new();
            let setting = kk.get_setting(*pid).expect("invalid pin");
            let name = String::from(&setting.name);
            p.set_async_interrupt(setting.trigger, move |lv| {
                key_event(name.to_string(), lv as i8);
            })
            .expect("invalid set async interrupt");
            p
        })
        .collect();
    pins
}

pub fn hook_keyevent<C>(key_event: C)
where
    C: FnMut(String, i8) + Send + 'static + Copy,
{
    let pins = get_pins(key_event);
    loop {}
}

