extern crate rppal;
extern crate sh1106;

// use rppal::{gpio, spi};
use rppal::gpio::{Gpio, InputPin,Trigger};
// use rppal::spi::{Spi, Bus, SlaveSelect};
// use spidev_sh1106::SpidevSH1106;

// use sh1106::prelude::*;
// use sh1106::Builder;

use std::collections::HashMap;

// use std::time::Duration;

// fn pin<'a>(gpio: &'a Gpio, pin_id: &'a i32) -> &'a InputPin {
//     return 
// }


fn hook_key( pid: u8, key_name: String) {
    // todo
    let gpio = Gpio::new().expect( "Failed Gpio::new" );
    
    let mut b1 : InputPin = {
        let mut p = gpio.get(pid).expect("get pin").into_input_pullup();
        p.set_interrupt(Trigger::Both).expect("failed set interrupt");
        p
    };
    println!("start {}", key_name);
    // b1.poll_interrupt()
    loop {
        // println!("loop");
        let s = b1.poll_interrupt(false, None).expect("v");
        // println!("{:?}", );
        match s {
            Some(n) => {
                println!("press {} {}", key_name, n)
            },
            None => {
                println!("none")
            }
        }
    }
}

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

    println!("Hello, world!!!!");
    for (pid, key_name) in keys.iter(){
        hook_key( *pid, key_name.to_string())
    };
    // let mut b1 : InputPin = {
    //     let mut p = gpio.get(21).expect("get 21 pin").into_input_pullup();
    //     p.set_interrupt(Trigger::Both).expect("failed set interrupt");
    //     p
    // };
    // b1.poll_interrupt()
    // loop {
    //     // println!("loop");
    //     let s = b1.poll_interrupt(false, None).expect("v");
    //     // println!("{:?}", );
    //     match s {
    //         Some(n) => {
    //             println!("N")
    //         },
    //         None => {
    //             println!("none")
    //         }
    //     }
    //     // let intr = gpio.poll_interrupts(
    //     //     &[&b1],
    //     //     true, 
    //     //     Some(Duration::new(1,0))
    //     // ).expect("invalid intr");
    //     // match intr {
    //     //     None => { println!("loop")},
    //     //     Some((_pin,_)) => {
    //     //         println!("pinn!")
    //     //     }
    //     // }

    //     // let z = b1.read();
    //     // println!("loop {}", z);
    // }
    // let pin_ids = &[6,19,5,26,13,21,20,16];
    // let pin_ids = &[21,20,16];
    // let pins: Vec<_> = pin_ids.iter()
    //     .map(|pid| gpio
    //         .get(*pid)
    //         .expect("Cannot pin")
    //         .into_input_pullup()
    //     )
    //     .collect();

    // let ps = pins.as_slice();
    // println!("loop start");
    
    // loop{
    //     let intr = gpio.poll_interrupts(
    //         // pins,
    //         &[
    //             &pins[0],
    //             &pins[1],
    //             &pins[2],
    //         ],
    //         true, 
    //         Some(Duration::new(1,0))
    //     ).expect("invalid intr");
    //     match intr {
    //         None => { println!("loop")},
    //         Some((_pin,_)) => {
    //             println!("pinn!")
    //         }
    //     }
    // }
}
