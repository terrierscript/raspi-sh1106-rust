  
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

#![no_std]
#![no_main]

extern crate linux_embedded_hal as hal;
// extern crate embedded_graphics;
extern crate sh1106;

use hal::I2cdev;
// use embedded_graphics::prelude::*;
// use embedded_graphics::primitives::{Line};
use sh1106::prelude::*;
use sh1106::Builder;

fn main() {

    let i2c = I2cdev::new("/dev/i2c-1").unwrap();

    let b = Builder::new();
    // let mut disp: GraphicsMode<
    //     // I2cInterface<hal::I2cdev>
    //     _
    // > = Builder::new()
    // .connect_i2c(i2c).into();

    // disp.init().unwrap();
    // disp.flush().unwrap();
    
    // disp.set_pixel(10, 10, 1);
    // disp.set_pixel(10, 11, 1);
    // disp.set_pixel(10, 12, 1);

    // disp.flush().unwrap();
}

// #[exception]
// fn HardFault(ef: &ExceptionFrame) -> ! {
//     panic!("{:#?}", ef);
// }