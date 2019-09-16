extern crate rppal;
extern crate sh1106;

use rppal::{gpio, spi};
use rppal::gpio::{Gpio, Level, InputPin,Trigger};
use rppal::spi::{Spi, Bus, SlaveSelect};
use spidev_sh1106::SpidevSH1106;

use sh1106::prelude::*;
use sh1106::Builder;

// use std::time::Duration;

// fn pin<'a>(gpio: &'a Gpio, pin_id: &'a i32) -> &'a InputPin {
//     return 
// }

fn main() -> ! {
    let sd = SpidevSH1106::new();
    let dc = SpidevSH1106::dc_pin();
    let mut disp: GraphicsMode<_> = Builder::new().connect_spi(sd.spidev, dc).into();
    
    SpidevSH1106::reset(&mut disp).expect("cannot reset");
    
    println!("Hello, world!!!!");
    let  gpio = Gpio::new().expect( "Failed Gpio::new" );
    println!("got gpio");
    
    let mut b1 : InputPin = gpio.get(21).expect("get 21 pin").into_input_pullup();
    b1.set_interrupt(Trigger::Both).expect("failed set interrupt");
    // b1.poll_interrupt()
    loop {
        // println!("loop");
        let s = b1.poll_interrupt(false, None).expect("v");
        println!("{:?}", s);
        // println!("{:?}", );
        match s {
            Some(n) => {
                println!("N")
            },
            None => {
                println!("none")
            }
        }
        // let intr = gpio.poll_interrupts(
        //     &[&b1],
        //     true, 
        //     Some(Duration::new(1,0))
        // ).expect("invalid intr");
        // match intr {
        //     None => { println!("loop")},
        //     Some((_pin,_)) => {
        //         println!("pinn!")
        //     }
        // }

        // let z = b1.read();
        // println!("loop {}", z);
    }
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
