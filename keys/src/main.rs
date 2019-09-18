extern crate rppal;
extern crate sh1106;

use rppal::gpio::{Gpio, InputPin, Trigger};

use std::borrow::Borrow;
use std::collections::HashMap;
use keymap::Keymap;

// enum Keymap {
//     KEY_UP_PIN, //(6),
//     KEY_DOWN_PIN, //(19),
//     KEY_LEFT_PIN, //(5),
//     KEY_RIGHT_PIN, //(26),
//     KEY_PRESS_PIN, //(13),
//     KEY1_PIN, //(21),
//     KEY2_PIN, //(20),
//     KEY3_PIN, //(16),
    
// }

// fn pin_to_keymap(pin:u8) -> Option<Keymap> {
//     match pin {
//         6 => Some(Keymap::KEY_UP_PIN),
//         19 => Some(Keymap::KEY_DOWN_PIN),
//         5 => Some(Keymap::KEY_LEFT_PIN),
//         26 => Some(Keymap::KEY_RIGHT_PIN),
//         13 => Some(Keymap::KEY_PRESS_PIN),
//         21 => Some(Keymap::KEY1_PIN),
//         20 => Some(Keymap::KEY2_PIN),
//         16 => Some(Keymap::KEY3_PIN),
//         _ => None
//     }
// }

// const fn gen_keymap() -> HashMap<u8, &'static str> {
//     [
//         (6, "KEY_UP_PIN"),
//         (19, "KEY_DOWN_PIN"),
//         (5, "KEY_LEFT_PIN"),
//         (26, "KEY_RIGHT_PIN"),
//         (13, "KEY_PRESS_PIN"),
//         (21, "KEY1_PIN"),
//         (20, "KEY2_PIN"),
//         (16, "KEY3_PIN"),
//     ].iter().cloned().collect()
// }
// fn get_keymap() -> HashMap<u8, &'static str>{
//     [
//         (6, "KEY_UP_PIN"),
//         (19, "KEY_DOWN_PIN"),
//         (5, "KEY_LEFT_PIN"),
//         (26, "KEY_RIGHT_PIN"),
//         (13, "KEY_PRESS_PIN"),
//         (21, "KEY1_PIN"),
//         (20, "KEY2_PIN"),
//         (16, "KEY3_PIN"),
//     ].iter().cloned().collect()
// }

fn main() -> ! {
    let gpio = Gpio::new().expect("Failed Gpio::new");
    let keymap = Keymap::new();
    // println!("Hello, world!!!!");
    // let v = vec![6, 19, 5, 26, 13, 21, 20, 16];
    // let v = 
    let pins: Vec<InputPin> = keymap.keys()
        .into_iter()
        .map(move |pid| {
            let mut p = gpio.get(*pid).expect("get pin").into_input_pullup();
            p.set_interrupt(Trigger::Both);
            p
        })
        .collect();
    // gpio.poll_interrupts(pins: &[&'a InputPin], reset: bool, timeout: Option<Duration>)
    for mut p in pins.into_iter() {
        p.set_async_interrupt(Trigger::Both, move |lv| {
            let pin_id = p.pin();
            let name = keymap.get_name(pin_id);
            println!("press {:?} {:?} {:?} ", name, pin_id, lv)
        })
        .expect("failed set interrupt");
    }
    loop {}
}
