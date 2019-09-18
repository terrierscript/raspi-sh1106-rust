extern crate rppal;
extern crate sh1106;

use rppal::gpio::{Gpio, InputPin, Trigger};

use std::borrow::Borrow;
use std::collections::HashMap;

fn main() -> ! {
    let mut keys = HashMap::new();
    keys.insert(6, "KEY_UP_PIN");
    keys.insert(19, "KEY_DOWN_PIN");
    keys.insert(5, "KEY_LEFT_PIN");
    keys.insert(26, "KEY_RIGHT_PIN");
    keys.insert(13, "KEY_PRESS_PIN");
    keys.insert(21, "KEY1_PIN");
    keys.insert(20, "KEY2_PIN");
    keys.insert(16, "KEY3_PIN");
    let gpio = Gpio::new().expect("Failed Gpio::new");

    println!("Hello, world!!!!");
    let v = vec![6, 19, 5, 26, 13, 21, 20, 16];
    let vec: Vec<InputPin> = v
        .into_iter()
        .map(|pid| {
            let mut p = gpio.get(pid).expect("get pin").into_input_pullup();
            p.set_interrupt(Trigger::Both);
            p.set_async_interrupt(Trigger::Both, move |lv| {
                let p = keys.get(&pid);
                println!("press {:?} {:?} {:?} ", p, pid, lv)
            })
            .expect("failed set interrupt");
            p
        })
        .collect();
    loop {}
}
