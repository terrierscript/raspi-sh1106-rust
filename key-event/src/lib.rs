extern crate rppal;
extern crate sh1106;

use rppal::gpio::{Gpio, InputPin};

// use std::borrow::Borrow;
// use std::collections::HashMap;
mod keymap;
use std::sync::mpsc::channel;

use crate::keymap::Keymap;

pub struct Pins {
    pub input_pin: InputPin,
    pub name: String,
}

pub fn hook_keyevent<C>(mut key_event: C)
where
    C: FnMut(u8) + Send + 'static,
{
    let (tx, rx) = channel();

    let gpio = Gpio::new().expect("Failed Gpio::new");
    let keymap = Keymap::new();
    let _pins: Vec<InputPin> = keymap
        .keys()
        .into_iter()
        .map(move |pid| {
            let mut p = gpio.get(*pid as u8).expect("get pin").into_input_pullup();
            let kk = Keymap::new();
            let trigger = kk.get_setting(*pid as u8).expect("invalid pin");
            let pp = pid.clone();
            let tt = tx.clone();
            p.set_async_interrupt(*trigger, move |_lv| {
                tt.send(pp).expect("send tx");
            })
            .expect("invalid set async interrupt");
            p
        })
        .collect();
    loop {
        rx.try_recv().map( |r| key_event(r as u8)).ok();
    }
}
