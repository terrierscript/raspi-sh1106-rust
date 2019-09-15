//! Print "Hello world!" with "Hello rust!" underneath. Uses the `embedded_graphics` crate to draw
//! the text with a 6x8 pixel font.
//!
//! This example is for the STM32F103 "Blue Pill" board using I2C1.
//!
//! Wiring connections are as follows for a CRIUS-branded display:
//!
//! ```
//!      Display -> Blue Pill
//! (black)  GND -> GND
//! (red)    +5V -> VCC
//! (yellow) SDA -> PB9
//! (green)  SCL -> PB8
//! ```
//!
//! Run on a Blue Pill with `cargo run --example text`.

// #![no_std]
// #![no_main]

extern crate linux_embedded_hal as hal;
extern crate embedded_graphics;
extern crate sh1106;

// use hal::I2cdev;
use hal::Spidev;
use hal::Pin;

use embedded_graphics::prelude::*;
// use embedded_graphics::primitives::{Line};
use sh1106::prelude::*;
use sh1106::Builder;

// #![main]
fn main() {
    println!("start");
    // let i2c = I2cdev::new("/dev/i2c-1").unwrap();
    let spi = Spidev::open("/dev/spidev0.0").unwrap_or_else(|e| -> {
        println!("cannnot open");
        println!("{:?}",e);
    });

    // let b = Builder::new();
    let dc = Pin::new(24);
    let mut disp: GraphicsMode<_> = Builder::new().connect_spi(spi, dc).into();

    disp.init().unwrap_or_else(|e| {
        println!("not initialized");
        println!("{:?}", e);
    });
    disp.flush().unwrap_or_else(|e| {
        println!("not flush");
        println!("{:?}", e);
    });
    
    disp.set_pixel(10, 10, 1);
    disp.set_pixel(10, 11, 1);
    disp.set_pixel(10, 12, 1);

    println!("end");
    disp.flush().unwrap();
}

// #[exception]
// fn HardFault(ef: &ExceptionFrame) -> ! {
//     panic!("{:#?}", ef);
// }

// #[panic_handler]
// fn my_panic(_info: &core::panic::PanicInfo) -> ! {
//     loop {}
// }
