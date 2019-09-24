extern crate rppal;
extern crate sh1106;

use rppal::gpio::{Gpio, InputPin};

// use std::borrow::Borrow;
// use std::collections::HashMap;
mod keymap;
use crate::keymap::Keymap;

pub struct Pins {
    pub input_pin: InputPin,
    pub name: String,
}
pub fn hook_keyevent<C>(mut key_event: C) -> Vec<InputPin>
where
    C: FnMut(u8) + Send + 'static + Copy,
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
            let pp = pid.clone();
            // let name = String::from(&setting.name);
            p.set_async_interrupt(setting.trigger, move |lv| {
                key_event(pp); //, lv as i8);
            })
            .expect("invalid set async interrupt");
            p
        })
        .collect();
    pins
}

pub fn hook_keyevent2<C>(mut key_event: C)
where
    C: FnMut(u8) + Send + 'static,
{
    let ff = || {
        // key_event()
    };
    let gpio = Gpio::new().expect("Failed Gpio::new");
    let keymap = Keymap::new();
    let pins: Vec<_> = keymap
        .keys()
        .into_iter()
        .map(move |pid| {
            let mut p = gpio.get(*pid).expect("get pin").into_input_pullup();
            let kk = Keymap::new();
            let setting = kk.get_setting(*pid).expect("invalid pin");
            let pp = pid.clone();
            // let name = String::from(&setting.name);
            p.set_async_interrupt(setting.trigger, move |lv| {
                // return pp;
                // key_event(pp); //, lv as i8);
            })
            .expect("");
            // .expect("invalid set async interrupt");
            // p.async_interrupt
        })
        .collect();
    // pins
}

// pub fn pin_to_pin<'a>() -> &'static [&'a InputPin]{
//     // gpio.poll_interrupts(pins: &[&'a InputPin], reset: bool, timeout: Option<Duration>)
    
//     let ff = || {
//         // key_event()
//     };
//     let gpio = Gpio::new().expect("Failed Gpio::new");
//     let keymap = Keymap::new();
//     let pins: Vec<_> = keymap
//         .keys()
//         .into_iter()
//         .map(move |pid| {
//             let mut p = gpio.get(*pid).expect("get pin").into_input_pullup();
//             &p
//         })
//         .collect();
//     // let pp : Vec<InputPin> = pins.map(|p| &p).collect();
//     return pins.as_slice();
// }

// pub fn hook_keyevent<C>(key_event: C)
// where
//     C: FnMut(u8) + Send + 'static ,
// {
//     let pins = get_pins(key_event);
//     loop {}
// }
