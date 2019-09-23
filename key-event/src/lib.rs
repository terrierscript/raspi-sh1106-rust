extern crate rppal;
extern crate sh1106;

use rppal::gpio::{Gpio, InputPin, Trigger};

// use std::borrow::Borrow;
// use std::collections::HashMap;
use keymap::Keymap;

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
            let name = kk.get_name(*pid).expect("invalid pin");
            p.set_interrupt(Trigger::Both).expect("set interrupt");
            p.set_async_interrupt(Trigger::Both, move |lv| {
                // let name = "a";
                // println!("aaa");
                // cb();
                // TOOD
                // let name = kk.get_name(*pid).expect("invalid pin");
                key_event(name.to_string(), lv as i8);
            })
            .expect("invalid set async interrupt");
            p
        })
        .collect();
    pins
    // pins.into_iter()
    //     .map(|p| {
    //         let pin_id = p.pin();
    //         let name = keymap.get_name(pin_id).expect("invalid pin");

    //         Pins {
    //             input_pin: p,
    //             name: name,
    //         }
    //     })
    //     .collect()
}

// type EventFn = Fn(String, i8) -> ();
// pub fn hook_keyevent_test<C>(mut key_event: C) where
//     C: FnMut(String, i8) + Send + 'static
// {
//     let gpio = Gpio::new().expect("Failed Gpio::new");

//     let mut pin = gpio.get(21).expect("failed gpio pin").into_input_pullup();
//     pin.set_interrupt(Trigger::Both).expect("set interrupt");
//     pin.set_async_interrupt(Trigger::Both, move |lv| {
//         println!("{:?} press", lv);
//         key_event("foo".to_string(), lv as i8);

//     })
//     .expect("failed async");
//     loop{}
// }
pub fn hook_keyevent<C>(key_event: C)
where
    C: FnMut(String, i8) + Send + 'static + Copy, //FnMut(String, i8) + Send + 'static,
{
    // loop {
    let pins = get_pins(key_event);
    // let keymap = Keymap::new();
    loop {}
}
