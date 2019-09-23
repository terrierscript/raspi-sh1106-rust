extern crate rppal;
extern crate sh1106;

use rppal::gpio::{Gpio, InputPin, Trigger};

// use std::borrow::Borrow;
// use std::collections::HashMap;
use keymap::Keymap;

pub struct Pins {
    pub input_pin: InputPin,
    pub name: String,
}
pub fn get_pins() -> Vec<Pins> {
    let gpio = Gpio::new().expect("Failed Gpio::new");
    let keymap = Keymap::new();
    let pins: Vec<InputPin> = keymap
        .keys()
        .into_iter()
        .map(move |pid| {
            let mut p = gpio.get(*pid).expect("get pin").into_input_pullup();
            // p.set_interrupt(Trigger::Both).expect("set interrupt");
            p
        })
        .collect();
    pins.into_iter()
        .map(|p| {
            let pin_id = p.pin();
            let name = keymap.get_name(pin_id).expect("invalid pin");

            Pins {
                input_pin: p,
                name: name,
            }
        })
        .collect()
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

pub fn hook_keyevent<C>(mut key_event: C)
where
    C: FnMut(String, i8) + Send + 'static,
{
        loop {

    let pins = get_pins();
    let keymap = Keymap::new();

    for mut p in pins.into_iter() {
        let pin_id = p.input_pin.pin();
        let name = keymap.get_name(pin_id).expect("invalid pin");
        // p.input_pin.set_async_interrupt(Trigger::Both, key_event);
        // p.input_pin.set_interrupt(Trigger::Both).expect("p");
        // p.input_pin
        //     .set_interrupt(Trigger::Both)
        //     .expect("set interrupt");
        p.input_pin
            .set_async_interrupt(Trigger::Both, move |lv| {
                println!("{:?} press", name.clone());
                //     // let int_lv:u8 = lv.into();
                println!("press {:?} {:?} {:?} ", name, pin_id, lv);
                // key_event(name.clone(), lv as i8);
            })
            .expect("failed set interrupt");
    }
        
    }
}
