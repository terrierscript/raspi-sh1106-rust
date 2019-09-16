extern crate rppal;
use rppal::{gpio, spi};
use rppal::gpio::{Gpio, Level, InputPin};
use rppal::spi::{Spi, Bus, SlaveSelect};

use std::time::Duration;

// fn pin<'a>(gpio: &'a Gpio, pin_id: &'a i32) -> &'a InputPin {
//     return 
// }

fn main() -> ! {
    println!("Hello, world!!!!");
    let mut gpio = Gpio::new().expect( "Failed Gpio::new" );
    println!("got gpio");
    
    let pin_ids = &[6,19,5,26,13,21,20,16];
    let pins: Vec<_> = pin_ids.iter()
        .map(|pid| gpio
            .get(*pid)
            .expect("Cannot pin")
            .into_input_pullup()
        )
        .collect();

    let ps = pins.as_slice();
    println!("loop start");
    loop{
        let intr = gpio.poll_interrupts(
            // pins,
            &[
                &pins[0],
                &pins[1],
                &pins[2],
            ],
            true, 
            Some(Duration::new(1,0))
        ).expect("invalid intr");
        match intr {
            None => { println!("loop")},
            Some((_pin,_)) => {
                println!("pinn!")
            }
        }
    }
}
