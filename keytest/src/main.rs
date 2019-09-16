extern crate rppal;
extern crate sh1106;

// use rppal::{gpio, spi};
use rppal::gpio::{Gpio, InputPin,Trigger};
// use rppal::spi::{Spi, Bus, SlaveSelect};
// use spidev_sh1106::SpidevSH1106;

// use sh1106::prelude::*;
// use sh1106::Builder;

use std::collections::HashMap;
use std::borrow::Borrow;   
// use std::time::Duration;

// fn pin<'a>(gpio: &'a Gpio, pin_id: &'a i32) -> &'a InputPin {
//     return 
// }

// fn d_pin<'a>(gpio: Gpio, pid: u8) -> &'a InputPin {
//     let mut p = gpio.get(pid).expect("get pin").into_input_pullup();
//     p.set_interrupt(Trigger::Both).expect("failed set interrupt");
//     &p
// }

// fn hook_key( pid: u8, key_name: String) {
//     // todo
//     let gpio = Gpio::new().expect( "Failed Gpio::new" );
    
//     let mut b1 : InputPin = {
//         let mut p = gpio.get(pid).expect("get pin").into_input_pullup();
//         p.set_interrupt(Trigger::Both).expect("failed set interrupt");
//         p
//     };
//     println!("start {}", key_name);
//     // b1.poll_interrupt()
//     loop {
//         // println!("loop");
//         let s = b1.poll_interrupt(false, None).expect("v");
//         // println!("{:?}", );
//         match s {
//             Some(n) => {
//                 println!("press {} {}", key_name, n)
//             },
//             None => {
//                 println!("none")
//             }
//         }
//     }
// }

fn main() {
    let mut keys = HashMap::new();
    keys.insert(    6, "KEY_UP_PIN"       );
    keys.insert(    19, "KEY_DOWN_PIN"    );
    keys.insert(    5, "KEY_LEFT_PIN"    );
    keys.insert(    26, "KEY_RIGHT_PIN"   );
    keys.insert(    13, "KEY_PRESS_PIN"   );
    keys.insert(    21, "KEY1_PIN"        );
    keys.insert(    20, "KEY2_PIN"        );
    keys.insert(    16, "KEY3_PIN"        );
    let gpio = Gpio::new().expect( "Failed Gpio::new" );
    'a : {
        println!("Hello, world!!!!");
        let vec : Vec<&InputPin> = keys.iter_mut().map(|(pid, _)| {
            let mut p : &'a InputPin = gpio.get(*pid).expect("get pin").into_input_pullup();
            p.set_interrupt(Trigger::Both).expect("failed set interrupt");
            &p
        }).collect();
        // let v2 : Vec<&InputPin> = vec.into_iter().map(|v| &v).collect();

        let pins = vec.as_slice();

        loop {
            // println!("loop");
            let s = gpio.poll_interrupts(pins, true, None).expect("v");
            // println!("{:?}", );
            match s {
                Some((pin,_)) => {
                    let key_name = keys
                        .get_mut(&pin.pin())
                        .expect("invalid")
                        .to_string();
                    println!("press {:?} ", key_name)
                },
                None => {
                    println!("none")
                }
            }
        }
    }

}



